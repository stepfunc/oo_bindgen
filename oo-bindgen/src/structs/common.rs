use std::collections::HashSet;
use std::fmt::{Debug, Formatter};

use crate::class::ClassDeclarationHandle;
use crate::collection::CollectionHandle;
use crate::doc::Doc;
use crate::enum_type::EnumHandle;
use crate::interface::InterfaceHandle;
use crate::iterator::IteratorHandle;
use crate::types::{DurationType, StringType, TypeValidator};
use crate::{BindResult, BindingError, Handle, LibraryBuilder, Statement, StructType};
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FieldName {
    name: Handle<String>,
}

impl FieldName {
    pub fn new<T: Into<String>>(name: T) -> Self {
        Self {
            name: Handle::new(name.into()),
        }
    }
}

impl From<&FieldName> for FieldName {
    fn from(x: &FieldName) -> Self {
        x.clone()
    }
}

impl std::ops::Deref for FieldName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.name
    }
}

impl std::fmt::Display for FieldName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*self.name)
    }
}

impl From<&str> for FieldName {
    fn from(x: &str) -> Self {
        FieldName::new(x)
    }
}

/// Value used to define constructor default
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Number {
    U8(u8),
    S8(i8),
    U16(u16),
    S16(i16),
    U32(u32),
    S32(i32),
    U64(u64),
    S64(i64),
    Float(f32),
    Double(f64),
}

/// Value used to define constructor default
#[derive(Debug, Clone, PartialEq)]
pub enum ConstructorDefault {
    Bool(bool),
    Numeric(Number),
    Duration(Duration),
    Enum(String),
    String(String),
    /// requires that the struct have a default constructor
    DefaultStruct,
}

trait ToDefaultVariant {
    fn default_variant(&self) -> ConstructorDefault;
}

trait ToDefaultString {
    fn default_string(&self) -> ConstructorDefault;
}

impl ToDefaultVariant for &str {
    fn default_variant(&self) -> ConstructorDefault {
        ConstructorDefault::Enum(self.to_string())
    }
}

impl ToDefaultString for &str {
    fn default_string(&self) -> ConstructorDefault {
        ConstructorDefault::String(self.to_string())
    }
}

impl From<Number> for ConstructorDefault {
    fn from(x: Number) -> Self {
        Self::Numeric(x)
    }
}

impl From<bool> for ConstructorDefault {
    fn from(x: bool) -> Self {
        ConstructorDefault::Bool(x)
    }
}

impl From<Duration> for ConstructorDefault {
    fn from(x: Duration) -> Self {
        ConstructorDefault::Duration(x)
    }
}

// Value used to define constructor default
#[derive(Debug, Clone)]
pub enum ValidatedConstructorDefault {
    Bool(bool),
    Numeric(Number),
    Duration(DurationType, Duration),
    Enum(EnumHandle, String),
    String(String),
    /// requires that the struct have a default constructor
    DefaultStruct(StructType, ConstructorType, String),
}

impl From<Number> for ValidatedConstructorDefault {
    fn from(x: Number) -> Self {
        Self::Numeric(x)
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::U8(x) => {
                write!(f, "{}", x)
            }
            Self::S8(x) => {
                write!(f, "{}", x)
            }
            Self::U16(x) => {
                write!(f, "{}", x)
            }
            Self::S16(x) => {
                write!(f, "{}", x)
            }
            Self::U32(x) => {
                write!(f, "{}", x)
            }
            Self::S32(x) => {
                write!(f, "{}", x)
            }
            Self::U64(x) => {
                write!(f, "{}", x)
            }
            Self::S64(x) => {
                write!(f, "{}", x)
            }
            Self::Float(x) => {
                write!(f, "{}", x)
            }
            Self::Double(x) => {
                write!(f, "{}", x)
            }
        }
    }
}

impl std::fmt::Display for ValidatedConstructorDefault {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool(x) => {
                write!(f, "{}", x)
            }
            Self::Numeric(x) => write!(f, "{}", x),
            Self::Duration(t, x) => match t {
                DurationType::Milliseconds => write!(f, "{} milliseconds", x.as_millis()),
                DurationType::Seconds => write!(f, "{} seconds", x.as_secs()),
            },
            Self::Enum(handle, x) => {
                write!(f, "{}::{}", handle.name, x)
            }
            Self::String(x) => {
                write!(f, "'{}'", x)
            }
            Self::DefaultStruct(x, _, _) => {
                write!(f, "default constructed value for {}", x.name())
            }
        }
    }
}

/// struct type affects the type of code the backend will generate
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Visibility {
    /// struct members are public
    Public,
    /// struct members are private (except C of course), and the struct is just an opaque "token"
    Private,
}

/// C-style structure forward declaration
#[derive(Debug)]
pub struct StructDeclaration {
    pub name: String,
}

impl StructDeclaration {
    pub(crate) fn new(name: String) -> Self {
        Self { name }
    }
}

pub type StructDeclarationHandle = Handle<StructDeclaration>;

pub trait ConstructorValidator {
    fn bad_constructor_value(
        field_type: String,
        value: &ConstructorDefault,
    ) -> BindResult<ValidatedConstructorDefault> {
        Err(BindingError::StructConstructorBadValueForType {
            field_type,
            value: value.clone(),
        })
    }

    /// Check that the value is valid for the type
    fn validate_constructor_default(
        &self,
        value: &ConstructorDefault,
    ) -> BindResult<ValidatedConstructorDefault>;
}

impl ConstructorValidator for IteratorHandle {
    fn validate_constructor_default(
        &self,
        value: &ConstructorDefault,
    ) -> BindResult<ValidatedConstructorDefault> {
        Self::bad_constructor_value("IteratorHandle".to_string(), value)
    }
}

impl ConstructorValidator for ClassDeclarationHandle {
    fn validate_constructor_default(
        &self,
        value: &ConstructorDefault,
    ) -> BindResult<ValidatedConstructorDefault> {
        Self::bad_constructor_value("ClassHandle".to_string(), value)
    }
}

impl ConstructorValidator for InterfaceHandle {
    fn validate_constructor_default(
        &self,
        value: &ConstructorDefault,
    ) -> BindResult<ValidatedConstructorDefault> {
        Self::bad_constructor_value("InterfaceHandle".to_string(), value)
    }
}

impl ConstructorValidator for CollectionHandle {
    fn validate_constructor_default(
        &self,
        value: &ConstructorDefault,
    ) -> BindResult<ValidatedConstructorDefault> {
        Self::bad_constructor_value("CollectionHandle".to_string(), value)
    }
}

impl ConstructorValidator for StringType {
    fn validate_constructor_default(
        &self,
        value: &ConstructorDefault,
    ) -> BindResult<ValidatedConstructorDefault> {
        match value {
            ConstructorDefault::String(x) => Ok(ValidatedConstructorDefault::String(x.clone())),
            _ => Self::bad_constructor_value("String".to_string(), value),
        }
    }
}

pub trait StructFieldType: Clone + Sized + TypeValidator + ConstructorValidator {
    /// convert a structure to a StructType
    fn create_struct_type(v: Handle<Struct<Self>>) -> StructType;
}

#[derive(Debug)]
pub struct StructField<F>
where
    F: StructFieldType,
{
    pub name: FieldName,
    pub field_type: F,
    pub doc: Doc,
}

/// C-style structure definition
#[derive(Debug)]
pub struct Struct<F>
where
    F: StructFieldType,
{
    pub visibility: Visibility,
    pub declaration: StructDeclarationHandle,
    pub fields: Vec<StructField<F>>,
    pub constructors: Vec<Handle<Constructor>>,
    pub doc: Doc,
}

impl<F> ConstructorValidator for Handle<Struct<F>>
where
    F: StructFieldType,
{
    fn validate_constructor_default(
        &self,
        value: &ConstructorDefault,
    ) -> BindResult<ValidatedConstructorDefault> {
        match value {
            ConstructorDefault::DefaultStruct => match self.get_default_constructor() {
                Some(c) => Ok(ValidatedConstructorDefault::DefaultStruct(
                    F::create_struct_type(self.clone()),
                    c.constructor_type,
                    c.name.to_string(),
                )),
                None => Err(
                    BindingError::StructConstructorStructFieldWithoutDefaultConstructor {
                        struct_name: self.name().to_string(),
                    },
                ),
            },
            _ => Err(BindingError::StructConstructorBadValueForType {
                field_type: "Struct".to_string(),
                value: value.clone(),
            }),
        }
    }
}

impl<F> Struct<F>
where
    F: StructFieldType,
{
    pub fn requires_constructor(&self) -> bool {
        true
    }

    pub fn has_field_named(&self, name: &str) -> bool {
        self.fields.iter().any(|x| x.name.as_str() == name)
    }

    pub fn name(&self) -> &str {
        &self.declaration.name
    }

    pub fn declaration(&self) -> StructDeclarationHandle {
        self.declaration.clone()
    }

    pub fn constructor_args(
        &self,
        constructor: Handle<Constructor>,
    ) -> impl Iterator<Item = &StructField<F>> {
        self.fields
            .iter()
            .filter(move |field| !constructor.values.iter().any(|c| c.name == field.name))
    }

    pub fn fields(&self) -> impl Iterator<Item = &StructField<F>> {
        self.fields.iter()
    }

    pub fn has_default_constructor(&self) -> bool {
        self.get_default_constructor().is_some()
    }

    pub fn has_full_constructor(&self) -> bool {
        self.get_full_constructor().is_some()
    }

    pub fn get_default_constructor(&self) -> Option<&Handle<Constructor>> {
        // do any of the constructors initialize ALL of the fields
        self.constructors
            .iter()
            .find(|c| c.values.len() == self.fields.len())
    }

    pub fn get_full_constructor(&self) -> Option<&Handle<Constructor>> {
        // do any of the constructors initialize NONE of the fields
        self.constructors.iter().find(|c| c.values.is_empty())
    }
}

pub struct StructFieldBuilder<'a, F>
where
    F: StructFieldType,
{
    lib: &'a mut LibraryBuilder,
    visibility: Visibility,
    declaration: StructDeclarationHandle,
    fields: Vec<StructField<F>>,
    field_names: HashSet<String>,
    doc: Option<Doc>,
}

impl<'a, F> StructFieldBuilder<'a, F>
where
    F: StructFieldType,
{
    pub(crate) fn new(lib: &'a mut LibraryBuilder, declaration: StructDeclarationHandle) -> Self {
        Self {
            lib,
            visibility: Visibility::Public, // defaults to a public struct
            declaration,
            fields: Vec::new(),
            field_names: HashSet::new(),
            doc: None,
        }
    }

    pub fn make_opaque(mut self) -> Self {
        self.visibility = Visibility::Private;
        self
    }

    pub fn add<S: Into<FieldName>, V: Into<F>, D: Into<Doc>>(
        mut self,
        name: S,
        field_type: V,
        doc: D,
    ) -> BindResult<Self> {
        let name = name.into();
        let field_type = field_type.into();

        self.lib.validate_type(&field_type)?;
        if self.field_names.insert((*name).clone()) {
            self.fields.push(StructField {
                name,
                field_type,
                doc: doc.into(),
            });
            Ok(self)
        } else {
            Err(BindingError::StructAlreadyContainsFieldWithSameName {
                handle: self.declaration,
                field_name: name,
            })
        }
    }

    pub fn doc<D: Into<Doc>>(mut self, doc: D) -> BindResult<Self> {
        match self.doc {
            None => {
                self.doc = Some(doc.into());
                Ok(self)
            }
            Some(_) => Err(BindingError::DocAlreadyDefined {
                symbol_name: self.declaration.name.clone(),
            }),
        }
    }

    pub fn end_fields(self) -> BindResult<MethodBuilder<'a, F>> {
        let doc = match self.doc {
            Some(doc) => doc,
            None => {
                return Err(BindingError::DocNotDefined {
                    symbol_name: self.declaration.name.clone(),
                })
            }
        };

        Ok(MethodBuilder {
            lib: self.lib,
            visibility: self.visibility,
            declaration: self.declaration,
            fields: self.fields,
            constructors: Vec::new(),
            doc,
        })
    }
}

#[derive(Debug, Clone)]
pub struct InitializedValue {
    pub name: FieldName,
    pub value: ValidatedConstructorDefault,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ConstructorType {
    /// Normal constructors map to actual language constructors
    /// A name is still required as some languages (e.g. C) don't support
    /// associated constructors and require free-standing functions instead
    Normal,
    /// Static constructors are mapped to static methods in languages that support them (e.g. C++, Java, C#)
    Static,
}

impl ConstructorType {
    pub fn is_normal(&self) -> bool {
        *self == ConstructorType::Normal
    }
}

#[derive(Debug, Clone)]
pub struct Constructor {
    pub name: String,
    pub constructor_type: ConstructorType,
    pub values: Vec<InitializedValue>,
    pub doc: Doc,
}

impl Constructor {
    fn argument_names(&self) -> HashSet<FieldName> {
        self.values
            .iter()
            .map(|x| x.name.clone())
            .collect::<HashSet<FieldName>>()
    }

    pub fn collides_with(&self, other: &Self) -> bool {
        if self.argument_names() == other.argument_names() {
            // this is only a problem is both are normal constructors
            return self.constructor_type.is_normal() && other.constructor_type.is_normal();
        }
        false
    }

    pub fn full(name: String, constructor_type: ConstructorType, doc: Doc) -> Self {
        Self {
            name,
            constructor_type,
            values: Vec::new(),
            doc,
        }
    }
}

pub struct ConstructorBuilder<'a, F>
where
    F: StructFieldType,
{
    name: String,
    constructor_type: ConstructorType,
    builder: MethodBuilder<'a, F>,
    fields: Vec<InitializedValue>,
    doc: Doc,
}

pub struct MethodBuilder<'a, F>
where
    F: StructFieldType,
{
    lib: &'a mut LibraryBuilder,
    visibility: Visibility,
    declaration: StructDeclarationHandle,
    fields: Vec<StructField<F>>,
    constructors: Vec<Handle<Constructor>>,
    doc: Doc,
}

impl<'a, F> MethodBuilder<'a, F>
where
    F: StructFieldType,
{
    pub fn begin_constructor<D: Into<Doc>, S: Into<String>>(
        self,
        name: S,
        constructor_type: ConstructorType,
        doc: D,
    ) -> BindResult<ConstructorBuilder<'a, F>> {
        let name = name.into();

        // check that we don't have any other constructors with this name
        if self.constructors.iter().any(|c| c.name == name) {
            return Err(BindingError::StructConstructorDuplicateName {
                struct_name: self.declaration.name.clone(),
                constructor_name: name,
            });
        }

        Ok(ConstructorBuilder {
            name,
            constructor_type,
            builder: self,
            fields: Vec::new(),
            doc: doc.into(),
        })
    }

    pub fn add_full_constructor<S: Into<String>>(self, name: S) -> BindResult<Self> {
        let struct_name = self.declaration.name.clone();
        self.begin_constructor(
            name,
            ConstructorType::Normal,
            format!(
                "Fully construct {{struct:{}}} specifying the value of each field",
                struct_name
            ),
        )?
        .end_constructor()
    }

    pub fn build(self) -> BindResult<Handle<Struct<F>>> {
        let handle = Handle::new(Struct {
            visibility: self.visibility,
            declaration: self.declaration.clone(),
            fields: self.fields,
            constructors: self.constructors,
            doc: self.doc,
        });

        self.lib
            .add_statement(Statement::StructDefinition(F::create_struct_type(
                handle.clone(),
            )))?;

        Ok(handle)
    }
}

impl<'a, F> ConstructorBuilder<'a, F>
where
    F: StructFieldType,
{
    pub fn add<D: Into<ConstructorDefault>>(
        mut self,
        name: &FieldName,
        value: D,
    ) -> BindResult<Self> {
        let value = value.into();

        // check that we haven't already defined this field
        if self.fields.iter().any(|f| f.name == *name) {
            return Err(BindingError::StructConstructorDuplicateField {
                struct_name: self.builder.declaration.name.clone(),
                field_name: name.to_string(),
            });
        }

        // find the field and validate it
        let value = match self.builder.fields.iter().find(|f| f.name == *name) {
            Some(x) => x.field_type.validate_constructor_default(&value)?,
            None => {
                return Err(BindingError::StructConstructorUnknownField {
                    struct_name: self.builder.declaration.name.clone(),
                    field_name: name.to_string(),
                });
            }
        };

        self.fields.push(InitializedValue {
            name: name.clone(),
            value,
        });

        Ok(self)
    }

    pub fn end_constructor(mut self) -> BindResult<MethodBuilder<'a, F>> {
        let constructor = Handle::new(Constructor {
            name: self.name,
            constructor_type: self.constructor_type,
            values: self.fields,
            doc: self.doc,
        });

        if let Some(x) = self
            .builder
            .constructors
            .iter()
            .find(|other| constructor.collides_with(other))
        {
            return Err(BindingError::StructDuplicateConstructorArgs {
                struct_name: self.builder.declaration.name.clone(),
                this_constructor: constructor.name.clone(),
                other_constructor: x.name.clone(),
            });
        }

        self.builder.constructors.push(constructor);
        Ok(self.builder)
    }
}
