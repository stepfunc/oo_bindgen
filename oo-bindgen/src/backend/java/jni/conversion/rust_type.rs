use crate::model::*;

/// Used to generate the interface conversions routines
pub(crate) trait RustType {
    /// get the Rust FFI type
    fn get_rust_type(&self, ffi_name: &str) -> String;
}

impl RustType for Primitive {
    fn get_rust_type(&self, _ffi_name: &str) -> String {
        self.get_c_rust_type().to_string()
    }
}

impl RustType for DurationType {
    fn get_rust_type(&self, _ffi_name: &str) -> String {
        match self {
            DurationType::Milliseconds | DurationType::Seconds => "u64".to_string(),
        }
    }
}

impl RustType for EnumHandle {
    fn get_rust_type(&self, _ffi_name: &str) -> String {
        "std::os::raw::c_int".to_string()
    }
}

impl RustType for BasicType {
    fn get_rust_type(&self, ffi_name: &str) -> String {
        match self {
            BasicType::Primitive(x) => x.get_rust_type(ffi_name),
            BasicType::Duration(x) => x.get_rust_type(ffi_name),
            BasicType::Enum(x) => x.get_rust_type(ffi_name),
        }
    }
}

impl RustType for StringType {
    fn get_rust_type(&self, _ffi_name: &str) -> String {
        "*const std::os::raw::c_char".to_string()
    }
}

impl RustType for AbstractIteratorHandle {
    fn get_rust_type(&self, ffi_name: &str) -> String {
        format!("*mut {}::{}", ffi_name, self.name().camel_case())
    }
}

impl RustType for ClassDeclarationHandle {
    fn get_rust_type(&self, ffi_name: &str) -> String {
        format!("*mut {}::{}", ffi_name, self.name.camel_case())
    }
}

impl RustType for UniversalOr<CallbackArgStructField> {
    fn get_rust_type(&self, ffi_name: &str) -> String {
        format!("{}::ffi::{}", ffi_name, self.name().camel_case())
    }
}

impl RustType for UniversalStructHandle {
    fn get_rust_type(&self, ffi_name: &str) -> String {
        format!("{}::ffi::{}", ffi_name, self.name().camel_case())
    }
}

impl RustType for CallbackArgument {
    fn get_rust_type(&self, ffi_name: &str) -> String {
        match self {
            CallbackArgument::Basic(x) => x.get_rust_type(ffi_name),
            CallbackArgument::String(x) => x.get_rust_type(ffi_name),
            CallbackArgument::Iterator(x) => x.get_rust_type(ffi_name),
            CallbackArgument::Class(x) => x.get_rust_type(ffi_name),
            CallbackArgument::Struct(x) => x.get_rust_type(ffi_name),
        }
    }
}

impl RustType for CallbackReturnValue {
    fn get_rust_type(&self, ffi_name: &str) -> String {
        match self {
            CallbackReturnValue::Basic(x) => x.get_rust_type(ffi_name),
            CallbackReturnValue::Struct(x) => x.get_rust_type(ffi_name),
        }
    }
}

impl RustType for OptionalReturnType<CallbackReturnValue, Validated> {
    fn get_rust_type(&self, ffi_name: &str) -> String {
        match self.get_value() {
            None => "()".to_string(),
            Some(x) => x.get_rust_type(ffi_name),
        }
    }
}
