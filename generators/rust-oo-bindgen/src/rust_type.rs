use heck::CamelCase;

use crate::type_converter::*;
use oo_bindgen::class::ClassDeclarationHandle;
use oo_bindgen::collection::CollectionHandle;
use oo_bindgen::function::{FArgument, FunctionReturnValue};
use oo_bindgen::interface::*;
use oo_bindgen::iterator::IteratorHandle;
use oo_bindgen::return_type::ReturnType;
use oo_bindgen::structs::*;
use oo_bindgen::types::*;
use oo_bindgen::UniversalOr;

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
            Self::U8 => "u8".to_string(),
            Self::S8 => "i8".to_string(),
            Self::U16 => "u16".to_string(),
            Self::S16 => "i16".to_string(),
            Self::U32 => "u32".to_string(),
            Self::S32 => "i32".to_string(),
            Self::U64 => "u64".to_string(),
            Self::S64 => "i64".to_string(),
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
            Self::U8 => None,
            Self::S8 => None,
            Self::U16 => None,
            Self::S16 => None,
            Self::U32 => None,
            Self::S32 => None,
            Self::U64 => None,
            Self::S64 => None,
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

impl<T> LifetimeInfo for UniversalOr<T>
where
    T: StructFieldType + LifetimeInfo,
{
    fn rust_requires_lifetime(&self) -> bool {
        match self {
            UniversalOr::Specific(x) => x.rust_requires_lifetime(),
            UniversalOr::Universal(x) => x.rust_requires_lifetime(),
        }
    }

    fn c_requires_lifetime(&self) -> bool {
        match self {
            UniversalOr::Specific(x) => x.c_requires_lifetime(),
            UniversalOr::Universal(x) => x.c_requires_lifetime(),
        }
    }
}

impl<T> LifetimeInfo for Struct<T>
where
    T: StructFieldType + LifetimeInfo,
{
    fn rust_requires_lifetime(&self) -> bool {
        self.fields
            .iter()
            .any(|f| f.field_type.rust_requires_lifetime())
    }

    fn c_requires_lifetime(&self) -> bool {
        self.fields
            .iter()
            .any(|f| f.field_type.c_requires_lifetime())
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

impl<T> RustType for UniversalOr<T>
where
    T: StructFieldType + LifetimeInfo,
{
    fn as_rust_type(&self) -> String {
        match self {
            UniversalOr::Specific(x) => x.as_rust_type(),
            UniversalOr::Universal(x) => x.as_rust_type(),
        }
    }

    fn as_c_type(&self) -> String {
        match self {
            UniversalOr::Specific(x) => x.as_c_type(),
            UniversalOr::Universal(x) => x.as_c_type(),
        }
    }

    fn is_copyable(&self) -> bool {
        match self {
            UniversalOr::Specific(x) => x.is_copyable(),
            UniversalOr::Universal(x) => x.is_copyable(),
        }
    }

    fn conversion(&self) -> Option<TypeConverter> {
        match self {
            UniversalOr::Specific(x) => x.conversion(),
            UniversalOr::Universal(x) => x.conversion(),
        }
    }
}

impl<T> RustType for Struct<T>
where
    T: StructFieldType + LifetimeInfo,
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

impl LifetimeInfo for FunctionReturnValue {
    fn rust_requires_lifetime(&self) -> bool {
        match self {
            FunctionReturnValue::Basic(x) => x.rust_requires_lifetime(),
            FunctionReturnValue::String(x) => x.rust_requires_lifetime(),
            FunctionReturnValue::ClassRef(x) => x.rust_requires_lifetime(),
            FunctionReturnValue::Struct(x) => x.rust_requires_lifetime(),
            FunctionReturnValue::StructRef(x) => x.rust_requires_lifetime(),
        }
    }

    fn c_requires_lifetime(&self) -> bool {
        match self {
            FunctionReturnValue::Basic(x) => x.c_requires_lifetime(),
            FunctionReturnValue::String(x) => x.c_requires_lifetime(),
            FunctionReturnValue::ClassRef(x) => x.c_requires_lifetime(),
            FunctionReturnValue::Struct(x) => x.c_requires_lifetime(),
            FunctionReturnValue::StructRef(x) => x.c_requires_lifetime(),
        }
    }
}

impl RustType for FunctionReturnValue {
    fn as_rust_type(&self) -> String {
        match self {
            FunctionReturnValue::Basic(x) => x.as_rust_type(),
            FunctionReturnValue::String(x) => x.as_rust_type(),
            FunctionReturnValue::ClassRef(x) => x.as_rust_type(),
            FunctionReturnValue::Struct(x) => x.as_rust_type(),
            FunctionReturnValue::StructRef(x) => x.as_rust_type(),
        }
    }

    fn as_c_type(&self) -> String {
        match self {
            FunctionReturnValue::Basic(x) => x.as_c_type(),
            FunctionReturnValue::String(x) => x.as_c_type(),
            FunctionReturnValue::ClassRef(x) => x.as_c_type(),
            FunctionReturnValue::Struct(x) => x.as_c_type(),
            FunctionReturnValue::StructRef(x) => x.as_c_type(),
        }
    }

    fn is_copyable(&self) -> bool {
        match self {
            FunctionReturnValue::Basic(x) => x.is_copyable(),
            FunctionReturnValue::String(x) => x.is_copyable(),
            FunctionReturnValue::ClassRef(x) => x.is_copyable(),
            FunctionReturnValue::Struct(x) => x.is_copyable(),
            FunctionReturnValue::StructRef(x) => x.is_copyable(),
        }
    }

    fn conversion(&self) -> Option<TypeConverter> {
        match self {
            FunctionReturnValue::Basic(x) => x.conversion(),
            FunctionReturnValue::String(x) => x.conversion(),
            FunctionReturnValue::ClassRef(x) => x.conversion(),
            FunctionReturnValue::Struct(x) => x.conversion(),
            FunctionReturnValue::StructRef(x) => x.conversion(),
        }
    }
}

impl RustType for FunctionArgStructField {
    fn as_rust_type(&self) -> String {
        match self {
            FunctionArgStructField::Basic(x) => x.as_rust_type(),
            FunctionArgStructField::String(x) => x.as_rust_type(),
            FunctionArgStructField::Interface(x) => x.as_rust_type(),
            FunctionArgStructField::Collection(x) => x.as_rust_type(),
            FunctionArgStructField::Struct(x) => x.as_rust_type(),
        }
    }

    fn as_c_type(&self) -> String {
        match self {
            FunctionArgStructField::Basic(x) => x.as_c_type(),
            FunctionArgStructField::String(x) => x.as_c_type(),
            FunctionArgStructField::Interface(x) => x.as_c_type(),
            FunctionArgStructField::Collection(x) => x.as_c_type(),
            FunctionArgStructField::Struct(x) => x.as_c_type(),
        }
    }

    fn is_copyable(&self) -> bool {
        match self {
            FunctionArgStructField::Basic(x) => x.is_copyable(),
            FunctionArgStructField::String(x) => x.is_copyable(),
            FunctionArgStructField::Interface(x) => x.is_copyable(),
            FunctionArgStructField::Collection(x) => x.is_copyable(),
            FunctionArgStructField::Struct(x) => x.is_copyable(),
        }
    }

    fn conversion(&self) -> Option<TypeConverter> {
        match self {
            FunctionArgStructField::Basic(x) => x.conversion(),
            FunctionArgStructField::String(x) => x.conversion(),
            FunctionArgStructField::Interface(x) => x.conversion(),
            FunctionArgStructField::Collection(x) => x.conversion(),
            FunctionArgStructField::Struct(x) => x.conversion(),
        }
    }
}

impl RustType for FunctionReturnStructField {
    fn as_rust_type(&self) -> String {
        match self {
            Self::Basic(x) => x.as_rust_type(),
            Self::ClassRef(x) => x.as_rust_type(),
            Self::Struct(x) => x.as_rust_type(),
            Self::Iterator(x) => x.as_rust_type(),
        }
    }

    fn as_c_type(&self) -> String {
        match self {
            Self::Basic(x) => x.as_c_type(),
            Self::ClassRef(x) => x.as_c_type(),
            Self::Struct(x) => x.as_c_type(),
            Self::Iterator(x) => x.as_c_type(),
        }
    }

    fn is_copyable(&self) -> bool {
        match self {
            Self::Basic(x) => x.is_copyable(),
            Self::ClassRef(x) => x.is_copyable(),
            Self::Struct(x) => x.is_copyable(),
            Self::Iterator(x) => x.is_copyable(),
        }
    }

    fn conversion(&self) -> Option<TypeConverter> {
        match self {
            Self::Basic(x) => x.conversion(),
            Self::ClassRef(x) => x.conversion(),
            Self::Struct(x) => x.conversion(),
            Self::Iterator(x) => x.conversion(),
        }
    }
}

impl RustType for CallbackArgStructField {
    fn as_rust_type(&self) -> String {
        match self {
            CallbackArgStructField::Basic(x) => x.as_rust_type(),
            CallbackArgStructField::Iterator(x) => x.as_rust_type(),
            CallbackArgStructField::Struct(x) => x.as_rust_type(),
        }
    }

    fn as_c_type(&self) -> String {
        match self {
            CallbackArgStructField::Basic(x) => x.as_c_type(),
            CallbackArgStructField::Iterator(x) => x.as_c_type(),
            CallbackArgStructField::Struct(x) => x.as_c_type(),
        }
    }

    fn is_copyable(&self) -> bool {
        match self {
            CallbackArgStructField::Basic(x) => x.is_copyable(),
            CallbackArgStructField::Iterator(x) => x.is_copyable(),
            CallbackArgStructField::Struct(x) => x.is_copyable(),
        }
    }

    fn conversion(&self) -> Option<TypeConverter> {
        match self {
            CallbackArgStructField::Basic(x) => x.conversion(),
            CallbackArgStructField::Iterator(x) => x.conversion(),
            CallbackArgStructField::Struct(x) => x.conversion(),
        }
    }
}

impl RustType for UniversalStructField {
    fn as_rust_type(&self) -> String {
        match self {
            UniversalStructField::Basic(x) => x.as_rust_type(),
            UniversalStructField::Struct(x) => x.as_rust_type(),
        }
    }

    fn as_c_type(&self) -> String {
        match self {
            UniversalStructField::Basic(x) => x.as_c_type(),
            UniversalStructField::Struct(x) => x.as_c_type(),
        }
    }

    fn is_copyable(&self) -> bool {
        match self {
            UniversalStructField::Basic(x) => x.is_copyable(),
            UniversalStructField::Struct(x) => x.is_copyable(),
        }
    }

    fn conversion(&self) -> Option<TypeConverter> {
        match self {
            UniversalStructField::Basic(x) => x.conversion(),
            UniversalStructField::Struct(x) => x.conversion(),
        }
    }
}

impl LifetimeInfo for CallbackArgument {
    fn rust_requires_lifetime(&self) -> bool {
        match self {
            CallbackArgument::Basic(x) => x.rust_requires_lifetime(),
            CallbackArgument::String(x) => x.rust_requires_lifetime(),
            CallbackArgument::Iterator(x) => x.rust_requires_lifetime(),
            CallbackArgument::Struct(x) => x.rust_requires_lifetime(),
            CallbackArgument::Class(x) => x.rust_requires_lifetime(),
        }
    }

    fn c_requires_lifetime(&self) -> bool {
        match self {
            CallbackArgument::Basic(x) => x.c_requires_lifetime(),
            CallbackArgument::String(x) => x.c_requires_lifetime(),
            CallbackArgument::Iterator(x) => x.c_requires_lifetime(),
            CallbackArgument::Struct(x) => x.c_requires_lifetime(),
            CallbackArgument::Class(x) => x.c_requires_lifetime(),
        }
    }
}

impl RustType for CallbackArgument {
    fn as_rust_type(&self) -> String {
        match self {
            CallbackArgument::Basic(x) => x.as_rust_type(),
            CallbackArgument::String(x) => x.as_rust_type(),
            CallbackArgument::Iterator(x) => x.as_rust_type(),
            CallbackArgument::Struct(x) => x.as_rust_type(),
            CallbackArgument::Class(x) => x.as_rust_type(),
        }
    }

    fn as_c_type(&self) -> String {
        match self {
            CallbackArgument::Basic(x) => x.as_c_type(),
            CallbackArgument::String(x) => x.as_c_type(),
            CallbackArgument::Iterator(x) => x.as_c_type(),
            CallbackArgument::Struct(x) => x.as_c_type(),
            CallbackArgument::Class(x) => x.as_c_type(),
        }
    }

    fn is_copyable(&self) -> bool {
        match self {
            CallbackArgument::Basic(x) => x.is_copyable(),
            CallbackArgument::String(x) => x.is_copyable(),
            CallbackArgument::Iterator(x) => x.is_copyable(),
            CallbackArgument::Struct(x) => x.is_copyable(),
            CallbackArgument::Class(x) => x.is_copyable(),
        }
    }

    fn conversion(&self) -> Option<TypeConverter> {
        match self {
            CallbackArgument::Basic(x) => x.conversion(),
            CallbackArgument::String(x) => x.conversion(),
            CallbackArgument::Iterator(x) => x.conversion(),
            CallbackArgument::Struct(x) => x.conversion(),
            CallbackArgument::Class(x) => x.conversion(),
        }
    }
}

impl LifetimeInfo for CallbackReturnValue {
    fn rust_requires_lifetime(&self) -> bool {
        match self {
            CallbackReturnValue::Basic(x) => x.rust_requires_lifetime(),
            CallbackReturnValue::Struct(x) => x.rust_requires_lifetime(),
        }
    }

    fn c_requires_lifetime(&self) -> bool {
        match self {
            CallbackReturnValue::Basic(x) => x.c_requires_lifetime(),
            CallbackReturnValue::Struct(x) => x.c_requires_lifetime(),
        }
    }
}

impl RustType for CallbackReturnValue {
    fn as_rust_type(&self) -> String {
        match self {
            CallbackReturnValue::Basic(x) => x.as_rust_type(),
            CallbackReturnValue::Struct(x) => x.as_rust_type(),
        }
    }

    fn as_c_type(&self) -> String {
        match self {
            CallbackReturnValue::Basic(x) => x.as_c_type(),
            CallbackReturnValue::Struct(x) => x.as_c_type(),
        }
    }

    fn is_copyable(&self) -> bool {
        match self {
            CallbackReturnValue::Basic(x) => x.is_copyable(),
            CallbackReturnValue::Struct(x) => x.is_copyable(),
        }
    }

    fn conversion(&self) -> Option<TypeConverter> {
        match self {
            CallbackReturnValue::Basic(x) => x.conversion(),
            CallbackReturnValue::Struct(x) => x.conversion(),
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

impl LifetimeInfo for FunctionArgStructField {
    fn rust_requires_lifetime(&self) -> bool {
        match self {
            FunctionArgStructField::Basic(x) => x.rust_requires_lifetime(),
            FunctionArgStructField::String(x) => x.rust_requires_lifetime(),
            FunctionArgStructField::Interface(x) => x.rust_requires_lifetime(),
            FunctionArgStructField::Collection(x) => x.rust_requires_lifetime(),
            FunctionArgStructField::Struct(x) => x.rust_requires_lifetime(),
        }
    }

    fn c_requires_lifetime(&self) -> bool {
        match self {
            FunctionArgStructField::Basic(x) => x.c_requires_lifetime(),
            FunctionArgStructField::String(x) => x.c_requires_lifetime(),
            FunctionArgStructField::Interface(x) => x.c_requires_lifetime(),
            FunctionArgStructField::Collection(x) => x.c_requires_lifetime(),
            FunctionArgStructField::Struct(x) => x.c_requires_lifetime(),
        }
    }
}

impl LifetimeInfo for CallbackArgStructField {
    fn rust_requires_lifetime(&self) -> bool {
        match self {
            CallbackArgStructField::Basic(x) => x.rust_requires_lifetime(),
            CallbackArgStructField::Iterator(x) => x.rust_requires_lifetime(),
            CallbackArgStructField::Struct(x) => x.rust_requires_lifetime(),
        }
    }

    fn c_requires_lifetime(&self) -> bool {
        match self {
            CallbackArgStructField::Basic(x) => x.c_requires_lifetime(),
            CallbackArgStructField::Iterator(x) => x.c_requires_lifetime(),
            CallbackArgStructField::Struct(x) => x.c_requires_lifetime(),
        }
    }
}

impl LifetimeInfo for FunctionReturnStructField {
    fn rust_requires_lifetime(&self) -> bool {
        match self {
            Self::Basic(x) => x.rust_requires_lifetime(),
            Self::ClassRef(x) => x.rust_requires_lifetime(),
            Self::Struct(x) => x.rust_requires_lifetime(),
            Self::Iterator(x) => x.rust_requires_lifetime(),
        }
    }

    fn c_requires_lifetime(&self) -> bool {
        match self {
            Self::Basic(x) => x.c_requires_lifetime(),
            Self::ClassRef(x) => x.c_requires_lifetime(),
            Self::Struct(x) => x.c_requires_lifetime(),
            Self::Iterator(x) => x.c_requires_lifetime(),
        }
    }
}

impl LifetimeInfo for UniversalStructField {
    fn rust_requires_lifetime(&self) -> bool {
        match self {
            UniversalStructField::Basic(x) => x.rust_requires_lifetime(),
            UniversalStructField::Struct(x) => x.rust_requires_lifetime(),
        }
    }

    fn c_requires_lifetime(&self) -> bool {
        match self {
            UniversalStructField::Basic(x) => x.c_requires_lifetime(),
            UniversalStructField::Struct(x) => x.c_requires_lifetime(),
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
