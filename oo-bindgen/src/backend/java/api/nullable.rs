use crate::model::*;

pub(crate) trait Nullable {
    fn is_nullable(&self) -> bool;
}

pub(crate) trait IsStruct {
    fn is_struct(&self) -> bool;
}

impl Nullable for Primitive {
    // the unsigned types are wrapper objects
    fn is_nullable(&self) -> bool {
        match self {
            Primitive::Bool => false,
            Primitive::U8 => true,
            Primitive::S8 => false,
            Primitive::U16 => true,
            Primitive::S16 => false,
            Primitive::U32 => true,
            Primitive::S32 => false,
            Primitive::U64 => true,
            Primitive::S64 => false,
            Primitive::Float => false,
            Primitive::Double => false,
        }
    }
}

impl Nullable for BasicType {
    fn is_nullable(&self) -> bool {
        match self {
            BasicType::Primitive(x) => x.is_nullable(),
            BasicType::Duration(_) => true,
            BasicType::Enum(_) => true,
        }
    }
}

impl Nullable for FunctionArgument {
    fn is_nullable(&self) -> bool {
        match self {
            Self::Basic(x) => x.is_nullable(),
            Self::String(_) => true,
            Self::Collection(_) => true,
            Self::Struct(_) => true,
            Self::StructRef(_) => true,
            Self::ClassRef(_) => true,
            Self::Interface(_) => true,
        }
    }
}

impl Nullable for FunctionArgStructField {
    fn is_nullable(&self) -> bool {
        match self {
            FunctionArgStructField::Basic(x) => x.is_nullable(),
            FunctionArgStructField::String(_) => true,
            FunctionArgStructField::Interface(_) => true,
            FunctionArgStructField::Struct(_) => true,
        }
    }
}

impl Nullable for FunctionReturnStructField {
    fn is_nullable(&self) -> bool {
        match self {
            FunctionReturnStructField::Basic(x) => x.is_nullable(),
            FunctionReturnStructField::ClassRef(_) => true,
            FunctionReturnStructField::Iterator(_) => true,
            FunctionReturnStructField::Struct(_) => true,
        }
    }
}

impl Nullable for CallbackArgStructField {
    fn is_nullable(&self) -> bool {
        match self {
            CallbackArgStructField::Basic(x) => x.is_nullable(),
            CallbackArgStructField::Iterator(_) => true,
            CallbackArgStructField::Struct(_) => true,
            CallbackArgStructField::String(_) => true,
        }
    }
}

impl Nullable for UniversalStructField {
    fn is_nullable(&self) -> bool {
        match self {
            UniversalStructField::Basic(x) => x.is_nullable(),
            UniversalStructField::Struct(_) => true,
            UniversalStructField::String(_) => false,
        }
    }
}

impl IsStruct for FunctionArgStructField {
    fn is_struct(&self) -> bool {
        match self {
            FunctionArgStructField::Basic(_) => false,
            FunctionArgStructField::String(_) => false,
            FunctionArgStructField::Interface(_) => false,
            FunctionArgStructField::Struct(_) => true,
        }
    }
}

impl IsStruct for FunctionReturnStructField {
    fn is_struct(&self) -> bool {
        match self {
            FunctionReturnStructField::Basic(_) => false,
            FunctionReturnStructField::ClassRef(_) => false,
            FunctionReturnStructField::Iterator(_) => false,
            FunctionReturnStructField::Struct(_) => true,
        }
    }
}

impl IsStruct for CallbackArgStructField {
    fn is_struct(&self) -> bool {
        match self {
            CallbackArgStructField::Basic(_) => false,
            CallbackArgStructField::Iterator(_) => false,
            CallbackArgStructField::Struct(_) => true,
            CallbackArgStructField::String(_) => false,
        }
    }
}

impl IsStruct for UniversalStructField {
    fn is_struct(&self) -> bool {
        match self {
            UniversalStructField::Basic(_) => false,
            UniversalStructField::Struct(_) => true,
            UniversalStructField::String(_) => false,
        }
    }
}

impl IsStruct for FunctionArgument {
    fn is_struct(&self) -> bool {
        match self {
            FunctionArgument::Basic(_) => false,
            FunctionArgument::String(_) => false,
            FunctionArgument::Collection(_) => false,
            FunctionArgument::Struct(_) => true,
            FunctionArgument::StructRef(_) => true,
            FunctionArgument::ClassRef(_) => false,
            FunctionArgument::Interface(_) => false,
        }
    }
}
