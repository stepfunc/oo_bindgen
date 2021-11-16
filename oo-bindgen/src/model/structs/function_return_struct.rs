use crate::model::*;

/// Types that can be used in a function return value
#[derive(Clone, Debug)]
pub enum FunctionReturnStructField {
    Basic(BasicType),
    ClassRef(ClassDeclarationHandle),
    // iterators must be allowed in return position so that you can have nested iterators
    Iterator(AbstractIteratorHandle),
    Struct(UniversalOr<FunctionReturnStructField>),
}

pub type FunctionReturnStructHandle = Handle<Struct<FunctionReturnStructField, Unvalidated>>;

impl StructFieldType for FunctionReturnStructField {
    fn create_struct_type(v: Handle<Struct<Self, Unvalidated>>) -> StructType<Unvalidated> {
        StructType::FunctionReturn(v)
    }
}

impl InitializerValidator for FunctionReturnStructField {
    fn validate_default_value(
        &self,
        value: &InitializerDefault,
    ) -> BindResult<ValidatedDefaultValue> {
        match self {
            Self::Basic(x) => x.validate_default_value(value),
            Self::ClassRef(x) => x.validate_default_value(value),
            Self::Struct(x) => x.validate_default_value(value),
            Self::Iterator(x) => x.validate_default_value(value),
        }
    }
}

impl From<Primitive> for FunctionReturnStructField {
    fn from(x: Primitive) -> Self {
        Self::Basic(x.into())
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

impl From<AbstractIteratorHandle> for FunctionReturnStructField {
    fn from(x: AbstractIteratorHandle) -> Self {
        Self::Iterator(x)
    }
}
