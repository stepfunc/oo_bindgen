use crate::structs::common::*;
use crate::types::AnyType;
use crate::*;

/// Types that can be used in a callback struct, some of which might have a default value
#[derive(Clone, Debug)]
pub enum RStructFieldType {
    Basic(BasicType),
    ClassRef(ClassDeclarationHandle),
    Struct(RStructHandle),
}

pub type RStructField = StructField<RStructFieldType>;
pub type RStruct = Struct<RStructFieldType>;
pub type RStructHandle = Handle<RStruct>;
pub type RStructBuilder<'a> = StructFieldBuilder<'a, RStructFieldType>;

impl StructFieldType for RStructFieldType {

    fn create_struct_type(v: Handle<Struct<Self>>) -> StructType {
        StructType::RStruct(v.clone(), v.to_any_struct())
    }

    fn to_any_type(&self) -> AnyType {
        match self {
            Self::Basic(x) => x.clone().into(),
            Self::Struct(x) => AnyType::Struct(x.to_any_struct()),
            Self::ClassRef(x) => AnyType::ClassRef(x.clone()),
        }
    }
}

impl From<BasicType> for RStructFieldType {
    fn from(x: BasicType) -> Self {
        RStructFieldType::Basic(x)
    }
}

impl From<ClassDeclarationHandle> for RStructFieldType {
    fn from(x: ClassDeclarationHandle) -> Self {
        RStructFieldType::ClassRef(x)
    }
}

impl From<RStructHandle> for RStructFieldType {
    fn from(x: RStructHandle) -> Self {
        RStructFieldType::Struct(x)
    }
}


