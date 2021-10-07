use crate::structs::common::*;
use crate::*;

use crate::iterator::IteratorHandle;
use crate::types::{TypeValidator, ValidatedType};

/// Types that can be used in a callback struct, some of which might have a default value
#[derive(Clone, Debug)]
pub enum CStructFieldType {
    Basic(BasicType),
    Iterator(IteratorHandle),
    Struct(CStructHandle),
}

impl TypeValidator for CStructFieldType {
    fn get_validated_type(&self) -> Option<ValidatedType> {
        match self {
            CStructFieldType::Basic(x) => x.get_validated_type(),
            CStructFieldType::Iterator(x) => x.get_validated_type(),
            CStructFieldType::Struct(x) => StructType::CStruct(x.clone()).get_validated_type(),
        }
    }
}

pub type CStructField = StructField<CStructFieldType>;
pub type CStruct = Struct<CStructFieldType>;
pub type CStructHandle = Handle<CStruct>;
pub type CStructBuilder<'a> = StructFieldBuilder<'a, CStructFieldType>;

impl StructFieldType for CStructFieldType {
    fn create_struct_type(v: Handle<Struct<CStructFieldType>>) -> StructType {
        StructType::CStruct(v)
    }
}

impl From<BasicType> for CStructFieldType {
    fn from(x: BasicType) -> Self {
        CStructFieldType::Basic(x)
    }
}

impl From<IteratorHandle> for CStructFieldType {
    fn from(x: IteratorHandle) -> Self {
        CStructFieldType::Iterator(x)
    }
}

impl From<CStructHandle> for CStructFieldType {
    fn from(x: CStructHandle) -> Self {
        CStructFieldType::Struct(x)
    }
}
