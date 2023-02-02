use crate::model::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum PassBy {
    Copy,
    ConstRef,
    MutRef,
    Move,
}

pub(crate) trait TypeInfo {
    fn pass_by(&self) -> PassBy;

    fn is_move_type(&self) -> bool {
        self.pass_by() == PassBy::Move
    }
}

impl<D> TypeInfo for Handle<Interface<D>>
where
    D: DocReference,
{
    fn pass_by(&self) -> PassBy {
        match self.mode {
            InterfaceCategory::Synchronous => PassBy::MutRef,
            InterfaceCategory::Asynchronous => PassBy::Move,
            InterfaceCategory::Future => PassBy::Move,
        }
    }
}

impl TypeInfo for Primitive {
    fn pass_by(&self) -> PassBy {
        match self {
            Primitive::Bool => PassBy::Copy,
            Primitive::U8 => PassBy::Copy,
            Primitive::S8 => PassBy::Copy,
            Primitive::U16 => PassBy::Copy,
            Primitive::S16 => PassBy::Copy,
            Primitive::U32 => PassBy::Copy,
            Primitive::S32 => PassBy::Copy,
            Primitive::U64 => PassBy::Copy,
            Primitive::S64 => PassBy::Copy,
            Primitive::Float => PassBy::Copy,
            Primitive::Double => PassBy::Copy,
        }
    }
}

impl TypeInfo for BasicType {
    fn pass_by(&self) -> PassBy {
        match self {
            BasicType::Primitive(x) => x.pass_by(),
            BasicType::Duration(_) => PassBy::Copy,
            BasicType::Enum(_) => PassBy::Copy,
        }
    }
}

impl<T, D> TypeInfo for Handle<Struct<T, D>>
where
    D: DocReference,
    T: StructFieldType + TypeInfo,
{
    fn pass_by(&self) -> PassBy {
        // structs are move types if any of their fields are move types
        if self.fields.iter().any(|f| f.field_type.is_move_type()) {
            PassBy::Move
        } else {
            PassBy::ConstRef
        }
    }
}

impl<T> TypeInfo for UniversalOr<T>
where
    T: StructFieldType + TypeInfo,
{
    fn pass_by(&self) -> PassBy {
        match self {
            UniversalOr::Specific(x) => x.pass_by(),
            UniversalOr::Universal(x) => x.pass_by(),
        }
    }
}

impl TypeInfo for UniversalStructField {
    fn pass_by(&self) -> PassBy {
        match self {
            UniversalStructField::Basic(x) => x.pass_by(),
            UniversalStructField::Struct(x) => x.pass_by(),
        }
    }
}

impl TypeInfo for ClassDeclarationHandle {
    fn pass_by(&self) -> PassBy {
        PassBy::ConstRef
    }
}

impl<D> TypeInfo for Handle<AbstractIterator<D>>
where
    D: DocReference,
{
    fn pass_by(&self) -> PassBy {
        PassBy::Move
    }
}

impl TypeInfo for FunctionReturnStructField {
    fn pass_by(&self) -> PassBy {
        match self {
            FunctionReturnStructField::Basic(x) => x.pass_by(),
            FunctionReturnStructField::ClassRef(x) => x.pass_by(),
            FunctionReturnStructField::Iterator(x) => x.pass_by(),
            FunctionReturnStructField::Struct(x) => x.pass_by(),
        }
    }
}

impl TypeInfo for CallbackArgStructField {
    fn pass_by(&self) -> PassBy {
        match self {
            CallbackArgStructField::Basic(x) => x.pass_by(),
            CallbackArgStructField::Iterator(x) => x.pass_by(),
            CallbackArgStructField::Struct(x) => x.pass_by(),
            CallbackArgStructField::String(x) => x.pass_by(),
        }
    }
}

impl TypeInfo for StringType {
    fn pass_by(&self) -> PassBy {
        PassBy::ConstRef
    }
}

impl TypeInfo for FunctionArgStructField {
    fn pass_by(&self) -> PassBy {
        match self {
            FunctionArgStructField::Basic(x) => x.pass_by(),
            FunctionArgStructField::String(x) => x.pass_by(),
            FunctionArgStructField::Interface(x) => x.inner.pass_by(),
            FunctionArgStructField::Struct(x) => x.pass_by(),
        }
    }
}

impl<D> TypeInfo for Handle<Collection<D>>
where
    D: DocReference,
{
    fn pass_by(&self) -> PassBy {
        PassBy::MutRef
    }
}

impl<T> TypeInfo for TypedStructDeclaration<T> {
    fn pass_by(&self) -> PassBy {
        PassBy::ConstRef
    }
}

impl TypeInfo for FunctionArgument {
    fn pass_by(&self) -> PassBy {
        match self {
            FunctionArgument::Basic(x) => x.pass_by(),
            FunctionArgument::String(x) => x.pass_by(),
            FunctionArgument::Collection(x) => x.pass_by(),
            FunctionArgument::Struct(x) => x.pass_by(),
            FunctionArgument::StructRef(x) => x.pass_by(),
            FunctionArgument::ClassRef(x) => x.pass_by(),
            FunctionArgument::Interface(x) => x.pass_by(),
        }
    }
}
