use heck::CamelCase;

use oo_bindgen::interface::*;
use oo_bindgen::return_type::ReturnType;
use oo_bindgen::structs::common::*;
use oo_bindgen::types::*;
use oo_bindgen::structs::function_struct::FStructFieldType;
use oo_bindgen::structs::callback_struct::CStructFieldType;
use oo_bindgen::structs::function_return_struct::RStructFieldType;
use oo_bindgen::structs::univeral_struct::UStructFieldType;
use oo_bindgen::function::{FArgument, FReturnValue};
use oo_bindgen::collection::CollectionHandle;
use oo_bindgen::class::ClassDeclarationHandle;
use crate::type_converter::*;
use oo_bindgen::iterator::IteratorHandle;

pub(crate) trait LifetimeInfo {
    fn rust_requires_lifetime(&self) -> bool;
    fn c_requires_lifetime(&self) -> bool;
}

pub(crate) trait RustType : LifetimeInfo {
    fn as_rust_type(&self) -> String;
    fn as_c_type(&self) -> String;
    fn is_copyable(&self) -> bool;
    fn conversion(&self) -> Option<TypeConverter>;
    fn has_conversion(&self) -> bool {
        self.conversion().is_some()
    }
}

impl LifetimeInfo for BasicType {
    fn rust_requires_lifetime(&self) -> bool {
        false
    }
    fn c_requires_lifetime(&self) -> bool {
        false
    }
}

impl RustType for BasicType {
    fn as_rust_type(&self) -> String {
        match self {
            Self::Bool => "bool".to_string(),
            Self::Uint8 => "u8".to_string(),
            Self::Sint8 => "i8".to_string(),
            Self::Uint16 => "u16".to_string(),
            Self::Sint16 => "i16".to_string(),
            Self::Uint32 => "u32".to_string(),
            Self::Sint32 => "i32".to_string(),
            Self::Uint64 => "u64".to_string(),
            Self::Sint64 => "i64".to_string(),
            Self::Float32 => "f32".to_string(),
            Self::Double64 => "f64".to_string(),
            Self::Duration(_) => "std::time::Duration".to_string(),
            Self::Enum(handle) => handle.name.to_camel_case(),
        }
    }

    fn as_c_type(&self) -> String {
        self.get_c_rust_type().to_string()
    }

    fn is_copyable(&self) -> bool {
        true
    }

    fn conversion(&self) -> Option<TypeConverter> {
        match self {
            Self::Bool => None,
            Self::Uint8 => None,
            Self::Sint8 => None,
            Self::Uint16 => None,
            Self::Sint16 => None,
            Self::Uint32 => None,
            Self::Sint32 => None,
            Self::Uint64 => None,
            Self::Sint64 => None,
            Self::Float32 => None,
            Self::Double64 => None,
            Self::Duration(x) => Some(TypeConverter::Duration(*x)),
            Self::Enum(x) => Some(TypeConverter::Enum(x.clone())),
        }
    }
}

impl LifetimeInfo for StringType {
    fn rust_requires_lifetime(&self) -> bool {
        true
    }

    fn c_requires_lifetime(&self) -> bool {
        false
    }
}

impl LifetimeInfo for CollectionHandle {
    fn rust_requires_lifetime(&self) -> bool {
        false
    }

    fn c_requires_lifetime(&self) -> bool {
        false
    }
}

impl<T> LifetimeInfo for Struct<T> where T: StructFieldType {
    fn rust_requires_lifetime(&self) -> bool {
        false
    }

    fn c_requires_lifetime(&self) -> bool {
        false
    }
}

impl LifetimeInfo for StructDeclarationHandle {
    fn rust_requires_lifetime(&self) -> bool {
        false
    }

    fn c_requires_lifetime(&self) -> bool {
        false
    }
}

impl LifetimeInfo for ClassDeclarationHandle {
    fn rust_requires_lifetime(&self) -> bool {
        false
    }

    fn c_requires_lifetime(&self) -> bool {
        false
    }
}

impl LifetimeInfo for InterfaceHandle {
    fn rust_requires_lifetime(&self) -> bool {
        false
    }

    fn c_requires_lifetime(&self) -> bool {
        false
    }
}

impl LifetimeInfo for IteratorHandle {
    fn rust_requires_lifetime(&self) -> bool {
        self.has_lifetime_annotation
    }

    fn c_requires_lifetime(&self) -> bool {
        self.has_lifetime_annotation
    }
}

impl LifetimeInfo for FArgument {
    fn rust_requires_lifetime(&self) -> bool {
        match self {
            FArgument::Basic(x) => x.rust_requires_lifetime(),
            FArgument::String(x) => x.rust_requires_lifetime(),
            FArgument::Collection(x) => x.rust_requires_lifetime(),
            FArgument::Struct(x) => x.rust_requires_lifetime(),
            FArgument::StructRef(x) => x.rust_requires_lifetime(),
            FArgument::ClassRef(x) => x.rust_requires_lifetime(),
            FArgument::Interface(x) => x.rust_requires_lifetime(),
        }
    }

    fn c_requires_lifetime(&self) -> bool {
        match self {
            FArgument::Basic(x) => x.c_requires_lifetime(),
            FArgument::String(x) => x.c_requires_lifetime(),
            FArgument::Collection(x) => x.c_requires_lifetime(),
            FArgument::Struct(x) => x.c_requires_lifetime(),
            FArgument::StructRef(x) => x.c_requires_lifetime(),
            FArgument::ClassRef(x) => x.c_requires_lifetime(),
            FArgument::Interface(x) => x.c_requires_lifetime(),
        }
    }
}

impl RustType for StringType {
    fn as_rust_type(&self) -> String {
        "&'a std::ffi::CStr".to_string()
    }

    fn as_c_type(&self) -> String {
        "*const std::os::raw::c_char".to_string()
    }

    fn is_copyable(&self) -> bool {
        true // just copying the reference
    }

    fn conversion(&self) -> Option<TypeConverter> {
        Some(TypeConverter::String(*self))
    }
}

impl RustType for CollectionHandle {
    fn as_rust_type(&self) -> String {
        format!("*mut crate::{}", self.name().to_camel_case())
    }

    fn as_c_type(&self) -> String {
        format!("*mut crate::{}", self.name().to_camel_case())
    }

    fn is_copyable(&self) -> bool {
        // just copying the pointer
        true
    }

    fn conversion(&self) -> Option<TypeConverter> {
        None
    }
}

impl<T> RustType for Struct<T> where T: StructFieldType {
    fn as_rust_type(&self) -> String {
        self.name().to_camel_case()
    }

    fn as_c_type(&self) -> String {
        self.name().to_camel_case()
    }

    fn is_copyable(&self) -> bool {
        false
    }

    fn conversion(&self) -> Option<TypeConverter> {
        None
    }
}

impl RustType for StructDeclarationHandle {
    fn as_rust_type(&self) -> String {
        format!("Option<&{}>", self.name.to_camel_case())
    }

    fn as_c_type(&self) -> String {
        format!("*const {}", self.name.to_camel_case())
    }

    fn is_copyable(&self) -> bool {
        true
    }

    fn conversion(&self) -> Option<TypeConverter> {
        Some(TypeConverter::Struct(self.clone()))
    }
}

impl RustType for ClassDeclarationHandle {
    fn as_rust_type(&self) -> String {
        format!("*mut crate::{}", self.name.to_camel_case())
    }

    fn as_c_type(&self) -> String {
        format!("*mut crate::{}", self.name.to_camel_case())
    }

    fn is_copyable(&self) -> bool {
        // Just copying the opaque pointer
        true
    }

    fn conversion(&self) -> Option<TypeConverter> {
        None
    }
}

impl RustType for InterfaceHandle {
    fn as_rust_type(&self) -> String {
        self.name.to_camel_case()
    }

    fn as_c_type(&self) -> String {
        self.name.to_camel_case()
    }

    fn is_copyable(&self) -> bool {
        false
    }

    fn conversion(&self) -> Option<TypeConverter> {
        None
    }
}

impl RustType for IteratorHandle {
    fn as_rust_type(&self) -> String {
        let lifetime = if self.has_lifetime_annotation {
            "<'a>"
        } else {
            ""
        };
        format!("*mut crate::{}{}", self.name().to_camel_case(), lifetime)
    }

    fn as_c_type(&self) -> String {
        // same
        self.as_rust_type()
    }

    fn is_copyable(&self) -> bool {
        true // just copying the pointer
    }

    fn conversion(&self) -> Option<TypeConverter> {
        None
    }
}

impl RustType for FArgument {
    fn as_rust_type(&self) -> String {
        match self {
            FArgument::Basic(x) => x.as_rust_type(),
            FArgument::String(x) => x.as_rust_type(),
            FArgument::Collection(x) => x.as_rust_type(),
            FArgument::Struct(x) => x.as_rust_type(),
            FArgument::StructRef(x) => x.as_rust_type(),
            FArgument::ClassRef(x) => x.as_rust_type(),
            FArgument::Interface(x) => x.as_rust_type(),
        }
    }

    fn as_c_type(&self) -> String {
        match self {
            FArgument::Basic(x) => x.as_c_type(),
            FArgument::String(x) => x.as_c_type(),
            FArgument::Collection(x) => x.as_c_type(),
            FArgument::Struct(x) => x.as_c_type(),
            FArgument::StructRef(x) => x.as_c_type(),
            FArgument::ClassRef(x) => x.as_c_type(),
            FArgument::Interface(x) => x.as_c_type(),
        }
    }

    fn is_copyable(&self) -> bool {
        match self {
            FArgument::Basic(x) => x.is_copyable(),
            FArgument::String(x) => x.is_copyable(),
            FArgument::Collection(x) => x.is_copyable(),
            FArgument::Struct(x) => x.is_copyable(),
            FArgument::StructRef(x) => x.is_copyable(),
            FArgument::ClassRef(x) => x.is_copyable(),
            FArgument::Interface(x) => x.is_copyable(),
        }
    }

    fn conversion(&self) -> Option<TypeConverter> {
        match self {
            FArgument::Basic(x) => x.conversion(),
            FArgument::String(x) => x.conversion(),
            FArgument::Collection(x) => x.conversion(),
            FArgument::Struct(x) => x.conversion(),
            FArgument::StructRef(x) => x.conversion(),
            FArgument::ClassRef(x) => x.conversion(),
            FArgument::Interface(x) => x.conversion(),
        }
    }
}

impl LifetimeInfo for FReturnValue {
    fn rust_requires_lifetime(&self) -> bool {
        match self {
            FReturnValue::Basic(x) => x.rust_requires_lifetime(),
            FReturnValue::String(x) => x.rust_requires_lifetime(),
            FReturnValue::ClassRef(x) => x.rust_requires_lifetime(),
            FReturnValue::Struct(x) => x.rust_requires_lifetime(),
            FReturnValue::StructRef(x) => x.rust_requires_lifetime(),
        }
    }

    fn c_requires_lifetime(&self) -> bool {
        match self {
            FReturnValue::Basic(x) => x.c_requires_lifetime(),
            FReturnValue::String(x) => x.c_requires_lifetime(),
            FReturnValue::ClassRef(x) => x.c_requires_lifetime(),
            FReturnValue::Struct(x) => x.c_requires_lifetime(),
            FReturnValue::StructRef(x) => x.c_requires_lifetime(),
        }
    }
}

impl RustType for FReturnValue {
    fn as_rust_type(&self) -> String {
        match self {
            FReturnValue::Basic(x) => x.as_rust_type(),
            FReturnValue::String(x) => x.as_rust_type(),
            FReturnValue::ClassRef(x) => x.as_rust_type(),
            FReturnValue::Struct(x) => x.as_rust_type(),
            FReturnValue::StructRef(x) => x.as_rust_type(),
        }
    }

    fn as_c_type(&self) -> String {
        match self {
            FReturnValue::Basic(x) => x.as_c_type(),
            FReturnValue::String(x) => x.as_c_type(),
            FReturnValue::ClassRef(x) => x.as_c_type(),
            FReturnValue::Struct(x) => x.as_c_type(),
            FReturnValue::StructRef(x) => x.as_c_type(),
        }
    }

    fn is_copyable(&self) -> bool {
        match self {
            FReturnValue::Basic(x) => x.is_copyable(),
            FReturnValue::String(x) => x.is_copyable(),
            FReturnValue::ClassRef(x) => x.is_copyable(),
            FReturnValue::Struct(x) => x.is_copyable(),
            FReturnValue::StructRef(x) => x.is_copyable(),
        }
    }

    fn conversion(&self) -> Option<TypeConverter> {
        match self {
            FReturnValue::Basic(x) => x.conversion(),
            FReturnValue::String(x) => x.conversion(),
            FReturnValue::ClassRef(x) => x.conversion(),
            FReturnValue::Struct(x) => x.conversion(),
            FReturnValue::StructRef(x) => x.conversion(),
        }
    }
}

impl RustType for FStructFieldType {
    fn as_rust_type(&self) -> String {
        match self {
            FStructFieldType::Basic(x) => x.as_rust_type(),
            FStructFieldType::String(x) => x.as_rust_type(),
            FStructFieldType::Interface(x) => x.as_rust_type(),
            FStructFieldType::Collection(x) => x.as_rust_type(),
            FStructFieldType::Struct(x) => x.as_rust_type(),
        }
    }

    fn as_c_type(&self) -> String {
        match self {
            FStructFieldType::Basic(x) => x.as_c_type(),
            FStructFieldType::String(x) => x.as_c_type(),
            FStructFieldType::Interface(x) => x.as_c_type(),
            FStructFieldType::Collection(x) => x.as_c_type(),
            FStructFieldType::Struct(x) => x.as_c_type(),
        }
    }

    fn is_copyable(&self) -> bool {
        match self {
            FStructFieldType::Basic(x) => x.is_copyable(),
            FStructFieldType::String(x) => x.is_copyable(),
            FStructFieldType::Interface(x) => x.is_copyable(),
            FStructFieldType::Collection(x) => x.is_copyable(),
            FStructFieldType::Struct(x) => x.is_copyable(),
        }
    }

    fn conversion(&self) -> Option<TypeConverter> {
        match self {
            FStructFieldType::Basic(x) => x.conversion(),
            FStructFieldType::String(x) => x.conversion(),
            FStructFieldType::Interface(x) => x.conversion(),
            FStructFieldType::Collection(x) => x.conversion(),
            FStructFieldType::Struct(x) => x.conversion(),
        }
    }
}

impl RustType for RStructFieldType {
    fn as_rust_type(&self) -> String {
        match self {
            RStructFieldType::Basic(x) => x.as_rust_type(),
            RStructFieldType::ClassRef(x) => x.as_rust_type(),
            RStructFieldType::Struct(x) => x.as_rust_type(),
        }
    }

    fn as_c_type(&self) -> String {
        match self {
            RStructFieldType::Basic(x) => x.as_c_type(),
            RStructFieldType::ClassRef(x) => x.as_c_type(),
            RStructFieldType::Struct(x) => x.as_c_type(),
        }
    }

    fn is_copyable(&self) -> bool {
        match self {
            RStructFieldType::Basic(x) => x.is_copyable(),
            RStructFieldType::ClassRef(x) => x.is_copyable(),
            RStructFieldType::Struct(x) => x.is_copyable(),
        }
    }

    fn conversion(&self) -> Option<TypeConverter> {
        match self {
            RStructFieldType::Basic(x) => x.conversion(),
            RStructFieldType::ClassRef(x) => x.conversion(),
            RStructFieldType::Struct(x) => x.conversion(),
        }
    }
}

impl RustType for CStructFieldType {
    fn as_rust_type(&self) -> String {
        match self {
            CStructFieldType::Basic(x) => x.as_rust_type(),
            CStructFieldType::Iterator(x) => x.as_rust_type(),
            CStructFieldType::Struct(x) => x.as_rust_type(),
        }
    }

    fn as_c_type(&self) -> String {
        match self {
            CStructFieldType::Basic(x) => x.as_c_type(),
            CStructFieldType::Iterator(x) => x.as_c_type(),
            CStructFieldType::Struct(x) => x.as_c_type(),
        }
    }

    fn is_copyable(&self) -> bool {
        match self {
            CStructFieldType::Basic(x) => x.is_copyable(),
            CStructFieldType::Iterator(x) => x.is_copyable(),
            CStructFieldType::Struct(x) => x.is_copyable(),
        }
    }

    fn conversion(&self) -> Option<TypeConverter> {
        match self {
            CStructFieldType::Basic(x) => x.conversion(),
            CStructFieldType::Iterator(x) => x.conversion(),
            CStructFieldType::Struct(x) => x.conversion(),
        }
    }
}

impl RustType for UStructFieldType {
    fn as_rust_type(&self) -> String {
        match self {
            UStructFieldType::Basic(x) => x.as_rust_type(),
            UStructFieldType::Struct(x) => x.as_rust_type(),
        }
    }

    fn as_c_type(&self) -> String {
        match self {
            UStructFieldType::Basic(x) => x.as_c_type(),
            UStructFieldType::Struct(x) => x.as_c_type(),
        }
    }

    fn is_copyable(&self) -> bool {
        match self {
            UStructFieldType::Basic(x) => x.is_copyable(),
            UStructFieldType::Struct(x) => x.is_copyable(),
        }
    }

    fn conversion(&self) -> Option<TypeConverter> {
        match self {
            UStructFieldType::Basic(x) => x.conversion(),
            UStructFieldType::Struct(x) => x.conversion(),
        }
    }
}

impl LifetimeInfo for CArgument {
    fn rust_requires_lifetime(&self) -> bool {
        match self {
            CArgument::Basic(x) => x.rust_requires_lifetime(),
            CArgument::String(x) => x.rust_requires_lifetime(),
            CArgument::Iterator(x) => x.rust_requires_lifetime(),
            CArgument::Struct(x) => x.rust_requires_lifetime(),
        }
    }

    fn c_requires_lifetime(&self) -> bool {
        match self {
            CArgument::Basic(x) => x.c_requires_lifetime(),
            CArgument::String(x) => x.c_requires_lifetime(),
            CArgument::Iterator(x) => x.c_requires_lifetime(),
            CArgument::Struct(x) => x.c_requires_lifetime(),
        }
    }
}

impl RustType for CArgument {
    fn as_rust_type(&self) -> String {
        match self {
            CArgument::Basic(x) => x.as_rust_type(),
            CArgument::String(x) => x.as_rust_type(),
            CArgument::Iterator(x) => x.as_rust_type(),
            CArgument::Struct(x) => x.as_rust_type(),
        }
    }

    fn as_c_type(&self) -> String {
        match self {
            CArgument::Basic(x) => x.as_c_type(),
            CArgument::String(x) => x.as_c_type(),
            CArgument::Iterator(x) => x.as_c_type(),
            CArgument::Struct(x) => x.as_c_type(),
        }
    }

    fn is_copyable(&self) -> bool {
        match self {
            CArgument::Basic(x) => x.is_copyable(),
            CArgument::String(x) => x.is_copyable(),
            CArgument::Iterator(x) => x.is_copyable(),
            CArgument::Struct(x) => x.is_copyable(),
        }
    }

    fn conversion(&self) -> Option<TypeConverter> {
        match self {
            CArgument::Basic(x) => x.conversion(),
            CArgument::String(x) => x.conversion(),
            CArgument::Iterator(x) => x.conversion(),
            CArgument::Struct(x) => x.conversion(),
        }
    }
}

impl LifetimeInfo for CReturnValue {
    fn rust_requires_lifetime(&self) -> bool {
        match self {
            CReturnValue::Basic(x) => x.rust_requires_lifetime(),
            CReturnValue::Struct(x) => x.rust_requires_lifetime(),
        }
    }

    fn c_requires_lifetime(&self) -> bool {
        match self {
            CReturnValue::Basic(x) => x.c_requires_lifetime(),
            CReturnValue::Struct(x) => x.c_requires_lifetime(),
        }
    }
}

impl RustType for CReturnValue {
    fn as_rust_type(&self) -> String {
        match self {
            CReturnValue::Basic(x) => x.as_rust_type(),
            CReturnValue::Struct(x) => x.as_rust_type(),
        }
    }

    fn as_c_type(&self) -> String {
        match self {
            CReturnValue::Basic(x) => x.as_c_type(),
            CReturnValue::Struct(x) => x.as_c_type(),
        }
    }

    fn is_copyable(&self) -> bool {
        match self {
            CReturnValue::Basic(x) => x.is_copyable(),
            CReturnValue::Struct(x) => x.is_copyable(),
        }
    }

    fn conversion(&self) -> Option<TypeConverter> {
        match self {
            CReturnValue::Basic(x) => x.conversion(),
            CReturnValue::Struct(x) => x.conversion(),
        }
    }
}

impl<T> LifetimeInfo for ReturnType<T> where T: LifetimeInfo {
    fn rust_requires_lifetime(&self) -> bool {
        if let Self::Type(t, _) = self {
            t.rust_requires_lifetime()
        } else {
            false
        }
    }

    fn c_requires_lifetime(&self) -> bool {
        if let Self::Type(t, _) = self {
            t.c_requires_lifetime()
        } else {
            false
        }
    }
}

impl<T> RustType for ReturnType<T> where  T: RustType
{
    fn as_rust_type(&self) -> String {
        if let Self::Type(t, _) = self {
            t.as_rust_type()
        } else {
            "()".to_string()
        }
    }

    fn as_c_type(&self) -> String {
        if let Self::Type(t, _) = self {
            t.as_c_type()
        } else {
            "()".to_string()
        }
    }

    fn is_copyable(&self) -> bool {
        if let Self::Type(t, _) = self {
            t.is_copyable()
        } else {
            true
        }
    }

    fn conversion(&self) -> Option<TypeConverter> {
        if let Self::Type(t, _) = self {
            t.conversion()
        } else {
            None
        }
    }
}



impl LifetimeInfo for FStructFieldType {
    fn rust_requires_lifetime(&self) -> bool {
        match self {
            FStructFieldType::Basic(x) => x.rust_requires_lifetime(),
            FStructFieldType::String(x) => x.rust_requires_lifetime(),
            FStructFieldType::Interface(x) => x.rust_requires_lifetime(),
            FStructFieldType::Collection(x) => x.rust_requires_lifetime(),
            FStructFieldType::Struct(x) => x.rust_requires_lifetime(),
        }
    }

    fn c_requires_lifetime(&self) -> bool {
        match self {
            FStructFieldType::Basic(x) => x.c_requires_lifetime(),
            FStructFieldType::String(x) => x.c_requires_lifetime(),
            FStructFieldType::Interface(x) => x.c_requires_lifetime(),
            FStructFieldType::Collection(x) => x.c_requires_lifetime(),
            FStructFieldType::Struct(x) => x.c_requires_lifetime(),
        }
    }
}

impl LifetimeInfo for CStructFieldType {
    fn rust_requires_lifetime(&self) -> bool {
        match self {
            CStructFieldType::Basic(x) => x.rust_requires_lifetime(),
            CStructFieldType::Iterator(x) => x.rust_requires_lifetime(),
            CStructFieldType::Struct(x) => x.rust_requires_lifetime(),
        }
    }

    fn c_requires_lifetime(&self) -> bool {
        match self {
            CStructFieldType::Basic(x) => x.c_requires_lifetime(),
            CStructFieldType::Iterator(x) => x.c_requires_lifetime(),
            CStructFieldType::Struct(x) => x.c_requires_lifetime(),
        }
    }
}

impl LifetimeInfo for RStructFieldType {
    fn rust_requires_lifetime(&self) -> bool {
        match self {
            RStructFieldType::Basic(x) => x.rust_requires_lifetime(),
            RStructFieldType::ClassRef(x) => x.rust_requires_lifetime(),
            RStructFieldType::Struct(x) => x.rust_requires_lifetime(),
        }
    }

    fn c_requires_lifetime(&self) -> bool {
        match self {
            RStructFieldType::Basic(x) => x.c_requires_lifetime(),
            RStructFieldType::ClassRef(x) => x.c_requires_lifetime(),
            RStructFieldType::Struct(x) => x.c_requires_lifetime(),
        }
    }
}

impl LifetimeInfo for UStructFieldType {
    fn rust_requires_lifetime(&self) -> bool {
        match self {
            UStructFieldType::Basic(x) => x.rust_requires_lifetime(),
            UStructFieldType::Struct(x) => x.rust_requires_lifetime(),
        }
    }

    fn c_requires_lifetime(&self) -> bool {
        match self {
            UStructFieldType::Basic(x) => x.c_requires_lifetime(),
            UStructFieldType::Struct(x) => x.c_requires_lifetime(),
        }
    }
}

impl LifetimeInfo for CallbackFunction {
    fn rust_requires_lifetime(&self) -> bool {
        self.arguments
            .iter()
            .any(|arg| arg.arg_type.rust_requires_lifetime())
    }

    fn c_requires_lifetime(&self) -> bool {
        self.arguments
            .iter()
            .any(|arg| arg.arg_type.c_requires_lifetime())
    }
}
