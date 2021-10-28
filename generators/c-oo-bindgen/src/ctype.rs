use oo_bindgen::class::ClassDeclarationHandle;
use oo_bindgen::collection::CollectionHandle;
use oo_bindgen::enum_type::EnumHandle;
use oo_bindgen::function::{FunctionArgument, FunctionReturnValue};
use oo_bindgen::interface::{CallbackArgument, CallbackReturnValue, InterfaceHandle};
use oo_bindgen::iterator::IteratorHandle;
use oo_bindgen::return_type::ReturnType;
use oo_bindgen::structs::*;
use oo_bindgen::types::{BasicType, StringType};
use oo_bindgen::{StructType, Symbol, UniversalOr};

use heck::SnakeCase;

pub(crate) trait CType {
    fn to_c_type(&self, prefix: &str) -> String;
}

struct Pointer<'a, T>
where
    T: CType,
{
    inner: &'a T,
}

fn pointer<T>(inner: &T) -> Pointer<T>
where
    T: CType,
{
    Pointer { inner }
}

impl<'a, T> CType for Pointer<'a, T>
where
    T: CType,
{
    fn to_c_type(&self, prefix: &str) -> String {
        format!("{}*", self.inner.to_c_type(prefix))
    }
}

impl CType for StringType {
    fn to_c_type(&self, _prefix: &str) -> String {
        "const char*".to_string()
    }
}

impl CType for IteratorHandle {
    fn to_c_type(&self, prefix: &str) -> String {
        format!("{}*", self.iter_type.to_c_type(prefix))
    }
}

impl CType for CallbackArgument {
    fn to_c_type(&self, prefix: &str) -> String {
        match self {
            CallbackArgument::Basic(x) => x.to_c_type(prefix),
            CallbackArgument::String(x) => x.to_c_type(prefix),
            CallbackArgument::Iterator(x) => x.to_c_type(prefix),
            CallbackArgument::Struct(x) => x.to_c_type(prefix),
            CallbackArgument::Class(x) => pointer(x).to_c_type(prefix),
        }
    }
}

impl CType for FunctionReturnValue {
    fn to_c_type(&self, prefix: &str) -> String {
        match self {
            FunctionReturnValue::Basic(x) => x.to_c_type(prefix),
            FunctionReturnValue::String(x) => x.to_c_type(prefix),
            FunctionReturnValue::ClassRef(x) => pointer(x).to_c_type(prefix),
            FunctionReturnValue::Struct(x) => x.to_c_type(prefix),
            FunctionReturnValue::StructRef(x) => pointer(&x.inner).to_c_type(prefix),
        }
    }
}

impl CType for CallbackReturnValue {
    fn to_c_type(&self, prefix: &str) -> String {
        match self {
            CallbackReturnValue::Basic(x) => x.to_c_type(prefix),
            CallbackReturnValue::Struct(x) => x.to_c_type(prefix),
        }
    }
}

impl CType for StructDeclarationHandle {
    fn to_c_type(&self, prefix: &str) -> String {
        format!("{}_{}_t", prefix.to_snake_case(), self.name.to_snake_case())
    }
}

impl CType for StructType {
    fn to_c_type(&self, prefix: &str) -> String {
        self.declaration().to_c_type(prefix)
    }
}

impl<T> CType for Struct<T>
where
    T: StructFieldType,
{
    fn to_c_type(&self, prefix: &str) -> String {
        format!(
            "{}_{}_t",
            prefix.to_snake_case(),
            self.name().to_snake_case()
        )
    }
}

impl CType for EnumHandle {
    fn to_c_type(&self, prefix: &str) -> String {
        format!("{}_{}_t", prefix.to_snake_case(), self.name.to_snake_case())
    }
}

impl CType for ClassDeclarationHandle {
    fn to_c_type(&self, prefix: &str) -> String {
        format!("{}_{}_t", prefix.to_snake_case(), self.name.to_snake_case())
    }
}

impl CType for InterfaceHandle {
    fn to_c_type(&self, prefix: &str) -> String {
        format!("{}_{}_t", prefix.to_snake_case(), self.name.to_snake_case())
    }
}

impl CType for CollectionHandle {
    fn to_c_type(&self, prefix: &str) -> String {
        self.collection_type.to_c_type(prefix)
    }
}

impl CType for Symbol {
    fn to_c_type(&self, prefix: &str) -> String {
        match self {
            Symbol::Function(handle) => format!("{}_{}", prefix.to_snake_case(), handle.name),
            Symbol::Struct(handle) => handle.declaration().to_c_type(prefix),
            Symbol::Enum(handle) => handle.to_c_type(prefix),
            Symbol::Class(handle) => handle.declaration.to_c_type(prefix),
            Symbol::StaticClass(_) => panic!("static classes cannot be referenced in C"),
            Symbol::Interface(handle) => handle.to_c_type(prefix),
            Symbol::Iterator(handle) => handle.iter_type.to_c_type(prefix),
            Symbol::Collection(handle) => handle.collection_type.to_c_type(prefix),
        }
    }
}

impl CType for BasicType {
    fn to_c_type(&self, prefix: &str) -> String {
        match self {
            Self::Bool => "bool".to_string(),
            Self::U8 => "uint8_t".to_string(),
            Self::S8 => "int8_t".to_string(),
            Self::U16 => "uint16_t".to_string(),
            Self::S16 => "int16_t".to_string(),
            Self::U32 => "uint32_t".to_string(),
            Self::S32 => "int32_t".to_string(),
            Self::U64 => "uint64_t".to_string(),
            Self::S64 => "int64_t".to_string(),
            Self::Float32 => "float".to_string(),
            Self::Double64 => "double".to_string(),
            Self::Duration(_) => "uint64_t".to_string(),
            Self::Enum(handle) => handle.to_c_type(prefix),
        }
    }
}

impl<T> CType for UniversalOr<T>
where
    T: StructFieldType,
{
    fn to_c_type(&self, prefix: &str) -> String {
        self.to_struct_type().to_c_type(prefix)
    }
}

impl CType for FunctionArgStructField {
    fn to_c_type(&self, prefix: &str) -> String {
        match self {
            FunctionArgStructField::Basic(x) => x.to_c_type(prefix),
            FunctionArgStructField::String(x) => x.to_c_type(prefix),
            FunctionArgStructField::Interface(x) => x.to_c_type(prefix),
            FunctionArgStructField::Collection(x) => pointer(x).to_c_type(prefix),
            FunctionArgStructField::Struct(x) => x.to_c_type(prefix),
        }
    }
}

impl CType for FunctionReturnStructField {
    fn to_c_type(&self, prefix: &str) -> String {
        match self {
            Self::Basic(x) => x.to_c_type(prefix),
            Self::ClassRef(x) => pointer(x).to_c_type(prefix),
            Self::Struct(x) => x.to_c_type(prefix),
            Self::Iterator(x) => x.to_c_type(prefix),
        }
    }
}

impl CType for CallbackArgStructField {
    fn to_c_type(&self, prefix: &str) -> String {
        match self {
            CallbackArgStructField::Basic(x) => x.to_c_type(prefix),
            CallbackArgStructField::Iterator(x) => pointer(x).to_c_type(prefix),
            CallbackArgStructField::Struct(x) => x.to_c_type(prefix),
        }
    }
}

impl CType for UniversalStructField {
    fn to_c_type(&self, prefix: &str) -> String {
        match self {
            UniversalStructField::Basic(x) => x.to_c_type(prefix),
            UniversalStructField::Struct(x) => x.to_c_type(prefix),
        }
    }
}

impl CType for FunctionArgument {
    fn to_c_type(&self, prefix: &str) -> String {
        match self {
            FunctionArgument::Basic(x) => x.to_c_type(prefix),
            FunctionArgument::String(x) => x.to_c_type(prefix),
            FunctionArgument::Collection(x) => pointer(x).to_c_type(prefix),
            FunctionArgument::Struct(x) => x.to_c_type(prefix),
            FunctionArgument::StructRef(x) => pointer(&x.inner).to_c_type(prefix),
            FunctionArgument::ClassRef(x) => pointer(x).to_c_type(prefix),
            FunctionArgument::Interface(x) => x.to_c_type(prefix),
        }
    }
}

/* TODO
impl CFormatting for AnyType {
    fn to_c_type(&self, prefix: &str) -> String {
        match self {
            AnyType::Basic(b) => b.to_c_type(prefix),
            AnyType::String => "const char*".to_string(),
            AnyType::Struct(handle) => handle.to_c_type(prefix),
            AnyType::StructRef(handle) => format!("{}*", handle.to_c_type(prefix)),
            AnyType::ClassRef(handle) => format!("{}*", handle.to_c_type(prefix)),
            AnyType::Interface(handle) => handle.to_c_type(prefix),
            AnyType::Iterator(handle) => format!("{}*", handle.iter_type.to_c_type(prefix)),
            AnyType::Collection(handle) => {
                format!("{}*", handle.collection_type.to_c_type(prefix))
            }
        }
    }
}
 */

impl<T> CType for ReturnType<T>
where
    T: CType,
{
    fn to_c_type(&self, prefix: &str) -> String {
        match self {
            Self::Void => "void".to_string(),
            Self::Type(return_type, _) => return_type.to_c_type(prefix),
        }
    }
}
