use jni::objects::JValue;
use jni::signature::*;

pub(crate) struct UByte {
    class: jni::objects::GlobalRef,
    to_rust: jni::objects::JMethodID<'static>,
    to_jni: jni::objects::JStaticMethodID<'static>,
}

pub(crate) struct UShort {
    class: jni::objects::GlobalRef,
    to_rust: jni::objects::JMethodID<'static>,
    to_jni: jni::objects::JStaticMethodID<'static>,
}

pub(crate) struct UInteger {
    class: jni::objects::GlobalRef,
    to_rust: jni::objects::JMethodID<'static>,
    to_jni: jni::objects::JStaticMethodID<'static>,
}

pub(crate) struct ULong {
    class: jni::objects::GlobalRef,
    to_rust: jni::objects::JMethodID<'static>,
    to_jni: jni::objects::JStaticMethodID<'static>,
}

impl UByte {
    fn init(env: &jni::JNIEnv) -> Self {
        let class = env.find_class("org/joou/UByte").expect("Unable to find org/joou/UByte class");
        let to_rust = env.get_method_id(class, "longValue", "()J").map(|mid| mid.into_inner().into()).expect("Unable to find UByte::longValue");
        let to_jni = env.get_static_method_id(class, "valueOf", "(J)Lorg/joou/UByte;").map(|mid| mid.into_inner().into()).expect("Unable to find UByte::valueOf");
        Self {
            class: env.new_global_ref(class).unwrap(),
            to_rust,
            to_jni
        }
    }

    pub(crate) fn to_rust(&self, env: &jni::JNIEnv, obj: jni::sys::jobject) -> u8 {
        env.call_method_unchecked(obj, self.to_rust, JavaType::Primitive(Primitive::Long), &[]).unwrap()
            .j().unwrap() as u8
    }

    pub(crate) fn to_jni(&self, env: &jni::JNIEnv, value: u8) -> jni::sys::jobject {
        env.call_static_method_unchecked(&self.class, self.to_jni, JavaType::Object("org/joou/UByte".to_string()), &[JValue::Long(value as i64)]).unwrap()
            .l().unwrap().into_inner()
    }
}

impl UShort {
    fn init(env: &jni::JNIEnv) -> Self {
        let class = env.find_class("org/joou/UShort").expect("Unable to find org/joou/UShort class");
        let to_rust = env.get_method_id(class, "longValue", "()J").map(|mid| mid.into_inner().into()).expect("Unable to find UShort::longValue");
        let to_jni = env.get_static_method_id(class, "valueOf", "(I)Lorg/joou/UShort;").map(|mid| mid.into_inner().into()).expect("Unable to find UShort::valueOf");
        Self {
            class: env.new_global_ref(class).unwrap(),
            to_rust,
            to_jni
        }
    }

    pub(crate) fn to_rust(&self, env: &jni::JNIEnv, obj: jni::sys::jobject) -> u16 {
        env.call_method_unchecked(obj, self.to_rust, JavaType::Primitive(Primitive::Long), &[]).unwrap()
            .j().unwrap() as u16
    }

    pub(crate) fn to_jni(&self, env: &jni::JNIEnv, value: u16) -> jni::sys::jobject {
        env.call_static_method_unchecked(&self.class, self.to_jni, JavaType::Object("org/joou/UShort".to_string()), &[JValue::Int(value as i32)]).unwrap()
            .l().unwrap().into_inner()
    }
}

impl UInteger {
    fn init(env: &jni::JNIEnv) -> Self {
        let class = env.find_class("org/joou/UInteger").expect("Unable to find org/joou/UInteger class");
        let to_rust = env.get_method_id(class, "longValue", "()J").map(|mid| mid.into_inner().into()).expect("Unable to find UInteger::longValue");
        let to_jni = env.get_static_method_id(class, "valueOf", "(J)Lorg/joou/UInteger;").map(|mid| mid.into_inner().into()).expect("Unable to find UInteger::valueOf");
        Self {
            class: env.new_global_ref(class).unwrap(),
            to_rust,
            to_jni
        }
    }

    pub(crate) fn to_rust(&self, env: &jni::JNIEnv, obj: jni::sys::jobject) -> u32 {
        env.call_method_unchecked(obj, self.to_rust, JavaType::Primitive(Primitive::Long), &[]).unwrap()
            .j().unwrap() as u32
    }

    pub(crate) fn to_jni(&self, env: &jni::JNIEnv, value: u32) -> jni::sys::jobject {
        env.call_static_method_unchecked(&self.class, self.to_jni, JavaType::Object("org/joou/UInteger".to_string()), &[JValue::Long(value as i64)]).unwrap()
            .l().unwrap().into_inner()
    }
}

impl ULong {
    fn init(env: &jni::JNIEnv) -> Self {
        let class = env.find_class("org/joou/ULong").expect("Unable to find org/joou/ULong class");
        let to_rust = env.get_method_id(class, "longValue", "()J").map(|mid| mid.into_inner().into()).expect("Unable to find ULong::longValue");
        let to_jni = env.get_static_method_id(class, "valueOf", "(J)Lorg/joou/ULong;").map(|mid| mid.into_inner().into()).expect("Unable to find ULong::valueOf");
        Self {
            class: env.new_global_ref(class).unwrap(),
            to_rust,
            to_jni
        }
    }

    pub(crate) fn to_rust(&self, env: &jni::JNIEnv, obj: jni::sys::jobject) -> u64 {
        env.call_method_unchecked(obj, self.to_rust, JavaType::Primitive(Primitive::Long), &[]).unwrap()
            .j().unwrap() as u64
    }

    pub(crate) fn to_jni(&self, env: &jni::JNIEnv, value: u64) -> jni::sys::jobject {
        env.call_static_method_unchecked(&self.class, self.to_jni, JavaType::Object("org/joou/ULong".to_string()), &[JValue::Long(value as i64)]).unwrap()
            .l().unwrap().into_inner()
    }
}



pub(crate) struct Unsigned {
    pub(crate) byte: UByte,
    pub(crate) short: UShort,
    pub(crate) integer: UInteger,
    pub(crate) long: ULong,
}

impl Unsigned {
    pub(crate) fn init(env: &jni::JNIEnv) -> Self {
        Self {
            byte: UByte::init(env),
            short: UShort::init(env),
            integer: UInteger::init(env),
            long: ULong::init(env),
        }
    }
}
