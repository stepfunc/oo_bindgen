use crate::cpp::by_const_ref;
use crate::cpp::core_type::CoreType;
use oo_bindgen::structs::{
    CallbackArgStructField, FunctionArgStructField, FunctionReturnStructField, UniversalStructField,
};

pub(crate) trait CppConstructorArgType {
    fn get_cpp_constructor_arg_type(&self) -> String;
}

impl CppConstructorArgType for FunctionArgStructField {
    fn get_cpp_constructor_arg_type(&self) -> String {
        match self {
            FunctionArgStructField::Basic(x) => x.core_type(),
            FunctionArgStructField::String(x) => by_const_ref(x.core_type()),
            FunctionArgStructField::Interface(x) => x.core_type(),
            FunctionArgStructField::Collection(x) => x.core_type(),
            FunctionArgStructField::Struct(x) => by_const_ref(x.core_type()),
        }
    }
}

impl CppConstructorArgType for FunctionReturnStructField {
    fn get_cpp_constructor_arg_type(&self) -> String {
        match self {
            FunctionReturnStructField::Basic(x) => x.core_type(),
            FunctionReturnStructField::ClassRef(x) => x.core_type(),
            FunctionReturnStructField::Iterator(x) => x.core_type(),
            FunctionReturnStructField::Struct(x) => by_const_ref(x.core_type()),
        }
    }
}

impl CppConstructorArgType for CallbackArgStructField {
    fn get_cpp_constructor_arg_type(&self) -> String {
        match self {
            CallbackArgStructField::Basic(x) => x.core_type(),
            CallbackArgStructField::Iterator(x) => x.core_type(),
            CallbackArgStructField::Struct(x) => by_const_ref(x.core_type()),
        }
    }
}

impl CppConstructorArgType for UniversalStructField {
    fn get_cpp_constructor_arg_type(&self) -> String {
        match self {
            UniversalStructField::Basic(x) => x.core_type(),
            UniversalStructField::Struct(x) => by_const_ref(x.core_type()),
        }
    }
}
