use std::collections::HashSet;
use std::fmt::{Debug, Formatter};

use crate::class::ClassDeclarationHandle;
use crate::collection::CollectionHandle;
use crate::doc::{Doc, DocReference, Unvalidated, Validated};
use crate::enum_type::EnumHandle;
use crate::interface::InterfaceHandle;
use crate::iterator::IteratorHandle;
use crate::name::{IntoName, Name};
use crate::structs::{
    CallbackArgStructField, FunctionArgStructField, FunctionReturnStructField, UniversalStructField,
};
use crate::types::{DurationType, StringType, TypeValidator};
use crate::{
    BindResult, BindingError, Handle, LibraryBuilder, LibrarySettings, Statement, StructType,
    UnvalidatedFields,
};
use std::marker::PhantomData;
use std::rc::Rc;
use std::time::Duration;

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

pub trait ToDefaultVariant {
    fn default_variant(&self) -> ConstructorDefault;
}

pub trait ToDefaultString {
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
    DefaultStruct(StructType<Unvalidated>, ConstructorType, String),
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
    pub name: Name,
    pub settings: Rc<LibrarySettings>,
}

impl StructDeclaration {
    pub fn new(name: Name, settings: Rc<LibrarySettings>) -> Self {
        Self { name, settings }
    }
}

pub type StructDeclarationHandle = Handle<StructDeclaration>;

/// Typed wrapper around an untyped struct declaration
#[derive(Debug, Clone, Eq)]
pub struct TypedStructDeclaration<T> {
    pub inner: StructDeclarationHandle,
    phantom: PhantomData<T>,
}

impl<T> AsRef<StructDeclarationHandle> for TypedStructDeclaration<T> {
    fn as_ref(&self) -> &StructDeclarationHandle {
        &self.inner
    }
}

impl<T> PartialEq for TypedStructDeclaration<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<T> TypedStructDeclaration<T> {
    pub(crate) fn new(inner: StructDeclarationHandle) -> Self {
        Self {
            inner,
            phantom: Default::default(),
        }
    }

    pub fn name(&self) -> &Name {
        &self.inner.name
    }
}

pub type UniversalStructDeclaration = TypedStructDeclaration<UniversalStructField>;
pub type FunctionArgStructDeclaration = TypedStructDeclaration<FunctionArgStructField>;
pub type FunctionReturnStructDeclaration = TypedStructDeclaration<FunctionReturnStructField>;
pub type CallbackArgStructDeclaration = TypedStructDeclaration<CallbackArgStructField>;

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
    fn create_struct_type(v: Handle<Struct<Self, Unvalidated>>) -> StructType<Unvalidated>;
}

#[derive(Debug)]
pub struct StructField<F, D>
where
    F: StructFieldType,
    D: DocReference,
{
    pub name: Name,
    pub field_type: F,
    pub doc: Doc<D>,
}

impl<F> StructField<F, Unvalidated>
where
    F: StructFieldType,
{
    pub(crate) fn validate(
        &self,
        lib: &UnvalidatedFields,
    ) -> BindResult<StructField<F, Validated>> {
        Ok(StructField {
            name: self.name.clone(),
            field_type: self.field_type.clone(),
            doc: self.doc.validate(&self.name, lib)?,
        })
    }
}

/// C-style structure definition
#[derive(Debug)]
pub struct Struct<F, D>
where
    F: StructFieldType,
    D: DocReference,
{
    pub visibility: Visibility,
    pub declaration: TypedStructDeclaration<F>,
    pub fields: Vec<StructField<F, D>>,
    pub constructors: Vec<Handle<Constructor<D>>>,
    pub doc: Doc<D>,
}

impl<F> Struct<F, Unvalidated>
where
    F: StructFieldType,
{
    pub(crate) fn validate(&self, lib: &UnvalidatedFields) -> BindResult<Struct<F, Validated>> {
        let fields: BindResult<Vec<StructField<F, Validated>>> =
            self.fields.iter().map(|x| x.validate(lib)).collect();
        let constructors: BindResult<Vec<Constructor<Validated>>> =
            self.constructors.iter().map(|x| x.validate(lib)).collect();
        let constructors: Vec<Handle<Constructor<Validated>>> = constructors?
            .iter()
            .map(|x| Handle::new(x.clone()))
            .collect();

        Ok(Struct {
            visibility: self.visibility,
            declaration: self.declaration.clone(),
            fields: fields?,
            constructors,
            doc: self.doc.validate(self.name(), lib)?,
        })
    }
}

impl<F> ConstructorValidator for Handle<Struct<F, Unvalidated>>
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

impl<F, D> Struct<F, D>
where
    F: StructFieldType,
    D: DocReference,
{
    pub fn settings(&self) -> &LibrarySettings {
        &self.declaration.inner.settings
    }

    pub fn requires_constructor(&self) -> bool {
        true
    }

    pub fn find_field_name(&self, name: &str) -> Option<Name> {
        self.fields
            .iter()
            .find(|x| x.name == name)
            .map(|x| x.name.clone())
    }

    pub fn name(&self) -> &Name {
        &self.declaration.inner.name
    }

    pub fn declaration(&self) -> StructDeclarationHandle {
        self.declaration.inner.clone()
    }

    pub fn constructor_args(
        &self,
        constructor: Handle<Constructor<D>>,
    ) -> impl Iterator<Item = &StructField<F, D>> {
        self.fields
            .iter()
            .filter(move |field| !constructor.values.iter().any(|c| c.name == field.name))
    }

    pub fn fields(&self) -> impl Iterator<Item = &StructField<F, D>> {
        self.fields.iter()
    }

    pub fn has_default_constructor(&self) -> bool {
        self.get_default_constructor().is_some()
    }

    pub fn has_full_constructor(&self) -> bool {
        self.get_full_constructor().is_some()
    }

    pub fn get_default_constructor(&self) -> Option<&Handle<Constructor<D>>> {
        // Are any of the constructors of normal type and initialize ALL of the fields
        self.constructors.iter().find(|c| {
            c.constructor_type == ConstructorType::Normal && c.values.len() == self.fields.len()
        })
    }

    pub fn get_full_constructor(&self) -> Option<&Handle<Constructor<D>>> {
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
    declaration: TypedStructDeclaration<F>,
    fields: Vec<StructField<F, Unvalidated>>,
    field_names: HashSet<String>,
    doc: Option<Doc<Unvalidated>>,
}

impl<'a, F> StructFieldBuilder<'a, F>
where
    F: StructFieldType,
{
    pub(crate) fn new(lib: &'a mut LibraryBuilder, declaration: TypedStructDeclaration<F>) -> Self {
        Self::new_impl(lib, declaration, Visibility::Public)
    }

    pub(crate) fn opaque(
        lib: &'a mut LibraryBuilder,
        declaration: TypedStructDeclaration<F>,
    ) -> Self {
        Self::new_impl(lib, declaration, Visibility::Private)
    }

    fn new_impl(
        lib: &'a mut LibraryBuilder,
        declaration: TypedStructDeclaration<F>,
        visibility: Visibility,
    ) -> Self {
        Self {
            lib,
            visibility,
            declaration,
            fields: Vec::new(),
            field_names: HashSet::new(),
            doc: None,
        }
    }

    pub fn add<S: IntoName, V: Into<F>, D: Into<Doc<Unvalidated>>>(
        mut self,
        name: S,
        field_type: V,
        doc: D,
    ) -> BindResult<Self> {
        let name = name.into_name()?;
        let field_type = field_type.into();

        self.lib.validate_type(&field_type)?;
        if self.field_names.insert(name.to_string()) {
            self.fields.push(StructField {
                name,
                field_type,
                doc: doc.into(),
            });
            Ok(self)
        } else {
            Err(BindingError::StructAlreadyContainsFieldWithSameName {
                handle: self.declaration.inner.clone(),
                field_name: name,
            })
        }
    }

    pub fn doc<D: Into<Doc<Unvalidated>>>(mut self, doc: D) -> BindResult<Self> {
        match self.doc {
            None => {
                self.doc = Some(doc.into());
                Ok(self)
            }
            Some(_) => Err(BindingError::DocAlreadyDefined {
                symbol_name: self.declaration.name().clone(),
            }),
        }
    }

    pub fn end_fields(self) -> BindResult<MethodBuilder<'a, F>> {
        let doc = match self.doc {
            Some(doc) => doc,
            None => {
                return Err(BindingError::DocNotDefined {
                    symbol_name: self.declaration.name().clone(),
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
    pub name: Name,
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
pub struct Constructor<D>
where
    D: DocReference,
{
    pub name: Name,
    pub constructor_type: ConstructorType,
    pub values: Rc<Vec<InitializedValue>>,
    pub doc: Doc<D>,
}

impl Constructor<Unvalidated> {
    pub(crate) fn validate(&self, lib: &UnvalidatedFields) -> BindResult<Constructor<Validated>> {
        Ok(Constructor {
            name: self.name.clone(),
            constructor_type: self.constructor_type,
            values: self.values.clone(),
            doc: self.doc.validate(&self.name, lib)?,
        })
    }
}

impl<D> Constructor<D>
where
    D: DocReference,
{
    fn argument_names(&self) -> HashSet<Name> {
        self.values
            .iter()
            .map(|x| x.name.clone())
            .collect::<HashSet<Name>>()
    }

    pub fn collides_with(&self, other: &Self) -> bool {
        if self.argument_names() == other.argument_names() {
            // this is only a problem is both are normal constructors
            return self.constructor_type.is_normal() && other.constructor_type.is_normal();
        }
        false
    }

    pub fn full(constructor_type: ConstructorType, doc: Doc<D>) -> Self {
        Self {
            name: Name::create("some_unused_name").unwrap(),
            constructor_type,
            values: Rc::new(Vec::new()),
            doc,
        }
    }
}

pub struct ConstructorBuilder<'a, F>
where
    F: StructFieldType,
{
    name: Name,
    constructor_type: ConstructorType,
    builder: MethodBuilder<'a, F>,
    fields: Vec<InitializedValue>,
    doc: Doc<Unvalidated>,
}

pub struct MethodBuilder<'a, F>
where
    F: StructFieldType,
{
    lib: &'a mut LibraryBuilder,
    visibility: Visibility,
    declaration: TypedStructDeclaration<F>,
    fields: Vec<StructField<F, Unvalidated>>,
    constructors: Vec<Handle<Constructor<Unvalidated>>>,
    doc: Doc<Unvalidated>,
}

impl<'a, F> MethodBuilder<'a, F>
where
    F: StructFieldType,
{
    pub fn begin_constructor<D: Into<Doc<Unvalidated>>, S: IntoName>(
        self,
        name: S,
        constructor_type: ConstructorType,
        doc: D,
    ) -> BindResult<ConstructorBuilder<'a, F>> {
        let name = name.into_name()?;

        // check that we don't have any other constructors with this name
        if self.constructors.iter().any(|c| name == c.name) {
            return Err(BindingError::StructConstructorDuplicateName {
                struct_name: self.declaration.name().clone(),
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

    pub fn add_full_constructor<S: IntoName>(self, name: S) -> BindResult<Self> {
        let name = name.into_name()?;
        let struct_name = self.declaration.name().clone();
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

    pub fn build(self) -> BindResult<Handle<Struct<F, Unvalidated>>> {
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
    pub fn default<D: Into<ConstructorDefault>>(
        mut self,
        name: &Name,
        value: D,
    ) -> BindResult<Self> {
        let value = value.into();

        // check that we haven't already defined this field
        if self.fields.iter().any(|f| f.name == *name) {
            return Err(BindingError::StructConstructorDuplicateField {
                struct_name: self.builder.declaration.name().clone(),
                field_name: name.clone(),
            });
        }

        // find the field and validate it
        let value = match self.builder.fields.iter().find(|f| f.name == *name) {
            Some(x) => x.field_type.validate_constructor_default(&value)?,
            None => {
                return Err(BindingError::StructConstructorUnknownField {
                    struct_name: self.builder.declaration.name().clone(),
                    field_name: name.clone(),
                });
            }
        };

        self.fields.push(InitializedValue {
            name: name.clone(),
            value,
        });

        Ok(self)
    }

    pub fn default_struct(self, name: &Name) -> BindResult<Self> {
        self.default(name, ConstructorDefault::DefaultStruct)
    }

    pub fn default_variant<S: Into<String>>(self, name: &Name, variant: S) -> BindResult<Self> {
        self.default(name, ConstructorDefault::Enum(variant.into()))
    }

    pub fn default_string<S: Into<String>>(self, name: &Name, value: S) -> BindResult<Self> {
        self.default(name, ConstructorDefault::String(value.into()))
    }

    pub fn end_constructor(mut self) -> BindResult<MethodBuilder<'a, F>> {
        let constructor = Handle::new(Constructor {
            name: self.name,
            constructor_type: self.constructor_type,
            values: Rc::new(self.fields),
            doc: self.doc,
        });

        if let Some(x) = self
            .builder
            .constructors
            .iter()
            .find(|other| constructor.collides_with(other))
        {
            return Err(BindingError::StructDuplicateConstructorArgs {
                struct_name: self.builder.declaration.name().clone(),
                this_constructor: constructor.name.clone(),
                other_constructor: x.name.clone(),
            });
        }

        self.builder.constructors.push(constructor);
        Ok(self.builder)
    }
}
