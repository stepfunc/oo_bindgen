use crate::cpp::conversion::CoreType;
use crate::cpp::formatting::pointer;
use oo_bindgen::function::{FunctionReturnType, FunctionReturnValue};

pub(crate) trait CppFunctionReturnType {
    fn get_cpp_function_return_type(&self) -> String;
}

impl CppFunctionReturnType for FunctionReturnType {
    fn get_cpp_function_return_type(&self) -> String {
        match self {
            FunctionReturnType::Void => "void".to_string(),
            FunctionReturnType::Type(t, _) => match t {
                FunctionReturnValue::Basic(x) => x.core_type(),
                FunctionReturnValue::String(x) => x.core_type(),
                FunctionReturnValue::ClassRef(x) => x.core_type(),
                FunctionReturnValue::Struct(x) => x.core_type(),
                FunctionReturnValue::StructRef(x) => pointer(x.inner.core_type()),
            },
        }
    }
}
