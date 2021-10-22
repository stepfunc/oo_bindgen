use oo_bindgen::structs::{FunctionArgStructField, FunctionReturnStructField, CallbackArgStructField, UniversalStructField, StructFieldType};
use oo_bindgen::types::{BasicType, StringType};
use crate::cpp::name::CppName;
use oo_bindgen::function::FunctionArgument;
use oo_bindgen::collection::CollectionHandle;
use oo_bindgen::UniversalOr;
use heck::CamelCase;

pub(crate) trait CppStructType {
    fn struct_member_type(&self) -> String;
}

impl CppStructType for BasicType {
    fn struct_member_type(&self) -> String {
        basic_type(self)
    }
}

fn basic_type(x: &BasicType) -> String {
    match x {
        BasicType::Bool => "bool".to_string(),
        BasicType::U8 => "uint8_t".to_string(),
        BasicType::S8 => "int8_t".to_string(),
        BasicType::U16 => "uint16_t".to_string(),
        BasicType::S16 => "int16_t".to_string(),
        BasicType::U32 => "uint32_t".to_string(),
        BasicType::S32 => "int32_t".to_string(),
        BasicType::U64 => "uint64_t".to_string(),
        BasicType::S64 => "int16_t".to_string(),
        BasicType::Float32 => "float".to_string(),
        BasicType::Double64 => "double".to_string(),
        BasicType::Duration(_) => "std::chrono::steady_clock::duration".to_string(),
        BasicType::Enum(x) => {
            x.cpp_name()
        }
    }
}

impl<T> CppName for UniversalOr<T> where T: StructFieldType {
    fn cpp_name(&self) -> String {
        self.name().to_camel_case()
    }
}

impl CppStructType for StringType {
    fn struct_member_type(&self) -> String {
        "std::string".to_string()
    }
}

impl CppStructType for CollectionHandle {
    fn struct_member_type(&self) -> String {
        format!("std::vector<{}>", self.item_type.struct_member_type())
    }
}

impl CppStructType for FunctionArgument {
    fn struct_member_type(&self) -> String {
        match self {
            FunctionArgument::Basic(x) => x.struct_member_type(),
            FunctionArgument::String(x) => x.struct_member_type(),
            FunctionArgument::Collection(x) => x.struct_member_type(),
            FunctionArgument::Struct(_) => unimplemented!(),
            FunctionArgument::StructRef(_) => unimplemented!(),
            FunctionArgument::ClassRef(_) => unimplemented!(),
            FunctionArgument::Interface(_) => unimplemented!(),
        }
    }
}


impl CppStructType for FunctionArgStructField {
    fn struct_member_type(&self) -> String {
        match self {
            FunctionArgStructField::Basic(x) => x.struct_member_type(),
            FunctionArgStructField::String(x) => x.struct_member_type(),
            FunctionArgStructField::Interface(x) => {
                format!("std::unique_ptr<{}>", x.cpp_name())
            }
            FunctionArgStructField::Collection(x) => {
                x.struct_member_type()
            }
            FunctionArgStructField::Struct(x) => {
                x.cpp_name()
            }
        }
    }
}

impl CppStructType for FunctionReturnStructField {
    fn struct_member_type(&self) -> String {
        match self {
            FunctionReturnStructField::Basic(x) => x.struct_member_type(),
            FunctionReturnStructField::ClassRef(x) => x.cpp_name(),
            FunctionReturnStructField::Iterator(x) => format!("{}&", x.cpp_name()),
            FunctionReturnStructField::Struct(x) => x.cpp_name(),
        }
    }
}

impl CppStructType for CallbackArgStructField {
    fn struct_member_type(&self) -> String {
        match self {
            CallbackArgStructField::Basic(x) => x.struct_member_type(),
            CallbackArgStructField::Iterator(x) => format!("{}&", x.cpp_name()),
            CallbackArgStructField::Struct(x) => x.cpp_name()
        }
    }
}

impl CppStructType for UniversalStructField {
    fn struct_member_type(&self) -> String {
        match self {
            UniversalStructField::Basic(x) => x.struct_member_type(),
            UniversalStructField::Struct(x) => x.cpp_name(),
        }
    }
}


