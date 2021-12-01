use oo_bindgen::model::*;

const NULL_DEFAULT_VALUE: &str = "jni::objects::JObject::null().into_inner()";

pub(crate) trait DefaultValue {
    /// Returns the default raw JNI type value (used when throwing exceptions). Almost always `JObject::null` except for native types.
    fn get_default_value(&self) -> &str;
}

impl DefaultValue for Primitive {
    fn get_default_value(&self) -> &str {
        match self {
            Self::Bool => "0",
            Self::U8 => NULL_DEFAULT_VALUE,
            Self::S8 => "0",
            Self::U16 => NULL_DEFAULT_VALUE,
            Self::S16 => "0",
            Self::U32 => NULL_DEFAULT_VALUE,
            Self::S32 => "0",
            Self::U64 => NULL_DEFAULT_VALUE,
            Self::S64 => "0",
            Self::Float => "0.0",
            Self::Double => "0.0",
        }
    }
}

impl DefaultValue for DurationType {
    fn get_default_value(&self) -> &str {
        NULL_DEFAULT_VALUE
    }
}

impl DefaultValue for EnumHandle {
    fn get_default_value(&self) -> &str {
        NULL_DEFAULT_VALUE
    }
}

impl DefaultValue for BasicType {
    fn get_default_value(&self) -> &str {
        match self {
            BasicType::Primitive(x) => x.get_default_value(),
            BasicType::Duration(x) => x.get_default_value(),
            BasicType::Enum(x) => x.get_default_value(),
        }
    }
}

impl DefaultValue for StringType {
    fn get_default_value(&self) -> &str {
        NULL_DEFAULT_VALUE
    }
}

impl DefaultValue for ClassDeclarationHandle {
    fn get_default_value(&self) -> &str {
        NULL_DEFAULT_VALUE
    }
}

impl DefaultValue for UniversalOr<FunctionReturnStructField> {
    fn get_default_value(&self) -> &str {
        NULL_DEFAULT_VALUE
    }
}

impl DefaultValue for UniversalDeclarationOr<FunctionReturnStructField> {
    fn get_default_value(&self) -> &str {
        NULL_DEFAULT_VALUE
    }
}

impl DefaultValue for FunctionReturnValue {
    fn get_default_value(&self) -> &str {
        match self {
            FunctionReturnValue::Basic(x) => x.get_default_value(),
            FunctionReturnValue::String(x) => x.get_default_value(),
            FunctionReturnValue::ClassRef(x) => x.get_default_value(),
            FunctionReturnValue::Struct(x) => x.get_default_value(),
            FunctionReturnValue::StructRef(x) => x.get_default_value(),
        }
    }
}
