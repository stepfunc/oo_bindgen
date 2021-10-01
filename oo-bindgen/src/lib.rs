#![deny(
// dead_code,
arithmetic_overflow,
invalid_type_param_default,
//missing_fragment_specifier,
mutable_transmutes,
no_mangle_const_items,
overflowing_literals,
patterns_in_fns_without_body,
pub_use_of_private_extern_crate,
unknown_crate_types,
const_err,
order_dependent_trait_objects,
illegal_floating_point_literal_pattern,
improper_ctypes,
late_bound_lifetime_arguments,
non_camel_case_types,
non_shorthand_field_patterns,
non_snake_case,
non_upper_case_globals,
no_mangle_generic_items,
private_in_public,
stable_features,
type_alias_bounds,
tyvar_behind_raw_pointer,
unconditional_recursion,
unused_comparisons,
unreachable_pub,
anonymous_parameters,
missing_copy_implementations,
// missing_debug_implementations,
// missing_docs,
trivial_casts,
trivial_numeric_casts,
unused_import_braces,
unused_qualifications,
clippy::all
)]
#![forbid(
    unsafe_code,
    //intra_doc_link_resolution_failure, broken_intra_doc_links
    unaligned_references,
    while_true,
    bare_trait_objects
)]

use crate::callback::*;
use crate::class::*;
use crate::doc::Doc;
use crate::native_enum::*;
use crate::native_function::*;
use crate::any_struct::*;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Deref;
use std::path::PathBuf;
use std::ptr;
use std::rc::Rc;
use thiserror::Error;

pub use semver::Version;

pub mod callback;
pub mod class;
pub mod collection;
pub mod constants;
pub mod doc;
pub mod error_type;
pub mod formatting;
pub mod function_struct;
pub mod iterator;
pub mod native_enum;
pub mod native_function;
pub mod any_struct;
pub mod platforms;
pub mod struct_common;
pub mod types;
pub mod util;

use crate::constants::{ConstantSetBuilder, ConstantSetHandle};
pub use crate::doc::doc;
use crate::error_type::{ErrorType, ErrorTypeBuilder, ExceptionType};
use crate::function_struct::{FStructBuilder, FStructHandle};
use crate::struct_common::{StructDeclaration, StructDeclarationHandle, Visibility};
use crate::types::{AnyType, BasicType};

type Result<T> = std::result::Result<T, BindingError>;

#[derive(Error, Debug)]
pub enum BindingError {
    // Global errors
    #[error("Symbol '{}' already used in the library", name)]
    SymbolAlreadyUsed { name: String },
    #[error("C FFI prefix already set")]
    FfiPrefixAlreadySet,

    // Documentation error
    #[error("Invalid documentation string")]
    InvalidDocString,
    #[error("Documentation of '{}' was already defined", symbol_name)]
    DocAlreadyDefined { symbol_name: String },
    #[error("Documentation of '{}' was not defined", symbol_name)]
    DocNotDefined { symbol_name: String },
    #[error(
        "Documentation of '{}' references '{}' which does not exist",
        symbol_name,
        ref_name
    )]
    DocInvalidReference {
        symbol_name: String,
        ref_name: String,
    },

    // Native function errors
    #[error("Native struct '{}' is not part of this library", handle.name)]
    NativeFunctionNotPartOfThisLib { handle: FunctionHandle },
    #[error(
        "Return type of native function '{}' was already defined to '{:?}'",
        native_func_name,
        return_type
    )]
    ReturnTypeAlreadyDefined {
        native_func_name: String,
        return_type: ReturnType,
    },
    #[error(
        "Return type of native function '{}' was not defined",
        native_func_name
    )]
    ReturnTypeNotDefined { native_func_name: String },

    // Native enum errors
    #[error("Native enum '{}' is not part of this library", handle.name)]
    NativeEnumNotPartOfThisLib { handle: EnumHandle },
    #[error(
        "Native enum '{}' already contains a variant with name '{}'",
        name,
        variant_name
    )]
    NativeEnumAlreadyContainsVariantWithSameName { name: String, variant_name: String },
    #[error(
        "Native enum '{}' already contains a variant with value '{}'",
        name,
        variant_value
    )]
    NativeEnumAlreadyContainsVariantWithSameValue { name: String, variant_value: i32 },
    #[error(
        "Native enum '{}' does not contain a variant named '{}'",
        name,
        variant_name
    )]
    NativeEnumDoesNotContainVariant { name: String, variant_name: String },

    // Structure errors
    #[error("Native struct '{}' was already defined", handle.name)]
    NativeStructAlreadyDefined { handle: StructDeclarationHandle },
    #[error("Native struct '{}' is not part of this library", handle.name)]
    NativeStructNotPartOfThisLib { handle: StructDeclarationHandle },
    #[error("Native struct '{}' already contains element with name '{}'", handle.name, element_name)]
    NativeStructAlreadyContainsElementWithSameName {
        handle: StructDeclarationHandle,
        element_name: String,
    },
    #[error("First parameter of native function '{}' is not of type '{}' as expected for a method of a struct", native_func.name, handle.name)]
    FirstMethodParameterIsNotStructType {
        handle: StructDeclarationHandle,
        native_func: FunctionHandle,
    },
    #[error("Struct '{}' was already defined", handle.name)]
    StructAlreadyDefined { handle: StructDeclarationHandle },
    #[error("Struct '{}' already contains element or method with name '{}'", handle.name, element_name)]
    StructAlreadyContainsElementWithSameName {
        handle: StructDeclarationHandle,
        element_name: String,
    },

    // Class errors
    #[error("Class '{}' was already defined", handle.name)]
    ClassAlreadyDefined { handle: ClassDeclarationHandle },
    #[error("Class '{}' is not part of this library", handle.name)]
    ClassNotPartOfThisLib { handle: ClassDeclarationHandle },
    #[error("First parameter of native function '{}' is not of type '{}' as expected for a method of a class", native_func.name, handle.name)]
    FirstMethodParameterIsNotClassType {
        handle: ClassDeclarationHandle,
        native_func: FunctionHandle,
    },
    #[error("Constructor for class '{}' was already defined", handle.name)]
    ConstructorAlreadyDefined { handle: ClassDeclarationHandle },
    #[error("Native function '{}' does not return '{}' as expected for a constructor", native_func.name, handle.name)]
    ConstructorReturnTypeDoesNotMatch {
        handle: ClassDeclarationHandle,
        native_func: FunctionHandle,
    },
    #[error("Destructor for class '{}' was already defined", handle.name)]
    DestructorAlreadyDefined { handle: ClassDeclarationHandle },
    #[error("Native function '{}' does not take a single '{}' parameter as expected for a destructor", native_func.name, handle.name)]
    DestructorTakesMoreThanOneParameter {
        handle: ClassDeclarationHandle,
        native_func: FunctionHandle,
    },
    #[error("Destructor for class '{}' cannot fail", handle.name)]
    DestructorCannotFail { handle: ClassDeclarationHandle },
    #[error("No destructor defined for class '{}', but asking for manual/disposable destruction", handle.name)]
    NoDestructorForManualDestruction { handle: ClassDeclarationHandle },

    // Async errors
    #[error("Native function '{}' cannot be used as an async method because it doesn't have a interface parameter", handle.name)]
    AsyncNativeMethodNoInterface { handle: FunctionHandle },
    #[error("Native function '{}' cannot be used as an async method because it has too many interface parameters", handle.name)]
    AsyncNativeMethodTooManyInterface { handle: FunctionHandle },
    #[error("Native function '{}' cannot be used as an async method because its interface parameter doesn't have a single callback", handle.name)]
    AsyncInterfaceNotSingleCallback { handle: FunctionHandle },
    #[error("Native function '{}' cannot be used as an async method because its interface parameter single callback does not have a single parameter (other than the arg param)", handle.name)]
    AsyncCallbackNotSingleParam { handle: FunctionHandle },
    #[error("Native function '{}' cannot be used as an async method because its interface parameter single callback does not return void", handle.name)]
    AsyncCallbackReturnTypeNotVoid { handle: FunctionHandle },

    // Interface errors
    #[error(
        "Interface '{}' already has element with the name '{}'",
        interface_name,
        element_name
    )]
    InterfaceHasElementWithSameName {
        interface_name: String,
        element_name: String,
    },
    #[error("Interface '{}' already has void* arg defined", interface_name)]
    InterfaceArgNameAlreadyDefined { interface_name: String },
    #[error(
        "Interface '{}' does not have a destroy callback defined",
        interface_name
    )]
    InterfaceDestroyCallbackNotDefined { interface_name: String },
    #[error(
        "Interface '{}' already has a destroy callback defined",
        interface_name
    )]
    InterfaceDestroyCallbackAlreadyDefined { interface_name: String },
    #[error("Interface '{}' is not part of this library", handle.name)]
    InterfaceNotPartOfThisLib { handle: InterfaceHandle },

    // Iterator errors
    #[error("Iterator native function '{}' does not take a single class ref parameter", handle.name)]
    IteratorNotSingleClassRefParam { handle: FunctionHandle },
    #[error("Iterator native function '{}' does not return a struct ref value", handle.name)]
    IteratorReturnTypeNotStructRef { handle: FunctionHandle },
    #[error("Iterator '{}' is not part of this library", handle.name())]
    IteratorNotPartOfThisLib { handle: iterator::IteratorHandle },
    #[error("Iterator native functions '{}' cannot fail", handle.name)]
    IteratorFunctionsCannotFail { handle: FunctionHandle },

    // Collection errors
    #[error("Invalid native function '{}' signature for create_func of collection", handle.name)]
    CollectionCreateFuncInvalidSignature { handle: FunctionHandle },
    #[error("Invalid native function '{}' signature for delete_func of collection", handle.name)]
    CollectionDeleteFuncInvalidSignature { handle: FunctionHandle },
    #[error("Invalid native function '{}' signature for add_func of collection", handle.name)]
    CollectionAddFuncInvalidSignature { handle: FunctionHandle },
    #[error("Collection native functions '{}' cannot fail", handle.name)]
    CollectionFunctionsCannotFail { handle: FunctionHandle },
    #[error("Collection '{}' is not part of this library", handle.name())]
    CollectionNotPartOfThisLib {
        handle: collection::CollectionHandle,
    },
    #[error(
        "ConstantSet '{}' already contains constant name  '{}'",
        set_name,
        constant_name
    )]
    ConstantNameAlreadyUsed {
        set_name: String,
        constant_name: String,
    },
    #[error(
        "Function '{}' already has an error type specified: '{}'",
        function,
        error_type
    )]
    ErrorTypeAlreadyDefined {
        function: String,
        error_type: String,
    },
}

pub struct Handle<T>(Rc<T>);

impl<T> Handle<T> {
    fn new(inner: T) -> Self {
        Self(Rc::new(inner))
    }
}

impl<T: Debug> Debug for Handle<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<T> Hash for Handle<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        ptr::hash(&*self.0, state)
    }
}

impl<T> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl<T> Eq for Handle<T> {}

impl<T> Deref for Handle<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub enum Symbol {
    Function(FunctionHandle),
    Struct(StructType),
    Enum(EnumHandle),
    Class(ClassHandle),
    StaticClass(StaticClassHandle),
    Interface(InterfaceHandle),
    Iterator(iterator::IteratorHandle),
    Collection(collection::CollectionHandle),
}

#[derive(Debug)]
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
    IteratorDeclaration(iterator::IteratorHandle),
    CollectionDeclaration(collection::CollectionHandle),
    FunctionDefinition(FunctionHandle),
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

pub struct Library {
    pub name: String,
    pub version: Version,
    pub c_ffi_prefix: String,
    pub info: LibraryInfo,
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

    pub fn structs(&self) -> impl Iterator<Item = &StructType> {
        self.structs.values()
    }

    pub fn find_struct<T: AsRef<str>>(&self, name: T) -> Option<&StructType> {
        self.symbol(name)
            .iter()
            .find_map(|symbol| {
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
        self.symbol(name)
            .iter()
            .find_map(|symbol| {
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
        self.symbol(name)
            .iter()
            .find_map(|symbol| match symbol {
                Symbol::Class(handle) => Some(&handle.declaration),
                Symbol::Iterator(handle) => Some(&handle.iter_type),
                Symbol::Collection(handle) => Some(&handle.collection_type),
                _ => None,
            })
    }

    pub fn find_class<T: AsRef<str>>(&self, name: T) -> Option<&ClassHandle> {
        self.symbol(name)
            .iter()
            .find_map(|symbol| {
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
        self.symbol(name)
            .iter()
            .find_map(|symbol| {
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
        self.symbol(name)
            .iter()
            .find_map(|symbol| {
                if let Symbol::Interface(handle) = symbol {
                    Some(handle)
                } else {
                    None
                }
            })
    }

    pub fn iterators(&self) -> impl Iterator<Item = &iterator::IteratorHandle> {
        self.statements().filter_map(|statement| match statement {
            Statement::IteratorDeclaration(handle) => Some(handle),
            _ => None,
        })
    }

    pub fn find_iterator<T: AsRef<str>>(&self, name: T) -> Option<&iterator::IteratorHandle> {
        self.statements
            .iter()
            .find_map(|statement| {
                if let Statement::IteratorDeclaration(handle) = statement {
                    if handle.name() == name.as_ref() {
                        return Some(handle);
                    }
                }

                None
            })
    }

    pub fn collections(&self) -> impl Iterator<Item = &collection::CollectionHandle> {
        self.statements().filter_map(|statement| match statement {
            Statement::CollectionDeclaration(handle) => Some(handle),
            _ => None,
        })
    }

    pub fn find_collection<T: AsRef<str>>(&self, name: T) -> Option<&collection::CollectionHandle> {
        self.statements
            .iter()
            .find_map(|statement| {
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
    /// this will disappear when we only have specialized structs
    Any(AnyStructHandle),
    /// structs that may be used as native function parameters
    FStruct(FStructHandle, AnyStructHandle),
}

impl From<AnyStructHandle> for StructType {
    fn from(x: AnyStructHandle) -> Self {
        StructType::Any(x)
    }
}

impl From<FStructHandle> for StructType {
    fn from(x: FStructHandle) -> Self {
        StructType::FStruct(x.clone(), x.to_any_struct())
    }
}

impl StructType {
    pub fn get_any_struct(&self) -> &AnyStructHandle {
        match self {
            StructType::Any(x) => x,
            StructType::FStruct(_, x) => x,
        }
    }

    pub fn declaration(&self) -> StructDeclarationHandle {
        match self {
            StructType::Any(x) => x.declaration.clone(),
            StructType::FStruct(_, x) => x.declaration.clone(),
        }
    }

    pub fn doc(&self) -> &Doc {
        match self {
            StructType::Any(x) => &x.doc,
            StructType::FStruct(_, x) => &x.doc,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            StructType::Any(x) => x.name(),
            StructType::FStruct(_, x) => x.name(),
        }
    }

    pub fn visibility(&self) -> Visibility {
        match self {
            StructType::Any(x) => x.visibility,
            StructType::FStruct(_, x) => x.visibility,
        }
    }

    pub fn fields(&self) -> impl Iterator<Item = &AnyStructField> {
        match self {
            StructType::Any(x) => x.fields.iter(),
            StructType::FStruct(_, x) => x.fields.iter(),
        }
    }

    pub fn all_fields_have_defaults(&self) -> bool {
        match self {
            StructType::Any(x) => x.all_fields_have_defaults(),
            StructType::FStruct(_, x) => x.all_fields_have_defaults(),
        }
    }

    pub fn find_field<T: AsRef<str>>(&self, field_name: T) -> Option<&AnyStructField> {
        self.fields().find(|f| f.name == field_name.as_ref())
    }
}

pub struct LibraryBuilder {
    name: String,
    version: Version,
    c_ffi_prefix: Option<String>,
    info: LibraryInfo,

    statements: Vec<Statement>,
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
    iterators: HashSet<iterator::IteratorHandle>,
    collections: HashSet<collection::CollectionHandle>,
}

impl LibraryBuilder {
    pub fn new<T: Into<String>>(name: T, version: Version, info: LibraryInfo) -> Self {
        Self {
            name: name.into(),
            version,
            c_ffi_prefix: None,
            info,

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

    pub fn build(self) -> Result<Library> {
        // Build symbols map
        let mut symbols = HashMap::new();
        for statement in &self.statements {
            match statement {
                Statement::Constants(_) => {}
                Statement::StructDeclaration(handle) => {
                    symbols.insert(
                        handle.name.clone(),
                        Symbol::Struct(self.structs.get(handle).unwrap().clone()),
                    );
                }
                Statement::StructDefinition(_) => (),
                Statement::EnumDefinition(handle) => {
                    symbols.insert(handle.name.clone(), Symbol::Enum(handle.clone()));
                }
                Statement::ErrorType(_) => {}
                Statement::ClassDeclaration(_) => (),
                Statement::ClassDefinition(handle) => {
                    symbols.insert(handle.name().to_string(), Symbol::Class(handle.clone()));
                }
                Statement::StaticClassDefinition(handle) => {
                    symbols.insert(handle.name.clone(), Symbol::StaticClass(handle.clone()));
                }
                Statement::InterfaceDefinition(handle) => {
                    symbols.insert(handle.name.clone(), Symbol::Interface(handle.clone()));
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
                    symbols.insert(handle.name.clone(), Symbol::Function(handle.clone()));
                }
            }
        }

        let lib = Library {
            name: self.name.clone(),
            version: self.version,
            c_ffi_prefix: self.c_ffi_prefix.unwrap_or(self.name),
            info: self.info,
            statements: self.statements,
            structs: self.structs,
            _classes: self.classes,
            symbols,
        };

        doc::validate_library_docs(&lib)?;

        Ok(lib)
    }

    pub fn c_ffi_prefix<T: Into<String>>(&mut self, c_ffi_prefix: T) -> Result<()> {
        match self.c_ffi_prefix {
            Some(_) => Err(BindingError::FfiPrefixAlreadySet),
            None => {
                self.c_ffi_prefix = Some(c_ffi_prefix.into());
                Ok(())
            }
        }
    }

    pub fn define_error_type<T: Into<String>>(
        &mut self,
        error_name: T,
        exception_name: T,
        exception_type: ExceptionType,
    ) -> Result<ErrorTypeBuilder> {
        let builder = self
            .define_enum(error_name)?
            .push("Ok", "Success, i.e. no error occurred")?;

        Ok(ErrorTypeBuilder::new(
            exception_name.into(),
            exception_type,
            builder,
        ))
    }

    pub fn define_constants<T: Into<String>>(&mut self, name: T) -> Result<ConstantSetBuilder> {
        let name = name.into();
        self.check_unique_symbol(&name)?;
        Ok(ConstantSetBuilder::new(self, name))
    }

    /// Forward declare a struct
    pub fn declare_struct<T: Into<String>>(&mut self, name: T) -> Result<StructDeclarationHandle> {
        let name = name.into();
        self.check_unique_symbol(&name)?;
        let handle = StructDeclarationHandle::new(StructDeclaration::new(name));
        self.structs_declarations.insert(handle.clone());
        self.statements
            .push(Statement::StructDeclaration(handle.clone()));
        Ok(handle)
    }

    /// Define ANY native structure - TODO - this will be removed in favor of specialized struct types
    pub fn define_any_struct(
        &mut self,
        declaration: &StructDeclarationHandle,
    ) -> Result<AnyStructBuilder> {
        self.validate_struct_declaration(declaration)?;
        if !self.structs.contains_key(declaration) {
            Ok(AnyStructBuilder::new(self, declaration.clone()))
        } else {
            Err(BindingError::NativeStructAlreadyDefined {
                handle: declaration.clone(),
            })
        }
    }

    /// Define a structure that can be used in native function arguments
    pub fn define_fstruct(
        &mut self,
        declaration: &StructDeclarationHandle,
    ) -> Result<FStructBuilder> {
        self.validate_struct_declaration(declaration)?;
        if !self.structs.contains_key(declaration) {
            Ok(FStructBuilder::new(self, declaration.clone()))
        } else {
            Err(BindingError::NativeStructAlreadyDefined {
                handle: declaration.clone(),
            })
        }
    }

    /// Define an enumeration
    pub fn define_enum<T: Into<String>>(&mut self, name: T) -> Result<EnumBuilder> {
        let name = name.into();
        self.check_unique_symbol(&name)?;
        Ok(EnumBuilder::new(self, name))
    }

    pub fn define_function<T: Into<String>>(&mut self, name: T) -> Result<FunctionBuilder> {
        let name = name.into();
        self.check_unique_symbol(&name)?;
        Ok(FunctionBuilder::new(self, name))
    }

    pub fn declare_class<T: Into<String>>(&mut self, name: T) -> Result<ClassDeclarationHandle> {
        let name = name.into();
        self.check_unique_symbol(&name)?;
        let handle = ClassDeclarationHandle::new(ClassDeclaration::new(name));
        self.class_declarations.insert(handle.clone());
        self.statements
            .push(Statement::ClassDeclaration(handle.clone()));
        Ok(handle)
    }

    pub fn define_class(&mut self, declaration: &ClassDeclarationHandle) -> Result<ClassBuilder> {
        self.validate_class_declaration(declaration)?;
        if !self.classes.contains_key(declaration) {
            Ok(ClassBuilder::new(self, declaration.clone()))
        } else {
            Err(BindingError::ClassAlreadyDefined {
                handle: declaration.clone(),
            })
        }
    }

    pub fn define_static_class<T: Into<String>>(&mut self, name: T) -> Result<StaticClassBuilder> {
        let name = name.into();
        self.check_unique_symbol(&name)?;
        Ok(StaticClassBuilder::new(self, name))
    }

    pub fn define_interface<T: Into<String>, D: Into<Doc>>(
        &mut self,
        name: T,
        doc: D,
    ) -> Result<InterfaceBuilder> {
        let name = name.into();
        self.check_unique_symbol(&name)?;
        Ok(InterfaceBuilder::new(self, name, doc.into()))
    }

    pub fn define_iterator(
        &mut self,
        native_func: &FunctionHandle,
        item_type: &AnyStructHandle,
    ) -> Result<iterator::IteratorHandle> {
        self.define_iterator_impl(false, native_func, item_type)
    }

    pub fn define_iterator_with_lifetime(
        &mut self,
        native_func: &FunctionHandle,
        item_type: &AnyStructHandle,
    ) -> Result<iterator::IteratorHandle> {
        self.define_iterator_impl(true, native_func, item_type)
    }

    fn define_iterator_impl(
        &mut self,
        has_lifetime: bool,
        native_func: &FunctionHandle,
        item_type: &AnyStructHandle,
    ) -> Result<iterator::IteratorHandle> {
        let iter = iterator::IteratorHandle::new(iterator::Iterator::new(
            has_lifetime,
            native_func,
            item_type,
        )?);
        self.iterators.insert(iter.clone());
        self.statements
            .push(Statement::IteratorDeclaration(iter.clone()));
        Ok(iter)
    }

    pub fn define_collection(
        &mut self,
        create_func: &FunctionHandle,
        delete_func: &FunctionHandle,
        add_func: &FunctionHandle,
    ) -> Result<collection::CollectionHandle> {
        let collection = collection::CollectionHandle::new(collection::Collection::new(
            create_func,
            delete_func,
            add_func,
        )?);
        self.collections.insert(collection.clone());
        self.statements
            .push(Statement::CollectionDeclaration(collection.clone()));
        Ok(collection)
    }

    fn check_unique_symbol(&mut self, name: &str) -> Result<()> {
        if self.symbol_names.insert(name.to_string()) {
            Ok(())
        } else {
            Err(BindingError::SymbolAlreadyUsed {
                name: name.to_string(),
            })
        }
    }

    fn validate_function(&self, native_function: &FunctionHandle) -> Result<()> {
        if self.functions.contains(native_function) {
            Ok(())
        } else {
            Err(BindingError::NativeFunctionNotPartOfThisLib {
                handle: native_function.clone(),
            })
        }
    }

    fn validate_type(&self, type_to_validate: &AnyType) -> Result<()> {
        match type_to_validate {
            AnyType::StructRef(native_struct) => self.validate_struct_declaration(native_struct),
            AnyType::Struct(native_struct) => {
                self.validate_struct(&StructType::Any(native_struct.clone()))
            }
            AnyType::Basic(BasicType::Enum(native_enum)) => self.validate_enum(native_enum),
            AnyType::Interface(interface) => self.validate_interface(interface),
            AnyType::ClassRef(class_declaration) => {
                self.validate_class_declaration(class_declaration)
            }
            AnyType::Iterator(iter) => self.validate_iterator(iter),
            AnyType::Collection(collection) => self.validate_collection(collection),
            _ => Ok(()),
        }
    }

    fn validate_struct_declaration(&self, native_struct: &StructDeclarationHandle) -> Result<()> {
        if self.structs_declarations.contains(native_struct) {
            Ok(())
        } else {
            Err(BindingError::NativeStructNotPartOfThisLib {
                handle: native_struct.clone(),
            })
        }
    }

    fn validate_struct(&self, native_struct: &StructType) -> Result<()> {
        if self.structs.contains_key(&native_struct.declaration()) {
            Ok(())
        } else {
            Err(BindingError::NativeStructNotPartOfThisLib {
                handle: native_struct.declaration(),
            })
        }
    }

    fn validate_enum(&self, native_enum: &EnumHandle) -> Result<()> {
        if self.enums.contains(native_enum) {
            Ok(())
        } else {
            Err(BindingError::NativeEnumNotPartOfThisLib {
                handle: native_enum.clone(),
            })
        }
    }

    fn validate_interface(&self, interface: &InterfaceHandle) -> Result<()> {
        if self.interfaces.contains(interface) {
            Ok(())
        } else {
            Err(BindingError::InterfaceNotPartOfThisLib {
                handle: interface.clone(),
            })
        }
    }

    fn validate_class_declaration(&self, class_declaration: &ClassDeclarationHandle) -> Result<()> {
        if self.class_declarations.contains(class_declaration) {
            Ok(())
        } else {
            Err(BindingError::ClassNotPartOfThisLib {
                handle: class_declaration.clone(),
            })
        }
    }

    fn validate_iterator(&self, iter: &iterator::IteratorHandle) -> Result<()> {
        if self.iterators.contains(iter) {
            Ok(())
        } else {
            Err(BindingError::IteratorNotPartOfThisLib {
                handle: iter.clone(),
            })
        }
    }

    fn validate_collection(&self, collection: &collection::CollectionHandle) -> Result<()> {
        if self.collections.contains(collection) {
            Ok(())
        } else {
            Err(BindingError::CollectionNotPartOfThisLib {
                handle: collection.clone(),
            })
        }
    }
}
