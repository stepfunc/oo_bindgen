use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use crate::class::*;
use crate::collection::{Collection, CollectionHandle};
use crate::constants::{ConstantSet, ConstantSetBuilder};
use crate::doc::{Doc, DocReference, Unvalidated, Validated};
use crate::enum_type::{EnumBuilder, EnumHandle};
use crate::error_type::{ErrorType, ErrorTypeBuilder, ExceptionType};
use crate::function::{FunctionBuilder, FunctionHandle};
use crate::interface::{InterfaceBuilder, InterfaceHandle};
use crate::iterator::{IteratorHandle, IteratorItemType};
use crate::name::{BadName, IntoName, Name};
use crate::structs::*;
use crate::types::{TypeValidator, ValidatedType};
use crate::*;
use crate::{BindingError, Version};
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum Statement<D>
where
    D: DocReference,
{
    Constants(Handle<ConstantSet<D>>),
    StructDeclaration(StructDeclarationHandle),
    StructDefinition(StructType<D>),
    EnumDefinition(Handle<Enum<D>>),
    ErrorType(ErrorType<D>),
    ClassDeclaration(ClassDeclarationHandle),
    ClassDefinition(Handle<Class<D>>),
    StaticClassDefinition(Handle<StaticClass<D>>),
    InterfaceDefinition(Handle<Interface<D>>),
    IteratorDeclaration(Handle<crate::iterator::Iterator<D>>),
    CollectionDeclaration(Handle<Collection<D>>),
    FunctionDefinition(Handle<Function<D>>),
}

impl Statement<Unvalidated> {
    pub(crate) fn unique_name(&self) -> Option<&Name> {
        match self {
            Statement::Constants(x) => Some(&x.name),
            Statement::StructDeclaration(x) => Some(&x.name),
            Statement::StructDefinition(_) => {
                // the name is shared with the declaration
                None
            }
            Statement::EnumDefinition(x) => Some(&x.name),
            Statement::ErrorType(x) => Some(&x.exception_name),
            Statement::ClassDeclaration(x) => Some(&x.name),
            Statement::ClassDefinition(_) => {
                // the name is shared with the declaration
                None
            }
            Statement::StaticClassDefinition(x) => Some(&x.name),
            Statement::InterfaceDefinition(x) => Some(&x.name),
            Statement::IteratorDeclaration(_) => {
                // the name is derived in a language specific way
                None
            }
            Statement::CollectionDeclaration(_) => {
                // the name is derived in a language specific way
                None
            }
            Statement::FunctionDefinition(x) => Some(&x.name),
        }
    }
}

pub struct DeveloperInfo {
    /// Full name of the developer
    pub name: String,
    /// Email of the developer
    pub email: String,
    /// Name of the organization the developer is working for
    pub organization: String,
    /// Organization website URL
    pub organization_url: String,
}

/// metadata related to the library
pub struct LibraryInfo {
    /// Description of the library
    pub description: String,
    /// URL of the project
    pub project_url: String,
    /// GitHub organisation and repo name (e.g. stepfunc/oo_bindgen)
    pub repository: String,
    /// License name
    pub license_name: String,
    /// Short description of the license (to put on every generated file)
    pub license_description: Vec<String>,
    /// Path to the license file from the root directory
    pub license_path: PathBuf,
    /// List of developers
    pub developers: Vec<DeveloperInfo>,
}

/// Settings that affect iterator function naming
#[derive(Debug)]
pub struct IteratorSettings {
    /// name of the C function which retrieve's the iterator's next value
    /// is automatically generated as `<c_ffi_prefix>_<iterator_class_name>_<next_function_suffix>`
    pub next_function_suffix: Name,
}

impl IteratorSettings {
    pub fn new(name: &'static str) -> Result<IteratorSettings, BadName> {
        Ok(Self {
            next_function_suffix: Name::create(name)?,
        })
    }

    pub fn default() -> Result<IteratorSettings, BadName> {
        Ok(Self {
            next_function_suffix: Name::create("next")?,
        })
    }
}

/// Settings that affect collection function naming
#[derive(Debug)]
pub struct CollectionSettings {
    /// name of the C function which creates a collection
    /// is automatically generated as `<c_ffi_prefix>_<collection_class_name>_<create_function_suffix>`
    pub create_function_suffix: Name,
    /// name of the C function which creates a collection
    /// is automatically generated as `<c_ffi_prefix>_<collection_class_name>_<add_function_suffix>`
    pub add_function_suffix: Name,
    /// name of the C function which destroys a collection
    /// is automatically generated as `<c_ffi_prefix>_<collection_class_name>_<destroy_function_suffix>`
    pub destroy_function_suffix: Name,
}

impl CollectionSettings {
    pub fn default() -> Result<CollectionSettings, BadName> {
        Ok(Self {
            create_function_suffix: Name::create("create")?,
            add_function_suffix: Name::create("add")?,
            destroy_function_suffix: Name::create("destroy")?,
        })
    }
}

/// Settings that affect the names of things
#[derive(Debug)]
pub struct LibrarySettings {
    /// name of the library
    pub name: Name,
    /// prefix given to all API types, e.g. structs, enums, functions, etc
    pub c_ffi_prefix: Name,
    /// settings that control iterator generation
    pub iterator: IteratorSettings,
    /// settings that control collection generation
    pub collection: CollectionSettings,
}

impl LibrarySettings {
    /// create an RC to the settings that is cheaply cloned
    pub fn create<S: IntoName, R: IntoName>(
        name: S,
        c_ffi_prefix: R,
        iterator: IteratorSettings,
        collection: CollectionSettings,
    ) -> BindResult<Rc<Self>> {
        Ok(Rc::new(Self {
            name: name.into_name()?,
            c_ffi_prefix: c_ffi_prefix.into_name()?,
            iterator,
            collection,
        }))
    }
}

pub struct Library {
    pub version: Version,
    pub info: Rc<LibraryInfo>,
    pub settings: Rc<LibrarySettings>,
    /// history of statements from which we can find other types
    statements: Vec<Statement<Validated>>,
}

impl Library {
    pub fn statements(&self) -> impl Iterator<Item = &Statement<Validated>> {
        self.statements.iter()
    }

    pub fn functions(&self) -> impl Iterator<Item = &Handle<Function<Validated>>> {
        self.statements().filter_map(|statement| match statement {
            Statement::FunctionDefinition(handle) => Some(handle),
            _ => None,
        })
    }

    pub fn async_method_interfaces(&self) -> impl Iterator<Item = &Handle<Interface<Validated>>> {
        let mut async_function_interfaces: HashSet<Handle<Interface<Validated>>> = HashSet::new();
        for c in self.classes() {
            for method in &c.async_methods {
                async_function_interfaces.insert(method.one_time_callback.clone());
            }
        }
        self.interfaces()
            .filter(move |x| async_function_interfaces.contains(x))
    }

    pub fn structs(&self) -> impl Iterator<Item = &StructType<Validated>> {
        self.statements
            .iter()
            .filter_map(|statement| match statement {
                Statement::StructDefinition(x) => Some(x),
                _ => None,
            })
    }

    pub fn constants(&self) -> impl Iterator<Item = &Handle<ConstantSet<Validated>>> {
        self.statements().filter_map(|statement| match statement {
            Statement::Constants(handle) => Some(handle),
            _ => None,
        })
    }

    pub fn enums(&self) -> impl Iterator<Item = &Handle<Enum<Validated>>> {
        self.statements().filter_map(|statement| match statement {
            Statement::EnumDefinition(handle) => Some(handle),
            _ => None,
        })
    }

    pub fn classes(&self) -> impl Iterator<Item = &Handle<Class<Validated>>> {
        self.statements().filter_map(|statement| match statement {
            Statement::ClassDefinition(handle) => Some(handle),
            _ => None,
        })
    }

    pub fn error_types(&self) -> impl Iterator<Item = &ErrorType<Validated>> {
        self.statements().filter_map(|statement| match statement {
            Statement::ErrorType(err) => Some(err),
            _ => None,
        })
    }

    pub fn static_classes(&self) -> impl Iterator<Item = &Handle<StaticClass<Validated>>> {
        self.statements().filter_map(|statement| match statement {
            Statement::StaticClassDefinition(handle) => Some(handle),
            _ => None,
        })
    }

    pub fn interfaces(&self) -> impl Iterator<Item = &Handle<Interface<Validated>>> {
        self.statements().filter_map(|statement| match statement {
            Statement::InterfaceDefinition(handle) => Some(handle),
            _ => None,
        })
    }

    pub fn iterators(&self) -> impl Iterator<Item = &Handle<crate::iterator::Iterator<Validated>>> {
        self.statements().filter_map(|statement| match statement {
            Statement::IteratorDeclaration(handle) => Some(handle),
            _ => None,
        })
    }

    pub fn collections(&self) -> impl Iterator<Item = &Handle<Collection<Validated>>> {
        self.statements().filter_map(|statement| match statement {
            Statement::CollectionDeclaration(handle) => Some(handle),
            _ => None,
        })
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
    pub(crate) fn validate(&self, lib: &UnvalidatedFields) -> BindResult<StructType<Validated>> {
        match self {
            StructType::FunctionArg(x) => {
                Ok(StructType::FunctionArg(Handle::new(x.validate(lib)?)))
            }
            StructType::FunctionReturn(x) => {
                Ok(StructType::FunctionReturn(Handle::new(x.validate(lib)?)))
            }
            StructType::CallbackArg(x) => {
                Ok(StructType::CallbackArg(Handle::new(x.validate(lib)?)))
            }
            StructType::Universal(x) => Ok(StructType::Universal(Handle::new(x.validate(lib)?))),
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

impl<T> ConstructorValidator for UniversalOr<T>
where
    T: StructFieldType,
{
    fn validate_constructor_default(
        &self,
        value: &ConstructorDefault,
    ) -> BindResult<ValidatedConstructorDefault> {
        match self {
            UniversalOr::Specific(x) => x.validate_constructor_default(value),
            UniversalOr::Universal(x) => x.validate_constructor_default(value),
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

impl<T> TypeValidator for UniversalOr<T>
where
    T: StructFieldType,
{
    fn get_validated_type(&self) -> Option<ValidatedType> {
        Some(ValidatedType::Struct(self.to_struct_type()))
    }
}

impl<D> StructType<D>
where
    D: DocReference,
{
    pub fn constructors(&self) -> &[Handle<Constructor<D>>] {
        match self {
            StructType::FunctionArg(x) => &x.constructors,
            StructType::FunctionReturn(x) => &x.constructors,
            StructType::CallbackArg(x) => &x.constructors,
            StructType::Universal(x) => &x.constructors,
        }
    }

    pub fn name(&self) -> &str {
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

    pub fn get_default_constructor(&self) -> Option<&Handle<Constructor<D>>> {
        match self {
            StructType::FunctionArg(x) => x.get_default_constructor(),
            StructType::FunctionReturn(x) => x.get_default_constructor(),
            StructType::CallbackArg(x) => x.get_default_constructor(),
            StructType::Universal(x) => x.get_default_constructor(),
        }
    }
}

pub(crate) struct UnvalidatedFields {
    // a record of statements preserved in order
    pub(crate) statements: Vec<Statement<Unvalidated>>,

    // native stuff
    pub(crate) structs_declarations: HashSet<StructDeclarationHandle>,
    pub(crate) structs: HashMap<StructDeclarationHandle, StructType<Unvalidated>>,
    pub(crate) functions: HashSet<Handle<Function<Unvalidated>>>,
    pub(crate) enums: HashSet<Handle<Enum<Unvalidated>>>,

    // oo stuff
    pub(crate) class_declarations: HashSet<ClassDeclarationHandle>,
    pub(crate) classes: HashMap<ClassDeclarationHandle, Handle<Class<Unvalidated>>>,
    pub(crate) static_classes: HashSet<Handle<StaticClass<Unvalidated>>>,
    pub(crate) interfaces: HashSet<Handle<Interface<Unvalidated>>>,

    // specialized types
    pub(crate) iterators: HashSet<Handle<crate::iterator::Iterator<Unvalidated>>>,
    pub(crate) collections: HashSet<Handle<Collection<Unvalidated>>>,
}

impl UnvalidatedFields {
    pub(crate) fn find_struct<T: AsRef<str>>(&self, name: T) -> Option<&StructType<Unvalidated>> {
        self.structs.values().find(|x| x.name() == name.as_ref())
    }

    pub(crate) fn find_enum<T: AsRef<str>>(&self, name: T) -> Option<&Handle<Enum<Unvalidated>>> {
        self.enums.iter().find(|x| x.name == name.as_ref())
    }

    pub(crate) fn find_class_declaration<T: AsRef<str>>(
        &self,
        name: T,
    ) -> Option<&ClassDeclarationHandle> {
        self.class_declarations
            .iter()
            .find(|x| x.name == name.as_ref())
    }

    pub(crate) fn find_class<T: AsRef<str>>(&self, name: T) -> Option<&Handle<Class<Unvalidated>>> {
        self.classes.values().find(|x| x.name() == name.as_ref())
    }

    pub(crate) fn find_interface<T: AsRef<str>>(
        &self,
        name: T,
    ) -> Option<&Handle<Interface<Unvalidated>>> {
        self.interfaces.iter().find(|x| x.name == name.as_ref())
    }

    pub(crate) fn new() -> Self {
        Self {
            statements: Vec::new(),

            structs_declarations: HashSet::new(),
            structs: HashMap::new(),

            enums: HashSet::new(),

            class_declarations: HashSet::new(),
            classes: HashMap::new(),
            static_classes: HashSet::new(),

            interfaces: HashSet::new(),

            iterators: HashSet::new(),
            collections: HashSet::new(),

            functions: HashSet::new(),
        }
    }
}

pub struct LibraryBuilder {
    version: Version,
    info: Rc<LibraryInfo>,

    pub(crate) settings: Rc<LibrarySettings>,

    // names of symbols used in the library
    symbol_names: HashSet<String>,
    fields: UnvalidatedFields,
}

impl LibraryBuilder {
    pub fn new(version: Version, info: LibraryInfo, settings: Rc<LibrarySettings>) -> Self {
        Self {
            version,
            info: Rc::new(info),
            settings,
            symbol_names: HashSet::new(),
            fields: UnvalidatedFields::new(),
        }
    }

    pub(crate) fn add_statement(&mut self, statement: Statement<Unvalidated>) -> BindResult<()> {
        if let Some(name) = statement.unique_name() {
            self.check_unique_symbol(name)?;
        }

        self.fields.statements.push(statement.clone());

        match statement {
            Statement::Constants(_) => {}
            Statement::StructDeclaration(x) => {
                self.fields.structs_declarations.insert(x);
            }
            Statement::StructDefinition(x) => {
                self.fields.structs.insert(x.declaration(), x);
            }
            Statement::EnumDefinition(x) => {
                self.fields.enums.insert(x);
            }
            Statement::ErrorType(_) => {}
            Statement::ClassDeclaration(x) => {
                self.fields.class_declarations.insert(x);
            }
            Statement::ClassDefinition(x) => {
                self.fields.classes.insert(x.declaration.clone(), x);
            }
            Statement::StaticClassDefinition(x) => {
                self.fields.static_classes.insert(x);
            }
            Statement::InterfaceDefinition(x) => {
                self.fields.interfaces.insert(x);
            }
            Statement::IteratorDeclaration(x) => {
                self.fields.iterators.insert(x);
            }
            Statement::CollectionDeclaration(x) => {
                self.fields.collections.insert(x);
            }
            Statement::FunctionDefinition(x) => {
                self.fields.functions.insert(x);
            }
        }

        Ok(())
    }

    fn validate_statement(
        &self,
        statement: &Statement<Unvalidated>,
    ) -> BindResult<Statement<Validated>> {
        match statement {
            Statement::Constants(x) => {
                Ok(Statement::Constants(Handle::new(x.validate(&self.fields)?)))
            }
            Statement::StructDeclaration(x) => Ok(Statement::StructDeclaration(x.clone())),
            Statement::StructDefinition(x) => {
                Ok(Statement::StructDefinition(x.validate(&self.fields)?))
            }
            Statement::EnumDefinition(x) => Ok(Statement::EnumDefinition(Handle::new(
                x.validate(&self.fields)?,
            ))),
            Statement::ErrorType(x) => Ok(Statement::ErrorType(x.validate(&self.fields)?)),
            Statement::ClassDeclaration(x) => Ok(Statement::ClassDeclaration(x.clone())),
            Statement::ClassDefinition(x) => Ok(Statement::ClassDefinition(Handle::new(
                x.validate(&self.fields)?,
            ))),
            Statement::StaticClassDefinition(x) => Ok(Statement::StaticClassDefinition(
                Handle::new(x.validate(&self.fields)?),
            )),
            Statement::InterfaceDefinition(x) => Ok(Statement::InterfaceDefinition(Handle::new(
                x.validate(&self.fields)?,
            ))),
            Statement::IteratorDeclaration(x) => Ok(Statement::IteratorDeclaration(Handle::new(
                x.validate(&self.fields)?,
            ))),
            Statement::CollectionDeclaration(x) => Ok(Statement::CollectionDeclaration(
                Handle::new(x.validate(&self.fields)?),
            )),
            Statement::FunctionDefinition(x) => Ok(Statement::FunctionDefinition(Handle::new(
                x.validate(&self.fields)?,
            ))),
        }
    }

    pub fn build(self) -> BindResult<Library> {
        let statements: BindResult<Vec<Statement<Validated>>> = self
            .fields
            .statements
            .iter()
            .map(|s| self.validate_statement(s))
            .collect();

        let lib = Library {
            version: self.version.clone(),
            info: self.info.clone(),
            settings: self.settings,
            statements: statements?,
        };

        Ok(lib)
    }

    pub fn define_error_type<T: IntoName>(
        &mut self,
        error_name: T,
        exception_name: T,
        exception_type: ExceptionType,
    ) -> BindResult<ErrorTypeBuilder> {
        let exception_name = exception_name.into_name()?;
        let builder = self
            .define_enum(error_name)?
            .push("ok", "Success, i.e. no error occurred")?;

        Ok(ErrorTypeBuilder::new(
            exception_name,
            exception_type,
            builder,
        ))
    }

    pub fn define_constants<T: IntoName>(&mut self, name: T) -> BindResult<ConstantSetBuilder> {
        Ok(ConstantSetBuilder::new(self, name.into_name()?))
    }

    pub(crate) fn declare_struct<T: IntoName>(
        &mut self,
        name: T,
    ) -> BindResult<StructDeclarationHandle> {
        let name = name.into_name()?;
        let handle =
            StructDeclarationHandle::new(StructDeclaration::new(name, self.settings.clone()));
        self.add_statement(Statement::StructDeclaration(handle.clone()))?;
        Ok(handle)
    }

    pub fn declare_universal_struct<T: IntoName>(
        &mut self,
        name: T,
    ) -> BindResult<UniversalStructDeclaration> {
        Ok(UniversalStructDeclaration::new(
            self.declare_struct(name.into_name()?)?,
        ))
    }

    pub fn declare_function_arg_struct<T: IntoName>(
        &mut self,
        name: T,
    ) -> BindResult<FunctionArgStructDeclaration> {
        Ok(FunctionArgStructDeclaration::new(
            self.declare_struct(name.into_name()?)?,
        ))
    }

    pub fn declare_function_return_struct<T: IntoName>(
        &mut self,
        name: T,
    ) -> BindResult<FunctionReturnStructDeclaration> {
        Ok(FunctionReturnStructDeclaration::new(
            self.declare_struct(name.into_name()?)?,
        ))
    }

    pub fn declare_callback_arg_struct<T: IntoName>(
        &mut self,
        name: T,
    ) -> BindResult<CallbackArgStructDeclaration> {
        Ok(CallbackArgStructDeclaration::new(
            self.declare_struct(name.into_name()?)?,
        ))
    }

    /// Define a structure that can be used in any context.
    ///
    /// Backends will generate bi-directional conversion routines
    /// for this type of struct.
    pub fn define_universal_struct(
        &mut self,
        declaration: UniversalStructDeclaration,
    ) -> BindResult<UniversalStructBuilder> {
        self.validate_struct_declaration(&declaration.inner)?;
        if self.fields.structs.contains_key(&declaration.inner) {
            Err(BindingError::StructAlreadyDefined {
                handle: declaration.inner,
            })
        } else {
            Ok(UniversalStructBuilder::new(self, declaration))
        }
    }

    /// Define an opaque structure which must be of universal type
    pub fn define_opaque_struct(
        &mut self,
        declaration: UniversalStructDeclaration,
    ) -> BindResult<UniversalStructBuilder> {
        self.validate_struct_declaration(&declaration.inner)?;
        if self.fields.structs.contains_key(&declaration.inner) {
            Err(BindingError::StructAlreadyDefined {
                handle: declaration.inner,
            })
        } else {
            Ok(UniversalStructBuilder::opaque(self, declaration))
        }
    }

    /// Define a structure that can be only be used in callback function arguments
    pub fn define_callback_argument_struct<T>(
        &mut self,
        declaration: T,
    ) -> BindResult<CallbackArgStructBuilder>
    where
        T: Into<CallbackArgStructDeclaration>,
    {
        let declaration = declaration.into();
        self.validate_struct_declaration(&declaration.inner)?;
        if self.fields.structs.contains_key(&declaration.inner) {
            Err(BindingError::StructAlreadyDefined {
                handle: declaration.inner,
            })
        } else {
            Ok(CallbackArgStructBuilder::new(self, declaration))
        }
    }

    /// Define a structure that can only be used as function return value
    pub fn define_function_return_struct<T>(
        &mut self,
        declaration: T,
    ) -> BindResult<FunctionReturnStructBuilder>
    where
        T: Into<FunctionReturnStructDeclaration>,
    {
        let declaration = declaration.into();
        self.validate_struct_declaration(&declaration.inner)?;
        if self.fields.structs.contains_key(&declaration.inner) {
            Err(BindingError::StructAlreadyDefined {
                handle: declaration.inner,
            })
        } else {
            Ok(FunctionReturnStructBuilder::new(self, declaration))
        }
    }

    /// Define a structure that can only be be used as a function argument
    pub fn define_function_argument_struct<T>(
        &mut self,
        declaration: T,
    ) -> BindResult<FunctionArgStructBuilder>
    where
        T: Into<FunctionArgStructDeclaration>,
    {
        let declaration = declaration.into();
        self.validate_struct_declaration(&declaration.inner)?;
        if self.fields.structs.contains_key(&declaration.inner) {
            Err(BindingError::StructAlreadyDefined {
                handle: declaration.inner,
            })
        } else {
            Ok(FunctionArgStructBuilder::new(self, declaration))
        }
    }

    /// Define an enumeration
    pub fn define_enum<T: IntoName>(&mut self, name: T) -> BindResult<EnumBuilder> {
        Ok(EnumBuilder::new(self, name.into_name()?))
    }

    pub fn define_function<T: IntoName>(&mut self, name: T) -> BindResult<FunctionBuilder> {
        Ok(FunctionBuilder::new(self, name.into_name()?))
    }

    pub fn declare_class<T: IntoName>(&mut self, name: T) -> BindResult<ClassDeclarationHandle> {
        self.declare_any_class(name, ClassType::Normal)
    }

    fn declare_iterator<T: IntoName>(&mut self, name: T) -> BindResult<IteratorClassDeclaration> {
        Ok(IteratorClassDeclaration::new(
            self.declare_any_class(name, ClassType::Iterator)?,
        ))
    }

    fn declare_collection<T: IntoName>(
        &mut self,
        name: T,
    ) -> BindResult<CollectionClassDeclaration> {
        Ok(CollectionClassDeclaration::new(self.declare_any_class(
            name.into_name()?,
            ClassType::Collection,
        )?))
    }

    fn declare_any_class<T: IntoName>(
        &mut self,
        name: T,
        class_type: ClassType,
    ) -> BindResult<ClassDeclarationHandle> {
        let name = name.into_name()?;
        let handle = ClassDeclarationHandle::new(ClassDeclaration::new(
            name,
            class_type,
            self.settings.clone(),
        ));
        self.add_statement(Statement::ClassDeclaration(handle.clone()))?;
        Ok(handle)
    }

    pub fn define_class(
        &mut self,
        declaration: &ClassDeclarationHandle,
    ) -> BindResult<ClassBuilder> {
        if declaration.class_type != ClassType::Normal {
            return Err(BindingError::WrongClassType {
                expected: ClassType::Normal,
                received: declaration.class_type,
            });
        }
        self.validate_class_declaration(declaration)?;
        if !self.fields.classes.contains_key(declaration) {
            Ok(ClassBuilder::new(self, declaration.clone()))
        } else {
            Err(BindingError::ClassAlreadyDefined {
                handle: declaration.clone(),
            })
        }
    }

    pub fn define_static_class<T: IntoName>(&mut self, name: T) -> BindResult<StaticClassBuilder> {
        Ok(StaticClassBuilder::new(self, name.into_name()?))
    }

    pub fn define_synchronous_interface<T: IntoName, D: Into<Doc<Unvalidated>>>(
        &mut self,
        name: T,
        doc: D,
    ) -> BindResult<InterfaceBuilder> {
        self.define_interface(name, InterfaceType::Synchronous, doc)
    }

    pub fn define_asynchronous_interface<T: IntoName, D: Into<Doc<Unvalidated>>>(
        &mut self,
        name: T,
        doc: D,
    ) -> BindResult<InterfaceBuilder> {
        self.define_interface(name, InterfaceType::Asynchronous, doc)
    }

    fn define_interface<T: IntoName, D: Into<Doc<Unvalidated>>>(
        &mut self,
        name: T,
        interface_type: InterfaceType,
        doc: D,
    ) -> BindResult<InterfaceBuilder> {
        Ok(InterfaceBuilder::new(
            self,
            name.into_name()?,
            interface_type,
            doc.into(),
        ))
    }

    pub fn define_iterator<N: IntoName, T: Into<IteratorItemType>>(
        &mut self,
        class_name: N,
        item_type: T,
    ) -> BindResult<IteratorHandle> {
        self.define_iterator_impl(class_name, false, item_type)
    }

    pub fn define_iterator_with_lifetime<N: IntoName, T: Into<IteratorItemType>>(
        &mut self,
        class_name: N,
        item_type: T,
    ) -> BindResult<IteratorHandle> {
        self.define_iterator_impl(class_name, true, item_type)
    }

    fn define_iterator_impl<N: IntoName, T: Into<IteratorItemType>>(
        &mut self,
        class_name: N,
        has_lifetime: bool,
        item_type: T,
    ) -> BindResult<IteratorHandle> {
        let class_name = class_name.into_name()?;
        let item_type = item_type.into();

        let class = self.declare_iterator(&class_name)?;
        let next_function = self
            .define_function(class_name.append(&self.settings.iterator.next_function_suffix))?
            .param(
                "iter",
                class.clone(),
                "opaque iterator on which to retrieve the next value",
            )?
            .doc("returns a pointer to the next value or NULL")?
            .returns(item_type.get_function_return_value(), "next value or NULL")?
            .build()?;

        let iter = IteratorHandle::new(crate::iterator::Iterator::new(
            has_lifetime,
            class.inner,
            next_function,
            item_type,
            self.settings.clone(),
        ));
        self.add_statement(Statement::IteratorDeclaration(iter.clone()))?;
        Ok(iter)
    }

    pub fn define_collection<N: IntoName, A: Into<FunctionArgument>>(
        &mut self,
        class_name: N,
        value_type: A,
        has_reserve: bool,
    ) -> BindResult<CollectionHandle> {
        let class_name = class_name.into_name()?;
        let value_type = value_type.into();

        let class_decl = self.declare_collection(&class_name)?;

        let builder = self
            .define_function(class_name.append(&self.settings.collection.create_function_suffix))?
            .doc("Creates an instance of the collection")?;

        let builder = if has_reserve {
            builder.param(
                "reserve_size",
                BasicType::U32,
                "preallocate a particular size",
            )?
        } else {
            builder
        };

        let create_func = builder
            .returns(class_decl.clone(), "Allocated opaque collection instance")?
            .build()?;

        let destroy_func = self
            .define_function(class_name.append(&self.settings.collection.destroy_function_suffix))?
            .doc("Destroys an instance of the collection")?
            .param("instance", class_decl.clone(), "instance to destroy")?
            .returns_nothing()?
            .build()?;

        let add_func = self
            .define_function(class_name.append(&self.settings.collection.add_function_suffix))?
            .doc("Add a value to the collection")?
            .param(
                "instance",
                class_decl.clone(),
                "instance to which to add the value",
            )?
            .param("value", value_type.clone(), "value to add to the instance")?
            .returns_nothing()?
            .build()?;

        let collection = Handle::new(crate::collection::Collection::new(
            class_decl.inner,
            value_type,
            create_func,
            destroy_func,
            add_func,
            has_reserve,
        ));

        self.add_statement(Statement::CollectionDeclaration(collection.clone()))?;
        Ok(collection)
    }

    fn check_unique_symbol(&mut self, name: &Name) -> BindResult<()> {
        if self.symbol_names.insert(name.to_string()) {
            Ok(())
        } else {
            Err(BindingError::SymbolAlreadyUsed { name: name.clone() })
        }
    }

    pub(crate) fn validate_function(&self, native_function: &FunctionHandle) -> BindResult<()> {
        if self.fields.functions.contains(native_function) {
            Ok(())
        } else {
            Err(BindingError::FunctionNotPartOfThisLib {
                handle: native_function.clone(),
            })
        }
    }

    pub(crate) fn validate_type<T>(&self, type_to_validate: &T) -> BindResult<()>
    where
        T: TypeValidator,
    {
        match type_to_validate.get_validated_type() {
            Some(x) => match x {
                ValidatedType::Enum(x) => self.validate_enum(&x),
                ValidatedType::StructRef(x) => self.validate_struct_declaration(&x),
                ValidatedType::Struct(x) => self.validate_struct(&x),
                ValidatedType::Interface(x) => self.validate_interface(&x),
                ValidatedType::ClassRef(x) => self.validate_class_declaration(&x),
                ValidatedType::Iterator(x) => self.validate_iterator(&x),
                ValidatedType::Collection(x) => self.validate_collection(&x),
            },
            None => Ok(()),
        }
    }

    fn validate_struct_declaration(
        &self,
        native_struct: &StructDeclarationHandle,
    ) -> BindResult<()> {
        if self.fields.structs_declarations.contains(native_struct) {
            Ok(())
        } else {
            Err(BindingError::StructNotPartOfThisLib {
                handle: native_struct.clone(),
            })
        }
    }

    fn validate_struct(&self, native_struct: &StructType<Unvalidated>) -> BindResult<()> {
        if self
            .fields
            .structs
            .contains_key(&native_struct.declaration())
        {
            Ok(())
        } else {
            Err(BindingError::StructNotPartOfThisLib {
                handle: native_struct.declaration(),
            })
        }
    }

    fn validate_enum(&self, native_enum: &EnumHandle) -> BindResult<()> {
        if self.fields.enums.contains(native_enum) {
            Ok(())
        } else {
            Err(BindingError::EnumNotPartOfThisLib {
                handle: native_enum.clone(),
            })
        }
    }

    fn validate_interface(&self, interface: &InterfaceHandle) -> BindResult<()> {
        if self.fields.interfaces.contains(interface) {
            Ok(())
        } else {
            Err(BindingError::InterfaceNotPartOfThisLib {
                handle: interface.clone(),
            })
        }
    }

    fn validate_class_declaration(
        &self,
        class_declaration: &ClassDeclarationHandle,
    ) -> BindResult<()> {
        if self.fields.class_declarations.contains(class_declaration) {
            Ok(())
        } else {
            Err(BindingError::ClassNotPartOfThisLib {
                handle: class_declaration.clone(),
            })
        }
    }

    fn validate_iterator(&self, iter: &IteratorHandle) -> BindResult<()> {
        if self.fields.iterators.contains(iter) {
            Ok(())
        } else {
            Err(BindingError::IteratorNotPartOfThisLib {
                handle: iter.clone(),
            })
        }
    }

    fn validate_collection(&self, collection: &CollectionHandle) -> BindResult<()> {
        if self.fields.collections.contains(collection) {
            Ok(())
        } else {
            Err(BindingError::CollectionNotPartOfThisLib {
                handle: collection.clone(),
            })
        }
    }
}

impl From<UniversalStructDeclaration> for FunctionReturnStructDeclaration {
    fn from(x: UniversalStructDeclaration) -> Self {
        FunctionReturnStructDeclaration::new(x.inner)
    }
}
