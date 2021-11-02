use heck::{CamelCase, SnakeCase};
use oo_bindgen::class::{ClassDeclarationHandle, ClassHandle, StaticClassHandle};
use oo_bindgen::collection::CollectionHandle;
use oo_bindgen::constants::Constant;
use oo_bindgen::enum_type::{EnumHandle, EnumVariant};
use oo_bindgen::error_type::ErrorType;
use oo_bindgen::function::FunctionArgument;
use oo_bindgen::interface::{CallbackFunction, InterfaceHandle};
use oo_bindgen::iterator::IteratorHandle;
use oo_bindgen::structs::*;
use oo_bindgen::types::{Arg, BasicType, StringType};
use oo_bindgen::{StructType, UniversalOr};

pub(crate) trait CoreCppType {
    fn core_cpp_type(&self) -> String;
}

impl CoreCppType for BasicType {
    fn core_cpp_type(&self) -> String {
        match self {
            BasicType::Bool => "bool".to_string(),
            BasicType::U8 => "uint8_t".to_string(),
            BasicType::S8 => "int8_t".to_string(),
            BasicType::U16 => "uint16_t".to_string(),
            BasicType::S16 => "int16_t".to_string(),
            BasicType::U32 => "uint32_t".to_string(),
            BasicType::S32 => "int32_t".to_string(),
            BasicType::U64 => "uint64_t".to_string(),
            BasicType::S64 => "int64_t".to_string(),
            BasicType::Float32 => "float".to_string(),
            BasicType::Double64 => "double".to_string(),
            BasicType::Duration(_) => "std::chrono::steady_clock::duration".to_string(),
            BasicType::Enum(x) => x.core_cpp_type(),
        }
    }
}

impl CoreCppType for StringType {
    fn core_cpp_type(&self) -> String {
        "std::string".to_string()
    }
}

impl<T> CoreCppType for Struct<T>
where
    T: StructFieldType,
{
    fn core_cpp_type(&self) -> String {
        self.name().to_camel_case()
    }
}

impl<T> CoreCppType for UniversalOr<T>
where
    T: StructFieldType,
{
    fn core_cpp_type(&self) -> String {
        self.name().to_camel_case()
    }
}

impl CoreCppType for StructDeclaration {
    fn core_cpp_type(&self) -> String {
        self.name.to_camel_case()
    }
}

impl CoreCppType for StructType {
    fn core_cpp_type(&self) -> String {
        self.declaration().core_cpp_type()
    }
}

impl CoreCppType for EnumHandle {
    fn core_cpp_type(&self) -> String {
        self.name.to_camel_case()
    }
}

impl CoreCppType for EnumVariant {
    fn core_cpp_type(&self) -> String {
        self.name.to_snake_case()
    }
}

impl CoreCppType for ErrorType {
    fn core_cpp_type(&self) -> String {
        self.exception_name.to_camel_case()
    }
}

impl CoreCppType for InterfaceHandle {
    fn core_cpp_type(&self) -> String {
        self.name.to_camel_case()
    }
}

impl CoreCppType for IteratorHandle {
    fn core_cpp_type(&self) -> String {
        self.iter_class.name.to_camel_case()
    }
}

impl<T> CoreCppType for Arg<T> {
    fn core_cpp_type(&self) -> String {
        self.name.to_snake_case()
    }
}

impl CoreCppType for CallbackFunction {
    fn core_cpp_type(&self) -> String {
        self.name.to_snake_case()
    }
}

impl CoreCppType for Constant {
    fn core_cpp_type(&self) -> String {
        self.name.to_snake_case()
    }
}

impl CoreCppType for ClassDeclarationHandle {
    fn core_cpp_type(&self) -> String {
        self.name.to_camel_case()
    }
}

impl CoreCppType for ClassHandle {
    fn core_cpp_type(&self) -> String {
        self.name().to_camel_case()
    }
}

impl CoreCppType for StaticClassHandle {
    fn core_cpp_type(&self) -> String {
        self.name.to_camel_case()
    }
}

impl CoreCppType for CollectionHandle {
    fn core_cpp_type(&self) -> String {
        let inner = match &self.item_type {
            FunctionArgument::Basic(x) => x.core_cpp_type(),
            FunctionArgument::String(x) => x.core_cpp_type(),
            FunctionArgument::Collection(x) => x.core_cpp_type(),
            FunctionArgument::Struct(x) => x.core_cpp_type(),
            FunctionArgument::StructRef(_) => unimplemented!(),
            FunctionArgument::ClassRef(_) => unimplemented!(),
            FunctionArgument::Interface(_) => unimplemented!(),
        };
        format!("std::vector<{}>", inner)
    }
}
