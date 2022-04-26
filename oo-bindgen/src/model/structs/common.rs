use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::rc::Rc;
use std::time::Duration;

use crate::model::*;

/// A numeric value
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum NumberValue {
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

/// Default value for a field in struct initializer
#[derive(Debug, Clone, PartialEq)]
pub enum InitializerDefault {
    Bool(bool),
    Numeric(NumberValue),
    Duration(Duration),
    Enum(String),
    String(String),
    /// requires that the struct have a default initializer
    DefaultStruct,
}

pub trait ToDefaultVariant {
    fn default_variant(&self) -> InitializerDefault;
}

pub trait ToDefaultString {
    fn default_string(&self) -> InitializerDefault;
}

impl ToDefaultVariant for &str {
    fn default_variant(&self) -> InitializerDefault {
        InitializerDefault::Enum(self.to_string())
    }
}

impl ToDefaultString for &str {
    fn default_string(&self) -> InitializerDefault {
        InitializerDefault::String(self.to_string())
    }
}

impl From<NumberValue> for InitializerDefault {
    fn from(x: NumberValue) -> Self {
        Self::Numeric(x)
    }
}

impl From<bool> for InitializerDefault {
    fn from(x: bool) -> Self {
        InitializerDefault::Bool(x)
    }
}

impl From<Duration> for InitializerDefault {
    fn from(x: Duration) -> Self {
        InitializerDefault::Duration(x)
    }
}

// Value used to define a default in a struct initializer
#[derive(Debug, Clone)]
pub enum ValidatedDefaultValue {
    Bool(bool),
    Number(NumberValue),
    Duration(DurationType, Duration),
    Enum(Handle<Enum<Unvalidated>>, Name),
    String(String),
    /// requires that the struct have a default initializer
    DefaultStruct(StructType<Unvalidated>, InitializerType, Name),
}

impl From<NumberValue> for ValidatedDefaultValue {
    fn from(x: NumberValue) -> Self {
        Self::Number(x)
    }
}

impl std::fmt::Display for NumberValue {
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

impl std::fmt::Display for ValidatedDefaultValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool(x) => {
                write!(f, "{}", x)
            }
            Self::Number(x) => write!(f, "{}", x),
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

pub trait InitializerValidator {
    fn bad_initializer_value(
        field_type: String,
        value: &InitializerDefault,
    ) -> BindResult<ValidatedDefaultValue> {
        Err(BindingError::StructInitializerBadValueForType {
            field_type,
            value: value.clone(),
        })
    }

    /// Check that the value is valid for the type
    fn validate_default_value(
        &self,
        value: &InitializerDefault,
    ) -> BindResult<ValidatedDefaultValue>;
}

impl InitializerValidator for AbstractIteratorHandle {
    fn validate_default_value(
        &self,
        value: &InitializerDefault,
    ) -> BindResult<ValidatedDefaultValue> {
        Self::bad_initializer_value("IteratorHandle".to_string(), value)
    }
}

impl InitializerValidator for ClassDeclarationHandle {
    fn validate_default_value(
        &self,
        value: &InitializerDefault,
    ) -> BindResult<ValidatedDefaultValue> {
        Self::bad_initializer_value("ClassHandle".to_string(), value)
    }
}

impl InitializerValidator for InterfaceHandle {
    fn validate_default_value(
        &self,
        value: &InitializerDefault,
    ) -> BindResult<ValidatedDefaultValue> {
        Self::bad_initializer_value("InterfaceHandle".to_string(), value)
    }
}

impl InitializerValidator for CollectionHandle {
    fn validate_default_value(
        &self,
        value: &InitializerDefault,
    ) -> BindResult<ValidatedDefaultValue> {
        Self::bad_initializer_value("CollectionHandle".to_string(), value)
    }
}

impl InitializerValidator for StringType {
    fn validate_default_value(
        &self,
        value: &InitializerDefault,
    ) -> BindResult<ValidatedDefaultValue> {
        match value {
            InitializerDefault::String(x) => Ok(ValidatedDefaultValue::String(x.clone())),
            _ => Self::bad_initializer_value("String".to_string(), value),
        }
    }
}

pub trait StructFieldType: Clone + Sized + InitializerValidator {
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
    pub(crate) fn validate(&self, lib: &LibraryFields) -> BindResult<StructField<F, Validated>> {
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
    pub initializers: Vec<Handle<Initializer<D>>>,
    pub doc: Doc<D>,
}

impl<F> Struct<F, Unvalidated>
where
    F: StructFieldType,
{
    pub(crate) fn validate(&self, lib: &LibraryFields) -> BindResult<Handle<Struct<F, Validated>>> {
        let fields: BindResult<Vec<StructField<F, Validated>>> =
            self.fields.iter().map(|x| x.validate(lib)).collect();
        let initializers: BindResult<Vec<Initializer<Validated>>> =
            self.initializers.iter().map(|x| x.validate(lib)).collect();
        let initializers: Vec<Handle<Initializer<Validated>>> = initializers?
            .iter()
            .map(|x| Handle::new(x.clone()))
            .collect();

        Ok(Handle::new(Struct {
            visibility: self.visibility,
            declaration: self.declaration.clone(),
            fields: fields?,
            initializers,
            doc: self.doc.validate(self.name(), lib)?,
        }))
    }
}

impl<F> InitializerValidator for Handle<Struct<F, Unvalidated>>
where
    F: StructFieldType,
{
    fn validate_default_value(
        &self,
        value: &InitializerDefault,
    ) -> BindResult<ValidatedDefaultValue> {
        match value {
            InitializerDefault::DefaultStruct => match self.get_default_initializer() {
                Some(c) => Ok(ValidatedDefaultValue::DefaultStruct(
                    F::create_struct_type(self.clone()),
                    c.initializer_type,
                    c.name.clone(),
                )),
                None => Err(
                    BindingError::StructInitializerStructFieldWithoutDefaultInitializer {
                        struct_name: self.name().to_string(),
                    },
                ),
            },
            _ => Err(BindingError::StructInitializerBadValueForType {
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

    pub fn initializer_args(
        &self,
        initializer: Handle<Initializer<D>>,
    ) -> impl Iterator<Item = &StructField<F, D>> {
        self.fields
            .iter()
            .filter(move |field| !initializer.values.iter().any(|c| c.name == field.name))
    }

    pub fn fields(&self) -> impl Iterator<Item = &StructField<F, D>> {
        self.fields.iter()
    }

    pub fn has_default_initializer(&self) -> bool {
        self.get_default_initializer().is_some()
    }

    pub fn has_full_initializer(&self) -> bool {
        self.get_full_initializer().is_some()
    }

    pub fn get_default_initializer(&self) -> Option<&Handle<Initializer<D>>> {
        // Are any of the initializers of normal type and initialize ALL of the fields
        self.initializers.iter().find(|c| {
            c.initializer_type == InitializerType::Normal && c.values.len() == self.fields.len()
        })
    }

    pub fn get_full_initializer(&self) -> Option<&Handle<Initializer<D>>> {
        self.initializers.iter().find(|c| c.values.is_empty())
    }
}

#[derive(Debug, Clone)]
pub struct InitializedValue {
    pub name: Name,
    pub value: ValidatedDefaultValue,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum InitializerType {
    /// Normal initializers map to actual language constructors
    /// A name is still required as some languages (e.g. C) don't support
    /// associated constructors and require free-standing functions instead
    Normal,
    /// Static initializers are mapped to static methods in languages that support them (e.g. C++, Java, C#)
    Static,
}

impl InitializerType {
    pub fn is_normal(&self) -> bool {
        *self == InitializerType::Normal
    }
}

// An initializer defines how to construct a struct
#[derive(Debug, Clone)]
pub struct Initializer<D>
where
    D: DocReference,
{
    pub name: Name,
    pub initializer_type: InitializerType,
    pub values: Rc<Vec<InitializedValue>>,
    pub doc: Doc<D>,
}

impl Initializer<Unvalidated> {
    pub(crate) fn validate(&self, lib: &LibraryFields) -> BindResult<Initializer<Validated>> {
        Ok(Initializer {
            name: self.name.clone(),
            initializer_type: self.initializer_type,
            values: self.values.clone(),
            doc: self.doc.validate(&self.name, lib)?,
        })
    }
}

impl<D> Initializer<D>
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
            // this is only a problem if both are normal initializers
            return self.initializer_type.is_normal() && other.initializer_type.is_normal();
        }
        false
    }

    pub fn full(initializer_type: InitializerType, doc: Doc<D>) -> Self {
        Self {
            name: Name::create("some_unused_name").unwrap(),
            initializer_type,
            values: Rc::new(Vec::new()),
            doc,
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum StructType<D>
where
    D: DocReference,
{
    /// structs that may be used as native function parameters
    FunctionArg(Handle<Struct<FunctionArgStructField, D>>),
    /// structs than can be used as native function return values
    FunctionReturn(Handle<Struct<FunctionReturnStructField, D>>),
    /// structs that may be used as callback function arguments in interfaces
    CallbackArg(Handle<Struct<CallbackArgStructField, D>>),
    /// structs that can be used in any context and only contain basic types
    Universal(Handle<Struct<UniversalStructField, D>>),
}

impl StructType<Unvalidated> {
    pub(crate) fn validate(&self, lib: &LibraryFields) -> BindResult<StructType<Validated>> {
        match self {
            StructType::FunctionArg(x) => Ok(StructType::FunctionArg(x.validate(lib)?)),
            StructType::FunctionReturn(x) => Ok(StructType::FunctionReturn(x.validate(lib)?)),
            StructType::CallbackArg(x) => Ok(StructType::CallbackArg(x.validate(lib)?)),
            StructType::Universal(x) => Ok(StructType::Universal(x.validate(lib)?)),
        }
    }
}

impl From<FunctionArgStructHandle> for StructType<Unvalidated> {
    fn from(x: FunctionArgStructHandle) -> Self {
        StructType::FunctionArg(x)
    }
}

impl From<FunctionReturnStructHandle> for StructType<Unvalidated> {
    fn from(x: FunctionReturnStructHandle) -> Self {
        StructType::FunctionReturn(x)
    }
}

impl From<CallbackArgStructHandle> for StructType<Unvalidated> {
    fn from(x: CallbackArgStructHandle) -> Self {
        StructType::CallbackArg(x)
    }
}

impl From<UniversalStructHandle> for StructType<Unvalidated> {
    fn from(x: UniversalStructHandle) -> Self {
        StructType::Universal(x)
    }
}

/// Structs refs can always be the Universal struct type, but may also be a
/// more specific type depending on context
#[derive(Debug, Clone)]
pub enum UniversalDeclarationOr<T>
where
    T: StructFieldType,
{
    Specific(TypedStructDeclaration<T>),
    Universal(UniversalStructDeclaration),
}

impl<T> UniversalDeclarationOr<T>
where
    T: StructFieldType,
{
    pub fn untyped(&self) -> &StructDeclarationHandle {
        match self {
            UniversalDeclarationOr::Specific(x) => &x.inner,
            UniversalDeclarationOr::Universal(x) => &x.inner,
        }
    }
}

impl<T> PartialEq for UniversalDeclarationOr<T>
where
    T: StructFieldType,
{
    fn eq(&self, other: &Self) -> bool {
        match self {
            UniversalDeclarationOr::Specific(y) => match other {
                UniversalDeclarationOr::Specific(x) => y == x,
                UniversalDeclarationOr::Universal(_) => false,
            },
            UniversalDeclarationOr::Universal(x) => match other {
                UniversalDeclarationOr::Specific(_) => false,
                UniversalDeclarationOr::Universal(y) => x == y,
            },
        }
    }
}

impl<T> Eq for UniversalDeclarationOr<T> where T: StructFieldType {}

/// Structs can always be the Universal struct type, but may also be a
/// more specific type depending on context
#[derive(Debug, Clone, Eq)]
pub enum UniversalOr<T>
where
    T: StructFieldType,
{
    Specific(Handle<Struct<T, Unvalidated>>),
    Universal(UniversalStructHandle),
}

impl<T> PartialEq for UniversalOr<T>
where
    T: StructFieldType,
{
    fn eq(&self, other: &Self) -> bool {
        match self {
            UniversalOr::Specific(y) => match other {
                UniversalOr::Specific(x) => y == x,
                UniversalOr::Universal(_) => false,
            },
            UniversalOr::Universal(x) => match other {
                UniversalOr::Specific(_) => false,
                UniversalOr::Universal(y) => x == y,
            },
        }
    }
}

impl<T> UniversalOr<T>
where
    T: StructFieldType,
{
    pub fn name(&self) -> &Name {
        match self {
            UniversalOr::Specific(x) => x.name(),
            UniversalOr::Universal(x) => x.name(),
        }
    }

    pub fn declaration(&self) -> StructDeclarationHandle {
        match self {
            UniversalOr::Specific(x) => x.declaration.inner.clone(),
            UniversalOr::Universal(x) => x.declaration.inner.clone(),
        }
    }

    pub fn typed_declaration(&self) -> UniversalDeclarationOr<T> {
        match self {
            UniversalOr::Specific(x) => UniversalDeclarationOr::Specific(x.declaration.clone()),
            UniversalOr::Universal(x) => UniversalDeclarationOr::Universal(x.declaration.clone()),
        }
    }

    pub fn to_struct_type(&self) -> StructType<Unvalidated> {
        match self {
            UniversalOr::Specific(x) => T::create_struct_type(x.clone()),
            UniversalOr::Universal(x) => StructType::Universal(x.clone()),
        }
    }
}

impl<T> InitializerValidator for UniversalOr<T>
where
    T: StructFieldType,
{
    fn validate_default_value(
        &self,
        value: &InitializerDefault,
    ) -> BindResult<ValidatedDefaultValue> {
        match self {
            UniversalOr::Specific(x) => x.validate_default_value(value),
            UniversalOr::Universal(x) => x.validate_default_value(value),
        }
    }
}

impl<T> From<Handle<Struct<T, Unvalidated>>> for UniversalOr<T>
where
    T: StructFieldType,
{
    fn from(x: Handle<Struct<T, Unvalidated>>) -> Self {
        UniversalOr::Specific(x)
    }
}

impl<D> StructType<D>
where
    D: DocReference,
{
    pub fn initializers(&self) -> &[Handle<Initializer<D>>] {
        match self {
            StructType::FunctionArg(x) => &x.initializers,
            StructType::FunctionReturn(x) => &x.initializers,
            StructType::CallbackArg(x) => &x.initializers,
            StructType::Universal(x) => &x.initializers,
        }
    }

    pub fn name(&self) -> &Name {
        match self {
            StructType::FunctionArg(x) => x.name(),
            StructType::CallbackArg(x) => x.name(),
            StructType::FunctionReturn(x) => x.name(),
            StructType::Universal(x) => x.name(),
        }
    }

    pub fn doc(&self) -> &Doc<D> {
        match self {
            StructType::FunctionArg(x) => &x.doc,
            StructType::FunctionReturn(x) => &x.doc,
            StructType::CallbackArg(x) => &x.doc,
            StructType::Universal(x) => &x.doc,
        }
    }

    pub fn settings(&self) -> &LibrarySettings {
        match self {
            StructType::FunctionArg(x) => &x.declaration.inner.settings,
            StructType::FunctionReturn(x) => &x.declaration.inner.settings,
            StructType::CallbackArg(x) => &x.declaration.inner.settings,
            StructType::Universal(x) => &x.declaration.inner.settings,
        }
    }

    pub fn declaration(&self) -> StructDeclarationHandle {
        match self {
            StructType::FunctionArg(x) => x.declaration.inner.clone(),
            StructType::CallbackArg(x) => x.declaration.inner.clone(),
            StructType::FunctionReturn(x) => x.declaration.inner.clone(),
            StructType::Universal(x) => x.declaration.inner.clone(),
        }
    }

    pub fn find_field_name(&self, name: &str) -> Option<Name> {
        match self {
            StructType::FunctionArg(x) => x.find_field_name(name),
            StructType::CallbackArg(x) => x.find_field_name(name),
            StructType::FunctionReturn(x) => x.find_field_name(name),
            StructType::Universal(x) => x.find_field_name(name),
        }
    }

    pub fn get_default_initializer(&self) -> Option<&Handle<Initializer<D>>> {
        match self {
            StructType::FunctionArg(x) => x.get_default_initializer(),
            StructType::FunctionReturn(x) => x.get_default_initializer(),
            StructType::CallbackArg(x) => x.get_default_initializer(),
            StructType::Universal(x) => x.get_default_initializer(),
        }
    }
}
