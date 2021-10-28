use crate::cpp::conversion::CoreCppType;
use crate::cpp::formatting::*;
use oo_bindgen::function::FunctionArgument;
use oo_bindgen::interface::InterfaceType;
use oo_bindgen::structs::{
    CallbackArgStructField, FunctionArgStructField, FunctionReturnStructField, UniversalStructField,
};
use oo_bindgen::UniversalOr;

pub(crate) trait CppFunctionArgType {
    fn get_cpp_function_arg_type(&self) -> String;
}

pub(crate) trait IsConstructByMove {
    fn is_construct_by_move(&self) -> bool;
}

impl IsConstructByMove for FunctionArgStructField {
    fn is_construct_by_move(&self) -> bool {
        match self {
            FunctionArgStructField::Basic(_) => false,
            FunctionArgStructField::String(_) => false,
            FunctionArgStructField::Interface(_) => true,
            FunctionArgStructField::Collection(_) => todo!(),
            FunctionArgStructField::Struct(x) => match x {
                UniversalOr::Specific(x) => {
                    x.fields.iter().any(|f| f.field_type.is_construct_by_move())
                }
                UniversalOr::Universal(x) => {
                    x.fields.iter().any(|f| f.field_type.is_construct_by_move())
                }
            },
        }
    }
}

impl IsConstructByMove for UniversalStructField {
    fn is_construct_by_move(&self) -> bool {
        match self {
            UniversalStructField::Basic(_) => false,
            UniversalStructField::Struct(x) => {
                x.fields.iter().any(|f| f.field_type.is_construct_by_move())
            }
        }
    }
}

impl IsConstructByMove for FunctionReturnStructField {
    fn is_construct_by_move(&self) -> bool {
        match self {
            FunctionReturnStructField::Basic(_) => false,
            FunctionReturnStructField::ClassRef(_) => false,
            FunctionReturnStructField::Iterator(_) => true,
            FunctionReturnStructField::Struct(_) => false,
        }
    }
}

impl IsConstructByMove for CallbackArgStructField {
    fn is_construct_by_move(&self) -> bool {
        match self {
            CallbackArgStructField::Basic(_) => false,
            CallbackArgStructField::Iterator(_) => true,
            CallbackArgStructField::Struct(_) => false,
        }
    }
}

impl CppFunctionArgType for FunctionArgument {
    fn get_cpp_function_arg_type(&self) -> String {
        match self {
            FunctionArgument::Basic(x) => x.core_cpp_type(),
            FunctionArgument::String(x) => const_ref(x.core_cpp_type()),
            FunctionArgument::Collection(x) => const_ref(x.core_cpp_type()),
            FunctionArgument::Struct(x) => const_ref(x.core_cpp_type()),
            FunctionArgument::StructRef(x) => const_ref(x.inner.core_cpp_type()),
            FunctionArgument::ClassRef(x) => mut_ref(x.core_cpp_type()),
            FunctionArgument::Interface(x) => match x.interface_type {
                InterfaceType::Synchronous => mut_ref(x.core_cpp_type()),
                InterfaceType::Asynchronous => unique_ptr(x.core_cpp_type()),
            },
        }
    }
}

impl CppFunctionArgType for FunctionArgStructField {
    fn get_cpp_function_arg_type(&self) -> String {
        match self {
            Self::Basic(x) => x.core_cpp_type(),
            Self::String(x) => const_ref(x.core_cpp_type()),
            Self::Interface(x) => unique_ptr(x.core_cpp_type()),
            Self::Collection(x) => x.core_cpp_type(),
            Self::Struct(x) => const_ref(x.core_cpp_type()),
        }
    }
}

impl CppFunctionArgType for FunctionReturnStructField {
    fn get_cpp_function_arg_type(&self) -> String {
        match self {
            Self::Basic(x) => x.core_cpp_type(),
            Self::ClassRef(x) => x.core_cpp_type(),
            Self::Iterator(x) => x.core_cpp_type(),
            Self::Struct(x) => const_ref(x.core_cpp_type()),
        }
    }
}

impl CppFunctionArgType for CallbackArgStructField {
    fn get_cpp_function_arg_type(&self) -> String {
        match self {
            Self::Basic(x) => x.core_cpp_type(),
            Self::Iterator(x) => x.core_cpp_type(),
            Self::Struct(x) => const_ref(x.core_cpp_type()),
        }
    }
}

impl CppFunctionArgType for UniversalStructField {
    fn get_cpp_function_arg_type(&self) -> String {
        match self {
            Self::Basic(x) => x.core_cpp_type(),
            Self::Struct(x) => const_ref(x.core_cpp_type()),
        }
    }
}
