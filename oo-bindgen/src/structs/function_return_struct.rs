use crate::iterator::IteratorHandle;
use crate::structs::common::*;
use crate::types::{TypeValidator, ValidatedType};
use crate::*;

/// Types that can be used in a function return value
#[derive(Clone, Debug)]
pub enum FunctionReturnStructField {
    Basic(BasicType),
    ClassRef(ClassDeclarationHandle),
    // iterators must be allowed in return position so that you can have nested iterators
    Iterator(IteratorHandle),
    Struct(UniversalOr<FunctionReturnStructField>),
}

impl TypeValidator for FunctionReturnStructField {
    fn get_validated_type(&self) -> Option<ValidatedType> {
        match self {
            Self::Basic(x) => x.get_validated_type(),
            Self::ClassRef(x) => x.get_validated_type(),
            Self::Struct(x) => x.to_struct_type().get_validated_type(),
            Self::Iterator(x) => x.get_validated_type(),
        }
    }
}

pub type FunctionReturnStructHandle = Handle<Struct<FunctionReturnStructField>>;
pub type FunctionReturnStructBuilder<'a> = StructFieldBuilder<'a, FunctionReturnStructField>;

impl StructFieldType for FunctionReturnStructField {
    fn create_struct_type(v: Handle<Struct<Self>>) -> StructType {
        StructType::RStruct(v)
    }
}

impl ConstructorValidator for FunctionReturnStructField {
    fn validate_constructor_default(
        &self,
        value: &ConstructorDefault,
    ) -> BindResult<ValidatedConstructorDefault> {
        match self {
            Self::Basic(x) => x.validate_constructor_default(value),
            Self::ClassRef(x) => x.validate_constructor_default(value),
            Self::Struct(x) => x.validate_constructor_default(value),
            Self::Iterator(x) => x.validate_constructor_default(value),
        }
    }
}

impl From<BasicType> for FunctionReturnStructField {
    fn from(x: BasicType) -> Self {
        Self::Basic(x)
    }
}

impl From<ClassDeclarationHandle> for FunctionReturnStructField {
    fn from(x: ClassDeclarationHandle) -> Self {
        Self::ClassRef(x)
    }
}

impl From<FunctionReturnStructHandle> for FunctionReturnStructField {
    fn from(x: FunctionReturnStructHandle) -> Self {
        Self::Struct(x.into())
    }
}

impl From<IteratorHandle> for FunctionReturnStructField {
    fn from(x: IteratorHandle) -> Self {
        Self::Iterator(x)
    }
}
