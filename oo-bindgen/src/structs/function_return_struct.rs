use crate::structs::common::*;
use crate::types::{TypeValidator, ValidatedType};
use crate::*;

/// Types that can be used in a function return value
#[derive(Clone, Debug)]
pub enum ReturnStructFieldType {
    Basic(BasicType),
    ClassRef(ClassDeclarationHandle),
    Struct(ReturnStructHandle),
}

impl TypeValidator for ReturnStructFieldType {
    fn get_validated_type(&self) -> Option<ValidatedType> {
        match self {
            ReturnStructFieldType::Basic(x) => x.get_validated_type(),
            ReturnStructFieldType::ClassRef(x) => x.get_validated_type(),
            ReturnStructFieldType::Struct(x) => StructType::RStruct(x.clone()).get_validated_type(),
        }
    }
}

pub type ReturnStructField = StructField<ReturnStructFieldType>;
pub type ReturnStruct = Struct<ReturnStructFieldType>;
pub type ReturnStructHandle = Handle<ReturnStruct>;
pub type ReturnStructBuilder<'a> = StructFieldBuilder<'a, ReturnStructFieldType>;

impl StructFieldType for ReturnStructFieldType {
    fn create_struct_type(v: Handle<Struct<Self>>) -> StructType {
        StructType::RStruct(v)
    }
}

impl From<BasicType> for ReturnStructFieldType {
    fn from(x: BasicType) -> Self {
        ReturnStructFieldType::Basic(x)
    }
}

impl From<ClassDeclarationHandle> for ReturnStructFieldType {
    fn from(x: ClassDeclarationHandle) -> Self {
        ReturnStructFieldType::ClassRef(x)
    }
}

impl From<ReturnStructHandle> for ReturnStructFieldType {
    fn from(x: ReturnStructHandle) -> Self {
        ReturnStructFieldType::Struct(x)
    }
}
