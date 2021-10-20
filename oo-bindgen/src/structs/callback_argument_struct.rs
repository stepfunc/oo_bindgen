use crate::iterator::IteratorHandle;
use crate::structs::common::*;
use crate::types::{DurationType, TypeValidator, ValidatedType};
use crate::*;

/// Types that can be used as a callback argument
#[derive(Clone, Debug)]
pub enum CallbackArgStructField {
    Basic(BasicType),
    Iterator(IteratorHandle),
    Struct(UniversalOr<CallbackArgStructField>),
}

impl TypeValidator for CallbackArgStructField {
    fn get_validated_type(&self) -> Option<ValidatedType> {
        match self {
            CallbackArgStructField::Basic(x) => x.get_validated_type(),
            CallbackArgStructField::Iterator(x) => x.get_validated_type(),
            CallbackArgStructField::Struct(x) => Some(ValidatedType::Struct(x.to_struct_type())),
        }
    }
}

pub type CallbackStructHandle = Handle<Struct<CallbackArgStructField>>;
pub type CallbackStructBuilder<'a> = StructFieldBuilder<'a, CallbackArgStructField>;

impl StructFieldType for CallbackArgStructField {
    fn create_struct_type(v: Handle<Struct<CallbackArgStructField>>) -> StructType {
        StructType::CStruct(v)
    }
}

impl ConstructorValidator for CallbackArgStructField {
    fn validate_constructor_default(
        &self,
        value: &ConstructorDefault,
    ) -> BindResult<ValidatedConstructorDefault> {
        match self {
            CallbackArgStructField::Basic(x) => x.validate_constructor_default(value),
            CallbackArgStructField::Iterator(x) => x.validate_constructor_default(value),
            CallbackArgStructField::Struct(x) => match x {
                UniversalOr::Specific(x) => x.validate_constructor_default(value),
                UniversalOr::Universal(x) => x.validate_constructor_default(value),
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

impl From<CallbackStructHandle> for CallbackArgStructField {
    fn from(x: CallbackStructHandle) -> Self {
        CallbackArgStructField::Struct(x.into())
    }
}
