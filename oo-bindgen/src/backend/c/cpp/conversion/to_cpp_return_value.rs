use crate::backend::c::cpp::conversion::ToCpp;
use crate::model::FunctionReturnValue;

pub(crate) trait ToCppReturnValue {
    fn to_cpp_return_value(&self, expr: String) -> String;
    fn transform_in_wrapper(&self) -> bool;
}

impl ToCppReturnValue for FunctionReturnValue {
    fn to_cpp_return_value(&self, expr: String) -> String {
        match self {
            FunctionReturnValue::Basic(x) => x.to_cpp(expr),
            FunctionReturnValue::String(x) => x.to_cpp(expr),
            FunctionReturnValue::ClassRef(_) => {
                format!("::convert::to_cpp({expr})")
            }
            FunctionReturnValue::Struct(_) => {
                format!("::convert::to_cpp({expr})")
            }
            FunctionReturnValue::StructRef(_) => {
                //  we don't transform struct refs in the wrappers
                expr
            }
            FunctionReturnValue::PrimitiveRef(_) => {
                // point to a primitive same in C++
                expr
            }
        }
    }

    fn transform_in_wrapper(&self) -> bool {
        match self {
            FunctionReturnValue::Basic(_) => true,
            FunctionReturnValue::String(_) => true,
            FunctionReturnValue::ClassRef(_) => false,
            FunctionReturnValue::Struct(_) => true,
            FunctionReturnValue::StructRef(_) => false,
            FunctionReturnValue::PrimitiveRef(_) => false,
        }
    }
}
