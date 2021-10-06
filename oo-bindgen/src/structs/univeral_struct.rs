use crate::structs::common::*;
use crate::types::AnyType;
use crate::*;

/// Types that can be used in a universal struct, some of which might have a default value
#[derive(Clone, Debug)]
pub enum UStructFieldType {
    Basic(BasicType),
    Struct(UStructHandle),
}

pub type UStructField = StructField<UStructFieldType>;
pub type UStruct = Struct<UStructFieldType>;
pub type UStructHandle = Handle<UStruct>;
pub type UStructBuilder<'a> = StructFieldBuilder<'a, UStructFieldType>;

impl StructFieldType for UStructFieldType {

    fn create_struct_type(v: Handle<Struct<UStructFieldType>>) -> StructType {
        StructType::UStruct(v.clone(), v.to_any_struct())
    }

    fn to_any_type(&self) -> AnyType {
        match self {
            Self::Basic(x) => x.clone().into(),
            Self::Struct(x) => AnyType::Struct(x.to_any_struct()),
        }
    }
}

impl From<BasicType> for UStructFieldType {
    fn from(x: BasicType) -> Self {
        UStructFieldType::Basic(x)
    }
}

impl From<UStructHandle> for UStructFieldType {
    fn from(x: UStructHandle) -> Self {
        UStructFieldType::Struct(x)
    }
}


