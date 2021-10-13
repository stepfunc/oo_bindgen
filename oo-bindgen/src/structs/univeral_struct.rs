use crate::structs::common::*;
use crate::types::{TypeValidator, ValidatedType};
use crate::*;

/// Types that can be used in a universal struct, some of which might have a default value
#[derive(Clone, Debug)]
pub enum UniversalStructFieldType {
    Basic(BasicType),
    Struct(UniversalStructHandle),
}
impl TypeValidator for UniversalStructFieldType {
    fn get_validated_type(&self) -> Option<ValidatedType> {
        match self {
            UniversalStructFieldType::Basic(x) => x.get_validated_type(),
            UniversalStructFieldType::Struct(x) => {
                StructType::UStruct(x.clone()).get_validated_type()
            }
        }
    }
}

pub type UniversalStructField = StructField<UniversalStructFieldType>;
pub type UniversalStruct = Struct<UniversalStructFieldType>;
pub type UniversalStructHandle = Handle<UniversalStruct>;
pub type UniversalStructBuilder<'a> = StructFieldBuilder<'a, UniversalStructFieldType>;

impl StructFieldType for UniversalStructFieldType {
    fn create_struct_type(v: Handle<Struct<UniversalStructFieldType>>) -> StructType {
        StructType::UStruct(v)
    }
}

impl ConstructorValidator for UniversalStructFieldType {
    fn validate_constructor_default(&self, value: &ConstructorValue) -> BindResult<()> {
        match self {
            UniversalStructFieldType::Basic(x) => x.validate_constructor_default(value),
            UniversalStructFieldType::Struct(x) => x.validate_constructor_default(value),
        }
    }
}

impl From<BasicType> for UniversalStructFieldType {
    fn from(x: BasicType) -> Self {
        UniversalStructFieldType::Basic(x)
    }
}

impl From<UniversalStructHandle> for UniversalStructFieldType {
    fn from(x: UniversalStructHandle) -> Self {
        UniversalStructFieldType::Struct(x)
    }
}
