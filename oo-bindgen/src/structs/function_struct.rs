use crate::collection::CollectionHandle;
use crate::structs::common::*;
use crate::types::{AnyType, DurationType};
use crate::*;

use std::time::Duration;

/// Types that can be used in a function struct, some of which might have a default value
#[derive(Clone, Debug)]
pub enum FStructFieldType {
    Bool(Option<bool>),
    Uint8(Option<u8>),
    Sint8(Option<i8>),
    Uint16(Option<u16>),
    Sint16(Option<i16>),
    Uint32(Option<u32>),
    Sint32(Option<i32>),
    Uint64(Option<u64>),
    Sint64(Option<i64>),
    Float(Option<f32>),
    Double(Option<f64>),
    String(Option<String>),
    Enum(EnumField),
    Interface(InterfaceHandle),
    Collection(CollectionHandle),
    Duration(DurationType, Option<Duration>),
    Struct(FStructHandle),
}

pub type FStructField = StructField<FStructFieldType>;
pub type FStruct = Struct<FStructFieldType>;
pub type FStructHandle = Handle<FStruct>;
pub type FStructBuilder<'a> = StructFieldBuilder<'a, FStructFieldType>;

impl StructFieldType for FStructFieldType {
    fn has_default(&self) -> bool {
        match self {
            Self::Bool(default) => default.is_some(),
            Self::Uint8(default) => default.is_some(),
            Self::Sint8(default) => default.is_some(),
            Self::Uint16(default) => default.is_some(),
            Self::Sint16(default) => default.is_some(),
            Self::Uint32(default) => default.is_some(),
            Self::Sint32(default) => default.is_some(),
            Self::Uint64(default) => default.is_some(),
            Self::Sint64(default) => default.is_some(),
            Self::Float(default) => default.is_some(),
            Self::Double(default) => default.is_some(),
            Self::String(default) => default.is_some(),
            Self::Enum(x) => x.default_variant.is_some(),
            Self::Interface(_) => false,
            Self::Collection(_) => false,
            Self::Duration(_, default) => default.is_some(),
            Self::Struct(x) => x.all_fields_have_defaults(),
        }
    }

    fn create_struct_type(v: Handle<Struct<FStructFieldType>>) -> StructType {
        StructType::FStruct(v.clone(), v.to_any_struct())
    }

    fn strings_allowed() -> bool {
        true
    }

    fn to_any_struct_field_type(self) -> AnyStructFieldType {
        match self {
            Self::Bool(x) => AnyStructFieldType::Bool(x),
            Self::Uint8(x) => AnyStructFieldType::Uint8(x),
            Self::Sint8(x) => AnyStructFieldType::Sint8(x),
            Self::Uint16(x) => AnyStructFieldType::Uint16(x),
            Self::Sint16(x) => AnyStructFieldType::Sint16(x),
            Self::Uint32(x) => AnyStructFieldType::Uint32(x),
            Self::Sint32(x) => AnyStructFieldType::Sint32(x),
            Self::Uint64(x) => AnyStructFieldType::Uint64(x),
            Self::Sint64(x) => AnyStructFieldType::Sint64(x),
            Self::Float(x) => AnyStructFieldType::Float(x),
            Self::Double(x) => AnyStructFieldType::Double(x),
            Self::String(x) => AnyStructFieldType::String(x),
            Self::Enum(x) => AnyStructFieldType::Enum(x),
            Self::Interface(handle) => AnyStructFieldType::Interface(handle),
            Self::Collection(handle) => AnyStructFieldType::Collection(handle),
            Self::Duration(t, x) => AnyStructFieldType::Duration(t, x),
            Self::Struct(x) => AnyStructFieldType::Struct(x.to_any_struct()),
        }
    }

    fn to_any_type(&self) -> AnyType {
        match self {
            FStructFieldType::Bool(_) => BasicType::Bool.into(),
            FStructFieldType::Uint8(_) => BasicType::Uint8.into(),
            FStructFieldType::Sint8(_) => BasicType::Sint8.into(),
            FStructFieldType::Uint16(_) => BasicType::Uint16.into(),
            FStructFieldType::Sint16(_) => BasicType::Sint16.into(),
            FStructFieldType::Uint32(_) => BasicType::Uint32.into(),
            FStructFieldType::Sint32(_) => BasicType::Sint32.into(),
            FStructFieldType::Uint64(_) => BasicType::Uint64.into(),
            FStructFieldType::Sint64(_) => BasicType::Uint64.into(),
            FStructFieldType::Float(_) => BasicType::Float.into(),
            FStructFieldType::Double(_) => BasicType::Double.into(),
            FStructFieldType::String(_) => AnyType::String,
            FStructFieldType::Enum(x) => BasicType::Enum(x.handle.clone()).into(),
            FStructFieldType::Interface(x) => AnyType::Interface(x.clone()),
            FStructFieldType::Collection(x) => AnyType::Collection(x.clone()),
            FStructFieldType::Duration(x, _) => BasicType::Duration(*x).into(),
            FStructFieldType::Struct(x) => AnyType::Struct(x.to_any_struct()),
        }
    }
}

impl From<FStructHandle> for FStructFieldType {
    fn from(x: FStructHandle) -> Self {
        FStructFieldType::Struct(x)
    }
}

impl From<InterfaceHandle> for FStructFieldType {
    fn from(x: InterfaceHandle) -> Self {
        FStructFieldType::Interface(x)
    }
}

impl From<EnumField> for FStructFieldType {
    fn from(x: EnumField) -> Self {
        Self::Enum(x)
    }
}
