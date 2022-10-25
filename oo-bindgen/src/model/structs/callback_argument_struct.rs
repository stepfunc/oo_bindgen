use crate::model::*;

/// Types that can be used as a callback argument
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum CallbackArgStructField {
    Basic(BasicType),
    Iterator(AbstractIteratorHandle),
    Struct(UniversalOr<CallbackArgStructField>),
}

pub type CallbackArgStructHandle = Handle<Struct<CallbackArgStructField, Unvalidated>>;

impl StructFieldType for CallbackArgStructField {
    fn create_struct_type(
        v: Handle<Struct<CallbackArgStructField, Unvalidated>>,
    ) -> StructType<Unvalidated> {
        StructType::CallbackArg(v)
    }
}

impl InitializerValidator for CallbackArgStructField {
    fn validate_default_value(
        &self,
        value: &InitializerDefault,
    ) -> BindResult<ValidatedDefaultValue> {
        match self {
            CallbackArgStructField::Basic(x) => x.validate_default_value(value),
            CallbackArgStructField::Iterator(x) => x.validate_default_value(value),
            CallbackArgStructField::Struct(x) => match x {
                UniversalOr::Specific(x) => x.validate_default_value(value),
                UniversalOr::Universal(x) => x.validate_default_value(value),
            },
        }
    }
}

impl From<Primitive> for CallbackArgStructField {
    fn from(x: Primitive) -> Self {
        Self::Basic(x.into())
    }
}

impl From<BasicType> for CallbackArgStructField {
    fn from(x: BasicType) -> Self {
        CallbackArgStructField::Basic(x)
    }
}

impl From<Handle<Enum<Unvalidated>>> for CallbackArgStructField {
    fn from(x: Handle<Enum<Unvalidated>>) -> Self {
        CallbackArgStructField::Basic(BasicType::Enum(x))
    }
}

impl From<DurationType> for CallbackArgStructField {
    fn from(x: DurationType) -> Self {
        CallbackArgStructField::Basic(BasicType::Duration(x))
    }
}

impl From<AbstractIteratorHandle> for CallbackArgStructField {
    fn from(x: AbstractIteratorHandle) -> Self {
        CallbackArgStructField::Iterator(x)
    }
}

impl From<CallbackArgStructHandle> for CallbackArgStructField {
    fn from(x: CallbackArgStructHandle) -> Self {
        CallbackArgStructField::Struct(x.into())
    }
}
