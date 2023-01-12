use crate::backend::rust::type_converter::TypeConverter;
use heck::ToUpperCamelCase;

use crate::model::*;

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

impl LifetimeInfo for Primitive {
    fn rust_requires_lifetime(&self) -> bool {
        false
    }

    fn c_requires_lifetime(&self) -> bool {
        false
    }
}

impl RustType for Primitive {
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
            Self::Float => "f32".to_string(),
            Self::Double => "f64".to_string(),
        }
    }

    fn as_c_type(&self) -> String {
        self.get_c_rust_type().to_string()
    }

    fn is_copyable(&self) -> bool {
        true
    }

    fn conversion(&self) -> Option<TypeConverter> {
        None
    }
}

impl RustType for BasicType {
    fn as_rust_type(&self) -> String {
        match self {
            Self::Primitive(x) => x.as_rust_type(),
            Self::Duration(_) => "std::time::Duration".to_string(),
            Self::Enum(handle) => handle.name.to_upper_camel_case(),
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
            Self::Primitive(x) => x.conversion(),
            Self::Duration(x) => Some(TypeConverter::Duration(*x)),
            Self::Enum(x) => Some(TypeConverter::UnvalidatedEnum(x.clone())),
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

impl<D> LifetimeInfo for Handle<Collection<D>>
where
    D: DocReference,
{
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

impl<T, D> LifetimeInfo for Struct<T, D>
where
    D: DocReference,
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

impl<D> LifetimeInfo for Handle<Interface<D>>
where
    D: DocReference,
{
    fn rust_requires_lifetime(&self) -> bool {
        false
    }

    fn c_requires_lifetime(&self) -> bool {
        false
    }
}

impl<D> LifetimeInfo for Handle<AbstractIterator<D>>
where
    D: DocReference,
{
    fn rust_requires_lifetime(&self) -> bool {
        self.has_lifetime_annotation
    }

    fn c_requires_lifetime(&self) -> bool {
        self.has_lifetime_annotation
    }
}

impl LifetimeInfo for FunctionArgument {
    fn rust_requires_lifetime(&self) -> bool {
        match self {
            FunctionArgument::Basic(x) => x.rust_requires_lifetime(),
            FunctionArgument::String(x) => x.rust_requires_lifetime(),
            FunctionArgument::Collection(x) => x.rust_requires_lifetime(),
            FunctionArgument::Struct(x) => x.rust_requires_lifetime(),
            FunctionArgument::StructRef(x) => x.inner.rust_requires_lifetime(),
            FunctionArgument::ClassRef(x) => x.rust_requires_lifetime(),
            FunctionArgument::Interface(x) => x.rust_requires_lifetime(),
        }
    }

    fn c_requires_lifetime(&self) -> bool {
        match self {
            FunctionArgument::Basic(x) => x.c_requires_lifetime(),
            FunctionArgument::String(x) => x.c_requires_lifetime(),
            FunctionArgument::Collection(x) => x.c_requires_lifetime(),
            FunctionArgument::Struct(x) => x.c_requires_lifetime(),
            FunctionArgument::StructRef(x) => x.inner.c_requires_lifetime(),
            FunctionArgument::ClassRef(x) => x.c_requires_lifetime(),
            FunctionArgument::Interface(x) => x.c_requires_lifetime(),
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

impl<D> RustType for Handle<Collection<D>>
where
    D: DocReference,
{
    fn as_rust_type(&self) -> String {
        format!("*mut crate::{}", self.name().to_upper_camel_case())
    }

    fn as_c_type(&self) -> String {
        format!("*mut crate::{}", self.name().to_upper_camel_case())
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

impl<T, D> RustType for Struct<T, D>
where
    D: DocReference,
    T: StructFieldType + LifetimeInfo,
{
    fn as_rust_type(&self) -> String {
        self.name().to_upper_camel_case()
    }

    fn as_c_type(&self) -> String {
        self.name().to_upper_camel_case()
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
        format!("Option<&{}>", self.name.to_upper_camel_case())
    }

    fn as_c_type(&self) -> String {
        format!("*const {}", self.name.to_upper_camel_case())
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
        format!("*mut crate::{}", self.name.to_upper_camel_case())
    }

    fn as_c_type(&self) -> String {
        format!("*mut crate::{}", self.name.to_upper_camel_case())
    }

    fn is_copyable(&self) -> bool {
        // Just copying the opaque pointer
        true
    }

    fn conversion(&self) -> Option<TypeConverter> {
        None
    }
}

impl<D> RustType for Handle<Interface<D>>
where
    D: DocReference,
{
    fn as_rust_type(&self) -> String {
        self.name.to_upper_camel_case()
    }

    fn as_c_type(&self) -> String {
        self.name.to_upper_camel_case()
    }

    fn is_copyable(&self) -> bool {
        false
    }

    fn conversion(&self) -> Option<TypeConverter> {
        match self.mode {
            InterfaceCategory::Synchronous => None,
            InterfaceCategory::Asynchronous => None,
            InterfaceCategory::Future => Some(TypeConverter::FutureInterface),
        }
    }
}

impl<D> RustType for Handle<AbstractIterator<D>>
where
    D: DocReference,
{
    fn as_rust_type(&self) -> String {
        let lifetime = if self.has_lifetime_annotation {
            "<'a>"
        } else {
            ""
        };
        format!(
            "*mut crate::{}{}",
            self.name().to_upper_camel_case(),
            lifetime
        )
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

impl RustType for FunctionArgument {
    fn as_rust_type(&self) -> String {
        match self {
            FunctionArgument::Basic(x) => x.as_rust_type(),
            FunctionArgument::String(x) => x.as_rust_type(),
            FunctionArgument::Collection(x) => x.as_rust_type(),
            FunctionArgument::Struct(x) => x.as_rust_type(),
            FunctionArgument::StructRef(x) => x.inner.as_rust_type(),
            FunctionArgument::ClassRef(x) => x.as_rust_type(),
            FunctionArgument::Interface(x) => x.as_rust_type(),
        }
    }

    fn as_c_type(&self) -> String {
        match self {
            FunctionArgument::Basic(x) => x.as_c_type(),
            FunctionArgument::String(x) => x.as_c_type(),
            FunctionArgument::Collection(x) => x.as_c_type(),
            FunctionArgument::Struct(x) => x.as_c_type(),
            FunctionArgument::StructRef(x) => x.inner.as_c_type(),
            FunctionArgument::ClassRef(x) => x.as_c_type(),
            FunctionArgument::Interface(x) => x.as_c_type(),
        }
    }

    fn is_copyable(&self) -> bool {
        match self {
            FunctionArgument::Basic(x) => x.is_copyable(),
            FunctionArgument::String(x) => x.is_copyable(),
            FunctionArgument::Collection(x) => x.is_copyable(),
            FunctionArgument::Struct(x) => x.is_copyable(),
            FunctionArgument::StructRef(x) => x.inner.is_copyable(),
            FunctionArgument::ClassRef(x) => x.is_copyable(),
            FunctionArgument::Interface(x) => x.is_copyable(),
        }
    }

    fn conversion(&self) -> Option<TypeConverter> {
        match self {
            FunctionArgument::Basic(x) => x.conversion(),
            FunctionArgument::String(x) => x.conversion(),
            FunctionArgument::Collection(x) => x.conversion(),
            FunctionArgument::Struct(x) => x.conversion(),
            FunctionArgument::StructRef(x) => x.inner.conversion(),
            FunctionArgument::ClassRef(x) => x.conversion(),
            FunctionArgument::Interface(x) => x.conversion(),
        }
    }
}

impl LifetimeInfo for PrimitiveRef {
    fn rust_requires_lifetime(&self) -> bool {
        false
    }

    fn c_requires_lifetime(&self) -> bool {
        false
    }
}

impl LifetimeInfo for FunctionReturnValue {
    fn rust_requires_lifetime(&self) -> bool {
        match self {
            FunctionReturnValue::Basic(x) => x.rust_requires_lifetime(),
            FunctionReturnValue::String(x) => x.rust_requires_lifetime(),
            FunctionReturnValue::ClassRef(x) => x.rust_requires_lifetime(),
            FunctionReturnValue::Struct(x) => x.rust_requires_lifetime(),
            FunctionReturnValue::StructRef(x) => x.untyped().rust_requires_lifetime(),
            FunctionReturnValue::PrimitiveRef(x) => x.rust_requires_lifetime(),
        }
    }

    fn c_requires_lifetime(&self) -> bool {
        match self {
            FunctionReturnValue::Basic(x) => x.c_requires_lifetime(),
            FunctionReturnValue::String(x) => x.c_requires_lifetime(),
            FunctionReturnValue::ClassRef(x) => x.c_requires_lifetime(),
            FunctionReturnValue::Struct(x) => x.c_requires_lifetime(),
            FunctionReturnValue::StructRef(x) => x.untyped().c_requires_lifetime(),
            FunctionReturnValue::PrimitiveRef(x) => x.c_requires_lifetime(),
        }
    }
}

impl RustType for PrimitiveRef {
    fn as_rust_type(&self) -> String {
        format!("*const {}", self.inner.as_rust_type())
    }

    fn as_c_type(&self) -> String {
        format!("*const {}", self.inner.as_rust_type())
    }

    fn is_copyable(&self) -> bool {
        true
    }

    fn conversion(&self) -> Option<TypeConverter> {
        None
    }
}

impl RustType for FunctionReturnValue {
    fn as_rust_type(&self) -> String {
        match self {
            FunctionReturnValue::Basic(x) => x.as_rust_type(),
            FunctionReturnValue::String(x) => x.as_rust_type(),
            FunctionReturnValue::ClassRef(x) => x.as_rust_type(),
            FunctionReturnValue::Struct(x) => x.as_rust_type(),
            FunctionReturnValue::StructRef(x) => x.untyped().as_rust_type(),
            FunctionReturnValue::PrimitiveRef(x) => x.as_rust_type(),
        }
    }

    fn as_c_type(&self) -> String {
        match self {
            FunctionReturnValue::Basic(x) => x.as_c_type(),
            FunctionReturnValue::String(x) => x.as_c_type(),
            FunctionReturnValue::ClassRef(x) => x.as_c_type(),
            FunctionReturnValue::Struct(x) => x.as_c_type(),
            FunctionReturnValue::StructRef(x) => x.untyped().as_c_type(),
            FunctionReturnValue::PrimitiveRef(x) => x.as_c_type(),
        }
    }

    fn is_copyable(&self) -> bool {
        match self {
            FunctionReturnValue::Basic(x) => x.is_copyable(),
            FunctionReturnValue::String(x) => x.is_copyable(),
            FunctionReturnValue::ClassRef(x) => x.is_copyable(),
            FunctionReturnValue::Struct(x) => x.is_copyable(),
            FunctionReturnValue::StructRef(x) => x.untyped().is_copyable(),
            FunctionReturnValue::PrimitiveRef(x) => x.is_copyable(),
        }
    }

    fn conversion(&self) -> Option<TypeConverter> {
        match self {
            FunctionReturnValue::Basic(x) => x.conversion(),
            FunctionReturnValue::String(x) => x.conversion(),
            FunctionReturnValue::ClassRef(x) => x.conversion(),
            FunctionReturnValue::Struct(x) => x.conversion(),
            FunctionReturnValue::StructRef(x) => x.untyped().conversion(),
            FunctionReturnValue::PrimitiveRef(x) => x.conversion(),
        }
    }
}

impl RustType for FunctionArgStructField {
    fn as_rust_type(&self) -> String {
        match self {
            FunctionArgStructField::Basic(x) => x.as_rust_type(),
            FunctionArgStructField::String(x) => x.as_rust_type(),
            FunctionArgStructField::Interface(x) => x.inner.as_rust_type(),
            FunctionArgStructField::Struct(x) => x.as_rust_type(),
        }
    }

    fn as_c_type(&self) -> String {
        match self {
            FunctionArgStructField::Basic(x) => x.as_c_type(),
            FunctionArgStructField::String(x) => x.as_c_type(),
            FunctionArgStructField::Interface(x) => x.inner.as_c_type(),
            FunctionArgStructField::Struct(x) => x.as_c_type(),
        }
    }

    fn is_copyable(&self) -> bool {
        match self {
            FunctionArgStructField::Basic(x) => x.is_copyable(),
            FunctionArgStructField::String(x) => x.is_copyable(),
            FunctionArgStructField::Interface(x) => x.inner.is_copyable(),
            FunctionArgStructField::Struct(x) => x.is_copyable(),
        }
    }

    fn conversion(&self) -> Option<TypeConverter> {
        match self {
            FunctionArgStructField::Basic(x) => x.conversion(),
            FunctionArgStructField::String(x) => x.conversion(),
            FunctionArgStructField::Interface(x) => x.inner.conversion(),
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

impl<T, D> LifetimeInfo for OptionalReturnType<T, D>
where
    D: DocReference,
    T: Clone + LifetimeInfo,
{
    fn rust_requires_lifetime(&self) -> bool {
        if let Some(v) = self.get_value() {
            v.rust_requires_lifetime()
        } else {
            false
        }
    }

    fn c_requires_lifetime(&self) -> bool {
        if let Some(v) = self.get_value() {
            v.c_requires_lifetime()
        } else {
            false
        }
    }
}

impl<T, D> RustType for OptionalReturnType<T, D>
where
    D: DocReference,
    T: Clone + RustType,
{
    fn as_rust_type(&self) -> String {
        if let Some(v) = self.get_value() {
            v.as_rust_type()
        } else {
            "()".to_string()
        }
    }

    fn as_c_type(&self) -> String {
        if let Some(v) = self.get_value() {
            v.as_c_type()
        } else {
            "()".to_string()
        }
    }

    fn is_copyable(&self) -> bool {
        if let Some(v) = self.get_value() {
            v.is_copyable()
        } else {
            true
        }
    }

    fn conversion(&self) -> Option<TypeConverter> {
        if let Some(v) = self.get_value() {
            v.conversion()
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
            FunctionArgStructField::Interface(x) => x.inner.rust_requires_lifetime(),
            FunctionArgStructField::Struct(x) => x.rust_requires_lifetime(),
        }
    }

    fn c_requires_lifetime(&self) -> bool {
        match self {
            FunctionArgStructField::Basic(x) => x.c_requires_lifetime(),
            FunctionArgStructField::String(x) => x.c_requires_lifetime(),
            FunctionArgStructField::Interface(x) => x.inner.c_requires_lifetime(),
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

impl LifetimeInfo for CallbackFunction<Validated> {
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
