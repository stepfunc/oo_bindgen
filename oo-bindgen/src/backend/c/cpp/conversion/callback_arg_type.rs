use crate::backend::c::cpp::conversion::*;
use crate::backend::c::cpp::formatting::*;
use crate::model::*;
pub(crate) trait CppCallbackArgType {
    fn get_cpp_callback_arg_type(&self) -> String;
}

impl CppCallbackArgType for CallbackArgument {
    fn get_cpp_callback_arg_type(&self) -> String {
        match self {
            CallbackArgument::Basic(x) => x.core_cpp_type(),
            CallbackArgument::String(_) => "const char*".to_string(),
            CallbackArgument::Iterator(x) => mut_ref(x.core_cpp_type()),
            CallbackArgument::Class(x) => mut_ref(x.core_cpp_type()),
            CallbackArgument::Struct(x) => const_ref(x.core_cpp_type()),
        }
    }
}

pub(crate) trait CppCallbackReturnType {
    fn get_cpp_callback_return_type(&self) -> String;
}

impl<D> CppCallbackReturnType for OptionalReturnType<CallbackReturnValue, D>
where
    D: DocReference,
{
    fn get_cpp_callback_return_type(&self) -> String {
        match self.get_value() {
            None => "void".to_string(),
            Some(x) => match x {
                CallbackReturnValue::Basic(x) => x.core_cpp_type(),
                CallbackReturnValue::Struct(x) => x.core_cpp_type(),
            },
        }
    }
}
