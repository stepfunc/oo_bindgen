use crate::cpp::conversion::CoreType;
use crate::cpp::{by_const_ref, by_ref, by_unique_ptr};
use oo_bindgen::function::FunctionArgument;
use oo_bindgen::structs::{
    CallbackArgStructField, FunctionArgStructField, FunctionReturnStructField, UniversalStructField,
};

pub(crate) trait CppFunctionArgType {
    fn get_cpp_function_arg_type(&self) -> String;
}

impl CppFunctionArgType for FunctionArgument {
    fn get_cpp_function_arg_type(&self) -> String {
        match self {
            FunctionArgument::Basic(x) => x.core_type(),
            FunctionArgument::String(x) => by_const_ref(x.core_type()),
            FunctionArgument::Collection(x) => by_const_ref(x.core_type()),
            FunctionArgument::Struct(x) => by_const_ref(x.core_type()),
            FunctionArgument::StructRef(_) => unimplemented!(),
            FunctionArgument::ClassRef(x) => by_ref(x.core_type()),
            FunctionArgument::Interface(x) => by_unique_ptr(x.core_type()),
        }
    }
}

impl CppFunctionArgType for FunctionArgStructField {
    fn get_cpp_function_arg_type(&self) -> String {
        match self {
            Self::Basic(x) => x.core_type(),
            Self::String(x) => by_const_ref(x.core_type()),
            Self::Interface(x) => by_unique_ptr(x.core_type()),
            Self::Collection(x) => x.core_type(),
            Self::Struct(x) => by_const_ref(x.core_type()),
        }
    }
}

impl CppFunctionArgType for FunctionReturnStructField {
    fn get_cpp_function_arg_type(&self) -> String {
        match self {
            Self::Basic(x) => x.core_type(),
            Self::ClassRef(x) => x.core_type(),
            Self::Iterator(x) => x.core_type(),
            Self::Struct(x) => by_const_ref(x.core_type()),
        }
    }
}

impl CppFunctionArgType for CallbackArgStructField {
    fn get_cpp_function_arg_type(&self) -> String {
        match self {
            Self::Basic(x) => x.core_type(),
            Self::Iterator(x) => x.core_type(),
            Self::Struct(x) => by_const_ref(x.core_type()),
        }
    }
}

impl CppFunctionArgType for UniversalStructField {
    fn get_cpp_function_arg_type(&self) -> String {
        match self {
            Self::Basic(x) => x.core_type(),
            Self::Struct(x) => by_const_ref(x.core_type()),
        }
    }
}
