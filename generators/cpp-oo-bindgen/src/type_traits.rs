use oo_bindgen::native_function::{ReturnType, Type};

use crate::name_traits::CppName;

pub(crate) enum PassingConvention {
    Copy,
    ConstRef,
    MutableRef,
}

pub(crate) trait CppType {
    fn get_cpp_type(&self) -> String;
}

pub(crate) trait CppStructType {
    fn get_cpp_struct_type(&self) -> String;
}

pub(crate) trait CppToNativeArgument: CppType {
    fn get_passing_convention(&self) -> PassingConvention;

    fn get_cpp_to_native_argument(&self) -> String {
        match self.get_passing_convention() {
            PassingConvention::Copy => self.get_cpp_type(),
            PassingConvention::ConstRef => format!("const {}&", self.get_cpp_type()),
            PassingConvention::MutableRef => format!("{}&", self.get_cpp_type()),
        }
    }
}

impl CppType for Type {
    fn get_cpp_type(&self) -> String {
        match self {
            Type::Bool => "bool".to_owned(),
            Type::Uint8 => "uint8_t".to_owned(),
            Type::Sint8 => "int8_t".to_owned(),
            Type::Uint16 => "uint16_t".to_owned(),
            Type::Sint16 => "int16_t".to_owned(),
            Type::Uint32 => "uint32_t".to_owned(),
            Type::Sint32 => "int32_t".to_owned(),
            Type::Uint64 => "uint64_t".to_owned(),
            Type::Sint64 => "int64_t".to_owned(),
            Type::Float => "float".to_owned(),
            Type::Double => "double".to_owned(),
            Type::String => "std::string".to_owned(),
            Type::Struct(x) => x.cpp_name(),
            Type::StructRef(x) => x.cpp_name(),
            Type::Enum(x) => x.cpp_name(),
            Type::ClassRef(x) => x.cpp_name(),
            Type::Interface(x) => x.cpp_name(),
            Type::Iterator(x) => x.cpp_name(),
            Type::Collection(_) => unimplemented!(),
            Type::Duration(_) => "std::chrono::steady_clock::duration".to_owned(),
        }
    }
}

impl CppToNativeArgument for Type {
    fn get_passing_convention(&self) -> PassingConvention {
        match self {
            Type::Bool => PassingConvention::Copy,
            Type::Uint8 => PassingConvention::Copy,
            Type::Sint8 => PassingConvention::Copy,
            Type::Uint16 => PassingConvention::Copy,
            Type::Sint16 => PassingConvention::Copy,
            Type::Uint32 => PassingConvention::Copy,
            Type::Sint32 => PassingConvention::Copy,
            Type::Uint64 => PassingConvention::Copy,
            Type::Sint64 => PassingConvention::Copy,
            Type::Float => PassingConvention::Copy,
            Type::Double => PassingConvention::Copy,
            Type::String => PassingConvention::ConstRef,
            Type::Struct(_) => PassingConvention::ConstRef,
            Type::StructRef(_) => PassingConvention::ConstRef,
            Type::Enum(_) => PassingConvention::Copy,
            Type::ClassRef(_) => PassingConvention::MutableRef,
            Type::Interface(_) => PassingConvention::MutableRef,
            Type::Iterator(_) => PassingConvention::MutableRef,
            Type::Collection(_) => PassingConvention::ConstRef,
            Type::Duration(_) => PassingConvention::Copy,
        }
    }
}

impl CppStructType for Type {
    fn get_cpp_struct_type(&self) -> String {
        match self {
            Type::Interface(_) => format!("std::unique_ptr<{}>", self.get_cpp_type()),
            _ => self.get_cpp_type(),
        }
    }
}

impl CppType for ReturnType {
    fn get_cpp_type(&self) -> String {
        match self {
            ReturnType::Void => "void".to_owned(),
            ReturnType::Type(t, _) => t.get_cpp_type(),
        }
    }
}
