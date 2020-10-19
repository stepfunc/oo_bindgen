use jni::objects::JValue;
use jni::signature::*;

pub struct Joou {
    class_ubyte: jni::objects::GlobalRef,
    class_ushort: jni::objects::GlobalRef,
    class_uinteger: jni::objects::GlobalRef,
    class_ulong: jni::objects::GlobalRef,
    ubyte_to_long: jni::objects::JMethodID<'static>,
    ubyte_from_long: jni::objects::JStaticMethodID<'static>,
    ushort_to_long: jni::objects::JMethodID<'static>,
    ushort_from_long: jni::objects::JStaticMethodID<'static>,
    uinteger_to_long: jni::objects::JMethodID<'static>,
    uinteger_from_long: jni::objects::JStaticMethodID<'static>,
    ulong_to_long: jni::objects::JMethodID<'static>,
    ulong_from_long: jni::objects::JStaticMethodID<'static>,
}

impl Joou {
    pub fn init(env: &jni::JNIEnv) -> Self {
        let class_ubyte = env.find_class("Lorg/joou/UByte;").expect("Unable to find org/joou/UByte class");
        let ubyte_to_long = env.get_method_id(class_ubyte, "longValue", "()J").map(|mid| mid.into_inner().into()).expect("Unable to find UByte::longValue");
        let ubyte_from_long = env.get_static_method_id(class_ubyte, "valueOf", "(J)Lorg/joou/UByte;").map(|mid| mid.into_inner().into()).expect("Unable to find UByte::valueOf");

        let class_ushort = env.find_class("Lorg/joou/UShort;").expect("Unable to find org/joou/UShort class");
        let ushort_to_long = env.get_method_id(class_ushort, "longValue", "()J").map(|mid| mid.into_inner().into()).expect("Unable to find UShort::longValue");
        let ushort_from_long = env.get_static_method_id(class_ushort, "valueOf", "(I)Lorg/joou/UShort;").map(|mid| mid.into_inner().into()).expect("Unable to find UShort::valueOf");

        let class_uinteger = env.find_class("Lorg/joou/UInteger;").expect("Unable to find org/joou/UInteger class");
        let uinteger_to_long = env.get_method_id(class_uinteger, "longValue", "()J").map(|mid| mid.into_inner().into()).expect("Unable to find UInteger::longValue");
        let uinteger_from_long = env.get_static_method_id(class_uinteger, "valueOf", "(J)Lorg/joou/UInteger;").map(|mid| mid.into_inner().into()).expect("Unable to find UInteger::valueOf");

        let class_ulong = env.find_class("Lorg/joou/ULong;").expect("Unable to find org/joou/ULong class");
        let ulong_to_long = env.get_method_id(class_ulong, "longValue", "()J").map(|mid| mid.into_inner().into()).expect("Unable to find ULong::longValue");
        let ulong_from_long = env.get_static_method_id(class_ulong, "valueOf", "(J)Lorg/joou/ULong;").map(|mid| mid.into_inner().into()).expect("Unable to find ULong::valueOf");

        Self {
            class_ubyte: env.new_global_ref(class_ubyte).unwrap(),
            class_ushort: env.new_global_ref(class_ushort).unwrap(),
            class_uinteger: env.new_global_ref(class_uinteger).unwrap(),
            class_ulong: env.new_global_ref(class_ulong).unwrap(),
            ubyte_to_long,
            ubyte_from_long,
            ushort_to_long,
            ushort_from_long,
            uinteger_to_long,
            uinteger_from_long,
            ulong_to_long,
            ulong_from_long,
        }
    }

    pub fn ubyte_to_rust(&self, env: &jni::JNIEnv, obj: jni::sys::jobject) -> u8 {
        env.call_method_unchecked(obj, self.ubyte_to_long, JavaType::Primitive(Primitive::Long), &[]).unwrap()
            .j().unwrap() as u8
    }

    pub fn ubyte_from_rust(&self, env: &jni::JNIEnv, value: u8) -> jni::sys::jobject {
        env.call_static_method_unchecked(&self.class_ubyte, self.ubyte_from_long, JavaType::Object("org/joou/UByte".to_string()), &[JValue::Long(value as i64)]).unwrap()
            .l().unwrap().into_inner()
    }

    pub fn ushort_to_rust(&self, env: &jni::JNIEnv, obj: jni::sys::jobject) -> u16 {
        env.call_method_unchecked(obj, self.ushort_to_long, JavaType::Primitive(Primitive::Long), &[]).unwrap()
            .j().unwrap() as u16
    }

    pub fn ushort_from_rust(&self, env: &jni::JNIEnv, value: u16) -> jni::sys::jobject {
        env.call_static_method_unchecked(&self.class_ubyte, self.ushort_from_long, JavaType::Object("org/joou/UShort".to_string()), &[JValue::Int(value as i32)]).unwrap()
            .l().unwrap().into_inner()
    }

    pub fn uinteger_to_rust(&self, env: &jni::JNIEnv, obj: jni::sys::jobject) -> u32 {
        env.call_method_unchecked(obj, self.uinteger_to_long, JavaType::Primitive(Primitive::Long), &[]).unwrap()
            .j().unwrap() as u32
    }

    pub fn uinteger_from_rust(&self, env: &jni::JNIEnv, value: u32) -> jni::sys::jobject {
        env.call_static_method_unchecked(&self.class_ubyte, self.uinteger_from_long, JavaType::Object("org/joou/UInteger".to_string()), &[JValue::Long(value as i64)]).unwrap()
            .l().unwrap().into_inner()
    }

    pub fn ulong_to_rust(&self, env: &jni::JNIEnv, obj: jni::sys::jobject) -> u64 {
        env.call_method_unchecked(obj, self.ulong_to_long, JavaType::Primitive(Primitive::Long), &[]).unwrap()
            .j().unwrap() as u64
    }

    pub fn ulong_from_rust(&self, env: &jni::JNIEnv, value: u64) -> jni::sys::jobject {
        env.call_static_method_unchecked(&self.class_ubyte, self.ulong_from_long, JavaType::Object("org/joou/ULong".to_string()), &[JValue::Long(value as i64)]).unwrap()
            .l().unwrap().into_inner()
    }
}
