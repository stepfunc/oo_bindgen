use crate::cpp::conversion::CoreCppType;
use crate::cpp::formatting::*;
use oo_bindgen::doc::DocReference;
use oo_bindgen::interface::{CallbackArgument, CallbackReturnType, CallbackReturnValue};

pub(crate) trait CppCallbackArgType {
    fn get_cpp_callback_arg_type(&self) -> String;
}

impl CppCallbackArgType for CallbackArgument {
    fn get_cpp_callback_arg_type(&self) -> String {
        match self {
            CallbackArgument::Basic(x) => x.core_cpp_type(),
            CallbackArgument::String(x) => x.core_cpp_type(),
            CallbackArgument::Iterator(x) => mut_ref(x.core_cpp_type()),
            CallbackArgument::Class(x) => mut_ref(x.core_cpp_type()),
            CallbackArgument::Struct(x) => const_ref(x.core_cpp_type()),
        }
    }
}

pub(crate) trait CppCallbackReturnType {
    fn get_cpp_callback_return_type(&self) -> String;
}

impl<D> CppCallbackReturnType for CallbackReturnType<D>
where
    D: DocReference,
{
    fn get_cpp_callback_return_type(&self) -> String {
        match self {
            CallbackReturnType::Void => "void".to_string(),
            CallbackReturnType::Type(x, _) => match x {
                CallbackReturnValue::Basic(x) => x.core_cpp_type(),
                CallbackReturnValue::Struct(x) => x.core_cpp_type(),
            },
        }
    }
}
