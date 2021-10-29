use crate::cpp::conversion::ToCpp;
use oo_bindgen::class::ClassDeclarationHandle;
use oo_bindgen::iterator::IteratorHandle;
use oo_bindgen::structs::{
    CallbackArgStructField, FunctionReturnStructField, Struct, StructFieldType,
    UniversalStructField,
};
use oo_bindgen::{Handle, UniversalOr};

pub(crate) trait ToCppStructField {
    /// takes a native type and converts it to a C++ type
    fn to_cpp_struct_field(&self, expr: String) -> String;
}

impl<T> ToCppStructField for Handle<Struct<T>>
where
    T: StructFieldType + ToCppStructField,
{
    fn to_cpp_struct_field(&self, expr: String) -> String {
        format!("to_cpp({})", expr)
    }
}

impl<T> ToCppStructField for UniversalOr<T>
where
    T: StructFieldType + ToCppStructField,
{
    fn to_cpp_struct_field(&self, expr: String) -> String {
        match self {
            UniversalOr::Specific(x) => x.to_cpp_struct_field(expr),
            UniversalOr::Universal(x) => x.to_cpp_struct_field(expr),
        }
    }
}

impl ToCppStructField for ClassDeclarationHandle {
    fn to_cpp_struct_field(&self, expr: String) -> String {
        self.to_cpp(expr)
    }
}

impl ToCppStructField for IteratorHandle {
    fn to_cpp_struct_field(&self, expr: String) -> String {
        format!("::convert::construct({})", expr)
    }
}

impl ToCppStructField for FunctionReturnStructField {
    fn to_cpp_struct_field(&self, expr: String) -> String {
        match self {
            FunctionReturnStructField::Basic(x) => x.to_cpp(expr),
            FunctionReturnStructField::ClassRef(x) => x.to_cpp_struct_field(expr),
            FunctionReturnStructField::Iterator(x) => x.to_cpp_struct_field(expr),
            FunctionReturnStructField::Struct(x) => x.to_cpp_struct_field(expr),
        }
    }
}

impl ToCppStructField for CallbackArgStructField {
    fn to_cpp_struct_field(&self, expr: String) -> String {
        match self {
            CallbackArgStructField::Basic(x) => x.to_cpp(expr),
            CallbackArgStructField::Iterator(x) => x.to_cpp_struct_field(expr),
            CallbackArgStructField::Struct(x) => x.to_cpp_struct_field(expr),
        }
    }
}

impl ToCppStructField for UniversalStructField {
    fn to_cpp_struct_field(&self, expr: String) -> String {
        match self {
            UniversalStructField::Basic(x) => x.to_cpp(expr),
            UniversalStructField::Struct(x) => x.to_cpp_struct_field(expr),
        }
    }
}
