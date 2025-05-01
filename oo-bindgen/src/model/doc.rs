//! Documentation
//!
//! Documentation is split in two; the `Doc` contains the mandatory brief description and optional details paragraph.
//! The `DocString` represents a single paragraph of text. It can parse references and create hyperlinks in the
//! generated doc. All the references are checked when creating the library and an error will be returned if a
//! reference cannot be resolved.
//!
//! When a `Doc` is needed, the user can provide a string and it will automatically be parsed and interpreted as
//! just a brief description. If additional details are needed, then the user may use the `doc()` function to
//! build a more complex documentation. The `doc()` functions takes the brief string as a parameter, then can be
//! chained with `details()` to add details paragraph or with `warning()` to add a warning paragraph.
//!
//! For parameters, only a `DocString` is accepted, because some generators do not support paragraphs for parameters.
//!
//! ### References
//!
//! In any `DocString`, you can put references that will print a hyperlink in the generated doc. You simply put
//! the reference between curly braces and the `DocString` parser will resolve them. All the names used must match
//! exactly what is specified __in the schema__, each generator takes care of the renaming the type for the target
//! language.
//!
//! Here are all the type of links available:
//! - `{param:MyParam}`: references the parameter `MyParam` of a method. __This is only valid within a method documentation__.
//!   Not all documentation generator supports hyperlinking this, so `code font` is used instead.
//! - `{class:MyClass}`: references the class `MyClass`.
//! - `{class:MyClass.foo()}`: references the method `foo()` of `MyClass`. Can be a static, non-static, or async method.
//!   No need to put parameters.
//! - `{class:MyClass.[constructor]}`: references `MyClass`'s constructor.
//! - `{class:MyClass.[destructor]}`: references `MyClass`'s destructor (the `Dispose()` method in C#, the `close()` method in Java).
//! - `{struct:MyStruct}`: references the structure `MyStruct`.
//! - `{struct:MyStruct.foo}`: references the `foo` element inside `MyStruct`.
//! - `{struct:MyStruct.foo()}`: references the `foo()` method of `MyStruct`. Can be a static or non-static method. No need to put parameters.
//! - `{enum:MyEnum}`: references the enum `MyEnum`.
//! - `{enum:MyEnum.foo}`: references the `foo` variant of `MyEnum`.
//! - `{interface:MyInterface}`: references the interface `MyInterface`.
//! - `{interface:MyInterface.foo()}`: references the `foo()` callback of `MyInterface`. This cannot reference the `on_destroy` callback.
//!
//! There other miscellaneous tag that can be used:
//! - `{null}`: prints `NULL` in C, or `null` in C# and Java.
//! - `{iterator}`: prints `iterator` in C, or `collection` in C# and Java.

use std::convert::TryFrom;
use std::fmt::Debug;

use lazy_static::lazy_static;
use regex::Regex;

use crate::model::*;

pub trait DocReference: Debug + Clone {}

pub fn doc<D: Into<DocString<Unvalidated>>>(brief: D) -> Doc<Unvalidated> {
    Doc {
        brief: brief.into(),
        details: Vec::new(),
    }
}

pub fn text(text: &str) -> DocString<Validated> {
    DocString {
        elements: vec![DocStringElement::Text(text.to_string())],
    }
}

pub fn brief(text: &str) -> Doc<Validated> {
    Doc {
        brief: DocString {
            elements: vec![DocStringElement::Text(text.to_string())],
        },
        details: Vec::new(),
    }
}

#[derive(Debug, Clone)]
pub struct Doc<T>
where
    T: DocReference,
{
    pub(crate) brief: DocString<T>,
    pub(crate) details: Vec<DocParagraph<T>>,
}

impl Doc<Validated> {
    #[must_use]
    pub fn warning(mut self, warning: &str) -> Self {
        self.details.push(DocParagraph::Warning(text(warning)));
        self
    }
}

impl Doc<Unvalidated> {
    pub(crate) fn validate(
        &self,
        symbol_name: &Name,
        lib: &LibraryFields,
    ) -> BindResult<Doc<Validated>> {
        self.validate_with_args(symbol_name, lib, None)
    }

    pub(crate) fn validate_with_args(
        &self,
        symbol_name: &Name,
        lib: &LibraryFields,
        args: Option<&[Name]>,
    ) -> BindResult<Doc<Validated>> {
        let details: BindResult<Vec<DocParagraph<Validated>>> = self
            .details
            .iter()
            .map(|x| x.validate_with_args(symbol_name, lib, args))
            .collect();
        Ok(Doc {
            brief: self.brief.validate_with_args(symbol_name, lib, args)?,
            details: details?,
        })
    }

    #[must_use]
    pub fn warning<D: Into<DocString<Unvalidated>>>(mut self, warning: D) -> Self {
        self.details.push(DocParagraph::Warning(warning.into()));
        self
    }

    #[must_use]
    pub fn details<D: Into<DocString<Unvalidated>>>(mut self, details: D) -> Self {
        self.details.push(DocParagraph::Details(details.into()));
        self
    }
}

impl<T: AsRef<str>> From<T> for Doc<Unvalidated> {
    fn from(from: T) -> Self {
        doc(from)
    }
}

/// Used in builders
pub(crate) struct OptionalDoc {
    parent_name: Name,
    inner: Option<Doc<Unvalidated>>,
}

impl OptionalDoc {
    pub(crate) fn new(parent_name: Name) -> Self {
        Self {
            parent_name,
            inner: None,
        }
    }

    pub(crate) fn set(&mut self, doc: Doc<Unvalidated>) -> BindResult<()> {
        match self.inner {
            None => {
                self.inner = Some(doc);
                Ok(())
            }
            Some(_) => Err(BindingErrorVariant::DocAlreadyDefined {
                symbol_name: self.parent_name.clone(),
            }
            .into()),
        }
    }

    pub(crate) fn extract(self) -> BindResult<Doc<Unvalidated>> {
        match self.inner {
            Some(doc) => Ok(doc),
            None => Err(BindingErrorVariant::DocNotDefined {
                symbol_name: self.parent_name.clone(),
            }
            .into()),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum DocParagraph<T>
where
    T: DocReference,
{
    Details(DocString<T>),
    Warning(DocString<T>),
}

impl DocParagraph<Unvalidated> {
    fn validate_with_args(
        &self,
        symbol_name: &Name,
        lib: &LibraryFields,
        args: Option<&[Name]>,
    ) -> BindResult<DocParagraph<Validated>> {
        Ok(match self {
            DocParagraph::Details(x) => {
                DocParagraph::Details(x.validate_with_args(symbol_name, lib, args)?)
            }
            DocParagraph::Warning(x) => {
                DocParagraph::Warning(x.validate_with_args(symbol_name, lib, args)?)
            }
        })
    }
}

#[derive(Debug, Clone)]
pub struct DocString<T>
where
    T: DocReference,
{
    elements: Vec<DocStringElement<T>>,
}

impl DocString<Unvalidated> {
    pub(crate) fn validate(
        &self,
        symbol_name: &Name,
        lib: &LibraryFields,
    ) -> BindResult<DocString<Validated>> {
        self.validate_with_args(symbol_name, lib, None)
    }

    pub(crate) fn validate_with_args(
        &self,
        symbol_name: &Name,
        lib: &LibraryFields,
        args: Option<&[Name]>,
    ) -> BindResult<DocString<Validated>> {
        let elements: BindResult<Vec<DocStringElement<Validated>>> = self
            .elements
            .iter()
            .map(|x| x.validate(symbol_name, lib, args))
            .collect();
        Ok(DocString {
            elements: elements?,
        })
    }
}

impl<T> DocString<T>
where
    T: DocReference,
{
    pub(crate) fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub(crate) fn push(&mut self, element: DocStringElement<T>) {
        self.elements.push(element);
    }

    pub(crate) fn elements(&self) -> impl Iterator<Item = &DocStringElement<T>> {
        self.elements.iter()
    }
}

impl<T> Default for DocString<T>
where
    T: DocReference,
{
    fn default() -> Self {
        DocString::new()
    }
}

impl<U: AsRef<str>> From<U> for DocString<Unvalidated> {
    fn from(from: U) -> DocString<Unvalidated> {
        let mut from = from.as_ref();
        let mut result = DocString::new();
        while let Some(start_idx) = from.find('{') {
            let (before_str, current_str) = from.split_at(start_idx);
            if let Some(end_idx) = current_str.find('}') {
                let (element_str, current_str) = current_str.split_at(end_idx + 1);
                let element = DocStringElement::try_from(element_str)
                    .expect("Invalid docstring: ill-formatted docstring element");

                if !before_str.is_empty() {
                    result.push(DocStringElement::Text(before_str.to_owned()));
                }
                result.push(element);
                from = current_str;
            } else {
                panic!("Invalid docstring: no end bracket");
            }
        }

        // Add remaining string
        if !from.is_empty() {
            result.push(DocStringElement::Text(from.to_owned()));
        }

        result
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum DocStringElement<T>
where
    T: DocReference,
{
    Text(String),
    Null,
    Iterator,
    Reference(T),
}

impl DocStringElement<Unvalidated> {
    pub(crate) fn validate(
        &self,
        symbol_name: &Name,
        lib: &LibraryFields,
        args: Option<&[Name]>,
    ) -> BindResult<DocStringElement<Validated>> {
        Ok(match self {
            DocStringElement::Text(x) => DocStringElement::Text(x.clone()),
            DocStringElement::Null => DocStringElement::Null,
            DocStringElement::Iterator => DocStringElement::Iterator,
            DocStringElement::Reference(x) => {
                DocStringElement::Reference(x.validate(symbol_name, lib, args)?)
            }
        })
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Unvalidated {
    /// Reference to an argument
    /// Can only be used within the context of a function or callback function
    Argument(String),
    /// Reference a class
    Class(String),
    /// Reference a class method
    ///
    /// First string is the class name, second is the method's name
    ClassMethod(String, String),
    /// Reference to the class constructor
    ClassConstructor(String),
    /// Reference to the class destructor
    ClassDestructor(String),
    /// Reference a struct
    Struct(String),
    /// Reference a field within a struct
    ///
    /// First string is the struct name, second is the field name
    StructField(String, String),
    /// Reference an enum
    Enum(String),
    /// Reference an enum variant
    ///
    /// First string is the enum name, second is the enum variant name
    EnumVariant(String, String),
    /// Reference an interface
    Interface(String),
    /// Reference a method of a interface
    ///
    /// First string is the interface name, second is the method's name
    InterfaceMethod(String, String),
}

impl DocReference for Unvalidated {}

impl Unvalidated {
    pub(crate) fn validate(
        &self,
        symbol_name: &Name,
        lib: &LibraryFields,
        args: Option<&[Name]>,
    ) -> BindResult<Validated> {
        match self {
            Self::Argument(name) => {
                let args = match args {
                    Some(args) => args,
                    None => {
                        return Err(BindingErrorVariant::DocInvalidArgumentContext {
                            symbol_name: symbol_name.to_string(),
                            ref_name: name.to_string(),
                        }
                        .into())
                    }
                };

                match args.iter().find(|arg| arg.as_ref() == name) {
                    Some(arg) => Ok(Validated::Argument(arg.clone())),
                    None => Err(BindingErrorVariant::DocInvalidReference {
                        symbol_name: symbol_name.to_string(),
                        ref_name: name.to_string(),
                    }
                    .into()),
                }
            }
            Self::Class(name) => match lib.find_class_declaration(name) {
                None => Err(BindingErrorVariant::DocInvalidReference {
                    symbol_name: symbol_name.to_string(),
                    ref_name: name.to_string(),
                }
                .into()),
                Some(x) => Ok(Validated::Class(x.clone())),
            },
            Self::ClassMethod(class_name, method_name) => {
                match lib
                    .find_class(class_name)
                    .and_then(|class| class.find_method(method_name).map(|func| (class, func)))
                {
                    None => Err(BindingErrorVariant::DocInvalidReference {
                        symbol_name: symbol_name.to_string(),
                        ref_name: format!("{class_name}.{method_name}()"),
                    }
                    .into()),
                    Some((class, (name, function))) => {
                        Ok(Validated::ClassMethod(class.clone(), name, function))
                    }
                }
            }
            Self::ClassConstructor(class_name) => {
                match lib
                    .find_class(class_name)
                    .and_then(|x| x.constructor.clone().map(|d| (x.clone(), d)))
                {
                    None => Err(BindingErrorVariant::DocInvalidReference {
                        symbol_name: symbol_name.to_string(),
                        ref_name: format!("{class_name}.[constructor]",),
                    }
                    .into()),
                    Some((class, constructor)) => {
                        Ok(Validated::ClassConstructor(class, constructor))
                    }
                }
            }
            Self::ClassDestructor(class_name) => {
                match lib
                    .find_class(class_name)
                    .and_then(|x| x.destructor.clone().map(|d| (x, d)))
                {
                    None => Err(BindingErrorVariant::DocInvalidReference {
                        symbol_name: symbol_name.to_string(),
                        ref_name: format!("{class_name}.[destructor]"),
                    }
                    .into()),
                    Some((class, destructor)) => {
                        Ok(Validated::ClassDestructor(class.clone(), destructor))
                    }
                }
            }
            Self::Struct(struct_name) => match lib.find_struct(struct_name) {
                None => Err(BindingErrorVariant::DocInvalidReference {
                    symbol_name: symbol_name.to_string(),
                    ref_name: struct_name.to_string(),
                }
                .into()),
                Some(x) => Ok(Validated::Struct(x.clone())),
            },
            Self::StructField(struct_name, field_name) => {
                match lib
                    .find_struct(struct_name)
                    .and_then(|st| st.find_field_name(field_name).map(|n| (st, n)))
                {
                    None => Err(BindingErrorVariant::DocInvalidReference {
                        symbol_name: symbol_name.to_string(),
                        ref_name: format!("{struct_name}.{field_name}"),
                    }
                    .into()),
                    Some((st, name)) => Ok(Validated::StructField(st.clone(), name)),
                }
            }
            Self::Enum(enum_name) => match lib.find_enum(enum_name) {
                None => Err(BindingErrorVariant::DocInvalidReference {
                    symbol_name: symbol_name.to_string(),
                    ref_name: enum_name.to_string(),
                }
                .into()),
                Some(x) => Ok(Validated::Enum(x.clone())),
            },
            Self::EnumVariant(enum_name, variant_name) => {
                match lib.find_enum(enum_name).and_then(|e| {
                    e.find_variant_by_name(variant_name)
                        .map(|v| (e, v.name.clone()))
                }) {
                    None => Err(BindingErrorVariant::DocInvalidReference {
                        symbol_name: symbol_name.to_string(),
                        ref_name: format!("{enum_name}.{variant_name}"),
                    }
                    .into()),
                    Some((e, v)) => Ok(Validated::EnumVariant(e.clone(), v)),
                }
            }
            Self::Interface(interface_name) => match lib.find_interface(interface_name) {
                None => Err(BindingErrorVariant::DocInvalidReference {
                    symbol_name: symbol_name.to_string(),
                    ref_name: interface_name.to_string(),
                }
                .into()),
                Some(x) => Ok(Validated::Interface(x.clone())),
            },
            Self::InterfaceMethod(interface_name, method_name) => {
                match lib
                    .find_interface(interface_name)
                    .and_then(|i| i.find_callback(method_name).map(|m| (i, m)))
                {
                    None => Err(BindingErrorVariant::DocInvalidReference {
                        symbol_name: symbol_name.to_string(),
                        ref_name: format!("{interface_name}.{method_name}()"),
                    }
                    .into()),
                    Some((i, m)) => Ok(Validated::InterfaceMethod(i.clone(), m.name.clone())),
                }
            }
        }
    }
}

/// Validated doc reference
#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum Validated {
    /// Reference to an argument
    /// can only be used in docs for functions or callback methods
    Argument(Name),
    /// Reference a class
    Class(ClassDeclarationHandle),
    /// Reference a class method
    ClassMethod(
        Handle<Class<Unvalidated>>,
        Name,
        Handle<Function<Unvalidated>>,
    ),
    /// Reference to the class constructor
    ClassConstructor(Handle<Class<Unvalidated>>, ClassConstructor<Unvalidated>),
    /// Reference to the class destructor
    ClassDestructor(Handle<Class<Unvalidated>>, ClassDestructor<Unvalidated>),
    /// Reference a struct
    Struct(StructType<Unvalidated>),
    /// Reference a field within a struct
    ///
    /// Second parameter is the field name inside that struct
    StructField(StructType<Unvalidated>, Name),
    /// Reference an enum
    Enum(Handle<Enum<Unvalidated>>),
    /// Reference an enum variant
    EnumVariant(Handle<Enum<Unvalidated>>, Name),
    /// Reference an interface
    Interface(Handle<Interface<Unvalidated>>),
    /// Reference a method of a interface
    InterfaceMethod(Handle<Interface<Unvalidated>>, Name),
}

impl DocReference for Validated {}

impl TryFrom<&str> for DocStringElement<Unvalidated> {
    type Error = BindingError;

    fn try_from(from: &str) -> Result<DocStringElement<Unvalidated>, BindingError> {
        lazy_static! {
            static ref RE_PARAM: Regex = Regex::new(r"\{param:([[:word:]]+)\}").unwrap();
            static ref RE_CLASS: Regex = Regex::new(r"\{class:([[:word:]]+)\}").unwrap();
            static ref RE_CLASS_METHOD: Regex =
                Regex::new(r"\{class:([[:word:]]+)\.([[:word:]]+)\(\)\}").unwrap();
            static ref RE_CLASS_CONSTRUCTOR: Regex =
                Regex::new(r"\{class:([[:word:]]+)\.\[constructor\]\}").unwrap();
            static ref RE_CLASS_DESTRUCTOR: Regex =
                Regex::new(r"\{class:([[:word:]]+)\.\[destructor\]\}").unwrap();
            static ref RE_STRUCT: Regex = Regex::new(r"\{struct:([[:word:]]+)\}").unwrap();
            static ref RE_STRUCT_ELEMENT: Regex =
                Regex::new(r"\{struct:([[:word:]]+)\.([[:word:]]+)\}").unwrap();
            static ref RE_STRUCT_METHOD: Regex =
                Regex::new(r"\{struct:([[:word:]]+)\.([[:word:]]+)\(\)\}").unwrap();
            static ref RE_ENUM: Regex = Regex::new(r"\{enum:([[:word:]]+)\}").unwrap();
            static ref RE_ENUM_VARIANT: Regex =
                Regex::new(r"\{enum:([[:word:]]+)\.([[:word:]]+)\}").unwrap();
            static ref RE_INTERFACE: Regex = Regex::new(r"\{interface:([[:word:]]+)\}").unwrap();
            static ref RE_INTERFACE_METHOD: Regex =
                Regex::new(r"\{interface:([[:word:]]+)\.([[:word:]]+)\(\)\}").unwrap();
        }

        fn try_get_regex(from: &str) -> Option<Unvalidated> {
            if let Some(capture) = RE_PARAM.captures(from) {
                return Some(Unvalidated::Argument(
                    capture.get(1).unwrap().as_str().to_owned(),
                ));
            }
            if let Some(capture) = RE_CLASS.captures(from) {
                return Some(Unvalidated::Class(
                    capture.get(1).unwrap().as_str().to_owned(),
                ));
            }
            if let Some(capture) = RE_CLASS_METHOD.captures(from) {
                return Some(Unvalidated::ClassMethod(
                    capture.get(1).unwrap().as_str().to_owned(),
                    capture.get(2).unwrap().as_str().to_owned(),
                ));
            }
            if let Some(capture) = RE_CLASS_CONSTRUCTOR.captures(from) {
                return Some(Unvalidated::ClassConstructor(
                    capture.get(1).unwrap().as_str().to_owned(),
                ));
            }
            if let Some(capture) = RE_CLASS_DESTRUCTOR.captures(from) {
                return Some(Unvalidated::ClassDestructor(
                    capture.get(1).unwrap().as_str().to_owned(),
                ));
            }
            if let Some(capture) = RE_STRUCT.captures(from) {
                return Some(Unvalidated::Struct(
                    capture.get(1).unwrap().as_str().to_owned(),
                ));
            }
            if let Some(capture) = RE_STRUCT_ELEMENT.captures(from) {
                return Some(Unvalidated::StructField(
                    capture.get(1).unwrap().as_str().to_owned(),
                    capture.get(2).unwrap().as_str().to_owned(),
                ));
            }
            if let Some(capture) = RE_ENUM.captures(from) {
                return Some(Unvalidated::Enum(
                    capture.get(1).unwrap().as_str().to_owned(),
                ));
            }
            if let Some(capture) = RE_ENUM_VARIANT.captures(from) {
                return Some(Unvalidated::EnumVariant(
                    capture.get(1).unwrap().as_str().to_owned(),
                    capture.get(2).unwrap().as_str().to_owned(),
                ));
            }
            if let Some(capture) = RE_INTERFACE.captures(from) {
                return Some(Unvalidated::Interface(
                    capture.get(1).unwrap().as_str().to_owned(),
                ));
            }
            if let Some(capture) = RE_INTERFACE_METHOD.captures(from) {
                return Some(Unvalidated::InterfaceMethod(
                    capture.get(1).unwrap().as_str().to_owned(),
                    capture.get(2).unwrap().as_str().to_owned(),
                ));
            }

            None
        }

        if let Some(x) = try_get_regex(from) {
            return Ok(DocStringElement::Reference(x));
        }

        if from == "{null}" {
            return Ok(DocStringElement::Null);
        }
        if from == "{iterator}" {
            return Ok(DocStringElement::Iterator);
        }

        Err(BindingErrorVariant::InvalidDocString.into())
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use super::*;

    #[test]
    fn parse_param_reference() {
        let doc: DocString<Unvalidated> = "This is a {param:foo} test.".into();
        assert_eq!(
            [
                DocStringElement::Text("This is a ".to_owned()),
                DocStringElement::Reference(Unvalidated::Argument("foo".to_owned())),
                DocStringElement::Text(" test.".to_owned()),
            ]
            .as_ref(),
            doc.elements.as_slice()
        );
    }

    #[test]
    fn parse_class_reference() {
        let doc: DocString<Unvalidated> = "This is a {class:MyClass} test.".into();
        assert_eq!(
            [
                DocStringElement::Text("This is a ".to_owned()),
                DocStringElement::Reference(Unvalidated::Class("MyClass".to_owned())),
                DocStringElement::Text(" test.".to_owned()),
            ]
            .as_ref(),
            doc.elements.as_slice()
        );
    }

    #[test]
    fn parse_class_reference_at_the_end() {
        let doc: DocString<Unvalidated> = "This is a test {class:MyClass2}".into();
        assert_eq!(
            [
                DocStringElement::Text("This is a test ".to_owned()),
                DocStringElement::Reference(Unvalidated::Class("MyClass2".to_owned())),
            ]
            .as_ref(),
            doc.elements.as_slice()
        );
    }

    #[test]
    fn parse_class_method() {
        let doc: DocString<Unvalidated> = "This is a {class:MyClass.do_something()} method."
            .into();
        assert_eq!(
            [
                DocStringElement::Text("This is a ".to_owned()),
                DocStringElement::Reference(Unvalidated::ClassMethod(
                    "MyClass".to_owned(),
                    "do_something".to_owned()
                )),
                DocStringElement::Text(" method.".to_owned()),
            ]
            .as_ref(),
            doc.elements.as_slice()
        );
    }

    #[test]
    fn parse_struct() {
        let doc: DocString<Unvalidated> = "This is a {struct:MyStruct} struct.".into();
        assert_eq!(
            [
                DocStringElement::Text("This is a ".to_owned()),
                DocStringElement::Reference(Unvalidated::Struct("MyStruct".to_owned())),
                DocStringElement::Text(" struct.".to_owned()),
            ]
            .as_ref(),
            doc.elements.as_slice()
        );
    }

    #[test]
    fn parse_struct_element() {
        let doc: DocString<Unvalidated> = "This is a {struct:MyStruct.foo} struct element."
            .into();
        assert_eq!(
            [
                DocStringElement::Text("This is a ".to_owned()),
                DocStringElement::Reference(Unvalidated::StructField(
                    "MyStruct".to_owned(),
                    "foo".to_owned()
                )),
                DocStringElement::Text(" struct element.".to_owned()),
            ]
            .as_ref(),
            doc.elements.as_slice()
        );
    }

    #[test]
    fn parse_enum() {
        let doc: DocString<Unvalidated> = "This is a {enum:MyEnum} enum.".into();
        assert_eq!(
            [
                DocStringElement::Text("This is a ".to_owned()),
                DocStringElement::Reference(Unvalidated::Enum("MyEnum".to_owned())),
                DocStringElement::Text(" enum.".to_owned()),
            ]
            .as_ref(),
            doc.elements.as_slice()
        );
    }

    #[test]
    fn parse_enum_element() {
        let doc: DocString<Unvalidated> = "This is a {enum:MyEnum.foo} enum variant."
            .into();
        assert_eq!(
            [
                DocStringElement::Text("This is a ".to_owned()),
                DocStringElement::Reference(Unvalidated::EnumVariant(
                    "MyEnum".to_owned(),
                    "foo".to_owned()
                )),
                DocStringElement::Text(" enum variant.".to_owned()),
            ]
            .as_ref(),
            doc.elements.as_slice()
        );
    }

    #[test]
    fn parse_interface() {
        let doc: DocString<Unvalidated> = "This is a {interface:Interface} interface."
            .into();
        assert_eq!(
            [
                DocStringElement::Text("This is a ".to_owned()),
                DocStringElement::Reference(Unvalidated::Interface("Interface".to_owned())),
                DocStringElement::Text(" interface.".to_owned()),
            ]
            .as_ref(),
            doc.elements.as_slice()
        );
    }

    #[test]
    fn parse_interface_method() {
        let doc: DocString<Unvalidated> = "This is a {interface:Interface.foo()} interface method."
            .into();
        assert_eq!(
            [
                DocStringElement::Text("This is a ".to_owned()),
                DocStringElement::Reference(Unvalidated::InterfaceMethod(
                    "Interface".to_owned(),
                    "foo".to_owned()
                )),
                DocStringElement::Text(" interface method.".to_owned()),
            ]
            .as_ref(),
            doc.elements.as_slice()
        );
    }

    #[test]
    fn parse_null() {
        let doc: DocString<Unvalidated> = "This is a {null} null.".into();
        assert_eq!(
            [
                DocStringElement::Text("This is a ".to_owned()),
                DocStringElement::Null,
                DocStringElement::Text(" null.".to_owned()),
            ]
            .as_ref(),
            doc.elements.as_slice()
        );
    }

    #[test]
    fn parse_iterator() {
        let doc: DocString<Unvalidated> = "This is a {iterator} iterator.".into();
        assert_eq!(
            [
                DocStringElement::Text("This is a ".to_owned()),
                DocStringElement::Iterator,
                DocStringElement::Text(" iterator.".to_owned()),
            ]
            .as_ref(),
            doc.elements.as_slice()
        );
    }

    #[test]
    fn parse_from_owned_string() {
        doc(format!("{{null}} this is a {}", "test"));
    }
}
