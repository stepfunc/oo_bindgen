use thiserror::Error;

use crate::class::{ClassDeclarationHandle, ClassType};
use crate::collection::CollectionHandle;
use crate::enum_type::EnumHandle;
use crate::function::FunctionHandle;
use crate::interface::InterfaceHandle;
use crate::name::{BadName, Name};
use crate::structs::{ConstructorDefault, StructDeclarationHandle};
use backtrace::Backtrace;
use std::fmt::Formatter;

pub type BindResult<T> = Result<T, BindingError>;
pub type BackTraced<T> = Result<T, BackTracedBindingError>;

#[derive(Debug)]
pub struct BackTracedBindingError {
    pub error: BindingError,
    pub backtrace: Backtrace,
}

impl From<BindingError> for BackTracedBindingError {
    fn from(error: BindingError) -> Self {
        BackTracedBindingError {
            error,
            backtrace: Backtrace::new(),
        }
    }
}

impl From<BadName> for BackTracedBindingError {
    fn from(err: BadName) -> Self {
        BindingError::BadName { err }.into()
    }
}

impl std::fmt::Display for BackTracedBindingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.error)?;
        writeln!(f, "origin:")?;
        writeln!(f, "{:?}", self.backtrace)
    }
}

impl std::error::Error for BackTracedBindingError {}

#[derive(Error, Debug)]
pub enum BindingError {
    // Global errors
    #[error("Symbol '{}' already used in the library", name)]
    SymbolAlreadyUsed { name: Name },

    // Bad name
    #[error("'{}'", err)]
    BadName { err: BadName },

    #[error("Method name '{}' contains the name of the owning class '{}'", class.name, method_name)]
    BadMethodName {
        class: ClassDeclarationHandle,
        method_name: Name,
    },

    // Documentation error
    #[error("Invalid documentation string")]
    InvalidDocString,
    #[error("Documentation of '{}' was already defined", symbol_name)]
    DocAlreadyDefined { symbol_name: Name },
    #[error("Documentation of '{}' was not defined", symbol_name)]
    DocNotDefined { symbol_name: Name },
    #[error(
        "Documentation of '{}' contains an argument reference to '{}' which is not valid in this context",
        symbol_name,
        ref_name
    )]
    DocInvalidArgumentContext {
        symbol_name: String,
        ref_name: String,
    },
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
    ReturnTypeAlreadyDefined { func_name: Name },
    #[error("Return type of native function '{}' was not defined", func_name)]
    ReturnTypeNotDefined { func_name: Name },

    // enum errors
    #[error("Enum '{}' is not part of this library", handle.name)]
    EnumNotPartOfThisLib { handle: EnumHandle },
    #[error(
        "Enum '{}' already contains a variant with name '{}'",
        name,
        variant_name
    )]
    EnumAlreadyContainsVariantWithSameName { name: Name, variant_name: String },
    #[error(
        "Enum '{}' already contains a variant with value '{}'",
        name,
        variant_value
    )]
    EnumAlreadyContainsVariantWithSameValue { name: Name, variant_value: i32 },
    #[error("Enum '{}' does not contain a variant named '{}'", name, variant_name)]
    EnumDoesNotContainVariant { name: Name, variant_name: String },

    // Structure errors
    #[error(
        "Duplicate constructor field definition '{}' in struct '{}",
        field_name,
        struct_name
    )]
    StructConstructorDuplicateField { struct_name: Name, field_name: Name },
    #[error(
        "Struct ({}) constructor {} uses the same arguments as constructor {}",
        struct_name,
        this_constructor,
        other_constructor
    )]
    StructDuplicateConstructorArgs {
        struct_name: Name,
        this_constructor: Name,
        other_constructor: Name,
    },
    #[error(
        "Constructor field '{}' doesn't exist within struct '{}",
        field_name,
        struct_name
    )]
    StructConstructorUnknownField { struct_name: Name, field_name: Name },
    #[error(
        "Constructor field type '{}' doesn't match value '{:?}",
        field_type,
        value
    )]
    StructConstructorBadValueForType {
        field_type: String,
        value: ConstructorDefault,
    },
    #[error("Constructor contains a default struct field but struct '{}' doesn't have a default constructor", struct_name)]
    StructConstructorStructFieldWithoutDefaultConstructor { struct_name: String },
    #[error(
        "Struct '{}' already contains a constructor with the name '{}'",
        struct_name,
        constructor_name
    )]
    StructConstructorDuplicateName {
        struct_name: Name,
        constructor_name: Name,
    },
    #[error("Native struct '{}' was already defined", handle.name)]
    StructAlreadyDefined { handle: StructDeclarationHandle },
    #[error("Native struct '{}' is not part of this library", handle.name)]
    StructNotPartOfThisLib { handle: StructDeclarationHandle },
    #[error("Native struct '{}' already contains field with name '{}'", handle.name, field_name)]
    StructAlreadyContainsFieldWithSameName {
        handle: StructDeclarationHandle,
        field_name: Name,
    },
    // Class errors
    #[error("Expected '{:?}' but received {:?}", expected, received)]
    WrongClassType {
        expected: ClassType,
        received: ClassType,
    },
    #[error("Class '{}' was already defined", handle.name)]
    ClassAlreadyDefined { handle: ClassDeclarationHandle },

    #[error("Method '{}' is associated with class '{}' but was added to '{}'", name, declared.name, added_to.name)]
    ClassMethodWrongAssociatedClass {
        name: Name,
        declared: ClassDeclarationHandle,
        added_to: ClassDeclarationHandle,
    },

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
    #[error("AsyncMethods can only have one (implicit) interface parameter")]
    AsyncMethodMoreThanOneInterface,
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
    #[error("Iterator '{}' is not part of this library", handle.name())]
    IteratorNotPartOfThisLib {
        handle: crate::iterator::IteratorHandle,
    },
    // Collection errors
    #[error("Invalid native function '{}' signature for create_func of collection", handle.name)]
    CollectionCreateFuncInvalidSignature { handle: FunctionHandle },
    #[error("Invalid native function '{}' signature for delete_func of collection", handle.name)]
    CollectionDeleteFuncInvalidSignature { handle: FunctionHandle },
    #[error("Invalid native function '{}' signature for add_func of collection", handle.name)]
    CollectionAddFuncInvalidSignature { handle: FunctionHandle },
    #[error("Collection native functions '{}' cannot fail", handle.name)]
    CollectionFunctionsCannotFail { handle: FunctionHandle },
    #[error("Collection has already been defined for class '{}'", handle.name)]
    CollectionAlreadyDefinedForClass { handle: ClassDeclarationHandle },
    #[error("Collection '{}' is not part of this library", handle.name())]
    CollectionNotPartOfThisLib { handle: CollectionHandle },
    #[error(
        "ConstantSet '{}' already contains constant name  '{}'",
        set_name,
        constant_name
    )]
    ConstantNameAlreadyUsed { set_name: Name, constant_name: Name },
    #[error(
        "Function '{}' already has an error type specified: '{}'",
        function,
        error_type
    )]
    ErrorTypeAlreadyDefined { function: Name, error_type: Name },
}

impl From<BadName> for BindingError {
    fn from(err: BadName) -> Self {
        BindingError::BadName { err }
    }
}
