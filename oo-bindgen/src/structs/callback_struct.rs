use crate::structs::common::*;
use crate::*;

use crate::iterator::IteratorHandle;
use crate::types::{TypeValidator, ValidatedType};

/// Types that can be used in a callback struct, some of which might have a default value
#[derive(Clone, Debug)]
pub enum CallbackStructFieldType {
    Basic(BasicType),
    Iterator(IteratorHandle),
    Struct(UniversalOr<CallbackStructFieldType>),
}

impl TypeValidator for CallbackStructFieldType {
    fn get_validated_type(&self) -> Option<ValidatedType> {
        match self {
            CallbackStructFieldType::Basic(x) => x.get_validated_type(),
            CallbackStructFieldType::Iterator(x) => x.get_validated_type(),
            CallbackStructFieldType::Struct(x) => Some(ValidatedType::Struct(x.to_struct_type())),
        }
    }
}

pub type CallbackStructHandle = Handle<Struct<CallbackStructFieldType>>;
pub type CallbackStructBuilder<'a> = StructFieldBuilder<'a, CallbackStructFieldType>;

impl StructFieldType for CallbackStructFieldType {
    fn create_struct_type(v: Handle<Struct<CallbackStructFieldType>>) -> StructType {
        StructType::CStruct(v)
    }
}

impl ConstructorValidator for CallbackStructFieldType {
    fn validate_constructor_default(
        &self,
        value: &ConstructorDefault,
    ) -> BindResult<ValidatedConstructorDefault> {
        match self {
            CallbackStructFieldType::Basic(x) => x.validate_constructor_default(value),
            CallbackStructFieldType::Iterator(x) => x.validate_constructor_default(value),
            CallbackStructFieldType::Struct(x) => match x {
                UniversalOr::Specific(x) => x.validate_constructor_default(value),
                UniversalOr::Universal(x) => x.validate_constructor_default(value),
            },
        }
    }
}

impl From<BasicType> for CallbackStructFieldType {
    fn from(x: BasicType) -> Self {
        CallbackStructFieldType::Basic(x)
    }
}

impl From<IteratorHandle> for CallbackStructFieldType {
    fn from(x: IteratorHandle) -> Self {
        CallbackStructFieldType::Iterator(x)
    }
}

impl From<CallbackStructHandle> for CallbackStructFieldType {
    fn from(x: CallbackStructHandle) -> Self {
        CallbackStructFieldType::Struct(x.into())
    }
}
