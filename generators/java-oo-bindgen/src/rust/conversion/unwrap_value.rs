use oo_bindgen::model::*;

const UNWRAP_OBJECT: &str = "l().unwrap().into_inner()";

/// The jni crate uses an enum for all the possible JNI types
///
/// This trait is used to convert from `jni::objects::JValue` to the actual type
/// by unwrapping it as the underlying type.
pub(crate) trait UnwrapValue {
    /// convert from `jni::objects::JValue` to the underlying raw JNI type
    fn unwrap_value(&self) -> &str;
}

impl UnwrapValue for Primitive {
    fn unwrap_value(&self) -> &str {
        match self {
            Self::Bool => "z().unwrap()",
            Self::U8 => "l().unwrap().into_inner()",
            Self::S8 => "b().unwrap()",
            Self::U16 => "l().unwrap().into_inner()",
            Self::S16 => "s().unwrap()",
            Self::U32 => "l().unwrap().into_inner()",
            Self::S32 => "i().unwrap()",
            Self::U64 => "l().unwrap().into_inner()",
            Self::S64 => "j().unwrap()",
            Self::Float => "f().unwrap()",
            Self::Double => "d().unwrap()",
        }
    }
}

impl UnwrapValue for DurationType {
    fn unwrap_value(&self) -> &str {
        match self {
            DurationType::Milliseconds => UNWRAP_OBJECT,
            DurationType::Seconds => UNWRAP_OBJECT,
        }
    }
}

impl UnwrapValue for EnumHandle {
    fn unwrap_value(&self) -> &str {
        UNWRAP_OBJECT
    }
}

impl UnwrapValue for StringType {
    fn unwrap_value(&self) -> &str {
        UNWRAP_OBJECT
    }
}

impl UnwrapValue for BasicType {
    fn unwrap_value(&self) -> &str {
        match self {
            BasicType::Primitive(x) => x.unwrap_value(),
            BasicType::Duration(x) => x.unwrap_value(),
            BasicType::Enum(x) => x.unwrap_value(),
        }
    }
}

impl UnwrapValue for InterfaceHandle {
    fn unwrap_value(&self) -> &str {
        UNWRAP_OBJECT
    }
}

impl UnwrapValue for AsynchronousInterface {
    fn unwrap_value(&self) -> &str {
        self.inner.unwrap_value()
    }
}

impl UnwrapValue for UniversalStructHandle {
    fn unwrap_value(&self) -> &str {
        UNWRAP_OBJECT
    }
}

impl UnwrapValue for ClassDeclarationHandle {
    fn unwrap_value(&self) -> &str {
        UNWRAP_OBJECT
    }
}

impl<T> UnwrapValue for UniversalOr<T>
where
    T: StructFieldType,
{
    fn unwrap_value(&self) -> &str {
        UNWRAP_OBJECT
    }
}

impl UnwrapValue for AbstractIteratorHandle {
    fn unwrap_value(&self) -> &str {
        UNWRAP_OBJECT
    }
}

impl UnwrapValue for CallbackReturnValue {
    fn unwrap_value(&self) -> &str {
        match self {
            CallbackReturnValue::Basic(x) => x.unwrap_value(),
            CallbackReturnValue::Struct(x) => x.unwrap_value(),
        }
    }
}

impl UnwrapValue for FunctionArgStructField {
    fn unwrap_value(&self) -> &str {
        match self {
            FunctionArgStructField::Basic(x) => x.unwrap_value(),
            FunctionArgStructField::String(x) => x.unwrap_value(),
            FunctionArgStructField::Interface(x) => x.unwrap_value(),
            FunctionArgStructField::Struct(x) => x.unwrap_value(),
        }
    }
}

impl UnwrapValue for FunctionReturnStructField {
    fn unwrap_value(&self) -> &str {
        match self {
            FunctionReturnStructField::Basic(x) => x.unwrap_value(),
            FunctionReturnStructField::ClassRef(x) => x.unwrap_value(),
            FunctionReturnStructField::Iterator(x) => x.unwrap_value(),
            FunctionReturnStructField::Struct(x) => x.unwrap_value(),
        }
    }
}

impl UnwrapValue for CallbackArgStructField {
    fn unwrap_value(&self) -> &str {
        match self {
            CallbackArgStructField::Basic(x) => x.unwrap_value(),
            CallbackArgStructField::Iterator(x) => x.unwrap_value(),
            CallbackArgStructField::Struct(x) => x.unwrap_value(),
        }
    }
}

impl UnwrapValue for UniversalStructField {
    fn unwrap_value(&self) -> &str {
        match self {
            UniversalStructField::Basic(x) => x.unwrap_value(),
            UniversalStructField::Struct(x) => x.unwrap_value(),
        }
    }
}
