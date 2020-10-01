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
    safe_packed_borrows,
    while_true,
    bare_trait_objects
)]

use crate::callback::*;
use crate::class::*;
use crate::doc::Doc;
use crate::native_enum::*;
use crate::native_function::*;
use crate::native_struct::*;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Deref;
use std::ptr;
use std::rc::Rc;
use thiserror::Error;

pub use semver::Version;

pub mod callback;
pub mod class;
pub mod collection;
pub mod doc;
pub mod formatting;
pub mod iterator;
pub mod native_enum;
pub mod native_function;
pub mod native_struct;
pub mod platforms;

pub use crate::doc::doc;

type Result<T> = std::result::Result<T, BindingError>;

#[derive(Error, Debug)]
pub enum BindingError {
    // Global errors
    #[error("Symbol '{}' already used in the library", name)]
    SymbolAlreadyUsed { name: String },
    #[error("Description already set")]
    DescriptionAlreadySet,

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
    NativeFunctionNotPartOfThisLib { handle: NativeFunctionHandle },
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
    NativeEnumNotPartOfThisLib { handle: NativeEnumHandle },
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

    // Structure errors
    #[error("Native struct '{}' was already defined", handle.name)]
    NativeStructAlreadyDefined {
        handle: NativeStructDeclarationHandle,
    },
    #[error("Native struct '{}' is not part of this library", handle.name)]
    NativeStructNotPartOfThisLib {
        handle: NativeStructDeclarationHandle,
    },
    #[error("Native struct '{}' already contains element with name '{}'", handle.name, element_name)]
    NativeStructAlreadyContainsElementWithSameName {
        handle: NativeStructDeclarationHandle,
        element_name: String,
    },
    #[error("First parameter of native function '{}' is not of type '{}' as expected for a method of a struct", native_func.name, handle.name)]
    FirstMethodParameterIsNotStructType {
        handle: NativeStructDeclarationHandle,
        native_func: NativeFunctionHandle,
    },
    #[error("Struct '{}' was already defined", handle.name)]
    StructAlreadyDefined {
        handle: NativeStructDeclarationHandle,
    },
    #[error("Struct '{}' already contains element or method with name '{}'", handle.name, element_name)]
    StructAlreadyContainsElementWithSameName {
        handle: NativeStructDeclarationHandle,
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
        native_func: NativeFunctionHandle,
    },
    #[error("Constructor for class '{}' was already defined", handle.name)]
    ConstructorAlreadyDefined { handle: ClassDeclarationHandle },
    #[error("Native function '{}' does not return '{}' as expected for a constructor", native_func.name, handle.name)]
    ConstructorReturnTypeDoesNotMatch {
        handle: ClassDeclarationHandle,
        native_func: NativeFunctionHandle,
    },
    #[error("Destructor for class '{}' was already defined", handle.name)]
    DestructorAlreadyDefined { handle: ClassDeclarationHandle },
    #[error("Native function '{}' does not take a single '{}' parameter as expected for a destructor", native_func.name, handle.name)]
    DestructorTakesMoreThanOneParameter {
        handle: ClassDeclarationHandle,
        native_func: NativeFunctionHandle,
    },

    // Async errors
    #[error("Native function '{}' cannot be used as an async method because it doesn't have a OneTimeCallback parameter", handle.name)]
    AsyncNativeMethodNoOneTimeCallback { handle: NativeFunctionHandle },
    #[error("Native function '{}' cannot be used as an async method because it has too many OneTimeCallback parameters", handle.name)]
    AsyncNativeMethodTooManyOneTimeCallback { handle: NativeFunctionHandle },
    #[error("Native function '{}' cannot be used as an async method because its OneTimeCallback parameter doesn't have a single callback", handle.name)]
    AsyncOneTimeCallbackNotSingleCallback { handle: NativeFunctionHandle },
    #[error("Native function '{}' cannot be used as an async method because its OneTimeCallback parameter single callback does not have a single parameter (other than the arg param)", handle.name)]
    AsyncCallbackNotSingleParam { handle: NativeFunctionHandle },
    #[error("Native function '{}' cannot be used as an async method because its OneTimeCallback parameter single callback does not return void", handle.name)]
    AsyncCallbackReturnTypeNotVoid { handle: NativeFunctionHandle },

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
    #[error("One-time callback '{}' is not part of this library", handle.name)]
    OneTimeCallbackNotPartOfThisLib { handle: OneTimeCallbackHandle },

    // Iterator errors
    #[error("Iterator native function '{}' does not take a single class ref parameter", handle.name)]
    IteratorNotSingleClassRefParam { handle: NativeFunctionHandle },
    #[error("Iterator native function '{}' does not return a struct ref value", handle.name)]
    IteratorReturnTypeNotStructRef { handle: NativeFunctionHandle },
    #[error("Iterator '{}' is not part of this library", handle.name())]
    IteratorNotPartOfThisLib { handle: iterator::IteratorHandle },

    // Collection errors
    #[error("Invalid native function '{}' signature for create_func of collection", handle.name)]
    CollectionCreateFuncInvalidSignature { handle: NativeFunctionHandle },
    #[error("Invalid native function '{}' signature for delete_func of collection", handle.name)]
    CollectionDeleteFuncInvalidSignature { handle: NativeFunctionHandle },
    #[error("Invalid native function '{}' signature for add_func of collection", handle.name)]
    CollectionAddFuncInvalidSignature { handle: NativeFunctionHandle },
    #[error("Collection '{}' is not part of this library", handle.name())]
    CollectionNotPartOfThisLib {
        handle: collection::CollectionHandle,
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
    NativeFunction(NativeFunctionHandle),
    Struct(StructHandle),
    Enum(NativeEnumHandle),
    Class(ClassHandle),
    Interface(InterfaceHandle),
    OneTimeCallback(OneTimeCallbackHandle),
    Iterator(iterator::IteratorHandle),
    Collection(collection::CollectionHandle),
}

#[derive(Debug)]
pub enum Statement {
    NativeStructDeclaration(NativeStructDeclarationHandle),
    NativeStructDefinition(NativeStructHandle),
    StructDefinition(StructHandle),
    EnumDefinition(NativeEnumHandle),
    ClassDeclaration(ClassDeclarationHandle),
    ClassDefinition(ClassHandle),
    InterfaceDefinition(InterfaceHandle),
    OneTimeCallbackDefinition(OneTimeCallbackHandle),
    IteratorDeclaration(iterator::IteratorHandle),
    CollectionDeclaration(collection::CollectionHandle),
    NativeFunctionDeclaration(NativeFunctionHandle),
}

pub struct Library {
    pub name: String,
    pub version: Version,
    pub description: Option<String>,
    pub license: Vec<String>,
    statements: Vec<Statement>,
    structs: HashMap<NativeStructDeclarationHandle, StructHandle>,
    _classes: HashMap<ClassDeclarationHandle, ClassHandle>,
    symbols: HashMap<String, Symbol>,
}

impl Library {
    pub fn native_functions(&self) -> impl Iterator<Item = &NativeFunctionHandle> {
        self.into_iter().filter_map(|statement| match statement {
            Statement::NativeFunctionDeclaration(handle) => Some(handle),
            _ => None,
        })
    }

    pub fn native_structs(&self) -> impl Iterator<Item = &NativeStructHandle> {
        self.into_iter().filter_map(|statement| match statement {
            Statement::NativeStructDefinition(handle) => Some(handle),
            _ => None,
        })
    }

    pub fn structs(&self) -> impl Iterator<Item = &StructHandle> {
        self.structs.values()
    }

    pub fn find_struct(&self, name: &str) -> Option<&StructHandle> {
        self.symbol(name)
            .iter()
            .filter_map(|symbol| {
                if let Symbol::Struct(handle) = symbol {
                    Some(handle)
                } else {
                    None
                }
            })
            .next()
    }

    pub fn native_enums(&self) -> impl Iterator<Item = &NativeEnumHandle> {
        self.into_iter().filter_map(|statement| match statement {
            Statement::EnumDefinition(handle) => Some(handle),
            _ => None,
        })
    }

    pub fn find_enum(&self, name: &str) -> Option<&NativeEnumHandle> {
        self.symbol(name)
            .iter()
            .filter_map(|symbol| {
                if let Symbol::Enum(handle) = symbol {
                    Some(handle)
                } else {
                    None
                }
            })
            .next()
    }

    pub fn classes(&self) -> impl Iterator<Item = &ClassHandle> {
        self.into_iter().filter_map(|statement| match statement {
            Statement::ClassDefinition(handle) => Some(handle),
            _ => None,
        })
    }

    pub fn find_class(&self, name: &str) -> Option<&ClassHandle> {
        self.symbol(name)
            .iter()
            .filter_map(|symbol| {
                if let Symbol::Class(handle) = symbol {
                    Some(handle)
                } else {
                    None
                }
            })
            .next()
    }

    pub fn interfaces(&self) -> impl Iterator<Item = &InterfaceHandle> {
        self.into_iter().filter_map(|statement| match statement {
            Statement::InterfaceDefinition(handle) => Some(handle),
            _ => None,
        })
    }

    pub fn one_time_callbacks(&self) -> impl Iterator<Item = &OneTimeCallbackHandle> {
        self.into_iter().filter_map(|statement| match statement {
            Statement::OneTimeCallbackDefinition(handle) => Some(handle),
            _ => None,
        })
    }

    pub fn symbol(&self, symbol_name: &str) -> Option<&Symbol> {
        self.symbols.get(symbol_name)
    }
}

impl<'a> IntoIterator for &'a Library {
    type Item = &'a Statement;
    type IntoIter = std::slice::Iter<'a, Statement>;
    fn into_iter(self) -> Self::IntoIter {
        self.statements.iter()
    }
}

pub struct LibraryBuilder {
    name: String,
    version: Version,
    description: Option<String>,
    license: Vec<String>,

    statements: Vec<Statement>,
    symbol_names: HashSet<String>,

    native_structs_declarations: HashSet<NativeStructDeclarationHandle>,
    native_structs: HashMap<NativeStructDeclarationHandle, NativeStructHandle>,
    defined_structs: HashMap<NativeStructHandle, StructHandle>,

    native_enums: HashSet<NativeEnumHandle>,

    class_declarations: HashSet<ClassDeclarationHandle>,
    classes: HashMap<ClassDeclarationHandle, ClassHandle>,

    interfaces: HashSet<InterfaceHandle>,
    one_time_callbacks: HashSet<OneTimeCallbackHandle>,

    iterators: HashSet<iterator::IteratorHandle>,
    collections: HashSet<collection::CollectionHandle>,

    native_functions: HashSet<NativeFunctionHandle>,
}

impl LibraryBuilder {
    pub fn new(name: &str, version: Version) -> Self {
        Self {
            name: name.to_string(),
            version,
            description: None,
            license: Vec::new(),

            statements: Vec::new(),
            symbol_names: HashSet::new(),

            native_structs_declarations: HashSet::new(),
            native_structs: HashMap::new(),
            defined_structs: HashMap::new(),

            native_enums: HashSet::new(),

            class_declarations: HashSet::new(),
            classes: HashMap::new(),

            interfaces: HashSet::new(),
            one_time_callbacks: HashSet::new(),

            iterators: HashSet::new(),
            collections: HashSet::new(),

            native_functions: HashSet::new(),
        }
    }

    pub fn build(self) -> Result<Library> {
        // Update all native structs to full structs
        let mut structs = HashMap::with_capacity(self.defined_structs.len());
        for structure in self.defined_structs.values() {
            structs.insert(structure.declaration(), structure.clone());
        }
        for native_struct in self.native_structs.values() {
            if !self.defined_structs.contains_key(&native_struct) {
                structs.insert(
                    native_struct.declaration(),
                    StructHandle::new(Struct::new(native_struct.clone())),
                );
            }
        }

        // Build symbols map
        let mut symbols = HashMap::new();
        for statement in &self.statements {
            match statement {
                Statement::NativeStructDeclaration(handle) => {
                    symbols.insert(
                        handle.name.clone(),
                        Symbol::Struct(structs.get(&handle).unwrap().clone()),
                    );
                }
                Statement::NativeStructDefinition(_) => (),
                Statement::StructDefinition(_) => (),
                Statement::EnumDefinition(handle) => {
                    symbols.insert(handle.name.clone(), Symbol::Enum(handle.clone()));
                }
                Statement::ClassDeclaration(_) => (),
                Statement::ClassDefinition(handle) => {
                    symbols.insert(handle.name().to_string(), Symbol::Class(handle.clone()));
                }
                Statement::InterfaceDefinition(handle) => {
                    symbols.insert(handle.name.clone(), Symbol::Interface(handle.clone()));
                }
                Statement::OneTimeCallbackDefinition(handle) => {
                    symbols.insert(handle.name.clone(), Symbol::OneTimeCallback(handle.clone()));
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
                Statement::NativeFunctionDeclaration(handle) => {
                    symbols.insert(handle.name.clone(), Symbol::NativeFunction(handle.clone()));
                }
            }
        }

        let lib = Library {
            name: self.name,
            version: self.version,
            description: self.description,
            license: self.license,

            statements: self.statements,
            structs,
            _classes: self.classes,
            symbols,
        };

        doc::validate_library_docs(&lib)?;

        Ok(lib)
    }

    pub fn description(&mut self, description: &str) -> Result<()> {
        match self.description {
            Some(_) => Err(BindingError::DescriptionAlreadySet),
            None => {
                self.description = Some(description.to_string());
                Ok(())
            }
        }
    }

    pub fn license(&mut self, license: Vec<String>) -> Result<()> {
        self.license = license;
        Ok(())
    }

    /// Forward declare a native structure
    pub fn declare_native_struct(&mut self, name: &str) -> Result<NativeStructDeclarationHandle> {
        self.check_unique_symbol(name)?;
        let handle =
            NativeStructDeclarationHandle::new(NativeStructDeclaration::new(name.to_string()));
        self.native_structs_declarations.insert(handle.clone());
        self.statements
            .push(Statement::NativeStructDeclaration(handle.clone()));
        Ok(handle)
    }

    /// Define a native structure
    pub fn define_native_struct(
        &mut self,
        declaration: &NativeStructDeclarationHandle,
    ) -> Result<NativeStructBuilder> {
        self.validate_native_struct_declaration(declaration)?;
        if !self.native_structs.contains_key(&declaration) {
            Ok(NativeStructBuilder::new(self, declaration.clone()))
        } else {
            Err(BindingError::NativeStructAlreadyDefined {
                handle: declaration.clone(),
            })
        }
    }

    pub fn define_struct(&mut self, definition: &NativeStructHandle) -> Result<StructBuilder> {
        self.validate_native_struct(definition)?;
        if !self.defined_structs.contains_key(&definition) {
            Ok(StructBuilder::new(self, definition.clone()))
        } else {
            Err(BindingError::StructAlreadyDefined {
                handle: definition.declaration(),
            })
        }
    }

    /// Define an enumeration
    pub fn define_native_enum(&mut self, name: &str) -> Result<NativeEnumBuilder> {
        self.check_unique_symbol(name)?;
        Ok(NativeEnumBuilder::new(self, name.to_string()))
    }

    pub fn declare_native_function(&mut self, name: &str) -> Result<NativeFunctionBuilder> {
        self.check_unique_symbol(name)?;
        Ok(NativeFunctionBuilder::new(self, name.to_string()))
    }

    pub fn declare_class(&mut self, name: &str) -> Result<ClassDeclarationHandle> {
        self.check_unique_symbol(name)?;
        let handle = ClassDeclarationHandle::new(ClassDeclaration::new(name.to_string()));
        self.class_declarations.insert(handle.clone());
        self.statements
            .push(Statement::ClassDeclaration(handle.clone()));
        Ok(handle)
    }

    pub fn define_class(&mut self, declaration: &ClassDeclarationHandle) -> Result<ClassBuilder> {
        self.validate_class_declaration(declaration)?;
        if !self.classes.contains_key(&declaration) {
            Ok(ClassBuilder::new(self, declaration.clone()))
        } else {
            Err(BindingError::ClassAlreadyDefined {
                handle: declaration.clone(),
            })
        }
    }

    pub fn define_interface<D: Into<Doc>>(
        &mut self,
        name: &str,
        doc: D,
    ) -> Result<InterfaceBuilder> {
        self.check_unique_symbol(name)?;
        Ok(InterfaceBuilder::new(self, name.to_string(), doc.into()))
    }

    pub fn define_one_time_callback<D: Into<Doc>>(
        &mut self,
        name: &str,
        doc: D,
    ) -> Result<OneTimeCallbackBuilder> {
        self.check_unique_symbol(name)?;
        Ok(OneTimeCallbackBuilder::new(
            self,
            name.to_string(),
            doc.into(),
        ))
    }

    pub fn define_iterator(
        &mut self,
        native_func: &NativeFunctionHandle,
        item_type: &NativeStructHandle,
    ) -> Result<iterator::IteratorHandle> {
        self.define_iterator_impl(false, native_func, item_type)
    }

    pub fn define_iterator_with_lifetime(
        &mut self,
        native_func: &NativeFunctionHandle,
        item_type: &NativeStructHandle,
    ) -> Result<iterator::IteratorHandle> {
        self.define_iterator_impl(true, native_func, item_type)
    }

    fn define_iterator_impl(
        &mut self,
        has_lifetime: bool,
        native_func: &NativeFunctionHandle,
        item_type: &NativeStructHandle,
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
        create_func: &NativeFunctionHandle,
        delete_func: &NativeFunctionHandle,
        add_func: &NativeFunctionHandle,
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

    fn validate_native_function(&self, native_function: &NativeFunctionHandle) -> Result<()> {
        if self.native_functions.contains(native_function) {
            Ok(())
        } else {
            Err(BindingError::NativeFunctionNotPartOfThisLib {
                handle: native_function.clone(),
            })
        }
    }

    fn validate_type(&self, type_to_validate: &Type) -> Result<()> {
        match type_to_validate {
            Type::StructRef(native_struct) => {
                self.validate_native_struct_declaration(native_struct)
            }
            Type::Struct(native_struct) => self.validate_native_struct(native_struct),
            Type::Enum(native_enum) => self.validate_native_enum(native_enum),
            Type::Interface(interface) => self.validate_interface(interface),
            Type::OneTimeCallback(cb) => self.validate_one_time_callback(cb),
            Type::ClassRef(class_declaration) => self.validate_class_declaration(class_declaration),
            Type::Iterator(iter) => self.validate_iterator(iter),
            Type::Collection(collection) => self.validate_collection(collection),
            _ => Ok(()),
        }
    }

    fn validate_native_struct_declaration(
        &self,
        native_struct: &NativeStructDeclarationHandle,
    ) -> Result<()> {
        if self.native_structs_declarations.contains(native_struct) {
            Ok(())
        } else {
            Err(BindingError::NativeStructNotPartOfThisLib {
                handle: native_struct.clone(),
            })
        }
    }

    fn validate_native_struct(&self, native_struct: &NativeStructHandle) -> Result<()> {
        if self.native_structs.contains_key(&native_struct.declaration) {
            Ok(())
        } else {
            Err(BindingError::NativeStructNotPartOfThisLib {
                handle: native_struct.declaration.clone(),
            })
        }
    }

    fn validate_native_enum(&self, native_enum: &NativeEnumHandle) -> Result<()> {
        if self.native_enums.contains(native_enum) {
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

    fn validate_one_time_callback(&self, cb: &OneTimeCallbackHandle) -> Result<()> {
        if self.one_time_callbacks.contains(cb) {
            Ok(())
        } else {
            Err(BindingError::OneTimeCallbackNotPartOfThisLib { handle: cb.clone() })
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
