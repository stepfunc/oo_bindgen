
use crate::structs::common::{Struct, StructField, StructFieldBuilder, StructFieldType};
use crate::types::AnyType;
use crate::*;


impl StructFieldType for AnyType {
    fn create_struct_type(v: Handle<Struct<Self>>) -> StructType {
        StructType::Any(v)
    }

    fn to_any_type(&self) -> AnyType {
        self.clone()
    }
}

pub type AnyStructFieldType = AnyType;
pub type AnyStructField = StructField<AnyStructFieldType>;
pub type AnyStruct = Struct<AnyStructFieldType>;
pub type AnyStructHandle = Handle<AnyStruct>;
pub type AnyStructBuilder<'a> = StructFieldBuilder<'a, AnyStructFieldType>;

impl From<AnyStructHandle> for AnyType {
    fn from(x: AnyStructHandle) -> Self {
        Self::Struct(x)
    }
}
