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

use crate::any_struct::*;
use crate::callback::*;
use crate::class::*;
use crate::doc::Doc;
use crate::native_enum::*;
use crate::native_function::*;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Deref;
use std::ptr;
use std::rc::Rc;
use thiserror::Error;

pub use semver::Version;

pub mod any_struct;
pub mod callback;
pub mod class;
pub mod collection;
pub mod constants;
pub mod doc;
pub mod error_type;
pub mod formatting;
pub mod function_struct;
pub mod iterator;
mod library;
pub mod native_enum;
pub mod native_function;
pub mod platforms;
pub mod struct_common;
pub mod types;
pub mod util;

pub use crate::doc::doc;
pub use library::*;

use crate::error_type::ErrorType;
use crate::function_struct::FStructHandle;
use crate::struct_common::StructDeclarationHandle;
use crate::types::{AnyType, BasicType};

type BindResult<T> = Result<T, BindingError>;

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

    // function errors
    #[error("Function '{}' is not part of this library", handle.name)]
    FunctionNotPartOfThisLib { handle: FunctionHandle },
    #[error(
        "Return type of native function '{}' was already defined to '{:?}'",
        func_name,
        return_type
    )]
    ReturnTypeAlreadyDefined {
        func_name: String,
        return_type: ReturnType,
    },
    #[error("Return type of native function '{}' was not defined", func_name)]
    ReturnTypeNotDefined { func_name: String },

    // enum errors
    #[error("Enum '{}' is not part of this library", handle.name)]
    EnumNotPartOfThisLib { handle: EnumHandle },
    #[error(
        "Enum '{}' already contains a variant with name '{}'",
        name,
        variant_name
    )]
    EnumAlreadyContainsVariantWithSameName { name: String, variant_name: String },
    #[error(
        "Enum '{}' already contains a variant with value '{}'",
        name,
        variant_value
    )]
    EnumAlreadyContainsVariantWithSameValue { name: String, variant_value: i32 },
    #[error("Enum '{}' does not contain a variant named '{}'", name, variant_name)]
    EnumDoesNotContainVariant { name: String, variant_name: String },

    // Structure errors
    #[error("Native struct '{}' was already defined", handle.name)]
    StructAlreadyDefined { handle: StructDeclarationHandle },
    #[error("Native struct '{}' is not part of this library", handle.name)]
    StructNotPartOfThisLib { handle: StructDeclarationHandle },
    #[error("Native struct '{}' already contains field with name '{}'", handle.name, field_name)]
    StructAlreadyContainsFieldWithSameName {
        handle: StructDeclarationHandle,
        field_name: String,
    },
    // Class errors
    #[error("Class '{}' was already defined", handle.name)]
    ClassAlreadyDefined { handle: ClassDeclarationHandle },
    #[error("Class '{}' is not part of this library", handle.name)]
    ClassNotPartOfThisLib { handle: ClassDeclarationHandle },
    #[error("First parameter of function '{}' is not of type '{}' as expected for a method of a class", function.name, handle.name)]
    FirstMethodParameterIsNotClassType {
        handle: ClassDeclarationHandle,
        function: FunctionHandle,
    },
    #[error("Constructor for class '{}' was already defined", handle.name)]
    ConstructorAlreadyDefined { handle: ClassDeclarationHandle },
    #[error("Native function '{}' does not return '{}' as expected for a constructor", function.name, handle.name)]
    ConstructorReturnTypeDoesNotMatch {
        handle: ClassDeclarationHandle,
        function: FunctionHandle,
    },
    #[error("Destructor for class '{}' was already defined", handle.name)]
    DestructorAlreadyDefined { handle: ClassDeclarationHandle },
    #[error("Function '{}' does not take a single '{}' parameter as expected for a destructor", function.name, handle.name)]
    DestructorTakesMoreThanOneParameter {
        handle: ClassDeclarationHandle,
        function: FunctionHandle,
    },
    #[error("Destructor for class '{}' cannot fail", handle.name)]
    DestructorCannotFail { handle: ClassDeclarationHandle },
    #[error("No destructor defined for class '{}', but asking for manual/disposable destruction", handle.name)]
    NoDestructorForManualDestruction { handle: ClassDeclarationHandle },

    // Async errors
    #[error("Function '{}' cannot be used as an async method because it doesn't have a interface parameter", handle.name)]
    AsyncMethodNoInterface { handle: FunctionHandle },
    #[error("Function '{}' cannot be used as an async method because it has too many interface parameters", handle.name)]
    AsyncMethodTooManyInterface { handle: FunctionHandle },
    #[error("Function '{}' cannot be used as an async method because its interface parameter doesn't have a single callback", handle.name)]
    AsyncInterfaceNotSingleCallback { handle: FunctionHandle },
    #[error("Function '{}' cannot be used as an async method because its interface parameter single callback does not have a single parameter (other than the arg param)", handle.name)]
    AsyncCallbackNotSingleParam { handle: FunctionHandle },
    #[error("Function '{}' cannot be used as an async method because its interface parameter single callback does not return void", handle.name)]
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
