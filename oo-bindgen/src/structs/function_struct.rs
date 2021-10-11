use crate::collection::CollectionHandle;
use crate::structs::common::*;
use crate::types::{DurationType, StringType, TypeValidator, ValidatedType};
use crate::*;

/// Types that can be used in a function struct, some of which might have a default value
#[derive(Clone, Debug)]
pub enum FunctionArgStructFieldType {
    Basic(BasicType),
    String(StringType),
    Interface(InterfaceHandle),
    Collection(CollectionHandle),
    Struct(FunctionArgStructHandle),
}

impl TypeValidator for FunctionArgStructFieldType {
    fn get_validated_type(&self) -> Option<ValidatedType> {
        match self {
            FunctionArgStructFieldType::Basic(x) => x.get_validated_type(),
            FunctionArgStructFieldType::String(x) => x.get_validated_type(),
            FunctionArgStructFieldType::Interface(x) => x.get_validated_type(),
            FunctionArgStructFieldType::Collection(x) => x.get_validated_type(),
            FunctionArgStructFieldType::Struct(x) => {
                StructType::FStruct(x.clone()).get_validated_type()
            }
        }
    }
}

pub type FunctionArgStructField = StructField<FunctionArgStructFieldType>;
pub type FunctionArgStruct = Struct<FunctionArgStructFieldType>;
pub type FunctionArgStructHandle = Handle<FunctionArgStruct>;
pub type FunctionArgStructBuilder<'a> = StructFieldBuilder<'a, FunctionArgStructFieldType>;

impl StructFieldType for FunctionArgStructFieldType {
    fn create_struct_type(v: Handle<Struct<FunctionArgStructFieldType>>) -> StructType {
        StructType::FStruct(v)
    }
}

impl From<BasicType> for FunctionArgStructFieldType {
    fn from(x: BasicType) -> Self {
        FunctionArgStructFieldType::Basic(x)
    }
}

impl From<StringType> for FunctionArgStructFieldType {
    fn from(x: StringType) -> Self {
        FunctionArgStructFieldType::String(x)
    }
}

impl From<FunctionArgStructHandle> for FunctionArgStructFieldType {
    fn from(x: FunctionArgStructHandle) -> Self {
        FunctionArgStructFieldType::Struct(x)
    }
}

impl From<InterfaceHandle> for FunctionArgStructFieldType {
    fn from(x: InterfaceHandle) -> Self {
        FunctionArgStructFieldType::Interface(x)
    }
}

impl From<DurationType> for FunctionArgStructFieldType {
    fn from(x: DurationType) -> Self {
        FunctionArgStructFieldType::Basic(BasicType::Duration(x))
    }
}

impl From<EnumHandle> for FunctionArgStructFieldType {
    fn from(x: EnumHandle) -> Self {
        BasicType::Enum(x).into()
    }
}
