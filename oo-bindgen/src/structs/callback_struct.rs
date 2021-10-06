use crate::structs::common::*;
use crate::types::{AnyType, DurationType};
use crate::*;

use crate::iterator::IteratorHandle;
use std::time::Duration;

/// Types that can be used in a callback struct, some of which might have a default value
#[derive(Clone, Debug)]
pub enum CStructFieldType {
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
    Enum(EnumField),
    Iterator(IteratorHandle),
    Duration(DurationType, Option<Duration>),
    Struct(CStructHandle),
}

pub type CStructField = StructField<CStructFieldType>;
pub type CStruct = Struct<CStructFieldType>;
pub type CStructHandle = Handle<CStruct>;
pub type CStructBuilder<'a> = StructFieldBuilder<'a, CStructFieldType>;

impl StructFieldType for CStructFieldType {
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
            Self::Enum(x) => x.default_variant.is_some(),
            Self::Duration(_, default) => default.is_some(),
            Self::Struct(x) => x.all_fields_have_defaults(),
            Self::Iterator(_) => false,
        }
    }

    fn create_struct_type(v: Handle<Struct<CStructFieldType>>) -> StructType {
        StructType::CStruct(v.clone(), v.to_any_struct())
    }

    fn strings_allowed() -> bool {
        false
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
            Self::Enum(x) => AnyStructFieldType::Enum(x),
            Self::Duration(t, x) => AnyStructFieldType::Duration(t, x),
            Self::Struct(x) => AnyStructFieldType::Struct(x.to_any_struct()),
            Self::Iterator(x) => AnyStructFieldType::Iterator(x),
        }
    }

    fn to_any_type(&self) -> AnyType {
        match self {
            Self::Bool(_) => BasicType::Bool.into(),
            Self::Uint8(_) => BasicType::Uint8.into(),
            Self::Sint8(_) => BasicType::Sint8.into(),
            Self::Uint16(_) => BasicType::Uint16.into(),
            Self::Sint16(_) => BasicType::Sint16.into(),
            Self::Uint32(_) => BasicType::Uint32.into(),
            Self::Sint32(_) => BasicType::Sint32.into(),
            Self::Uint64(_) => BasicType::Uint64.into(),
            Self::Sint64(_) => BasicType::Uint64.into(),
            Self::Float(_) => BasicType::Float.into(),
            Self::Double(_) => BasicType::Double.into(),
            Self::Enum(x) => BasicType::Enum(x.handle.clone()).into(),
            Self::Duration(x, _) => BasicType::Duration(*x).into(),
            Self::Struct(x) => AnyType::Struct(x.to_any_struct()),
            Self::Iterator(x) => AnyType::Iterator(x.clone()),
        }
    }
}

impl From<CStructHandle> for CStructFieldType {
    fn from(x: CStructHandle) -> Self {
        CStructFieldType::Struct(x)
    }
}

impl From<EnumField> for CStructFieldType {
    fn from(x: EnumField) -> Self {
        Self::Enum(x)
    }
}
