use crate::structs::common::*;
use crate::types::{DurationType, TypeValidator, ValidatedType};
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

pub type UniversalStructHandle = Handle<Struct<UniversalStructFieldType>>;
pub type UniversalStructBuilder<'a> = StructFieldBuilder<'a, UniversalStructFieldType>;

impl StructFieldType for UniversalStructFieldType {
    fn create_struct_type(v: Handle<Struct<UniversalStructFieldType>>) -> StructType {
        StructType::UStruct(v)
    }
}

impl ConstructorValidator for UniversalStructFieldType {
    fn validate_constructor_default(
        &self,
        value: &ConstructorDefault,
    ) -> BindResult<ValidatedConstructorDefault> {
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

impl From<DurationType> for UniversalStructFieldType {
    fn from(x: DurationType) -> Self {
        BasicType::Duration(x).into()
    }
}

impl From<UniversalStructHandle> for UniversalStructFieldType {
    fn from(x: UniversalStructHandle) -> Self {
        UniversalStructFieldType::Struct(x)
    }
}
