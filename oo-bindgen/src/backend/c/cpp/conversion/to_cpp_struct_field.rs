use crate::backend::c::cpp::conversion::ToCpp;
use crate::model::*;

pub(crate) trait ToCppStructField {
    /// takes a native type and converts it to a C++ type
    fn to_cpp_struct_field(&self, expr: String) -> String;
}

impl<T, D> ToCppStructField for Handle<Struct<T, D>>
where
    D: DocReference,
    T: StructFieldType + ToCppStructField,
{
    fn to_cpp_struct_field(&self, expr: String) -> String {
        format!("to_cpp({expr})")
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

impl<D> ToCppStructField for Handle<AbstractIterator<D>>
where
    D: DocReference,
{
    fn to_cpp_struct_field(&self, expr: String) -> String {
        format!("::convert::construct({expr})")
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
            CallbackArgStructField::String(x) => x.to_cpp(expr),
        }
    }
}

impl ToCppStructField for UniversalStructField {
    fn to_cpp_struct_field(&self, expr: String) -> String {
        match self {
            UniversalStructField::Basic(x) => x.to_cpp(expr),
            UniversalStructField::Struct(x) => x.to_cpp_struct_field(expr),
            UniversalStructField::String(x) => x.to_cpp(expr),
        }
    }
}
