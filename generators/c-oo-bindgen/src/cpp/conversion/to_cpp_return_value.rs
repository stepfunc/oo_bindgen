use crate::cpp::conversion::ToCpp;
use oo_bindgen::function::FunctionReturnValue;

pub(crate) trait ToCppReturnValue {
    fn to_cpp_return_value(&self, expr: String) -> String;
}

impl ToCppReturnValue for FunctionReturnValue {
    fn to_cpp_return_value(&self, expr: String) -> String {
        match self {
            FunctionReturnValue::Basic(x) => x.to_cpp(expr),
            FunctionReturnValue::String(x) => x.to_cpp(expr),
            FunctionReturnValue::ClassRef(x) => x.to_cpp(expr),
            FunctionReturnValue::Struct(_x) => {
                unimplemented!()
            }
            FunctionReturnValue::StructRef(_) => {
                unimplemented!()
            }
        }
    }
}
