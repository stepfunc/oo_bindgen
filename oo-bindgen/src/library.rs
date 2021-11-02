use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use crate::class::*;
use crate::collection::CollectionHandle;
use crate::constants::{ConstantSetBuilder, ConstantSetHandle};
use crate::doc::Doc;
use crate::enum_type::{EnumBuilder, EnumHandle};
use crate::error_type::{ErrorType, ErrorTypeBuilder, ExceptionType};
use crate::function::{FunctionBuilder, FunctionHandle};
use crate::interface::{InterfaceBuilder, InterfaceHandle};
use crate::iterator::IteratorHandle;
use crate::name::{BadName, IntoName, Name};
use crate::structs::*;
use crate::types::{TypeValidator, ValidatedType};
use crate::*;
use crate::{BindingError, Version};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Symbol {
    Function(FunctionHandle),
    Struct(StructType),
    Enum(EnumHandle),
    Class(ClassHandle),
    StaticClass(StaticClassHandle),
    Interface(InterfaceHandle),
    Iterator(IteratorHandle),
    Collection(CollectionHandle),
}

#[derive(Clone, Debug)]
pub enum Statement {
    Constants(ConstantSetHandle),
    StructDeclaration(StructDeclarationHandle),
    StructDefinition(StructType),
    EnumDefinition(EnumHandle),
    ErrorType(ErrorType),
    ClassDeclaration(ClassDeclarationHandle),
    ClassDefinition(ClassHandle),
    StaticClassDefinition(StaticClassHandle),
    InterfaceDefinition(InterfaceHandle),
    IteratorDeclaration(IteratorHandle),
    CollectionDeclaration(CollectionHandle),
    FunctionDefinition(FunctionHandle),
}

impl Statement {
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

/// Settings that affect the names of things
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

/// Settings that affect the names of things
#[derive(Debug)]
pub struct LibrarySettings {
    /// name of the library
    pub name: Name,
    /// prefix given to all API types, e.g. structs, enums, functions, etc
    pub c_ffi_prefix: Name,
    /// settings that control iterator generation
    pub iterator: IteratorSettings,
}

impl LibrarySettings {
    /// create an RC to the settings that is cheaply cloned
    pub fn create<S: IntoName, R: IntoName>(
        name: S,
        c_ffi_prefix: R,
        iterator: IteratorSettings,
    ) -> BindResult<Rc<Self>> {
        Ok(Rc::new(Self {
            name: name.into_name()?,
            c_ffi_prefix: c_ffi_prefix.into_name()?,
            iterator,
        }))
    }
}

pub struct Library {
    pub version: Version,
    pub info: LibraryInfo,
    pub settings: Rc<LibrarySettings>,
    statements: Vec<Statement>,
    structs: HashMap<StructDeclarationHandle, StructType>,
    _classes: HashMap<ClassDeclarationHandle, ClassHandle>,
    symbols: HashMap<String, Symbol>,
}

impl Library {
    pub fn statements(&self) -> impl Iterator<Item = &Statement> {
        self.statements.iter()
    }

    pub fn functions(&self) -> impl Iterator<Item = &FunctionHandle> {
        self.statements().filter_map(|statement| match statement {
            Statement::FunctionDefinition(handle) => Some(handle),
            _ => None,
        })
    }

    pub fn async_method_interfaces(&self) -> impl Iterator<Item = &InterfaceHandle> {
        let mut async_function_interfaces: HashSet<InterfaceHandle> = HashSet::new();
        for c in self.classes() {
            for method in &c.async_methods {
                async_function_interfaces.insert(method.one_time_callback.clone());
            }
        }
        self.interfaces()
            .filter(move |x| async_function_interfaces.contains(x))
    }

    pub fn structs(&self) -> impl Iterator<Item = &StructType> {
        self.structs.values()
    }

    pub fn find_struct<T: AsRef<str>>(&self, name: T) -> Option<&StructType> {
        self.symbol(name).iter().find_map(|symbol| {
            if let Symbol::Struct(handle) = symbol {
                Some(handle)
            } else {
                None
            }
        })
    }

    pub fn constants(&self) -> impl Iterator<Item = &ConstantSetHandle> {
        self.statements().filter_map(|statement| match statement {
            Statement::Constants(handle) => Some(handle),
            _ => None,
        })
    }

    pub fn enums(&self) -> impl Iterator<Item = &EnumHandle> {
        self.statements().filter_map(|statement| match statement {
            Statement::EnumDefinition(handle) => Some(handle),
            _ => None,
        })
    }

    pub fn error_types(&self) -> impl Iterator<Item = &ErrorType> {
        self.statements().filter_map(|statement| match statement {
            Statement::ErrorType(err) => Some(err),
            _ => None,
        })
    }

    pub fn find_enum<T: AsRef<str>>(&self, name: T) -> Option<&EnumHandle> {
        self.symbol(name).iter().find_map(|symbol| {
            if let Symbol::Enum(handle) = symbol {
                Some(handle)
            } else {
                None
            }
        })
    }

    pub fn classes(&self) -> impl Iterator<Item = &ClassHandle> {
        self.statements().filter_map(|statement| match statement {
            Statement::ClassDefinition(handle) => Some(handle),
            _ => None,
        })
    }

    pub fn find_class_declaration<T: AsRef<str>>(
        &self,
        name: T,
    ) -> Option<&ClassDeclarationHandle> {
        self.symbol(name).iter().find_map(|symbol| match symbol {
            Symbol::Class(handle) => Some(&handle.declaration),
            Symbol::Iterator(handle) => Some(&handle.iter_class),
            Symbol::Collection(handle) => Some(&handle.collection_type),
            _ => None,
        })
    }

    pub fn find_class<T: AsRef<str>>(&self, name: T) -> Option<&ClassHandle> {
        self.symbol(name).iter().find_map(|symbol| {
            if let Symbol::Class(handle) = symbol {
                Some(handle)
            } else {
                None
            }
        })
    }

    pub fn static_classes(&self) -> impl Iterator<Item = &StaticClassHandle> {
        self.statements().filter_map(|statement| match statement {
            Statement::StaticClassDefinition(handle) => Some(handle),
            _ => None,
        })
    }

    pub fn find_static_class<T: AsRef<str>>(&self, name: T) -> Option<&StaticClassHandle> {
        self.symbol(name).iter().find_map(|symbol| {
            if let Symbol::StaticClass(handle) = symbol {
                Some(handle)
            } else {
                None
            }
        })
    }

    pub fn interfaces(&self) -> impl Iterator<Item = &InterfaceHandle> {
        self.statements().filter_map(|statement| match statement {
            Statement::InterfaceDefinition(handle) => Some(handle),
            _ => None,
        })
    }

    pub fn find_interface<T: AsRef<str>>(&self, name: T) -> Option<&InterfaceHandle> {
        self.symbol(name).iter().find_map(|symbol| {
            if let Symbol::Interface(handle) = symbol {
                Some(handle)
            } else {
                None
            }
        })
    }

    pub fn iterators(&self) -> impl Iterator<Item = &IteratorHandle> {
        self.statements().filter_map(|statement| match statement {
            Statement::IteratorDeclaration(handle) => Some(handle),
            _ => None,
        })
    }

    pub fn find_iterator<T: AsRef<str>>(&self, name: T) -> Option<&IteratorHandle> {
        self.statements.iter().find_map(|statement| {
            if let Statement::IteratorDeclaration(handle) = statement {
                if handle.name() == name.as_ref() {
                    return Some(handle);
                }
            }

            None
        })
    }

    pub fn collections(&self) -> impl Iterator<Item = &CollectionHandle> {
        self.statements().filter_map(|statement| match statement {
            Statement::CollectionDeclaration(handle) => Some(handle),
            _ => None,
        })
    }

    pub fn find_collection<T: AsRef<str>>(&self, name: T) -> Option<&CollectionHandle> {
        self.statements.iter().find_map(|statement| {
            if let Statement::CollectionDeclaration(handle) = statement {
                if handle.name() == name.as_ref() {
                    return Some(handle);
                }
            }

            None
        })
    }

    pub fn symbol<T: AsRef<str>>(&self, symbol_name: T) -> Option<&Symbol> {
        self.symbols.get(symbol_name.as_ref())
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum StructType {
    /// structs that may be used as native function parameters
    FunctionArg(FunctionArgStructHandle),
    /// structs than can be used as native function return values
    FunctionReturn(FunctionReturnStructHandle),
    /// structs that may be used as callback function arguments in interfaces
    CallbackArg(CallbackArgStructHandle),
    /// structs that can be used in any context and only contain basic types
    Universal(UniversalStructHandle),
}

impl From<FunctionArgStructHandle> for StructType {
    fn from(x: FunctionArgStructHandle) -> Self {
        StructType::FunctionArg(x)
    }
}

impl From<FunctionReturnStructHandle> for StructType {
    fn from(x: FunctionReturnStructHandle) -> Self {
        StructType::FunctionReturn(x)
    }
}

impl From<CallbackArgStructHandle> for StructType {
    fn from(x: CallbackArgStructHandle) -> Self {
        StructType::CallbackArg(x)
    }
}

impl From<UniversalStructHandle> for StructType {
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
    Specific(Handle<Struct<T>>),
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
    pub fn name(&self) -> &str {
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

    pub fn to_struct_type(&self) -> StructType {
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

impl<T> From<Handle<Struct<T>>> for UniversalOr<T>
where
    T: StructFieldType,
{
    fn from(x: Handle<Struct<T>>) -> Self {
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

impl StructType {
    pub fn construtors(&self) -> &[Handle<Constructor>] {
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

    pub fn doc(&self) -> &Doc {
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

    pub fn has_field_named(&self, name: &str) -> bool {
        match self {
            StructType::FunctionArg(x) => x.has_field_named(name),
            StructType::CallbackArg(x) => x.has_field_named(name),
            StructType::FunctionReturn(x) => x.has_field_named(name),
            StructType::Universal(x) => x.has_field_named(name),
        }
    }

    pub fn get_default_constructor(&self) -> Option<&Handle<Constructor>> {
        match self {
            StructType::FunctionArg(x) => x.get_default_constructor(),
            StructType::FunctionReturn(x) => x.get_default_constructor(),
            StructType::CallbackArg(x) => x.get_default_constructor(),
            StructType::Universal(x) => x.get_default_constructor(),
        }
    }
}

pub struct LibraryBuilder {
    version: Version,
    info: LibraryInfo,
    pub(crate) settings: Rc<LibrarySettings>,

    // a record of statements preserved in order
    statements: Vec<Statement>,
    // names of symbols used in the library
    symbol_names: HashSet<String>,

    // native stuff
    structs_declarations: HashSet<StructDeclarationHandle>,
    structs: HashMap<StructDeclarationHandle, StructType>,
    functions: HashSet<FunctionHandle>,
    enums: HashSet<EnumHandle>,

    // oo stuff
    class_declarations: HashSet<ClassDeclarationHandle>,
    classes: HashMap<ClassDeclarationHandle, ClassHandle>,
    static_classes: HashSet<StaticClassHandle>,
    interfaces: HashSet<InterfaceHandle>,

    // specialized types
    iterators: HashSet<IteratorHandle>,
    collections: HashSet<CollectionHandle>,
}

impl LibraryBuilder {
    pub fn new(version: Version, info: LibraryInfo, settings: Rc<LibrarySettings>) -> Self {
        Self {
            version,
            info,
            settings,

            statements: Vec::new(),
            symbol_names: HashSet::new(),

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

    pub(crate) fn add_statement(&mut self, statement: Statement) -> BindResult<()> {
        if let Some(name) = statement.unique_name() {
            self.check_unique_symbol(name)?;
        }

        self.statements.push(statement.clone());

        match statement {
            Statement::Constants(_) => {}
            Statement::StructDeclaration(x) => {
                self.structs_declarations.insert(x);
            }
            Statement::StructDefinition(x) => {
                self.structs.insert(x.declaration(), x);
            }
            Statement::EnumDefinition(x) => {
                self.enums.insert(x);
            }
            Statement::ErrorType(_) => {}
            Statement::ClassDeclaration(x) => {
                self.class_declarations.insert(x);
            }
            Statement::ClassDefinition(x) => {
                self.classes.insert(x.declaration.clone(), x);
            }
            Statement::StaticClassDefinition(x) => {
                self.static_classes.insert(x);
            }
            Statement::InterfaceDefinition(x) => {
                self.interfaces.insert(x);
            }
            Statement::IteratorDeclaration(x) => {
                self.iterators.insert(x);
            }
            Statement::CollectionDeclaration(x) => {
                self.collections.insert(x);
            }
            Statement::FunctionDefinition(x) => {
                self.functions.insert(x);
            }
        }

        Ok(())
    }

    pub fn build(self) -> BindResult<Library> {
        // Build symbols map
        let mut symbols: HashMap<String, Symbol> = HashMap::new();
        for statement in &self.statements {
            match statement {
                Statement::Constants(_) => {}
                Statement::StructDeclaration(handle) => {
                    symbols.insert(
                        handle.name.to_string(),
                        Symbol::Struct(self.structs.get(handle).unwrap().clone()),
                    );
                }
                Statement::StructDefinition(_) => (),
                Statement::EnumDefinition(handle) => {
                    symbols.insert(handle.name.to_string(), Symbol::Enum(handle.clone()));
                }
                Statement::ErrorType(_) => {}
                Statement::ClassDeclaration(_) => (),
                Statement::ClassDefinition(handle) => {
                    symbols.insert(handle.name().to_string(), Symbol::Class(handle.clone()));
                }
                Statement::StaticClassDefinition(handle) => {
                    symbols.insert(handle.name.to_string(), Symbol::StaticClass(handle.clone()));
                }
                Statement::InterfaceDefinition(handle) => {
                    symbols.insert(handle.name.to_string(), Symbol::Interface(handle.clone()));
                }
                Statement::IteratorDeclaration(handle) => {
                    symbols.insert(handle.name().to_string(), Symbol::Iterator(handle.clone()));
                }
                Statement::CollectionDeclaration(handle) => {
                    symbols.insert(
                        handle.name().to_string(),
                        Symbol::Collection(handle.clone()),
                    );
                }
                Statement::FunctionDefinition(handle) => {
                    symbols.insert(handle.name.to_string(), Symbol::Function(handle.clone()));
                }
            }
        }

        let lib = Library {
            version: self.version,
            info: self.info,
            settings: self.settings.clone(),
            statements: self.statements,
            structs: self.structs,
            _classes: self.classes,
            symbols,
        };

        crate::doc::validate_library_docs(&lib)?;

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
        if self.structs.contains_key(&declaration.inner) {
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
        if self.structs.contains_key(&declaration.inner) {
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
        if self.structs.contains_key(&declaration.inner) {
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
        if self.structs.contains_key(&declaration.inner) {
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
        if self.structs.contains_key(&declaration.inner) {
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

    pub fn declare_iterator<T: IntoName>(
        &mut self,
        name: T,
    ) -> BindResult<IteratorClassDeclaration> {
        Ok(IteratorClassDeclaration::new(
            self.declare_any_class(name, ClassType::Iterator)?,
        ))
    }

    pub fn declare_collection<T: IntoName>(
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
        if !self.classes.contains_key(declaration) {
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

    pub fn define_synchronous_interface<T: IntoName, D: Into<Doc>>(
        &mut self,
        name: T,
        doc: D,
    ) -> BindResult<InterfaceBuilder> {
        self.define_interface(name, InterfaceType::Synchronous, doc)
    }

    pub fn define_asynchronous_interface<T: IntoName, D: Into<Doc>>(
        &mut self,
        name: T,
        doc: D,
    ) -> BindResult<InterfaceBuilder> {
        self.define_interface(name, InterfaceType::Asynchronous, doc)
    }

    fn define_interface<T: IntoName, D: Into<Doc>>(
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

    pub fn define_iterator<N: IntoName, T: Into<UniversalOr<FunctionReturnStructField>>>(
        &mut self,
        class_name: N,
        item_type: T,
    ) -> BindResult<IteratorHandle> {
        self.define_iterator_impl(class_name, false, item_type)
    }

    pub fn define_iterator_with_lifetime<
        N: IntoName,
        T: Into<UniversalOr<FunctionReturnStructField>>,
    >(
        &mut self,
        class_name: N,
        item_type: T,
    ) -> BindResult<IteratorHandle> {
        self.define_iterator_impl(class_name, true, item_type)
    }

    fn define_iterator_impl<N: IntoName, T: Into<UniversalOr<FunctionReturnStructField>>>(
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
            .returns(
                FunctionReturnValue::StructRef(item_type.typed_declaration()),
                "next value or NULL",
            )?
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

    pub fn define_collection(
        &mut self,
        create_func: &FunctionHandle,
        delete_func: &FunctionHandle,
        add_func: &FunctionHandle,
    ) -> BindResult<CollectionHandle> {
        let collection = CollectionHandle::new(crate::collection::Collection::new(
            create_func,
            delete_func,
            add_func,
        )?);

        if self
            .collections
            .iter()
            .any(|col| col.collection_type == collection.collection_type)
        {
            return Err(BindingError::CollectionAlreadyDefinedForClass {
                handle: collection.collection_type.clone(),
            });
        }

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
        if self.functions.contains(native_function) {
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
        if self.structs_declarations.contains(native_struct) {
            Ok(())
        } else {
            Err(BindingError::StructNotPartOfThisLib {
                handle: native_struct.clone(),
            })
        }
    }

    fn validate_struct(&self, native_struct: &StructType) -> BindResult<()> {
        if self.structs.contains_key(&native_struct.declaration()) {
            Ok(())
        } else {
            Err(BindingError::StructNotPartOfThisLib {
                handle: native_struct.declaration(),
            })
        }
    }

    fn validate_enum(&self, native_enum: &EnumHandle) -> BindResult<()> {
        if self.enums.contains(native_enum) {
            Ok(())
        } else {
            Err(BindingError::EnumNotPartOfThisLib {
                handle: native_enum.clone(),
            })
        }
    }

    fn validate_interface(&self, interface: &InterfaceHandle) -> BindResult<()> {
        if self.interfaces.contains(interface) {
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
        if self.class_declarations.contains(class_declaration) {
            Ok(())
        } else {
            Err(BindingError::ClassNotPartOfThisLib {
                handle: class_declaration.clone(),
            })
        }
    }

    fn validate_iterator(&self, iter: &IteratorHandle) -> BindResult<()> {
        if self.iterators.contains(iter) {
            Ok(())
        } else {
            Err(BindingError::IteratorNotPartOfThisLib {
                handle: iter.clone(),
            })
        }
    }

    fn validate_collection(&self, collection: &CollectionHandle) -> BindResult<()> {
        if self.collections.contains(collection) {
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
