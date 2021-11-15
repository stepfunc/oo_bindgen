use crate::cpp::conversion::{CoreCppType, PassBy, TypeInfo};
use crate::cpp::formatting::*;
use oo_bindgen::doc::{DocReference, Unvalidated};
use oo_bindgen::function::FunctionArgument;
use oo_bindgen::interface::{Interface, InterfaceMode};
use oo_bindgen::structs::{
    CallbackArgStructField, FunctionArgStructField, FunctionReturnStructField, Struct,
    StructFieldType, UniversalOr, UniversalStructField,
};
use oo_bindgen::types::{BasicType, StringType};
use oo_bindgen::Handle;

pub(crate) trait CppFunctionArgType {
    fn get_cpp_function_arg_type(&self) -> String;
}

impl<T, D> CppFunctionArgType for Handle<Struct<T, D>>
where
    D: DocReference,
    T: StructFieldType + TypeInfo,
{
    fn get_cpp_function_arg_type(&self) -> String {
        match self.pass_by() {
            PassBy::Copy => self.core_cpp_type(),
            PassBy::ConstRef => const_ref(self.core_cpp_type()),
            PassBy::Move => self.core_cpp_type(),
            PassBy::MutRef => mut_ref(self.core_cpp_type()),
        }
    }
}

impl CppFunctionArgType for StringType {
    fn get_cpp_function_arg_type(&self) -> String {
        const_ref(self.core_cpp_type())
    }
}

impl CppFunctionArgType for BasicType {
    fn get_cpp_function_arg_type(&self) -> String {
        self.core_cpp_type()
    }
}

impl CppFunctionArgType for Handle<Interface<Unvalidated>> {
    fn get_cpp_function_arg_type(&self) -> String {
        match self.mode {
            InterfaceMode::Synchronous => mut_ref(self.core_cpp_type()),
            InterfaceMode::Asynchronous => unique_ptr(self.core_cpp_type()),
            InterfaceMode::Future => unique_ptr(self.core_cpp_type()),
        }
    }
}

impl<T> CppFunctionArgType for UniversalOr<T>
where
    T: StructFieldType + TypeInfo,
{
    fn get_cpp_function_arg_type(&self) -> String {
        match self {
            UniversalOr::Specific(x) => x.get_cpp_function_arg_type(),
            UniversalOr::Universal(x) => x.get_cpp_function_arg_type(),
        }
    }
}

impl CppFunctionArgType for FunctionArgument {
    fn get_cpp_function_arg_type(&self) -> String {
        match self {
            FunctionArgument::Basic(x) => x.get_cpp_function_arg_type(),
            FunctionArgument::String(x) => x.get_cpp_function_arg_type(),
            FunctionArgument::Collection(x) => const_ref(x.core_cpp_type()),
            FunctionArgument::Struct(x) => x.get_cpp_function_arg_type(),
            FunctionArgument::StructRef(x) => const_ref(x.inner.core_cpp_type()),
            FunctionArgument::ClassRef(x) => mut_ref(x.core_cpp_type()),
            FunctionArgument::Interface(x) => x.get_cpp_function_arg_type(),
        }
    }
}

impl CppFunctionArgType for FunctionArgStructField {
    fn get_cpp_function_arg_type(&self) -> String {
        match self {
            Self::Basic(x) => x.get_cpp_function_arg_type(),
            Self::String(x) => x.get_cpp_function_arg_type(),
            Self::Interface(x) => x.inner.get_cpp_function_arg_type(),
            Self::Struct(x) => x.get_cpp_function_arg_type(),
        }
    }
}

impl CppFunctionArgType for FunctionReturnStructField {
    fn get_cpp_function_arg_type(&self) -> String {
        match self {
            Self::Basic(x) => x.get_cpp_function_arg_type(),
            Self::ClassRef(x) => x.core_cpp_type(),
            Self::Iterator(x) => x.core_cpp_type(),
            Self::Struct(x) => const_ref(x.core_cpp_type()),
        }
    }
}

impl CppFunctionArgType for CallbackArgStructField {
    fn get_cpp_function_arg_type(&self) -> String {
        match self {
            Self::Basic(x) => x.get_cpp_function_arg_type(),
            Self::Iterator(x) => x.core_cpp_type(),
            Self::Struct(x) => const_ref(x.core_cpp_type()),
        }
    }
}

impl CppFunctionArgType for UniversalStructField {
    fn get_cpp_function_arg_type(&self) -> String {
        match self {
            Self::Basic(x) => x.get_cpp_function_arg_type(),
            Self::Struct(x) => x.get_cpp_function_arg_type(),
        }
    }
}
