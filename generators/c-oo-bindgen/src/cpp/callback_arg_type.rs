use crate::cpp::core_type::CoreType;
use oo_bindgen::interface::{CallbackArgument, CallbackReturnType, CallbackReturnValue};

pub(crate) trait CppCallbackArgType {
    fn get_cpp_callback_arg_type(&self) -> String;
}

fn by_ref(expr: String) -> String {
    format!("{}&", expr)
}

fn by_const_ref(expr: String) -> String {
    format!("const {}&", expr)
}

impl CppCallbackArgType for CallbackArgument {
    fn get_cpp_callback_arg_type(&self) -> String {
        match self {
            CallbackArgument::Basic(x) => x.core_type(),
            CallbackArgument::String(x) => x.core_type(),
            CallbackArgument::Iterator(x) => by_ref(x.core_type()),
            CallbackArgument::Class(x) => by_ref(x.core_type()),
            CallbackArgument::Struct(x) => by_const_ref(x.core_type()),
        }
    }
}

pub(crate) trait CppCallbackReturnType {
    fn get_cpp_callback_return_type(&self) -> String;
}

impl CppCallbackReturnType for CallbackReturnType {
    fn get_cpp_callback_return_type(&self) -> String {
        match self {
            CallbackReturnType::Void => "void".to_string(),
            CallbackReturnType::Type(x, _) => match x {
                CallbackReturnValue::Basic(x) => x.core_type(),
                CallbackReturnValue::Struct(x) => x.core_type(),
            },
        }
    }
}
