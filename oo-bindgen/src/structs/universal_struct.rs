use crate::structs::common::*;
use crate::types::{DurationType, TypeValidator, ValidatedType};
use crate::*;

/// Types that can be used in a universal struct, some of which might have a default value
#[derive(Clone, Debug)]
pub enum UniversalStructField {
    Basic(BasicType),
    Struct(UniversalStructHandle),
}
impl TypeValidator for UniversalStructField {
    fn get_validated_type(&self) -> Option<ValidatedType> {
        match self {
            UniversalStructField::Basic(x) => x.get_validated_type(),
            UniversalStructField::Struct(x) => StructType::UStruct(x.clone()).get_validated_type(),
        }
    }
}

pub type UniversalStructHandle = Handle<Struct<UniversalStructField>>;
pub type UniversalStructBuilder<'a> = StructFieldBuilder<'a, UniversalStructField>;

impl StructFieldType for UniversalStructField {
    fn create_struct_type(v: Handle<Struct<UniversalStructField>>) -> StructType {
        StructType::UStruct(v)
    }
}

impl ConstructorValidator for UniversalStructField {
    fn validate_constructor_default(
        &self,
        value: &ConstructorDefault,
    ) -> BindResult<ValidatedConstructorDefault> {
        match self {
            UniversalStructField::Basic(x) => x.validate_constructor_default(value),
            UniversalStructField::Struct(x) => x.validate_constructor_default(value),
        }
    }
}

impl From<BasicType> for UniversalStructField {
    fn from(x: BasicType) -> Self {
        UniversalStructField::Basic(x)
    }
}

impl From<DurationType> for UniversalStructField {
    fn from(x: DurationType) -> Self {
        BasicType::Duration(x).into()
    }
}

impl From<UniversalStructHandle> for UniversalStructField {
    fn from(x: UniversalStructHandle) -> Self {
        UniversalStructField::Struct(x)
    }
}
