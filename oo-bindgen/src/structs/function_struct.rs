use crate::collection::CollectionHandle;
use crate::structs::common::*;
use crate::types::{AnyType, DurationType, StringType};
use crate::*;

/// Types that can be used in a function struct, some of which might have a default value
#[derive(Clone, Debug)]
pub enum FStructFieldType {
    Basic(BasicType),
    String,
    Interface(InterfaceHandle),
    Collection(CollectionHandle),
    Struct(FStructHandle),
}

pub type FStructField = StructField<FStructFieldType>;
pub type FStruct = Struct<FStructFieldType>;
pub type FStructHandle = Handle<FStruct>;
pub type FStructBuilder<'a> = StructFieldBuilder<'a, FStructFieldType>;

impl StructFieldType for FStructFieldType {

    fn create_struct_type(v: Handle<Struct<FStructFieldType>>) -> StructType {
        StructType::FStruct(v.clone(), v.to_any_struct())
    }

    fn to_any_type(&self) -> AnyType {
        match self {
            Self::Basic(x) => x.clone().into(),
            Self::String => AnyType::String,
            Self::Interface(x) => AnyType::Interface(x.clone()),
            Self::Collection(x) => AnyType::Collection(x.clone()),
            Self::Struct(x) => AnyType::Struct(x.to_any_struct()),
        }
    }
}

impl From<BasicType> for FStructFieldType {
    fn from(x: BasicType) -> Self {
        FStructFieldType::Basic(x)
    }
}

impl From<StringType> for FStructFieldType {
    fn from(_: StringType) -> Self {
        FStructFieldType::String
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

impl From<DurationType> for FStructFieldType {
    fn from(x: DurationType) -> Self {
        FStructFieldType::Basic(BasicType::Duration(x))
    }
}

impl From<EnumHandle> for FStructFieldType {
    fn from(x: EnumHandle) -> Self {
        BasicType::Enum(x).into()
    }
}
