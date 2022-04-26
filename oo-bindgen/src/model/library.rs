use std::path::PathBuf;
use std::rc::Rc;

use crate::model::*;

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
    InterfaceDefinition(InterfaceType<D>),
    IteratorDeclaration(Handle<AbstractIterator<D>>),
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
            Statement::InterfaceDefinition(x) => Some(&x.untyped().name),
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
    /// Logo of the company (in PNG)
    ///
    /// Use `include_bytes` to import the data
    pub logo_png: &'static [u8],
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

/// Settings that affect C interface member naming
#[derive(Debug)]
pub struct InterfaceSettings {
    /// Name of the C void* context variable, defaults to "ctx"
    pub context_variable_name: Name,
    /// Name of the function that destroys an interface when it is dropped, defaults to "on_destroy"
    pub destroy_func_name: Name,
}

impl InterfaceSettings {
    pub fn new(context_variable_name: Name, destroy_func_name: Name) -> Self {
        Self {
            context_variable_name,
            destroy_func_name,
        }
    }
}

impl Default for InterfaceSettings {
    fn default() -> Self {
        Self {
            context_variable_name: Name::create("ctx").unwrap(),
            destroy_func_name: Name::create("on_destroy").unwrap(),
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
    /// The name given to the success completion method on interface
    pub success_callback_method_name: Name,
    /// The name given to the result parameter of the success completion method
    pub success_single_parameter_name: Name,
    /// The name given to the failure completion method on interface
    pub failure_callback_method_name: Name,
    /// The name given to the error parameter of the failure completion method
    pub failure_single_parameter_name: Name,
    /// The name given to the final callback parameter of the async methods
    pub async_method_callback_parameter_name: Name,
}

impl FutureSettings {
    pub fn new(
        success_callback_method_name: Name,
        success_single_parameter_name: Name,
        failure_callback_method_name: Name,
        failure_single_parameter_name: Name,
        async_method_callback_parameter_name: Name,
    ) -> Self {
        Self {
            success_callback_method_name,
            success_single_parameter_name,
            failure_callback_method_name,
            failure_single_parameter_name,
            async_method_callback_parameter_name,
        }
    }
}

impl Default for FutureSettings {
    fn default() -> Self {
        Self {
            success_callback_method_name: Name::create("on_complete").unwrap(),
            success_single_parameter_name: Name::create("result").unwrap(),
            failure_callback_method_name: Name::create("on_failure").unwrap(),
            failure_single_parameter_name: Name::create("error").unwrap(),
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
    /// settings that control C interface member naming
    pub interface: InterfaceSettings,
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
        interface: InterfaceSettings,
    ) -> BindResult<Rc<Self>> {
        Ok(Rc::new(Self {
            name: name.into_name()?,
            c_ffi_prefix: c_ffi_prefix.into_name()?,
            class,
            iterator,
            collection,
            future,
            interface,
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
    pub(crate) fn new(
        version: Version,
        info: Rc<LibraryInfo>,
        settings: Rc<LibrarySettings>,
        statements: Vec<Statement<Validated>>,
    ) -> Self {
        Self {
            version,
            info,
            settings,
            statements,
        }
    }

    pub fn statements(&self) -> impl Iterator<Item = &Statement<Validated>> {
        self.statements.iter()
    }

    pub fn functions(&self) -> impl Iterator<Item = &Handle<Function<Validated>>> {
        self.statements().filter_map(|statement| match statement {
            Statement::FunctionDefinition(handle) => Some(handle),
            _ => None,
        })
    }

    pub fn future_interfaces(&self) -> impl Iterator<Item = &FutureInterface<Validated>> {
        self.statements.iter().filter_map(|x| match x {
            Statement::InterfaceDefinition(InterfaceType::Future(x)) => Some(x),
            _ => None,
        })
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

    pub fn untyped_interfaces(&self) -> impl Iterator<Item = &Handle<Interface<Validated>>> {
        self.interfaces().map(|x| x.untyped())
    }

    pub fn interfaces(&self) -> impl Iterator<Item = &InterfaceType<Validated>> {
        self.statements
            .iter()
            .filter_map(|statement| match statement {
                Statement::InterfaceDefinition(t) => Some(t),
                _ => None,
            })
    }

    pub fn iterators(&self) -> impl Iterator<Item = &Handle<AbstractIterator<Validated>>> {
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

impl From<UniversalStructDeclaration> for FunctionReturnStructDeclaration {
    fn from(x: UniversalStructDeclaration) -> Self {
        FunctionReturnStructDeclaration::new(x.inner)
    }
}
