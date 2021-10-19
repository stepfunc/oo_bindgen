use oo_bindgen::class::ClassDeclarationHandle;
use oo_bindgen::enum_type::EnumHandle;
use oo_bindgen::function::{FArgument, FReturnValue};
use oo_bindgen::interface::{CArgument, CReturnValue, InterfaceHandle};
use oo_bindgen::iterator::IteratorHandle;
use oo_bindgen::structs::common::{Struct, StructDeclarationHandle, StructFieldType};
use oo_bindgen::types::{BasicType, StringType};
use oo_bindgen::{StructType, Symbol, UniversalOr};

use heck::SnakeCase;
use oo_bindgen::collection::CollectionHandle;
use oo_bindgen::return_type::ReturnType;
use oo_bindgen::structs::callback_struct::CallbackStructFieldType;
use oo_bindgen::structs::function_return_struct::ReturnStructFieldType;
use oo_bindgen::structs::function_struct::FunctionArgStructFieldType;
use oo_bindgen::structs::univeral_struct::UniversalStructFieldType;

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

impl CType for CArgument {
    fn to_c_type(&self, prefix: &str) -> String {
        match self {
            CArgument::Basic(x) => x.to_c_type(prefix),
            CArgument::String(x) => x.to_c_type(prefix),
            CArgument::Iterator(x) => x.to_c_type(prefix),
            CArgument::Struct(x) => x.to_c_type(prefix),
        }
    }
}

impl CType for FReturnValue {
    fn to_c_type(&self, prefix: &str) -> String {
        match self {
            FReturnValue::Basic(x) => x.to_c_type(prefix),
            FReturnValue::String(x) => x.to_c_type(prefix),
            FReturnValue::ClassRef(x) => pointer(x).to_c_type(prefix),
            FReturnValue::Struct(x) => x.to_c_type(prefix),
            FReturnValue::StructRef(x) => pointer(x).to_c_type(prefix),
        }
    }
}

impl CType for CReturnValue {
    fn to_c_type(&self, prefix: &str) -> String {
        match self {
            CReturnValue::Basic(x) => x.to_c_type(prefix),
            CReturnValue::Struct(x) => x.to_c_type(prefix),
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
            Self::Uint8 => "uint8_t".to_string(),
            Self::Sint8 => "int8_t".to_string(),
            Self::Uint16 => "uint16_t".to_string(),
            Self::Sint16 => "int16_t".to_string(),
            Self::Uint32 => "uint32_t".to_string(),
            Self::Sint32 => "int32_t".to_string(),
            Self::Uint64 => "uint64_t".to_string(),
            Self::Sint64 => "int64_t".to_string(),
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

impl CType for FunctionArgStructFieldType {
    fn to_c_type(&self, prefix: &str) -> String {
        match self {
            FunctionArgStructFieldType::Basic(x) => x.to_c_type(prefix),
            FunctionArgStructFieldType::String(x) => x.to_c_type(prefix),
            FunctionArgStructFieldType::Interface(x) => x.to_c_type(prefix),
            FunctionArgStructFieldType::Collection(x) => pointer(x).to_c_type(prefix),
            FunctionArgStructFieldType::Struct(x) => x.to_c_type(prefix),
        }
    }
}

impl CType for ReturnStructFieldType {
    fn to_c_type(&self, prefix: &str) -> String {
        match self {
            Self::Basic(x) => x.to_c_type(prefix),
            Self::ClassRef(x) => pointer(x).to_c_type(prefix),
            Self::Struct(x) => x.to_c_type(prefix),
            Self::Iterator(x) => x.to_c_type(prefix),
        }
    }
}

impl CType for CallbackStructFieldType {
    fn to_c_type(&self, prefix: &str) -> String {
        match self {
            CallbackStructFieldType::Basic(x) => x.to_c_type(prefix),
            CallbackStructFieldType::Iterator(x) => pointer(x).to_c_type(prefix),
            CallbackStructFieldType::Struct(x) => x.to_c_type(prefix),
        }
    }
}

impl CType for UniversalStructFieldType {
    fn to_c_type(&self, prefix: &str) -> String {
        match self {
            UniversalStructFieldType::Basic(x) => x.to_c_type(prefix),
            UniversalStructFieldType::Struct(x) => x.to_c_type(prefix),
        }
    }
}

impl CType for FArgument {
    fn to_c_type(&self, prefix: &str) -> String {
        match self {
            FArgument::Basic(x) => x.to_c_type(prefix),
            FArgument::String(x) => x.to_c_type(prefix),
            FArgument::Collection(x) => pointer(x).to_c_type(prefix),
            FArgument::Struct(x) => x.to_c_type(prefix),
            FArgument::StructRef(x) => pointer(x).to_c_type(prefix),
            FArgument::ClassRef(x) => pointer(x).to_c_type(prefix),
            FArgument::Interface(x) => x.to_c_type(prefix),
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
