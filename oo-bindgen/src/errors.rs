use crate::class::ClassDeclarationHandle;
use crate::collection::CollectionHandle;
use crate::enum_type::EnumHandle;
use crate::function::FunctionHandle;
use crate::interface::InterfaceHandle;
use crate::structs::common::{FieldName, StructDeclarationHandle};
use thiserror::Error;

pub type BindResult<T> = Result<T, BindingError>;

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
    #[error("Return type of native function '{}' was already defined", func_name)]
    ReturnTypeAlreadyDefined { func_name: String },
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
        field_name: FieldName,
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
    #[error(
        "Symbol '{}' is reserved and cannot be used as an interface method name",
        name
    )]
    InterfaceMethodWithReservedName { name: &'static str },
    #[error("Interface '{}' is not part of this library", handle.name)]
    InterfaceNotPartOfThisLib { handle: InterfaceHandle },
    #[error(
        "Symbol '{}' is reserved and cannot be used as a callback argument name",
        name
    )]
    CallbackMethodArgumentWithReservedName { name: &'static str },
    // Iterator errors
    #[error("Iterator native function '{}' does not take a single class ref parameter", handle.name)]
    IteratorNotSingleClassRefParam { handle: FunctionHandle },
    #[error("Iterator native function '{}' does not return a struct ref value", handle.name)]
    IteratorReturnTypeNotStructRef { handle: FunctionHandle },
    #[error("Iterator '{}' is not part of this library", handle.name())]
    IteratorNotPartOfThisLib {
        handle: crate::iterator::IteratorHandle,
    },
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
    CollectionNotPartOfThisLib { handle: CollectionHandle },
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
