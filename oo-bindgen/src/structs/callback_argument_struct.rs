use crate::doc::Unvalidated;
use crate::iterator::IteratorHandle;
use crate::structs::common::*;
use crate::types::DurationType;
use crate::*;

/// Types that can be used as a callback argument
#[derive(Clone, Debug)]
pub enum CallbackArgStructField {
    Basic(BasicType),
    Iterator(IteratorHandle),
    Struct(UniversalOr<CallbackArgStructField>),
}

pub type CallbackArgStructHandle = Handle<Struct<CallbackArgStructField, Unvalidated>>;
pub type CallbackArgStructBuilder<'a> = StructFieldBuilder<'a, CallbackArgStructField>;

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

impl From<BasicType> for CallbackArgStructField {
    fn from(x: BasicType) -> Self {
        CallbackArgStructField::Basic(x)
    }
}

impl From<EnumHandle> for CallbackArgStructField {
    fn from(x: EnumHandle) -> Self {
        CallbackArgStructField::Basic(BasicType::Enum(x))
    }
}

impl From<DurationType> for CallbackArgStructField {
    fn from(x: DurationType) -> Self {
        CallbackArgStructField::Basic(BasicType::Duration(x))
    }
}

impl From<IteratorHandle> for CallbackArgStructField {
    fn from(x: IteratorHandle) -> Self {
        CallbackArgStructField::Iterator(x)
    }
}

impl From<CallbackArgStructHandle> for CallbackArgStructField {
    fn from(x: CallbackArgStructHandle) -> Self {
        CallbackArgStructField::Struct(x.into())
    }
}
