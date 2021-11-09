use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use crate::class::*;
use crate::collection::{Collection, CollectionHandle};
use crate::constants::{ConstantSet, ConstantSetBuilder};
use crate::doc::{Doc, DocReference, DocString, Unvalidated, Validated};
use crate::enum_type::{EnumBuilder, EnumHandle};
use crate::error_type::{ErrorType, ErrorTypeBuilder, ExceptionType};
use crate::function::{FunctionBuilder, FunctionHandle};
use crate::interface::{InterfaceBuilder, InterfaceHandle};
use crate::iterator::{IteratorHandle, IteratorItemType};
use crate::name::{IntoName, Name};
use crate::structs::*;
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
    pub fn new(next_function_suffix: Name) -> IteratorSettings {
        Self {
            next_function_suffix,
        }
    }
}

impl Default for IteratorSettings {
    fn default() -> Self {
        Self {
            next_function_suffix: Name::create("next").unwrap(),
        }
    }
}

/// Settings that affect class method naming
#[derive(Debug)]
pub struct ClassSettings {
    /// Methods in C always take an instance of the class at the first parameter.
    /// This setting controls the name automatically assigned to this parameter.
    ///
    /// This value defaults to "instance"
    pub method_instance_argument_name: Name,
    /// suffix for C destructors.
    /// The full C function name is automatically generated as `<c_ffi_prefix>_<class_name>_<class_destructor_suffix>`
    ///
    /// This value defaults to 'destroy'
    pub class_destructor_suffix: Name,
    /// suffix for C constructors.
    /// The full C function name is automatically generated as `<c_ffi_prefix>_<class_name>_<class_constructor_suffix>`
    ///
    /// This value defaults to 'new'
    pub class_constructor_suffix: Name,
}

impl ClassSettings {
    pub fn new(
        method_instance_argument_name: Name,
        class_destructor_suffix: Name,
        class_constructor_suffix: Name,
    ) -> Self {
        Self {
            method_instance_argument_name,
            class_destructor_suffix,
            class_constructor_suffix,
        }
    }
}

impl Default for ClassSettings {
    fn default() -> ClassSettings {
        Self {
            method_instance_argument_name: Name::create("instance").unwrap(),
            class_destructor_suffix: Name::create("destroy").unwrap(),
            class_constructor_suffix: Name::create("create").unwrap(),
        }
    }
}

/// Settings that affect how things are named in future-style callback interfaces
#[derive(Debug)]
pub struct FutureSettings {
    /// The name given to the completion method on interface
    pub success_callback_method_name: Name,
    /// The name given to the result parameter of the completion method
    pub success_single_parameter_name: Name,
    /// The name given to the final callback parameter of the async methods
    pub async_method_callback_parameter_name: Name,
}

impl FutureSettings {
    pub fn new(
        success_callback_method_name: Name,
        success_single_parameter_name: Name,
        async_method_callback_parameter_name: Name,
    ) -> Self {
        Self {
            success_callback_method_name,
            success_single_parameter_name,
            async_method_callback_parameter_name,
        }
    }
}

impl Default for FutureSettings {
    fn default() -> Self {
        Self {
            success_callback_method_name: Name::create("on_complete").unwrap(),
            success_single_parameter_name: Name::create("result").unwrap(),
            async_method_callback_parameter_name: Name::create("callback").unwrap(),
        }
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
    pub fn new(
        create_function_suffix: Name,
        add_function_suffix: Name,
        destroy_function_suffix: Name,
    ) -> Self {
        Self {
            create_function_suffix,
            add_function_suffix,
            destroy_function_suffix,
        }
    }
}

impl Default for CollectionSettings {
    fn default() -> CollectionSettings {
        Self {
            create_function_suffix: Name::create("create").unwrap(),
            add_function_suffix: Name::create("add").unwrap(),
            destroy_function_suffix: Name::create("destroy").unwrap(),
        }
    }
}

/// Settings that affect the names of things
#[derive(Debug)]
pub struct LibrarySettings {
    /// name of the library
    pub name: Name,
    /// prefix given to all API types, e.g. structs, enums, functions, etc
    pub c_ffi_prefix: Name,
    /// settings that control class generation
    pub class: ClassSettings,
    /// settings that control iterator generation
    pub iterator: IteratorSettings,
    /// settings that control collection generation
    pub collection: CollectionSettings,
    /// settings that control future-style interface generation
    pub future: FutureSettings,
}

impl LibrarySettings {
    /// create an RC to the settings that is cheaply cloned
    pub fn create<S: IntoName, R: IntoName>(
        name: S,
        c_ffi_prefix: R,
        class: ClassSettings,
        iterator: IteratorSettings,
        collection: CollectionSettings,
        future: FutureSettings,
    ) -> BindResult<Rc<Self>> {
        Ok(Rc::new(Self {
            name: name.into_name()?,
            c_ffi_prefix: c_ffi_prefix.into_name()?,
            class,
            iterator,
            collection,
            future,
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

    pub fn future_interfaces(&self) -> impl Iterator<Item = &Handle<Interface<Validated>>> {
        self.interfaces()
            .filter(|x| x.interface_type == InterfaceType::Future)
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
        // verify that all the pieces of the statement are from this library
        self.check_statement(&statement)?;

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
            Statement::Constants(x) => Ok(Statement::Constants(x.validate(&self.fields)?)),
            Statement::StructDeclaration(x) => Ok(Statement::StructDeclaration(x.clone())),
            Statement::StructDefinition(x) => {
                Ok(Statement::StructDefinition(x.validate(&self.fields)?))
            }
            Statement::EnumDefinition(x) => {
                Ok(Statement::EnumDefinition(x.validate(&self.fields)?))
            }
            Statement::ErrorType(x) => Ok(Statement::ErrorType(x.validate(&self.fields)?)),
            Statement::ClassDeclaration(x) => Ok(Statement::ClassDeclaration(x.clone())),
            Statement::ClassDefinition(x) => {
                Ok(Statement::ClassDefinition(x.validate(&self.fields)?))
            }
            Statement::StaticClassDefinition(x) => {
                Ok(Statement::StaticClassDefinition(x.validate(&self.fields)?))
            }
            Statement::InterfaceDefinition(x) => {
                Ok(Statement::InterfaceDefinition(x.validate(&self.fields)?))
            }
            Statement::IteratorDeclaration(x) => {
                Ok(Statement::IteratorDeclaration(x.validate(&self.fields)?))
            }
            Statement::CollectionDeclaration(x) => {
                Ok(Statement::CollectionDeclaration(x.validate(&self.fields)?))
            }
            Statement::FunctionDefinition(x) => {
                Ok(Statement::FunctionDefinition(x.validate(&self.fields)?))
            }
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
        self.define_function_with_category(name, FunctionCategory::Native)
    }

    pub fn define_method<T: IntoName>(
        &mut self,
        name: T,
        class: ClassDeclarationHandle,
    ) -> BindResult<ClassMethodBuilder> {
        ClassMethodBuilder::new(self, name.into_name()?, class)
    }

    pub fn define_future_method<T: IntoName>(
        &mut self,
        name: T,
        class: ClassDeclarationHandle,
        future: FutureInterface<Unvalidated>,
    ) -> BindResult<FutureMethodBuilder> {
        FutureMethodBuilder::new(self, name.into_name()?, class, future)
    }

    pub fn define_constructor(
        &mut self,
        class: ClassDeclarationHandle,
    ) -> BindResult<ClassConstructorBuilder> {
        ClassConstructorBuilder::new(self, class)
    }

    pub fn define_destructor<D: Into<Doc<Unvalidated>>>(
        &mut self,
        class: ClassDeclarationHandle,
        doc: D,
    ) -> BindResult<ClassDestructor<Unvalidated>> {
        ClassDestructor::new(self, class, doc.into())
    }

    pub(crate) fn define_function_with_category<T: IntoName>(
        &mut self,
        name: T,
        category: FunctionCategory,
    ) -> BindResult<FunctionBuilder> {
        Ok(FunctionBuilder::new(self, name.into_name()?, category))
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
        self.check_class_declaration(declaration)?;
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

    /// A synchronous interface is one that is invoked only during a function call which
    /// takes it as an argument. The Rust backend will NOT generate `Send` and `Sync`
    /// implementations so that it be cannot be transferred across thread boundaries.
    pub fn define_synchronous_interface<T: IntoName, D: Into<Doc<Unvalidated>>>(
        &mut self,
        name: T,
        doc: D,
    ) -> BindResult<InterfaceBuilder> {
        self.define_interface(name, InterfaceType::Synchronous, doc)
    }

    /// An asynchronous interface is one that is invoked some time after it is
    /// passed as a function argument. The Rust backend will mark the C representation
    /// of this interface as `Send` and `Sync` so that it be transferred across thread
    /// boundaries.
    pub fn define_asynchronous_interface<T: IntoName, D: Into<Doc<Unvalidated>>>(
        &mut self,
        name: T,
        doc: D,
    ) -> BindResult<InterfaceBuilder> {
        self.define_interface(name, InterfaceType::Asynchronous, doc)
    }

    /// A future interface is a specialized asynchronous which consists of
    /// a single callback method providing a single value. The callback
    /// represents the completion of a "future" and is mapped appropriately
    /// in backends.
    pub fn define_future_interface<
        T: IntoName,
        D: Into<Doc<Unvalidated>>,
        E: Into<DocString<Unvalidated>>,
        V: Into<CallbackArgument>,
    >(
        &mut self,
        name: T,
        interface_docs: D,
        value_type: V,
        value_type_docs: E,
    ) -> BindResult<FutureInterface<Unvalidated>> {
        let on_complete_name = self.settings.future.success_callback_method_name.clone();
        let result_name = self.settings.future.success_single_parameter_name.clone();
        let value_type = value_type.into();
        let value_type_docs = value_type_docs.into();

        let interface = self
            .define_interface(name, InterfaceType::Future, interface_docs)?
            .begin_callback(
                on_complete_name,
                "Invoked when the asynchronous operation completes",
            )?
            .param(result_name, value_type.clone(), value_type_docs.clone())?
            .returns_nothing()?
            .end_callback()?
            .build()?;

        Ok(FutureInterface::new(value_type, interface, value_type_docs))
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
            .define_function_with_category(
                class_name.append(&self.settings.iterator.next_function_suffix),
                FunctionCategory::IteratorNext,
            )?
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
            .define_function_with_category(
                class_name.append(&self.settings.collection.create_function_suffix),
                FunctionCategory::CollectionCreate,
            )?
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
            .define_function_with_category(
                class_name.append(&self.settings.collection.destroy_function_suffix),
                FunctionCategory::CollectionDestroy,
            )?
            .doc("Destroys an instance of the collection")?
            .param("instance", class_decl.clone(), "instance to destroy")?
            .returns_nothing()?
            .build()?;

        let add_func = self
            .define_function_with_category(
                class_name.append(&self.settings.collection.add_function_suffix),
                FunctionCategory::CollectionAdd,
            )?
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

    fn check_statement(&self, statement: &Statement<Unvalidated>) -> BindResult<()> {
        match statement {
            // no internals that can be from another library
            Statement::Constants(_) => Ok(()),
            Statement::StructDeclaration(_) => Ok(()),
            Statement::EnumDefinition(_) => Ok(()),
            Statement::ClassDeclaration(_) => Ok(()),
            // these types have internals that must be checked
            Statement::StructDefinition(x) => self.check_struct_declaration(&x.declaration()),
            Statement::ErrorType(x) => self.check_enum(&x.inner),
            Statement::ClassDefinition(x) => {
                self.check_class_declaration(&x.declaration)?;
                for x in x.constructor.iter() {
                    self.check_function(&x.function)?;
                }
                for x in x.destructor.iter() {
                    self.check_function(&x.function)?;
                }
                for x in x.static_methods.iter() {
                    self.check_function(&x.native_function)?
                }
                for x in x.methods.iter() {
                    self.check_function(&x.native_function)?
                }
                for x in x.future_methods.iter() {
                    self.check_function(&x.native_function)?
                }
                Ok(())
            }
            Statement::StaticClassDefinition(x) => {
                for x in x.static_methods.iter() {
                    self.check_function(&x.native_function)?;
                }
                Ok(())
            }
            Statement::InterfaceDefinition(x) => {
                for cb in x.callbacks.iter() {
                    for arg in cb.arguments.iter() {
                        self.check_callback_argument(&arg.arg_type)?;
                    }
                    if let Some(ret) = cb.return_type.get() {
                        self.check_callback_return_value(ret)?;
                    }
                }
                Ok(())
            }
            Statement::IteratorDeclaration(x) => {
                self.check_class_declaration(&x.iter_class)?;
                self.check_function(&x.next_function)?;
                match &x.item_type {
                    IteratorItemType::Struct(x) => {
                        self.check_struct_declaration(&x.declaration())?;
                    }
                }
                Ok(())
            }
            Statement::CollectionDeclaration(x) => {
                self.check_class_declaration(&x.collection_class)?;
                self.check_function(&x.create_func)?;
                self.check_function(&x.add_func)?;
                self.check_function(&x.delete_func)?;
                self.check_function_argument(&x.item_type)?;
                Ok(())
            }
            Statement::FunctionDefinition(x) => {
                for p in x.parameters.iter() {
                    self.check_function_argument(&p.arg_type)?;
                }
                if let Some(r) = x.return_type.get() {
                    self.check_function_return_type(r)?;
                }
                for err in x.error_type.iter() {
                    self.check_enum(&err.inner)?;
                }
                Ok(())
            }
        }
    }

    fn check_struct_declaration(&self, native_struct: &StructDeclarationHandle) -> BindResult<()> {
        if self.fields.structs_declarations.contains(native_struct) {
            Ok(())
        } else {
            Err(BindingError::StructNotPartOfThisLib {
                handle: native_struct.clone(),
            })
        }
    }

    fn check_function(&self, native_function: &FunctionHandle) -> BindResult<()> {
        if self.fields.functions.contains(native_function) {
            Ok(())
        } else {
            Err(BindingError::FunctionNotPartOfThisLib {
                handle: native_function.clone(),
            })
        }
    }

    fn check_enum(&self, native_enum: &EnumHandle) -> BindResult<()> {
        if self.fields.enums.contains(native_enum) {
            Ok(())
        } else {
            Err(BindingError::EnumNotPartOfThisLib {
                handle: native_enum.clone(),
            })
        }
    }

    fn check_interface(&self, interface: &InterfaceHandle) -> BindResult<()> {
        if self.fields.interfaces.contains(interface) {
            Ok(())
        } else {
            Err(BindingError::InterfaceNotPartOfThisLib {
                handle: interface.clone(),
            })
        }
    }

    fn check_class_declaration(
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

    fn check_function_argument(&self, arg: &FunctionArgument) -> BindResult<()> {
        match arg {
            FunctionArgument::Basic(x) => self.check_basic_type(x),
            FunctionArgument::String(_) => Ok(()),
            FunctionArgument::Collection(x) => self.check_collection(x),
            FunctionArgument::Struct(x) => self.check_struct_declaration(&x.declaration()),
            FunctionArgument::StructRef(x) => self.check_struct_declaration(&x.inner),
            FunctionArgument::ClassRef(x) => self.check_class_declaration(x),
            FunctionArgument::Interface(x) => self.check_interface(x),
        }
    }

    fn check_callback_argument(&self, arg: &CallbackArgument) -> BindResult<()> {
        match arg {
            CallbackArgument::Basic(x) => self.check_basic_type(x),
            CallbackArgument::String(_) => Ok(()),
            CallbackArgument::Iterator(x) => self.check_iterator(x),
            CallbackArgument::Class(x) => self.check_class_declaration(x),
            CallbackArgument::Struct(x) => self.check_struct_declaration(&x.declaration()),
        }
    }

    fn check_callback_return_value(&self, arg: &CallbackReturnValue) -> BindResult<()> {
        match arg {
            CallbackReturnValue::Basic(x) => self.check_basic_type(x),
            CallbackReturnValue::Struct(x) => self.check_struct_declaration(&x.declaration()),
        }
    }

    fn check_basic_type(&self, arg: &BasicType) -> BindResult<()> {
        match arg {
            BasicType::Bool => Ok(()),
            BasicType::U8 => Ok(()),
            BasicType::S8 => Ok(()),
            BasicType::U16 => Ok(()),
            BasicType::S16 => Ok(()),
            BasicType::U32 => Ok(()),
            BasicType::S32 => Ok(()),
            BasicType::U64 => Ok(()),
            BasicType::S64 => Ok(()),
            BasicType::Float32 => Ok(()),
            BasicType::Double64 => Ok(()),
            BasicType::Duration(_) => Ok(()),
            BasicType::Enum(x) => self.check_enum(x),
        }
    }

    fn check_function_return_type(&self, value: &FunctionReturnValue) -> BindResult<()> {
        match value {
            FunctionReturnValue::Basic(x) => self.check_basic_type(x),
            FunctionReturnValue::String(_) => Ok(()),
            FunctionReturnValue::ClassRef(x) => self.check_class_declaration(x),
            FunctionReturnValue::Struct(x) => self.check_struct_declaration(&x.declaration()),
            FunctionReturnValue::StructRef(x) => self.check_struct_declaration(x.untyped()),
        }
    }

    fn check_iterator(&self, iter: &IteratorHandle) -> BindResult<()> {
        if self.fields.iterators.contains(iter) {
            Ok(())
        } else {
            Err(BindingError::IteratorNotPartOfThisLib {
                handle: iter.clone(),
            })
        }
    }

    fn check_collection(&self, collection: &CollectionHandle) -> BindResult<()> {
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
