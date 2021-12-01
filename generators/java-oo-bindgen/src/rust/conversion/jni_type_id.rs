use crate::rust::conversion::JniType;
use oo_bindgen::model::*;

fn jni_object_sig(lib_path: &str, object_name: &Name) -> String {
    format!("L{}/{};", lib_path, object_name.camel_case())
}

pub(crate) trait JniTypeId {
    /// get the JNI identifier of the type used to disambiguate methods
    fn jni_type_id(&self, lib_path: &str) -> String;
}

impl JniTypeId for Primitive {
    fn jni_type_id(&self, _lib_path: &str) -> String {
        match self {
            Self::Bool => "Z".to_string(),
            Self::U8 => "Lorg/joou/UByte;".to_string(),
            Self::S8 => "B".to_string(),
            Self::U16 => "Lorg/joou/UShort;".to_string(),
            Self::S16 => "S".to_string(),
            Self::U32 => "Lorg/joou/UInteger;".to_string(),
            Self::S32 => "I".to_string(),
            Self::U64 => "Lorg/joou/ULong;".to_string(),
            Self::S64 => "J".to_string(),
            Self::Float => "F".to_string(),
            Self::Double => "D".to_string(),
        }
    }
}

impl JniTypeId for StringType {
    fn jni_type_id(&self, _lib_path: &str) -> String {
        "Ljava/lang/String;".to_string()
    }
}

impl JniTypeId for DurationType {
    fn jni_type_id(&self, _lib_path: &str) -> String {
        "Ljava/time/Duration;".to_string()
    }
}

impl JniTypeId for EnumHandle {
    fn jni_type_id(&self, lib_path: &str) -> String {
        format!("L{}/{};", lib_path, self.name.camel_case())
    }
}

impl JniTypeId for BasicType {
    fn jni_type_id(&self, lib_path: &str) -> String {
        match self {
            BasicType::Primitive(x) => x.jni_type_id(lib_path),
            BasicType::Duration(x) => x.jni_type_id(lib_path),
            BasicType::Enum(x) => x.jni_type_id(lib_path),
        }
    }
}

impl JniTypeId for AbstractIteratorHandle {
    fn jni_type_id(&self, _lib_path: &str) -> String {
        "Ljava/util/List;".to_string()
    }
}

impl JniTypeId for ClassDeclarationHandle {
    fn jni_type_id(&self, lib_path: &str) -> String {
        jni_object_sig(lib_path, &self.name)
    }
}

impl<T> JniTypeId for UniversalOr<T>
where
    T: StructFieldType,
{
    fn jni_type_id(&self, lib_path: &str) -> String {
        jni_object_sig(lib_path, self.name())
    }
}

impl JniTypeId for UniversalStructHandle {
    fn jni_type_id(&self, lib_path: &str) -> String {
        jni_object_sig(lib_path, &self.name())
    }
}

impl JniTypeId for CallbackArgument {
    fn jni_type_id(&self, lib_path: &str) -> String {
        match self {
            CallbackArgument::Basic(x) => x.jni_type_id(lib_path),
            CallbackArgument::String(x) => x.jni_type_id(lib_path),
            CallbackArgument::Iterator(x) => x.jni_type_id(lib_path),
            CallbackArgument::Class(x) => x.jni_type_id(lib_path),
            CallbackArgument::Struct(x) => x.as_jni_sig(lib_path),
        }
    }
}

impl JniTypeId for CallbackReturnValue {
    fn jni_type_id(&self, lib_path: &str) -> String {
        match self {
            CallbackReturnValue::Basic(x) => x.jni_type_id(lib_path),
            CallbackReturnValue::Struct(x) => x.jni_type_id(lib_path),
        }
    }
}

impl JniTypeId for OptionalReturnType<CallbackReturnValue, Validated> {
    fn jni_type_id(&self, lib_path: &str) -> String {
        match self.get_value() {
            None => "V".to_string(),
            Some(x) => x.jni_type_id(lib_path),
        }
    }
}

impl JniTypeId for InterfaceHandle {
    fn jni_type_id(&self, lib_path: &str) -> String {
        jni_object_sig(lib_path, &self.name)
    }
}

impl JniTypeId for FunctionArgStructField {
    fn jni_type_id(&self, lib_path: &str) -> String {
        match self {
            FunctionArgStructField::Basic(x) => x.jni_type_id(lib_path),
            FunctionArgStructField::String(x) => x.jni_type_id(lib_path),
            FunctionArgStructField::Interface(x) => x.inner.jni_type_id(lib_path),
            FunctionArgStructField::Struct(x) => x.jni_type_id(lib_path),
        }
    }
}

impl JniTypeId for FunctionReturnStructField {
    fn jni_type_id(&self, lib_path: &str) -> String {
        match self {
            FunctionReturnStructField::Basic(x) => x.jni_type_id(lib_path),
            FunctionReturnStructField::ClassRef(x) => x.jni_type_id(lib_path),
            FunctionReturnStructField::Iterator(x) => x.jni_type_id(lib_path),
            FunctionReturnStructField::Struct(x) => x.jni_type_id(lib_path),
        }
    }
}

impl JniTypeId for CallbackArgStructField {
    fn jni_type_id(&self, lib_path: &str) -> String {
        match self {
            CallbackArgStructField::Basic(x) => x.jni_type_id(lib_path),
            CallbackArgStructField::Iterator(x) => x.jni_type_id(lib_path),
            CallbackArgStructField::Struct(x) => x.jni_type_id(lib_path),
        }
    }
}

impl JniTypeId for UniversalStructField {
    fn jni_type_id(&self, lib_path: &str) -> String {
        match self {
            UniversalStructField::Basic(x) => x.jni_type_id(lib_path),
            UniversalStructField::Struct(x) => x.jni_type_id(lib_path),
        }
    }
}
