use crate::collection::CollectionHandle;
use crate::structs::common::*;
use crate::types::{DurationType, StringType, TypeValidator, ValidatedType};
use crate::*;

/// Types that can be used in a function struct, some of which might have a default value
#[derive(Clone, Debug)]
pub enum FunctionArgStructField {
    Basic(BasicType),
    String(StringType),
    Interface(InterfaceHandle),
    Collection(CollectionHandle),
    Struct(UniversalOr<FunctionArgStructField>),
}

impl TypeValidator for FunctionArgStructField {
    fn get_validated_type(&self) -> Option<ValidatedType> {
        match self {
            FunctionArgStructField::Basic(x) => x.get_validated_type(),
            FunctionArgStructField::String(x) => x.get_validated_type(),
            FunctionArgStructField::Interface(x) => x.get_validated_type(),
            FunctionArgStructField::Collection(x) => x.get_validated_type(),
            FunctionArgStructField::Struct(x) => x.get_validated_type(),
        }
    }
}

pub type FunctionArgStructHandle = Handle<Struct<FunctionArgStructField>>;
pub type FunctionArgStructBuilder<'a> = StructFieldBuilder<'a, FunctionArgStructField>;

impl StructFieldType for FunctionArgStructField {
    fn create_struct_type(v: Handle<Struct<FunctionArgStructField>>) -> StructType {
        v.into()
    }
}

impl ConstructorValidator for FunctionArgStructField {
    fn validate_constructor_default(
        &self,
        value: &ConstructorDefault,
    ) -> BindResult<ValidatedConstructorDefault> {
        match self {
            FunctionArgStructField::Basic(x) => x.validate_constructor_default(value),
            FunctionArgStructField::String(x) => x.validate_constructor_default(value),
            FunctionArgStructField::Interface(x) => x.validate_constructor_default(value),
            FunctionArgStructField::Collection(x) => x.validate_constructor_default(value),
            FunctionArgStructField::Struct(x) => x.validate_constructor_default(value),
        }
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

impl From<InterfaceHandle> for FunctionArgStructField {
    fn from(x: InterfaceHandle) -> Self {
        FunctionArgStructField::Interface(x)
    }
}

impl From<DurationType> for FunctionArgStructField {
    fn from(x: DurationType) -> Self {
        FunctionArgStructField::Basic(BasicType::Duration(x))
    }
}

impl From<EnumHandle> for FunctionArgStructField {
    fn from(x: EnumHandle) -> Self {
        BasicType::Enum(x).into()
    }
}
