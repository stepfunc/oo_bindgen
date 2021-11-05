use crate::cpp::conversion::CoreCppType;
use crate::cpp::formatting::pointer;
use oo_bindgen::doc::Validated;
use oo_bindgen::function::{FunctionReturnType, FunctionReturnValue};

pub(crate) trait CppFunctionReturnType {
    fn get_cpp_function_return_type(&self) -> String;
}

impl CppFunctionReturnType for FunctionReturnType<Validated> {
    fn get_cpp_function_return_type(&self) -> String {
        match self {
            FunctionReturnType::Void => "void".to_string(),
            FunctionReturnType::Type(t, _) => match t {
                FunctionReturnValue::Basic(x) => x.core_cpp_type(),
                FunctionReturnValue::String(x) => x.core_cpp_type(),
                FunctionReturnValue::ClassRef(x) => x.core_cpp_type(),
                FunctionReturnValue::Struct(x) => x.core_cpp_type(),
                FunctionReturnValue::StructRef(x) => pointer(x.untyped().core_cpp_type()),
            },
        }
    }
}
