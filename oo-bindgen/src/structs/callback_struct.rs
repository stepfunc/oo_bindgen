use crate::structs::common::*;
use crate::types::AnyType;
use crate::*;

use crate::iterator::IteratorHandle;

/// Types that can be used in a callback struct, some of which might have a default value
#[derive(Clone, Debug)]
pub enum CStructFieldType {
    Basic(BasicType),
    Iterator(IteratorHandle),
    Struct(CStructHandle),
}

pub type CStructField = StructField<CStructFieldType>;
pub type CStruct = Struct<CStructFieldType>;
pub type CStructHandle = Handle<CStruct>;
pub type CStructBuilder<'a> = StructFieldBuilder<'a, CStructFieldType>;

impl StructFieldType for CStructFieldType {

    fn create_struct_type(v: Handle<Struct<CStructFieldType>>) -> StructType {
        StructType::CStruct(v.clone(), v.to_any_struct())
    }

    fn to_any_type(&self) -> AnyType {
        match self {
            Self::Basic(x) => x.clone().into(),
            Self::Struct(x) => AnyType::Struct(x.to_any_struct()),
            Self::Iterator(x) => AnyType::Iterator(x.clone()),
        }
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

