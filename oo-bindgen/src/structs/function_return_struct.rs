use crate::iterator::IteratorHandle;
use crate::structs::common::*;
use crate::types::{TypeValidator, ValidatedType};
use crate::*;

/// Types that can be used in a function return value
#[derive(Clone, Debug)]
pub enum ReturnStructFieldType {
    Basic(BasicType),
    ClassRef(ClassDeclarationHandle),
    Struct(MaybeUniversal<ReturnStructFieldType>),
    // iterators must be allowed in return position so that you can have nested iterators
    Iterator(IteratorHandle),
}

impl TypeValidator for ReturnStructFieldType {
    fn get_validated_type(&self) -> Option<ValidatedType> {
        match self {
            Self::Basic(x) => x.get_validated_type(),
            Self::ClassRef(x) => x.get_validated_type(),
            Self::Struct(x) => x.to_struct_type().get_validated_type(),
            Self::Iterator(x) => x.get_validated_type(),
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

impl ConstructorValidator for ReturnStructFieldType {
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

impl From<BasicType> for ReturnStructFieldType {
    fn from(x: BasicType) -> Self {
        Self::Basic(x)
    }
}

impl From<ClassDeclarationHandle> for ReturnStructFieldType {
    fn from(x: ClassDeclarationHandle) -> Self {
        Self::ClassRef(x)
    }
}

impl From<ReturnStructHandle> for ReturnStructFieldType {
    fn from(x: ReturnStructHandle) -> Self {
        Self::Struct(x.into())
    }
}

impl From<IteratorHandle> for ReturnStructFieldType {
    fn from(x: IteratorHandle) -> Self {
        Self::Iterator(x)
    }
}
