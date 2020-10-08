use crate::callback::*;
use crate::native_function::Parameter;
use crate::{BindingError, Library};
use lazy_static::lazy_static;
use regex::Regex;
use std::convert::TryFrom;

pub fn doc<D: Into<DocString>>(brief: D) -> Doc {
    Doc {
        brief: brief.into(),
        details: Vec::new(),
    }
}

#[derive(Debug, Clone)]
pub struct Doc {
    pub brief: DocString,
    pub details: Vec<DocParagraph>,
}

impl Doc {
    pub fn details<D: Into<DocString>>(mut self, details: D) -> Self {
        self.details.push(DocParagraph::Details(details.into()));
        self
    }

    pub fn warning<D: Into<DocString>>(mut self, warning: D) -> Self {
        self.details.push(DocParagraph::Warning(warning.into()));
        self
    }

    fn references(&self) -> impl Iterator<Item = &DocReference> {
        self.brief
            .references()
            .chain(self.details.iter().flat_map(|para| para.references()))
    }
}

impl From<&str> for Doc {
    fn from(from: &str) -> Self {
        doc(from)
    }
}

#[derive(Debug, Clone)]
pub enum DocParagraph {
    Details(DocString),
    Warning(DocString),
}

impl DocParagraph {
    fn references(&self) -> impl Iterator<Item = &DocReference> {
        match self {
            Self::Details(string) => string.references(),
            Self::Warning(string) => string.references(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DocString {
    elements: Vec<DocStringElement>,
}

impl DocString {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn push(&mut self, element: DocStringElement) {
        self.elements.push(element);
    }

    pub fn elements(&self) -> impl Iterator<Item = &DocStringElement> {
        self.elements.iter()
    }

    fn references(&self) -> impl Iterator<Item = &DocReference> {
        self.elements.iter().filter_map(|el| {
            if let DocStringElement::Reference(reference) = el {
                Some(reference)
            } else {
                None
            }
        })
    }
}

impl Default for DocString {
    fn default() -> Self {
        DocString::new()
    }
}

impl From<&str> for DocString {
    fn from(mut from: &str) -> DocString {
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

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum DocStringElement {
    Text(String),
    Null,
    Iterator,
    Reference(DocReference),
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum DocReference {
    /// Reference to a parameter
    Param(String),
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
    /// Reference an element in a struct
    ///
    /// First string is the struct name, second is the element name inside that struct
    StructElement(String, String),
    /// Reference a method of a struct
    ///
    /// First string is the struct name, second is the method's name
    StructMethod(String, String),
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
    /// Reference an OneTimeCallback
    OneTimeCallback(String),
    /// Reference a method of a OneTimeCallback
    ///
    /// First string is the OneTimeCallback name, second is the method's name
    OneTimeCallbackMethod(String, String),
}

impl TryFrom<&str> for DocStringElement {
    type Error = BindingError;

    fn try_from(from: &str) -> Result<DocStringElement, BindingError> {
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
            static ref RE_ONETIME_CALLBACK: Regex =
                Regex::new(r"\{callback:([[:word:]]+)\}").unwrap();
            static ref RE_ONETIME_CALLBACK_METHOD: Regex =
                Regex::new(r"\{callback:([[:word:]]+)\.([[:word:]]+)\(\)\}").unwrap();
        }

        if let Some(capture) = RE_PARAM.captures(from) {
            return Ok(DocStringElement::Reference(DocReference::Param(
                capture.get(1).unwrap().as_str().to_owned(),
            )));
        }
        if let Some(capture) = RE_CLASS.captures(from) {
            return Ok(DocStringElement::Reference(DocReference::Class(
                capture.get(1).unwrap().as_str().to_owned(),
            )));
        }
        if let Some(capture) = RE_CLASS_METHOD.captures(from) {
            return Ok(DocStringElement::Reference(DocReference::ClassMethod(
                capture.get(1).unwrap().as_str().to_owned(),
                capture.get(2).unwrap().as_str().to_owned(),
            )));
        }
        if let Some(capture) = RE_CLASS_CONSTRUCTOR.captures(from) {
            return Ok(DocStringElement::Reference(DocReference::ClassConstructor(
                capture.get(1).unwrap().as_str().to_owned(),
            )));
        }
        if let Some(capture) = RE_CLASS_DESTRUCTOR.captures(from) {
            return Ok(DocStringElement::Reference(DocReference::ClassDestructor(
                capture.get(1).unwrap().as_str().to_owned(),
            )));
        }
        if let Some(capture) = RE_STRUCT.captures(from) {
            return Ok(DocStringElement::Reference(DocReference::Struct(
                capture.get(1).unwrap().as_str().to_owned(),
            )));
        }
        if let Some(capture) = RE_STRUCT_ELEMENT.captures(from) {
            return Ok(DocStringElement::Reference(DocReference::StructElement(
                capture.get(1).unwrap().as_str().to_owned(),
                capture.get(2).unwrap().as_str().to_owned(),
            )));
        }
        if let Some(capture) = RE_STRUCT_METHOD.captures(from) {
            return Ok(DocStringElement::Reference(DocReference::StructMethod(
                capture.get(1).unwrap().as_str().to_owned(),
                capture.get(2).unwrap().as_str().to_owned(),
            )));
        }
        if let Some(capture) = RE_ENUM.captures(from) {
            return Ok(DocStringElement::Reference(DocReference::Enum(
                capture.get(1).unwrap().as_str().to_owned(),
            )));
        }
        if let Some(capture) = RE_ENUM_VARIANT.captures(from) {
            return Ok(DocStringElement::Reference(DocReference::EnumVariant(
                capture.get(1).unwrap().as_str().to_owned(),
                capture.get(2).unwrap().as_str().to_owned(),
            )));
        }
        if let Some(capture) = RE_INTERFACE.captures(from) {
            return Ok(DocStringElement::Reference(DocReference::Interface(
                capture.get(1).unwrap().as_str().to_owned(),
            )));
        }
        if let Some(capture) = RE_INTERFACE_METHOD.captures(from) {
            return Ok(DocStringElement::Reference(DocReference::InterfaceMethod(
                capture.get(1).unwrap().as_str().to_owned(),
                capture.get(2).unwrap().as_str().to_owned(),
            )));
        }
        if let Some(capture) = RE_ONETIME_CALLBACK.captures(from) {
            return Ok(DocStringElement::Reference(DocReference::OneTimeCallback(
                capture.get(1).unwrap().as_str().to_owned(),
            )));
        }
        if let Some(capture) = RE_ONETIME_CALLBACK_METHOD.captures(from) {
            return Ok(DocStringElement::Reference(
                DocReference::OneTimeCallbackMethod(
                    capture.get(1).unwrap().as_str().to_owned(),
                    capture.get(2).unwrap().as_str().to_owned(),
                ),
            ));
        }
        if from == "{null}" {
            return Ok(DocStringElement::Null);
        }
        if from == "{iterator}" {
            return Ok(DocStringElement::Iterator);
        }

        Err(BindingError::InvalidDocString)
    }
}

pub(crate) fn validate_library_docs(lib: &Library) -> Result<(), BindingError> {
    for native_function in lib.native_functions() {
        validate_doc_with_params(
            &native_function.name,
            &native_function.doc,
            &native_function.parameters,
            lib,
        )?;
    }

    for class in lib.classes() {
        validate_doc(class.name(), &class.doc, lib)?;
    }

    for structure in lib.structs() {
        validate_doc(structure.name(), structure.doc(), lib)?;
        for element in structure.elements() {
            validate_doc(
                &format!("{}.{}()", structure.name(), element.name),
                &element.doc,
                lib,
            )?;
        }
    }

    for enumeration in lib.native_enums() {
        validate_doc(&enumeration.name, &enumeration.doc, lib)?;
        for variant in &enumeration.variants {
            validate_doc(
                &format!("{}.{}", enumeration.name, variant.name),
                &variant.doc,
                lib,
            )?;
        }
    }

    for interface in lib.interfaces() {
        validate_doc(&interface.name, &interface.doc, lib)?;
        for callback in interface.callbacks() {
            let params = callback
                .parameters
                .iter()
                .filter_map(|param| match param {
                    CallbackParameter::Parameter(param) => Some(param.clone()),
                    _ => None,
                })
                .collect::<Vec<_>>();

            validate_doc_with_params(
                &format!("{}.{}", interface.name, callback.name),
                &callback.doc,
                params.as_slice(),
                lib,
            )?;
        }
    }

    for interface in lib.one_time_callbacks() {
        validate_doc(&interface.name, &interface.doc, lib)?;
        for callback in interface.callbacks() {
            let params = callback
                .parameters
                .iter()
                .filter_map(|param| match param {
                    CallbackParameter::Parameter(param) => Some(param.clone()),
                    _ => None,
                })
                .collect::<Vec<_>>();

            validate_doc_with_params(
                &format!("{}.{}", interface.name, callback.name),
                &callback.doc,
                params.as_slice(),
                lib,
            )?;
        }
    }

    Ok(())
}

fn validate_doc(symbol_name: &str, doc: &Doc, lib: &Library) -> Result<(), BindingError> {
    validate_doc_with_params(symbol_name, doc, &[], lib)
}

fn validate_doc_with_params(
    symbol_name: &str,
    doc: &Doc,
    params: &[Parameter],
    lib: &Library,
) -> Result<(), BindingError> {
    for reference in doc.references() {
        match reference {
            DocReference::Param(param_name) => {
                if params
                    .iter()
                    .find(|param| &param.name == param_name)
                    .is_none()
                {
                    return Err(BindingError::DocInvalidReference {
                        symbol_name: symbol_name.to_string(),
                        ref_name: param_name.to_string(),
                    });
                }
            }
            DocReference::Class(class_name) => {
                if lib.find_class(class_name).is_none() {
                    return Err(BindingError::DocInvalidReference {
                        symbol_name: symbol_name.to_string(),
                        ref_name: class_name.to_string(),
                    });
                }
            }
            DocReference::ClassMethod(class_name, method_name) => {
                if let Some(handle) = lib.find_class(class_name) {
                    if handle.find_method(method_name).is_none() {
                        return Err(BindingError::DocInvalidReference {
                            symbol_name: symbol_name.to_string(),
                            ref_name: format!(
                                "{}.{}()",
                                class_name.to_string(),
                                method_name.to_string()
                            ),
                        });
                    }
                } else {
                    return Err(BindingError::DocInvalidReference {
                        symbol_name: symbol_name.to_string(),
                        ref_name: format!(
                            "{}.{}()",
                            class_name.to_string(),
                            method_name.to_string()
                        ),
                    });
                }
            }
            DocReference::ClassConstructor(class_name) => {
                if let Some(handle) = lib.find_class(class_name) {
                    if handle.constructor.is_none() {
                        return Err(BindingError::DocInvalidReference {
                            symbol_name: symbol_name.to_string(),
                            ref_name: format!("{}.[constructor]", class_name.to_string(),),
                        });
                    }
                }
            }
            DocReference::ClassDestructor(class_name) => {
                if let Some(handle) = lib.find_class(class_name) {
                    if handle.destructor.is_none() {
                        return Err(BindingError::DocInvalidReference {
                            symbol_name: symbol_name.to_string(),
                            ref_name: format!("{}.[destructor]", class_name.to_string(),),
                        });
                    }
                }
            }
            DocReference::Struct(struct_name) => {
                if lib.find_struct(struct_name).is_none() {
                    return Err(BindingError::DocInvalidReference {
                        symbol_name: symbol_name.to_string(),
                        ref_name: struct_name.to_string(),
                    });
                }
            }
            DocReference::StructElement(struct_name, method_name) => {
                if let Some(handle) = lib.find_struct(struct_name) {
                    if handle.find_element(method_name).is_none() {
                        return Err(BindingError::DocInvalidReference {
                            symbol_name: symbol_name.to_string(),
                            ref_name: format!(
                                "{}.{}",
                                struct_name.to_string(),
                                method_name.to_string()
                            ),
                        });
                    }
                } else {
                    return Err(BindingError::DocInvalidReference {
                        symbol_name: symbol_name.to_string(),
                        ref_name: format!(
                            "{}.{}",
                            struct_name.to_string(),
                            method_name.to_string()
                        ),
                    });
                }
            }
            DocReference::StructMethod(struct_name, element_name) => {
                if let Some(handle) = lib.find_struct(struct_name) {
                    if handle.find_method(element_name).is_none() {
                        return Err(BindingError::DocInvalidReference {
                            symbol_name: symbol_name.to_string(),
                            ref_name: format!(
                                "{}.{}()",
                                struct_name.to_string(),
                                element_name.to_string()
                            ),
                        });
                    }
                } else {
                    return Err(BindingError::DocInvalidReference {
                        symbol_name: symbol_name.to_string(),
                        ref_name: format!(
                            "{}.{}()",
                            struct_name.to_string(),
                            element_name.to_string()
                        ),
                    });
                }
            }
            DocReference::Enum(enum_name) => {
                if lib.find_enum(enum_name).is_none() {
                    return Err(BindingError::DocInvalidReference {
                        symbol_name: symbol_name.to_string(),
                        ref_name: enum_name.to_string(),
                    });
                }
            }
            DocReference::EnumVariant(enum_name, variant_name) => {
                if let Some(handle) = lib.find_enum(enum_name) {
                    if handle.find_variant(variant_name).is_none() {
                        return Err(BindingError::DocInvalidReference {
                            symbol_name: symbol_name.to_string(),
                            ref_name: format!(
                                "{}.{}",
                                enum_name.to_string(),
                                variant_name.to_string()
                            ),
                        });
                    }
                } else {
                    return Err(BindingError::DocInvalidReference {
                        symbol_name: symbol_name.to_string(),
                        ref_name: format!("{}.{}", enum_name.to_string(), variant_name.to_string()),
                    });
                }
            }
            DocReference::Interface(interface_name) => {
                if lib.find_interface(interface_name).is_none() {
                    return Err(BindingError::DocInvalidReference {
                        symbol_name: symbol_name.to_string(),
                        ref_name: interface_name.to_string(),
                    });
                }
            }
            DocReference::InterfaceMethod(interface_name, method_name) => {
                if let Some(handle) = lib.find_interface(interface_name) {
                    if handle.find_callback(method_name).is_none() {
                        return Err(BindingError::DocInvalidReference {
                            symbol_name: symbol_name.to_string(),
                            ref_name: format!(
                                "{}.{}()",
                                interface_name.to_string(),
                                method_name.to_string()
                            ),
                        });
                    }
                } else {
                    return Err(BindingError::DocInvalidReference {
                        symbol_name: symbol_name.to_string(),
                        ref_name: format!(
                            "{}.{}()",
                            interface_name.to_string(),
                            method_name.to_string()
                        ),
                    });
                }
            }
            DocReference::OneTimeCallback(interface_name) => {
                if lib.find_one_time_callback(interface_name).is_none() {
                    return Err(BindingError::DocInvalidReference {
                        symbol_name: symbol_name.to_string(),
                        ref_name: interface_name.to_string(),
                    });
                }
            }
            DocReference::OneTimeCallbackMethod(interface_name, method_name) => {
                if let Some(handle) = lib.find_one_time_callback(interface_name) {
                    if handle.find_callback(method_name).is_none() {
                        return Err(BindingError::DocInvalidReference {
                            symbol_name: symbol_name.to_string(),
                            ref_name: format!(
                                "{}.{}()",
                                interface_name.to_string(),
                                method_name.to_string()
                            ),
                        });
                    }
                } else {
                    return Err(BindingError::DocInvalidReference {
                        symbol_name: symbol_name.to_string(),
                        ref_name: format!(
                            "{}.{}()",
                            interface_name.to_string(),
                            method_name.to_string()
                        ),
                    });
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn parse_param_reference() {
        let doc: DocString = "This is a {param:foo} test.".try_into().unwrap();
        assert_eq!(
            [
                DocStringElement::Text("This is a ".to_owned()),
                DocStringElement::Reference(DocReference::Param("foo".to_owned())),
                DocStringElement::Text(" test.".to_owned()),
            ]
            .as_ref(),
            doc.elements.as_slice()
        );
    }

    #[test]
    fn parse_class_reference() {
        let doc: DocString = "This is a {class:MyClass} test.".try_into().unwrap();
        assert_eq!(
            [
                DocStringElement::Text("This is a ".to_owned()),
                DocStringElement::Reference(DocReference::Class("MyClass".to_owned())),
                DocStringElement::Text(" test.".to_owned()),
            ]
            .as_ref(),
            doc.elements.as_slice()
        );
    }

    #[test]
    fn parse_class_reference_at_the_end() {
        let doc: DocString = "This is a test {class:MyClass2}".try_into().unwrap();
        assert_eq!(
            [
                DocStringElement::Text("This is a test ".to_owned()),
                DocStringElement::Reference(DocReference::Class("MyClass2".to_owned())),
            ]
            .as_ref(),
            doc.elements.as_slice()
        );
    }

    #[test]
    fn parse_class_method() {
        let doc: DocString = "This is a {class:MyClass.do_something()} method."
            .try_into()
            .unwrap();
        assert_eq!(
            [
                DocStringElement::Text("This is a ".to_owned()),
                DocStringElement::Reference(DocReference::ClassMethod(
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
        let doc: DocString = "This is a {struct:MyStruct} struct.".try_into().unwrap();
        assert_eq!(
            [
                DocStringElement::Text("This is a ".to_owned()),
                DocStringElement::Reference(DocReference::Struct("MyStruct".to_owned())),
                DocStringElement::Text(" struct.".to_owned()),
            ]
            .as_ref(),
            doc.elements.as_slice()
        );
    }

    #[test]
    fn parse_struct_element() {
        let doc: DocString = "This is a {struct:MyStruct.foo} struct element."
            .try_into()
            .unwrap();
        assert_eq!(
            [
                DocStringElement::Text("This is a ".to_owned()),
                DocStringElement::Reference(DocReference::StructElement(
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
    fn parse_struct_method() {
        let doc: DocString = "This is a {struct:MyStruct.bar()} struct method."
            .try_into()
            .unwrap();
        assert_eq!(
            [
                DocStringElement::Text("This is a ".to_owned()),
                DocStringElement::Reference(DocReference::StructMethod(
                    "MyStruct".to_owned(),
                    "bar".to_owned()
                )),
                DocStringElement::Text(" struct method.".to_owned()),
            ]
            .as_ref(),
            doc.elements.as_slice()
        );
    }

    #[test]
    fn parse_enum() {
        let doc: DocString = "This is a {enum:MyEnum} enum.".try_into().unwrap();
        assert_eq!(
            [
                DocStringElement::Text("This is a ".to_owned()),
                DocStringElement::Reference(DocReference::Enum("MyEnum".to_owned())),
                DocStringElement::Text(" enum.".to_owned()),
            ]
            .as_ref(),
            doc.elements.as_slice()
        );
    }

    #[test]
    fn parse_enum_element() {
        let doc: DocString = "This is a {enum:MyEnum.foo} enum variant."
            .try_into()
            .unwrap();
        assert_eq!(
            [
                DocStringElement::Text("This is a ".to_owned()),
                DocStringElement::Reference(DocReference::EnumVariant(
                    "MyEnum".to_owned(),
                    "foo".to_owned()
                )),
                DocStringElement::Text(" enum variant.".to_owned()),
            ]
            .as_ref(),
            doc.elements.as_slice()
        );
    }
}
