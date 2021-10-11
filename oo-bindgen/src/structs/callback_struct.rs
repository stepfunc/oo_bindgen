use crate::structs::common::*;
use crate::*;

use crate::iterator::IteratorHandle;
use crate::types::{TypeValidator, ValidatedType};

/// Types that can be used in a callback struct, some of which might have a default value
#[derive(Clone, Debug)]
pub enum CallbackStructFieldType {
    Basic(BasicType),
    Iterator(IteratorHandle),
    Struct(CallbackStructHandle),
}

impl TypeValidator for CallbackStructFieldType {
    fn get_validated_type(&self) -> Option<ValidatedType> {
        match self {
            CallbackStructFieldType::Basic(x) => x.get_validated_type(),
            CallbackStructFieldType::Iterator(x) => x.get_validated_type(),
            CallbackStructFieldType::Struct(x) => {
                StructType::CStruct(x.clone()).get_validated_type()
            }
        }
    }
}

pub type CallbackStructField = StructField<CallbackStructFieldType>;
pub type CallbackStruct = Struct<CallbackStructFieldType>;
pub type CallbackStructHandle = Handle<CallbackStruct>;
pub type CallbackStructBuilder<'a> = StructFieldBuilder<'a, CallbackStructFieldType>;

impl StructFieldType for CallbackStructFieldType {
    fn create_struct_type(v: Handle<Struct<CallbackStructFieldType>>) -> StructType {
        StructType::CStruct(v)
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
        CallbackStructFieldType::Struct(x)
    }
}
