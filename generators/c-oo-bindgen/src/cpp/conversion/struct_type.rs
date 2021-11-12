use crate::cpp::conversion::CoreCppType;
use crate::cpp::formatting::*;
use oo_bindgen::structs::{
    CallbackArgStructField, FunctionArgStructField, FunctionReturnStructField, UniversalStructField,
};
use oo_bindgen::types::{BasicType, StringType};

pub(crate) trait CppStructType {
    fn struct_member_type(&self) -> String;
}

impl CppStructType for BasicType {
    fn struct_member_type(&self) -> String {
        self.core_cpp_type()
    }
}

impl CppStructType for StringType {
    fn struct_member_type(&self) -> String {
        self.core_cpp_type()
    }
}

impl CppStructType for FunctionArgStructField {
    fn struct_member_type(&self) -> String {
        match self {
            FunctionArgStructField::Basic(x) => x.struct_member_type(),
            FunctionArgStructField::String(x) => x.struct_member_type(),
            FunctionArgStructField::Interface(x) => unique_ptr(x.inner.core_cpp_type()),
            FunctionArgStructField::Struct(x) => x.core_cpp_type(),
        }
    }
}

impl CppStructType for FunctionReturnStructField {
    fn struct_member_type(&self) -> String {
        match self {
            FunctionReturnStructField::Basic(x) => x.struct_member_type(),
            FunctionReturnStructField::ClassRef(x) => x.core_cpp_type(),
            FunctionReturnStructField::Iterator(x) => x.core_cpp_type(),
            FunctionReturnStructField::Struct(x) => x.core_cpp_type(),
        }
    }
}

impl CppStructType for CallbackArgStructField {
    fn struct_member_type(&self) -> String {
        match self {
            CallbackArgStructField::Basic(x) => x.struct_member_type(),
            CallbackArgStructField::Iterator(x) => mut_ref(x.core_cpp_type()),
            CallbackArgStructField::Struct(x) => x.core_cpp_type(),
        }
    }
}

impl CppStructType for UniversalStructField {
    fn struct_member_type(&self) -> String {
        match self {
            UniversalStructField::Basic(x) => x.struct_member_type(),
            UniversalStructField::Struct(x) => x.core_cpp_type(),
        }
    }
}
