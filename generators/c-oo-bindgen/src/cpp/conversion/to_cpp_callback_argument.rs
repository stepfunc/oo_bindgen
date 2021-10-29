use crate::cpp::conversion::{ToCpp, ToCppStructField};
use oo_bindgen::interface::CallbackArgument;
use oo_bindgen::types::{BasicType, StringType};

pub(crate) trait ToNativeCallbackArgument {
    fn to_cpp_callback_argument(&self, expr: String) -> String;
    fn requires_shadow_parameter(&self) -> bool;
}

impl ToNativeCallbackArgument for BasicType {
    fn to_cpp_callback_argument(&self, expr: String) -> String {
        self.to_cpp(expr)
    }

    fn requires_shadow_parameter(&self) -> bool {
        false
    }
}

impl ToNativeCallbackArgument for StringType {
    fn to_cpp_callback_argument(&self, expr: String) -> String {
        self.to_cpp(expr)
    }

    fn requires_shadow_parameter(&self) -> bool {
        false
    }
}

impl ToNativeCallbackArgument for CallbackArgument {
    fn to_cpp_callback_argument(&self, expr: String) -> String {
        match self {
            CallbackArgument::Basic(x) => x.to_cpp_callback_argument(expr),
            CallbackArgument::String(x) => x.to_cpp_callback_argument(expr),
            CallbackArgument::Iterator(x) => x.to_cpp_struct_field(expr),
            CallbackArgument::Class(x) => x.to_cpp_struct_field(expr),
            CallbackArgument::Struct(x) => x.to_cpp_struct_field(expr),
        }
    }

    fn requires_shadow_parameter(&self) -> bool {
        match self {
            CallbackArgument::Basic(_) => false,
            CallbackArgument::String(_) => false,
            CallbackArgument::Iterator(_) => true,
            CallbackArgument::Class(_) => true,
            CallbackArgument::Struct(_) => false,
        }
    }
}
