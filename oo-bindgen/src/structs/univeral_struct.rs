use crate::structs::common::*;
use crate::types::{TypeValidator, ValidatedType};
use crate::*;

/// Types that can be used in a universal struct, some of which might have a default value
#[derive(Clone, Debug)]
pub enum UStructFieldType {
    Basic(BasicType),
    Struct(UStructHandle),
}

impl TypeValidator for UStructFieldType {
    fn get_validated_type(&self) -> Option<ValidatedType> {
        match self {
            UStructFieldType::Basic(x) => x.get_validated_type(),
            UStructFieldType::Struct(x) => StructType::UStruct(x.clone()).get_validated_type(),
        }
    }
}

pub type UStructField = StructField<UStructFieldType>;
pub type UStruct = Struct<UStructFieldType>;
pub type UStructHandle = Handle<UStruct>;
pub type UStructBuilder<'a> = StructFieldBuilder<'a, UStructFieldType>;

impl StructFieldType for UStructFieldType {
    fn create_struct_type(v: Handle<Struct<UStructFieldType>>) -> StructType {
        StructType::UStruct(v)
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
