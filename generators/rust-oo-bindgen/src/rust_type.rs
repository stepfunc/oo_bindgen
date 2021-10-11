use heck::CamelCase;

use crate::type_converter::*;
use oo_bindgen::class::ClassDeclarationHandle;
use oo_bindgen::collection::CollectionHandle;
use oo_bindgen::function::{FArgument, FReturnValue};
use oo_bindgen::interface::*;
use oo_bindgen::iterator::IteratorHandle;
use oo_bindgen::return_type::ReturnType;
use oo_bindgen::structs::callback_struct::CallbackStructFieldType;
use oo_bindgen::structs::common::*;
use oo_bindgen::structs::function_return_struct::ReturnStructFieldType;
use oo_bindgen::structs::function_struct::FunctionArgStructFieldType;
use oo_bindgen::structs::univeral_struct::UniversalStructFieldType;
use oo_bindgen::types::*;

pub(crate) trait LifetimeInfo {
    fn rust_requires_lifetime(&self) -> bool;
    fn c_requires_lifetime(&self) -> bool;
}

pub(crate) trait RustType: LifetimeInfo {
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

impl<T> LifetimeInfo for Struct<T>
where
    T: StructFieldType,
{
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

impl<T> RustType for Struct<T>
where
    T: StructFieldType,
{
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

impl RustType for FunctionArgStructFieldType {
    fn as_rust_type(&self) -> String {
        match self {
            FunctionArgStructFieldType::Basic(x) => x.as_rust_type(),
            FunctionArgStructFieldType::String(x) => x.as_rust_type(),
            FunctionArgStructFieldType::Interface(x) => x.as_rust_type(),
            FunctionArgStructFieldType::Collection(x) => x.as_rust_type(),
            FunctionArgStructFieldType::Struct(x) => x.as_rust_type(),
        }
    }

    fn as_c_type(&self) -> String {
        match self {
            FunctionArgStructFieldType::Basic(x) => x.as_c_type(),
            FunctionArgStructFieldType::String(x) => x.as_c_type(),
            FunctionArgStructFieldType::Interface(x) => x.as_c_type(),
            FunctionArgStructFieldType::Collection(x) => x.as_c_type(),
            FunctionArgStructFieldType::Struct(x) => x.as_c_type(),
        }
    }

    fn is_copyable(&self) -> bool {
        match self {
            FunctionArgStructFieldType::Basic(x) => x.is_copyable(),
            FunctionArgStructFieldType::String(x) => x.is_copyable(),
            FunctionArgStructFieldType::Interface(x) => x.is_copyable(),
            FunctionArgStructFieldType::Collection(x) => x.is_copyable(),
            FunctionArgStructFieldType::Struct(x) => x.is_copyable(),
        }
    }

    fn conversion(&self) -> Option<TypeConverter> {
        match self {
            FunctionArgStructFieldType::Basic(x) => x.conversion(),
            FunctionArgStructFieldType::String(x) => x.conversion(),
            FunctionArgStructFieldType::Interface(x) => x.conversion(),
            FunctionArgStructFieldType::Collection(x) => x.conversion(),
            FunctionArgStructFieldType::Struct(x) => x.conversion(),
        }
    }
}

impl RustType for ReturnStructFieldType {
    fn as_rust_type(&self) -> String {
        match self {
            ReturnStructFieldType::Basic(x) => x.as_rust_type(),
            ReturnStructFieldType::ClassRef(x) => x.as_rust_type(),
            ReturnStructFieldType::Struct(x) => x.as_rust_type(),
        }
    }

    fn as_c_type(&self) -> String {
        match self {
            ReturnStructFieldType::Basic(x) => x.as_c_type(),
            ReturnStructFieldType::ClassRef(x) => x.as_c_type(),
            ReturnStructFieldType::Struct(x) => x.as_c_type(),
        }
    }

    fn is_copyable(&self) -> bool {
        match self {
            ReturnStructFieldType::Basic(x) => x.is_copyable(),
            ReturnStructFieldType::ClassRef(x) => x.is_copyable(),
            ReturnStructFieldType::Struct(x) => x.is_copyable(),
        }
    }

    fn conversion(&self) -> Option<TypeConverter> {
        match self {
            ReturnStructFieldType::Basic(x) => x.conversion(),
            ReturnStructFieldType::ClassRef(x) => x.conversion(),
            ReturnStructFieldType::Struct(x) => x.conversion(),
        }
    }
}

impl RustType for CallbackStructFieldType {
    fn as_rust_type(&self) -> String {
        match self {
            CallbackStructFieldType::Basic(x) => x.as_rust_type(),
            CallbackStructFieldType::Iterator(x) => x.as_rust_type(),
            CallbackStructFieldType::Struct(x) => x.as_rust_type(),
        }
    }

    fn as_c_type(&self) -> String {
        match self {
            CallbackStructFieldType::Basic(x) => x.as_c_type(),
            CallbackStructFieldType::Iterator(x) => x.as_c_type(),
            CallbackStructFieldType::Struct(x) => x.as_c_type(),
        }
    }

    fn is_copyable(&self) -> bool {
        match self {
            CallbackStructFieldType::Basic(x) => x.is_copyable(),
            CallbackStructFieldType::Iterator(x) => x.is_copyable(),
            CallbackStructFieldType::Struct(x) => x.is_copyable(),
        }
    }

    fn conversion(&self) -> Option<TypeConverter> {
        match self {
            CallbackStructFieldType::Basic(x) => x.conversion(),
            CallbackStructFieldType::Iterator(x) => x.conversion(),
            CallbackStructFieldType::Struct(x) => x.conversion(),
        }
    }
}

impl RustType for UniversalStructFieldType {
    fn as_rust_type(&self) -> String {
        match self {
            UniversalStructFieldType::Basic(x) => x.as_rust_type(),
            UniversalStructFieldType::Struct(x) => x.as_rust_type(),
        }
    }

    fn as_c_type(&self) -> String {
        match self {
            UniversalStructFieldType::Basic(x) => x.as_c_type(),
            UniversalStructFieldType::Struct(x) => x.as_c_type(),
        }
    }

    fn is_copyable(&self) -> bool {
        match self {
            UniversalStructFieldType::Basic(x) => x.is_copyable(),
            UniversalStructFieldType::Struct(x) => x.is_copyable(),
        }
    }

    fn conversion(&self) -> Option<TypeConverter> {
        match self {
            UniversalStructFieldType::Basic(x) => x.conversion(),
            UniversalStructFieldType::Struct(x) => x.conversion(),
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

impl<T> LifetimeInfo for ReturnType<T>
where
    T: LifetimeInfo,
{
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

impl<T> RustType for ReturnType<T>
where
    T: RustType,
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

impl LifetimeInfo for FunctionArgStructFieldType {
    fn rust_requires_lifetime(&self) -> bool {
        match self {
            FunctionArgStructFieldType::Basic(x) => x.rust_requires_lifetime(),
            FunctionArgStructFieldType::String(x) => x.rust_requires_lifetime(),
            FunctionArgStructFieldType::Interface(x) => x.rust_requires_lifetime(),
            FunctionArgStructFieldType::Collection(x) => x.rust_requires_lifetime(),
            FunctionArgStructFieldType::Struct(x) => x.rust_requires_lifetime(),
        }
    }

    fn c_requires_lifetime(&self) -> bool {
        match self {
            FunctionArgStructFieldType::Basic(x) => x.c_requires_lifetime(),
            FunctionArgStructFieldType::String(x) => x.c_requires_lifetime(),
            FunctionArgStructFieldType::Interface(x) => x.c_requires_lifetime(),
            FunctionArgStructFieldType::Collection(x) => x.c_requires_lifetime(),
            FunctionArgStructFieldType::Struct(x) => x.c_requires_lifetime(),
        }
    }
}

impl LifetimeInfo for CallbackStructFieldType {
    fn rust_requires_lifetime(&self) -> bool {
        match self {
            CallbackStructFieldType::Basic(x) => x.rust_requires_lifetime(),
            CallbackStructFieldType::Iterator(x) => x.rust_requires_lifetime(),
            CallbackStructFieldType::Struct(x) => x.rust_requires_lifetime(),
        }
    }

    fn c_requires_lifetime(&self) -> bool {
        match self {
            CallbackStructFieldType::Basic(x) => x.c_requires_lifetime(),
            CallbackStructFieldType::Iterator(x) => x.c_requires_lifetime(),
            CallbackStructFieldType::Struct(x) => x.c_requires_lifetime(),
        }
    }
}

impl LifetimeInfo for ReturnStructFieldType {
    fn rust_requires_lifetime(&self) -> bool {
        match self {
            ReturnStructFieldType::Basic(x) => x.rust_requires_lifetime(),
            ReturnStructFieldType::ClassRef(x) => x.rust_requires_lifetime(),
            ReturnStructFieldType::Struct(x) => x.rust_requires_lifetime(),
        }
    }

    fn c_requires_lifetime(&self) -> bool {
        match self {
            ReturnStructFieldType::Basic(x) => x.c_requires_lifetime(),
            ReturnStructFieldType::ClassRef(x) => x.c_requires_lifetime(),
            ReturnStructFieldType::Struct(x) => x.c_requires_lifetime(),
        }
    }
}

impl LifetimeInfo for UniversalStructFieldType {
    fn rust_requires_lifetime(&self) -> bool {
        match self {
            UniversalStructFieldType::Basic(x) => x.rust_requires_lifetime(),
            UniversalStructFieldType::Struct(x) => x.rust_requires_lifetime(),
        }
    }

    fn c_requires_lifetime(&self) -> bool {
        match self {
            UniversalStructFieldType::Basic(x) => x.c_requires_lifetime(),
            UniversalStructFieldType::Struct(x) => x.c_requires_lifetime(),
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
