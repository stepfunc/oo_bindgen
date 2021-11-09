use crate::doc::Unvalidated;
use crate::structs::common::*;
use crate::types::DurationType;
use crate::*;

/// Types that can be used in a universal struct, some of which might have a default value
#[derive(Clone, Debug)]
pub enum UniversalStructField {
    Basic(BasicType),
    Struct(UniversalStructHandle),
}

pub type UniversalStructHandle = Handle<Struct<UniversalStructField, Unvalidated>>;
pub type UniversalStructBuilder<'a> = StructFieldBuilder<'a, UniversalStructField>;

impl StructFieldType for UniversalStructField {
    fn create_struct_type(
        v: Handle<Struct<UniversalStructField, Unvalidated>>,
    ) -> StructType<Unvalidated> {
        StructType::Universal(v)
    }
}

impl InitializerValidator for UniversalStructField {
    fn validate_default_value(
        &self,
        value: &InitializerDefault,
    ) -> BindResult<ValidatedDefaultValue> {
        match self {
            UniversalStructField::Basic(x) => x.validate_default_value(value),
            UniversalStructField::Struct(x) => x.validate_default_value(value),
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

impl From<EnumHandle> for UniversalStructField {
    fn from(x: EnumHandle) -> Self {
        Self::Basic(BasicType::Enum(x))
    }
}

impl From<UniversalStructHandle> for UniversalStructField {
    fn from(x: UniversalStructHandle) -> Self {
        UniversalStructField::Struct(x)
    }
}
