use crate::cpp::conversion::CoreType;
use crate::cpp::formatting::friend_class;
use heck::SnakeCase;
use oo_bindgen::class::ClassDeclarationHandle;
use oo_bindgen::enum_type::EnumHandle;
use oo_bindgen::iterator::IteratorHandle;
use oo_bindgen::structs::{
    CallbackArgStructField, FunctionReturnStructField, Struct, StructFieldType,
    UniversalStructField,
};
use oo_bindgen::types::{BasicType, DurationType};
use oo_bindgen::{Handle, UniversalOr};

pub(crate) trait ToCppStructField {
    /// takes a native type and converts it to a C++ type
    fn to_cpp_struct_field(&self, expr: String) -> String;
}

impl ToCppStructField for DurationType {
    fn to_cpp_struct_field(&self, expr: String) -> String {
        match self {
            DurationType::Milliseconds => format!("convert::from_milli_sec_u64({})", expr),
            DurationType::Seconds => format!("convert::from_sec_u64({})", expr),
        }
    }
}

impl ToCppStructField for EnumHandle {
    fn to_cpp_struct_field(&self, expr: String) -> String {
        format!("enum_from_native({})", expr)
    }
}

impl ToCppStructField for BasicType {
    fn to_cpp_struct_field(&self, expr: String) -> String {
        match self {
            BasicType::Bool => expr,
            BasicType::U8 => expr,
            BasicType::S8 => expr,
            BasicType::U16 => expr,
            BasicType::S16 => expr,
            BasicType::U32 => expr,
            BasicType::S32 => expr,
            BasicType::U64 => expr,
            BasicType::S64 => expr,
            BasicType::Float32 => expr,
            BasicType::Double64 => expr,
            BasicType::Duration(x) => x.to_cpp_struct_field(expr),
            BasicType::Enum(x) => x.to_cpp_struct_field(expr),
        }
    }
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
        format!("cpp_{}_init({})", self.name.to_snake_case(), expr)
    }
}

impl ToCppStructField for IteratorHandle {
    fn to_cpp_struct_field(&self, expr: String) -> String {
        format!("{}::init({})", friend_class(self.core_type()), expr)
    }
}

impl ToCppStructField for FunctionReturnStructField {
    fn to_cpp_struct_field(&self, expr: String) -> String {
        match self {
            FunctionReturnStructField::Basic(x) => x.to_cpp_struct_field(expr),
            FunctionReturnStructField::ClassRef(x) => x.to_cpp_struct_field(expr),
            FunctionReturnStructField::Iterator(x) => x.to_cpp_struct_field(expr),
            FunctionReturnStructField::Struct(x) => x.to_cpp_struct_field(expr),
        }
    }
}

impl ToCppStructField for CallbackArgStructField {
    fn to_cpp_struct_field(&self, expr: String) -> String {
        match self {
            CallbackArgStructField::Basic(x) => x.to_cpp_struct_field(expr),
            CallbackArgStructField::Iterator(x) => x.to_cpp_struct_field(expr),
            CallbackArgStructField::Struct(x) => x.to_cpp_struct_field(expr),
        }
    }
}

impl ToCppStructField for UniversalStructField {
    fn to_cpp_struct_field(&self, expr: String) -> String {
        match self {
            UniversalStructField::Basic(x) => x.to_cpp_struct_field(expr),
            UniversalStructField::Struct(x) => x.to_cpp_struct_field(expr),
        }
    }
}
