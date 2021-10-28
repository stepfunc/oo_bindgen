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

pub(crate) trait CoreType {
    fn core_type(&self) -> String;
}

impl CoreType for BasicType {
    fn core_type(&self) -> String {
        match self {
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
            BasicType::Enum(x) => x.core_type(),
        }
    }
}

impl CoreType for StringType {
    fn core_type(&self) -> String {
        "std::string".to_string()
    }
}

impl<T> CoreType for Struct<T>
where
    T: StructFieldType,
{
    fn core_type(&self) -> String {
        self.name().to_camel_case()
    }
}

impl<T> CoreType for UniversalOr<T>
where
    T: StructFieldType,
{
    fn core_type(&self) -> String {
        self.name().to_camel_case()
    }
}

impl CoreType for StructDeclaration {
    fn core_type(&self) -> String {
        self.name.to_camel_case()
    }
}

impl CoreType for StructType {
    fn core_type(&self) -> String {
        self.declaration().core_type()
    }
}

impl CoreType for EnumHandle {
    fn core_type(&self) -> String {
        self.name.to_camel_case()
    }
}

impl CoreType for EnumVariant {
    fn core_type(&self) -> String {
        self.name.to_snake_case()
    }
}

impl CoreType for ErrorType {
    fn core_type(&self) -> String {
        self.exception_name.to_camel_case()
    }
}

impl CoreType for InterfaceHandle {
    fn core_type(&self) -> String {
        self.name.to_camel_case()
    }
}

impl CoreType for IteratorHandle {
    fn core_type(&self) -> String {
        self.iter_type.name.to_camel_case()
    }
}

impl<T> CoreType for Arg<T> {
    fn core_type(&self) -> String {
        self.name.to_snake_case()
    }
}

impl CoreType for CallbackFunction {
    fn core_type(&self) -> String {
        self.name.to_snake_case()
    }
}

impl CoreType for Constant {
    fn core_type(&self) -> String {
        self.name.to_snake_case()
    }
}

impl CoreType for ClassDeclarationHandle {
    fn core_type(&self) -> String {
        self.name.to_camel_case()
    }
}

impl CoreType for ClassHandle {
    fn core_type(&self) -> String {
        self.name().to_camel_case()
    }
}

impl CoreType for StaticClassHandle {
    fn core_type(&self) -> String {
        self.name.to_camel_case()
    }
}

impl CoreType for CollectionHandle {
    fn core_type(&self) -> String {
        let inner = match &self.item_type {
            FunctionArgument::Basic(x) => x.core_type(),
            FunctionArgument::String(x) => x.core_type(),
            FunctionArgument::Collection(x) => x.core_type(),
            FunctionArgument::Struct(x) => x.core_type(),
            FunctionArgument::StructRef(_) => unimplemented!(),
            FunctionArgument::ClassRef(_) => unimplemented!(),
            FunctionArgument::Interface(_) => unimplemented!(),
        };
        format!("std::vector<{}>", inner)
    }
}
