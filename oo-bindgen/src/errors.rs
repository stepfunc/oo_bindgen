use thiserror::Error;

use crate::class::ClassDeclarationHandle;
use crate::name::{BadName, Name};
use crate::structs::{InitializerDefault, StructDeclarationHandle};
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
    // ---------------- global errors -----------------------------------
    #[error("Symbol '{}' already used in the library", name)]
    SymbolAlreadyUsed { name: Name },
    #[error("Item '{}' is not part of this library", name)]
    NotPartOfThisLibrary { name: Name },
    // ---------------- name errors -----------------------------------
    #[error("'{}'", err)]
    BadName { err: BadName },
    // ---------------- documentation errors --------------------------
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
    // Documentation error
    #[error("Invalid documentation string")]
    InvalidDocString,
    // ---------------- class definition errors -----------------------
    #[error("Class '{}' was already defined", handle.name)]
    ClassAlreadyDefined { handle: ClassDeclarationHandle },
    #[error("Constructor for class '{}' was already defined", handle.name)]
    ConstructorAlreadyDefined { handle: ClassDeclarationHandle },
    #[error("Destructor for class '{}' was already defined", handle.name)]
    DestructorAlreadyDefined { handle: ClassDeclarationHandle },
    #[error("Member '{}' is associated with class '{}' but was added to '{}'", name, declared.name, added_to.name)]
    ClassMemberWrongAssociatedClass {
        name: Name,
        declared: ClassDeclarationHandle,
        added_to: ClassDeclarationHandle,
    },
    #[error("Method name '{}' contains the name of the owning class '{}'", class.name, method_name)]
    BadMethodName {
        class: ClassDeclarationHandle,
        method_name: Name,
    },
    #[error("No destructor defined for class '{}', but asking for manual/disposable destruction", handle.name)]
    NoDestructorForManualDestruction { handle: ClassDeclarationHandle },
    // ----------------- constant definition errors -------------------
    #[error(
        "ConstantSet '{}' already contains constant name  '{}'",
        set_name,
        constant_name
    )]
    ConstantNameAlreadyUsed { set_name: Name, constant_name: Name },
    // ----------------- enum errors -------------------
    #[error("Enum '{}' does not contain a variant named '{}'", name, variant_name)]
    UnknownEnumVariant { name: Name, variant_name: String },
    #[error(
        "Enum '{}' already contains a variant with name '{}'",
        name,
        variant_name
    )]
    DuplicateEnumVariantName { name: Name, variant_name: String },
    #[error(
        "Enum '{}' already contains a variant with value '{}'",
        name,
        variant_value
    )]
    DuplicateEnumVariantValue { name: Name, variant_value: i32 },
    // ----------------- function errors -------------------
    #[error("Return type of native function '{}' was already defined", func_name)]
    ReturnTypeAlreadyDefined { func_name: Name },
    #[error(
        "Function '{}' already has an error type specified: '{}'",
        function,
        error_type
    )]
    ErrorTypeAlreadyDefined { function: Name, error_type: Name },
    // ----------------- interface errors -------------------
    #[error(
        "Symbol '{}' is reserved and cannot be used as an interface method name",
        name
    )]
    InterfaceMethodWithReservedName { name: Name },
    #[error(
        "Interface '{}' already has callback with the name '{}'",
        interface_name,
        callback_name
    )]
    InterfaceDuplicateCallbackName {
        interface_name: Name,
        callback_name: Name,
    },
    #[error(
        "Symbol '{}' is reserved and cannot be used as a callback argument name",
        name
    )]
    CallbackMethodArgumentWithReservedName { name: Name },

    // ----------------- struct errors -------------------
    #[error("Native struct '{}' was already defined", handle.name)]
    StructAlreadyDefined { handle: StructDeclarationHandle },
    #[error(
        "Initializer field type '{}' doesn't match value '{:?}",
        field_type,
        value
    )]
    StructInitializerBadValueForType {
        field_type: String,
        value: InitializerDefault,
    },
    #[error("Initializer contains a default struct field but struct '{}' doesn't have a default initializer", struct_name)]
    StructInitializerStructFieldWithoutDefaultInitializer { struct_name: String },
    #[error("Native struct '{}' already contains field with name '{}'", handle.name, field_name)]
    StructFieldDuplicateName {
        handle: StructDeclarationHandle,
        field_name: Name,
    },
    #[error(
        "Struct '{}' already contains an initializer with the name '{}'",
        struct_name,
        initializer_name
    )]
    StructInitializerDuplicateName {
        struct_name: Name,
        initializer_name: Name,
    },
    #[error(
        "Initializer field '{}' doesn't exist within struct '{}",
        field_name,
        struct_name
    )]
    StructInitializerUnknownField { struct_name: Name, field_name: Name },
    #[error(
        "Duplicate initializer field default '{}' in struct '{}",
        field_name,
        struct_name
    )]
    StructInitializerDuplicateField { struct_name: Name, field_name: Name },
    #[error(
        "Struct ({}) initializer {} uses the same arguments as initializer {}",
        struct_name,
        this_initializer,
        other_initializer
    )]
    StructDuplicateInitializerArgs {
        struct_name: Name,
        this_initializer: Name,
        other_initializer: Name,
    },
}

impl From<BadName> for BindingError {
    fn from(err: BadName) -> Self {
        BindingError::BadName { err }
    }
}
