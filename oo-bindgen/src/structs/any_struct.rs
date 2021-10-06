use crate::collection::CollectionHandle;
use crate::iterator::IteratorHandle;
use crate::structs::common::{
    EnumField, Struct, StructDeclarationHandle, StructField, StructFieldBuilder, StructFieldType,
};
use crate::types::{AnyType, BasicType, DurationType};
use crate::*;

use std::time::Duration;

#[derive(Clone, Debug)]
pub enum AnyStructFieldType {
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
    Struct(AnyStructHandle),
    StructRef(StructDeclarationHandle),
    Enum(EnumField),
    ClassRef(ClassDeclarationHandle),
    Interface(InterfaceHandle),
    Iterator(IteratorHandle),
    Collection(CollectionHandle),
    Duration(DurationType, Option<Duration>),
}

impl StructFieldType for AnyStructFieldType {
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
            Self::Struct(handle) => handle.all_fields_have_defaults(),
            Self::StructRef(_) => false,
            Self::Enum(f) => f.default_variant.is_some(),
            Self::ClassRef(_) => false,
            Self::Interface(_) => false,
            Self::Iterator(_) => false,
            Self::Collection(_) => false,
            Self::Duration(_, default) => default.is_some(),
        }
    }

    fn create_struct_type(v: Handle<Struct<Self>>) -> StructType {
        StructType::Any(v)
    }

    fn strings_allowed() -> bool {
        true
    }

    fn to_any_struct_field_type(self) -> AnyStructFieldType {
        self
    }

    fn to_any_type(&self) -> AnyType {
        match self {
            AnyStructFieldType::Bool(_) => BasicType::Bool.into(),
            AnyStructFieldType::Uint8(_) => BasicType::Uint8.into(),
            AnyStructFieldType::Sint8(_) => BasicType::Sint8.into(),
            AnyStructFieldType::Uint16(_) => BasicType::Uint16.into(),
            AnyStructFieldType::Sint16(_) => BasicType::Sint16.into(),
            AnyStructFieldType::Uint32(_) => BasicType::Uint32.into(),
            AnyStructFieldType::Sint32(_) => BasicType::Sint32.into(),
            AnyStructFieldType::Uint64(_) => BasicType::Uint64.into(),
            AnyStructFieldType::Sint64(_) => BasicType::Sint64.into(),
            AnyStructFieldType::Float(_) => BasicType::Float.into(),
            AnyStructFieldType::Double(_) => BasicType::Double.into(),
            AnyStructFieldType::String(_) => AnyType::String,
            AnyStructFieldType::Struct(x) => AnyType::Struct(x.to_any_struct()),
            AnyStructFieldType::StructRef(x) => AnyType::StructRef(x.clone()),
            AnyStructFieldType::Enum(x) => BasicType::Enum(x.handle.clone()).into(),
            AnyStructFieldType::ClassRef(x) => AnyType::ClassRef(x.clone()),
            AnyStructFieldType::Interface(x) => AnyType::Interface(x.clone()),
            AnyStructFieldType::Iterator(x) => AnyType::Iterator(x.clone()),
            AnyStructFieldType::Collection(x) => AnyType::Collection(x.clone()),
            AnyStructFieldType::Duration(x, _) => BasicType::Duration(*x).into(),
        }
    }
}

pub type AnyStructField = StructField<AnyStructFieldType>;
pub type AnyStruct = Struct<AnyStructFieldType>;
pub type AnyStructHandle = Handle<AnyStruct>;
pub type AnyStructBuilder<'a> = StructFieldBuilder<'a, AnyStructFieldType>;

impl From<AnyStructHandle> for AnyType {
    fn from(x: AnyStructHandle) -> Self {
        Self::Struct(x)
    }
}
