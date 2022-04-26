use oo_bindgen::model::*;

/// the raw JNI type for non-primitives (except strings which have their own special type)
const JNI_SYS_JOBJECT: &str = "jni::sys::jobject";

/// Trait for types used in JNI function signatures
pub(crate) trait JniSignatureType {
    /// get the raw JNI type (from jni::sys::* module) used in
    /// the Rust JNI function signatures
    fn jni_signature_type(&self) -> &str;
}

impl JniSignatureType for Primitive {
    fn jni_signature_type(&self) -> &str {
        match self {
            Self::Bool => "jni::sys::jboolean",
            Self::U8 => JNI_SYS_JOBJECT,
            Self::S8 => "jni::sys::jbyte",
            Self::U16 => JNI_SYS_JOBJECT,
            Self::S16 => "jni::sys::jshort",
            Self::U32 => JNI_SYS_JOBJECT,
            Self::S32 => "jni::sys::jint",
            Self::U64 => JNI_SYS_JOBJECT,
            Self::S64 => "jni::sys::jlong",
            Self::Float => "jni::sys::jfloat",
            Self::Double => "jni::sys::jdouble",
        }
    }
}

impl JniSignatureType for DurationType {
    fn jni_signature_type(&self) -> &str {
        JNI_SYS_JOBJECT
    }
}

impl JniSignatureType for EnumHandle {
    fn jni_signature_type(&self) -> &str {
        JNI_SYS_JOBJECT
    }
}

impl JniSignatureType for BasicType {
    fn jni_signature_type(&self) -> &str {
        match self {
            BasicType::Primitive(x) => x.jni_signature_type(),
            BasicType::Duration(x) => x.jni_signature_type(),
            BasicType::Enum(x) => x.jni_signature_type(),
        }
    }
}

impl JniSignatureType for StringType {
    fn jni_signature_type(&self) -> &str {
        "jni::sys::jstring"
    }
}

impl JniSignatureType for CollectionHandle {
    fn jni_signature_type(&self) -> &str {
        JNI_SYS_JOBJECT
    }
}

impl<T> JniSignatureType for UniversalOr<T>
where
    T: StructFieldType,
{
    fn jni_signature_type(&self) -> &str {
        JNI_SYS_JOBJECT
    }
}

impl<T> JniSignatureType for TypedStructDeclaration<T> {
    fn jni_signature_type(&self) -> &str {
        JNI_SYS_JOBJECT
    }
}

impl<T> JniSignatureType for UniversalDeclarationOr<T>
where
    T: StructFieldType,
{
    fn jni_signature_type(&self) -> &str {
        JNI_SYS_JOBJECT
    }
}

impl JniSignatureType for ClassDeclarationHandle {
    fn jni_signature_type(&self) -> &str {
        JNI_SYS_JOBJECT
    }
}

impl JniSignatureType for InterfaceHandle {
    fn jni_signature_type(&self) -> &str {
        JNI_SYS_JOBJECT
    }
}

impl JniSignatureType for FunctionArgument {
    fn jni_signature_type(&self) -> &str {
        match self {
            FunctionArgument::Basic(x) => x.jni_signature_type(),
            FunctionArgument::String(x) => x.jni_signature_type(),
            FunctionArgument::Collection(x) => x.jni_signature_type(),
            FunctionArgument::Struct(x) => x.jni_signature_type(),
            FunctionArgument::StructRef(x) => x.jni_signature_type(),
            FunctionArgument::ClassRef(x) => x.jni_signature_type(),
            FunctionArgument::Interface(x) => x.jni_signature_type(),
        }
    }
}

impl JniSignatureType for PrimitiveRef {
    fn jni_signature_type(&self) -> &str {
        JNI_SYS_JOBJECT
    }
}

impl JniSignatureType for FunctionReturnValue {
    fn jni_signature_type(&self) -> &str {
        match self {
            FunctionReturnValue::Basic(x) => x.jni_signature_type(),
            FunctionReturnValue::String(x) => x.jni_signature_type(),
            FunctionReturnValue::ClassRef(x) => x.jni_signature_type(),
            FunctionReturnValue::Struct(x) => x.jni_signature_type(),
            FunctionReturnValue::StructRef(x) => x.jni_signature_type(),
            FunctionReturnValue::PrimitiveRef(x) => x.jni_signature_type(),
        }
    }
}
