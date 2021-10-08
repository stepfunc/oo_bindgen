use crate::structs::common::*;
use crate::types::{TypeValidator, ValidatedType};
use crate::*;

/// Types that can be used in a callback struct, some of which might have a default value
#[derive(Clone, Debug)]
pub enum RStructFieldType {
    Basic(BasicType),
    ClassRef(ClassDeclarationHandle),
    Struct(RStructHandle),
}

impl TypeValidator for RStructFieldType {
    fn get_validated_type(&self) -> Option<ValidatedType> {
        match self {
            RStructFieldType::Basic(x) => x.get_validated_type(),
            RStructFieldType::ClassRef(x) => x.get_validated_type(),
            RStructFieldType::Struct(x) => StructType::RStruct(x.clone()).get_validated_type(),
        }
    }
}

pub type RStructField = StructField<RStructFieldType>;
pub type RStruct = Struct<RStructFieldType>;
pub type RStructHandle = Handle<RStruct>;
pub type RStructBuilder<'a> = StructFieldBuilder<'a, RStructFieldType>;

impl StructFieldType for RStructFieldType {
    fn create_struct_type(v: Handle<Struct<Self>>) -> StructType {
        StructType::RStruct(v)
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
