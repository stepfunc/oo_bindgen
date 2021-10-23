use crate::cpp::conversion::CoreType;
use crate::cpp::formatting::*;
use oo_bindgen::function::FunctionArgument;
use oo_bindgen::interface::InterfaceType;
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
            FunctionArgument::String(x) => const_ref(x.core_type()),
            FunctionArgument::Collection(x) => const_ref(x.core_type()),
            FunctionArgument::Struct(x) => const_ref(x.core_type()),
            FunctionArgument::StructRef(x) => const_ref(x.core_type()),
            FunctionArgument::ClassRef(x) => mut_ref(x.core_type()),
            FunctionArgument::Interface(x) => match x.interface_type {
                InterfaceType::Synchronous => mut_ref(x.core_type()),
                InterfaceType::Asynchronous => unique_ptr(x.core_type()),
            },
        }
    }
}

impl CppFunctionArgType for FunctionArgStructField {
    fn get_cpp_function_arg_type(&self) -> String {
        match self {
            Self::Basic(x) => x.core_type(),
            Self::String(x) => const_ref(x.core_type()),
            Self::Interface(x) => unique_ptr(x.core_type()),
            Self::Collection(x) => x.core_type(),
            Self::Struct(x) => const_ref(x.core_type()),
        }
    }
}

impl CppFunctionArgType for FunctionReturnStructField {
    fn get_cpp_function_arg_type(&self) -> String {
        match self {
            Self::Basic(x) => x.core_type(),
            Self::ClassRef(x) => x.core_type(),
            Self::Iterator(x) => x.core_type(),
            Self::Struct(x) => const_ref(x.core_type()),
        }
    }
}

impl CppFunctionArgType for CallbackArgStructField {
    fn get_cpp_function_arg_type(&self) -> String {
        match self {
            Self::Basic(x) => x.core_type(),
            Self::Iterator(x) => x.core_type(),
            Self::Struct(x) => const_ref(x.core_type()),
        }
    }
}

impl CppFunctionArgType for UniversalStructField {
    fn get_cpp_function_arg_type(&self) -> String {
        match self {
            Self::Basic(x) => x.core_type(),
            Self::Struct(x) => const_ref(x.core_type()),
        }
    }
}
