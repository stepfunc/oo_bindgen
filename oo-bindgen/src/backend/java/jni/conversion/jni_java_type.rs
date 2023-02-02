use crate::model::*;

/// The string argument is not used in `call_method_unchecked` in the jni crate
const OBJECT_TYPE: &str = "jni::signature::JavaType::Object(String::new())";

pub(crate) trait JniJavaType {
    fn jni_java_type(&self) -> &'static str;
}

impl JniJavaType for Primitive {
    fn jni_java_type(&self) -> &'static str {
        match self {
            Primitive::Bool => {
                "jni::signature::JavaType::Primitive(jni::signature::Primitive::Boolean)"
            }
            Primitive::U8 => OBJECT_TYPE,
            Primitive::S8 => "jni::signature::JavaType::Primitive(jni::signature::Primitive::Byte)",
            Primitive::U16 => OBJECT_TYPE,
            Primitive::S16 => {
                "jni::signature::JavaType::Primitive(jni::signature::Primitive::Short)"
            }
            Primitive::U32 => OBJECT_TYPE,
            Primitive::S32 => "jni::signature::JavaType::Primitive(jni::signature::Primitive::Int)",
            Primitive::U64 => OBJECT_TYPE,
            Primitive::S64 => {
                "jni::signature::JavaType::Primitive(jni::signature::Primitive::Long)"
            }
            Primitive::Float => {
                "jni::signature::JavaType::Primitive(jni::signature::Primitive::Float)"
            }
            Primitive::Double => {
                "jni::signature::JavaType::Primitive(jni::signature::Primitive::Double)"
            }
        }
    }
}

impl JniJavaType for DurationType {
    fn jni_java_type(&self) -> &'static str {
        OBJECT_TYPE
    }
}

impl JniJavaType for EnumHandle {
    fn jni_java_type(&self) -> &'static str {
        OBJECT_TYPE
    }
}

impl JniJavaType for BasicType {
    fn jni_java_type(&self) -> &'static str {
        match self {
            BasicType::Primitive(x) => x.jni_java_type(),
            BasicType::Duration(x) => x.jni_java_type(),
            BasicType::Enum(x) => x.jni_java_type(),
        }
    }
}

impl JniJavaType for StringType {
    fn jni_java_type(&self) -> &'static str {
        OBJECT_TYPE
    }
}

impl JniJavaType for AsynchronousInterface {
    fn jni_java_type(&self) -> &'static str {
        OBJECT_TYPE
    }
}

impl JniJavaType for UniversalOr<FunctionArgStructField> {
    fn jni_java_type(&self) -> &'static str {
        OBJECT_TYPE
    }
}

impl JniJavaType for UniversalStructHandle {
    fn jni_java_type(&self) -> &'static str {
        OBJECT_TYPE
    }
}

impl JniJavaType for FunctionArgStructField {
    fn jni_java_type(&self) -> &'static str {
        match self {
            FunctionArgStructField::Basic(x) => x.jni_java_type(),
            FunctionArgStructField::String(x) => x.jni_java_type(),
            FunctionArgStructField::Interface(x) => x.jni_java_type(),
            FunctionArgStructField::Struct(x) => x.jni_java_type(),
        }
    }
}

impl JniJavaType for UniversalStructField {
    fn jni_java_type(&self) -> &'static str {
        match self {
            UniversalStructField::Basic(x) => x.jni_java_type(),
            UniversalStructField::Struct(x) => x.jni_java_type(),
            UniversalStructField::String(x) => x.jni_java_type(),
        }
    }
}

impl JniJavaType for CallbackReturnValue {
    fn jni_java_type(&self) -> &'static str {
        match self {
            CallbackReturnValue::Basic(x) => x.jni_java_type(),
            CallbackReturnValue::Struct(x) => x.jni_java_type(),
        }
    }
}

impl JniJavaType for OptionalReturnType<CallbackReturnValue, Validated> {
    fn jni_java_type(&self) -> &'static str {
        match self.get_value() {
            None => "jni::signature::JavaType::Primitive(jni::signature::Primitive::Void)",
            Some(x) => x.jni_java_type(),
        }
    }
}
