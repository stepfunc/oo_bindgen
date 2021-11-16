use oo_bindgen::model::*;

use crate::cpp::conversion::CoreCppType;
use crate::cpp::formatting::pointer;

pub(crate) trait CppFunctionReturnType {
    fn get_cpp_function_return_type(&self) -> String;
}

impl CppFunctionReturnType for OptionalReturnType<FunctionReturnValue, Validated> {
    fn get_cpp_function_return_type(&self) -> String {
        match self.get_value() {
            None => "void".to_string(),
            Some(t) => match t {
                FunctionReturnValue::Basic(x) => x.core_cpp_type(),
                FunctionReturnValue::String(x) => x.core_cpp_type(),
                FunctionReturnValue::ClassRef(x) => x.core_cpp_type(),
                FunctionReturnValue::Struct(x) => x.core_cpp_type(),
                FunctionReturnValue::StructRef(x) => pointer(x.untyped().core_cpp_type()),
            },
        }
    }
}
