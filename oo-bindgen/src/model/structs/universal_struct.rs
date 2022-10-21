use crate::model::*;

/// Types that can be used in a universal struct, some of which might have a default value
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum UniversalStructField {
    Basic(BasicType),
    Struct(UniversalStructHandle),
}

pub type UniversalStructHandle = Handle<Struct<UniversalStructField, Unvalidated>>;

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

impl From<Primitive> for UniversalStructField {
    fn from(x: Primitive) -> Self {
        Self::Basic(x.into())
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

impl From<Handle<Enum<Unvalidated>>> for UniversalStructField {
    fn from(x: Handle<Enum<Unvalidated>>) -> Self {
        Self::Basic(BasicType::Enum(x))
    }
}

impl From<UniversalStructHandle> for UniversalStructField {
    fn from(x: UniversalStructHandle) -> Self {
        UniversalStructField::Struct(x)
    }
}
