use crate::structs::common::*;
use crate::types::{AnyType, DurationType};
use crate::*;

use std::time::Duration;

/// Types that can be used in a callback struct, some of which might have a default value
#[derive(Clone, Debug)]
pub enum RStructFieldType {
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
    Duration(DurationType, Option<Duration>),
    ClassRef(ClassDeclarationHandle),
    Struct(RStructHandle),
}

pub type RStructField = StructField<RStructFieldType>;
pub type RStruct = Struct<RStructFieldType>;
pub type RStructHandle = Handle<RStruct>;
pub type RStructBuilder<'a> = StructBuilder<'a, RStructFieldType>;

impl StructFieldType for RStructFieldType {
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
            Self::ClassRef(_) => false,
        }
    }

    fn create_struct_type(v: Handle<Struct<Self>>) -> StructType {
        StructType::RStruct(v.clone(), v.to_any_struct())
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
            Self::ClassRef(x) => AnyStructFieldType::ClassRef(x),
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
            Self::ClassRef(x) => AnyType::ClassRef(x.clone()),
        }
    }
}

impl From<RStructHandle> for RStructFieldType {
    fn from(x: RStructHandle) -> Self {
        RStructFieldType::Struct(x)
    }
}

impl From<EnumField> for RStructFieldType {
    fn from(x: EnumField) -> Self {
        Self::Enum(x)
    }
}

impl From<BasicType> for RStructFieldType {
    fn from(x: BasicType) -> Self {
        match x {
            BasicType::Bool => Self::Bool(None),
            BasicType::Uint8 => Self::Uint8(None),
            BasicType::Sint8 => Self::Sint8(None),
            BasicType::Uint16 => Self::Uint16(None),
            BasicType::Sint16 => Self::Sint16(None),
            BasicType::Uint32 => Self::Uint32(None),
            BasicType::Sint32 => Self::Sint32(None),
            BasicType::Uint64 => Self::Uint64(None),
            BasicType::Sint64 => Self::Sint64(None),
            BasicType::Float => Self::Float(None),
            BasicType::Double => Self::Double(None),
            BasicType::Duration(x) => Self::Duration(x, None),
            BasicType::Enum(x) => Self::Enum(EnumField::new(x)),
        }
    }
}
