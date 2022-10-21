use crate::model::*;

/// Types that can be used in a function struct, some of which might have a default value
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum FunctionArgStructField {
    Basic(BasicType),
    String(StringType),
    Interface(AsynchronousInterface),
    Struct(UniversalOr<FunctionArgStructField>),
}

pub type FunctionArgStructHandle = Handle<Struct<FunctionArgStructField, Unvalidated>>;

impl StructFieldType for FunctionArgStructField {
    fn create_struct_type(
        v: Handle<Struct<FunctionArgStructField, Unvalidated>>,
    ) -> StructType<Unvalidated> {
        StructType::FunctionArg(v)
    }
}

impl InitializerValidator for FunctionArgStructField {
    fn validate_default_value(
        &self,
        value: &InitializerDefault,
    ) -> BindResult<ValidatedDefaultValue> {
        match self {
            FunctionArgStructField::Basic(x) => x.validate_default_value(value),
            FunctionArgStructField::String(x) => x.validate_default_value(value),
            FunctionArgStructField::Interface(x) => x.inner.validate_default_value(value),
            FunctionArgStructField::Struct(x) => x.validate_default_value(value),
        }
    }
}

impl From<Primitive> for FunctionArgStructField {
    fn from(x: Primitive) -> Self {
        Self::Basic(x.into())
    }
}

impl From<BasicType> for FunctionArgStructField {
    fn from(x: BasicType) -> Self {
        FunctionArgStructField::Basic(x)
    }
}

impl From<StringType> for FunctionArgStructField {
    fn from(x: StringType) -> Self {
        FunctionArgStructField::String(x)
    }
}

impl From<FunctionArgStructHandle> for FunctionArgStructField {
    fn from(x: FunctionArgStructHandle) -> Self {
        FunctionArgStructField::Struct(x.into())
    }
}

impl From<UniversalStructHandle> for FunctionArgStructField {
    fn from(x: UniversalStructHandle) -> Self {
        Self::Struct(UniversalOr::Universal(x))
    }
}

impl From<AsynchronousInterface> for FunctionArgStructField {
    fn from(x: AsynchronousInterface) -> Self {
        FunctionArgStructField::Interface(x)
    }
}

impl From<DurationType> for FunctionArgStructField {
    fn from(x: DurationType) -> Self {
        FunctionArgStructField::Basic(BasicType::Duration(x))
    }
}

impl From<Handle<Enum<Unvalidated>>> for FunctionArgStructField {
    fn from(x: Handle<Enum<Unvalidated>>) -> Self {
        BasicType::Enum(x).into()
    }
}
