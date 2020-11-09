use jni::objects::JValue;
use jni::signature::*;

pub struct Duration {
    class: jni::objects::GlobalRef,
    of_millis_method: jni::objects::JStaticMethodID<'static>,
    of_seconds_method: jni::objects::JStaticMethodID<'static>,
    to_millis_method: jni::objects::JMethodID<'static>,
    get_seconds_method: jni::objects::JMethodID<'static>,
    get_nano_method: jni::objects::JMethodID<'static>,
    with_nanos_method: jni::objects::JMethodID<'static>,
}

impl Duration {
    pub fn init(env: &jni::JNIEnv) -> Self {
        let class = env.find_class("Ljava/time/Duration;").expect("Unable to find java/time/Duration class");

        let of_millis_method = env.get_static_method_id(class, "ofMillis", "(J)Ljava/time/Duration;").map(|mid| mid.into_inner().into()).expect("Unable to find Duration::ofMillis()");
        let of_seconds_method = env.get_static_method_id(class, "ofSeconds", "(J)Ljava/time/Duration;").map(|mid| mid.into_inner().into()).expect("Unable to find Duration::ofSeconds()");
        let to_millis_method = env.get_method_id(class, "toMillis", "()J").map(|mid| mid.into_inner().into()).expect("Unable to find Duration::toMillis()");
        let get_seconds_method = env.get_method_id(class, "getSeconds", "()J").map(|mid| mid.into_inner().into()).expect("Unable to find Duration::getSeconds()");
        let get_nano_method = env.get_method_id(class, "getNano", "()I").map(|mid| mid.into_inner().into()).expect("Unable to find Duration::getNano()");
        let with_nanos_method = env.get_method_id(class, "withNanos", "(I)Ljava/time/Duration;").map(|mid| mid.into_inner().into()).expect("Unable to find Duration::withNanos()");

        Self {
            class: env.new_global_ref(class).unwrap(),
            of_millis_method,
            of_seconds_method,
            to_millis_method,
            get_seconds_method,
            get_nano_method,
            with_nanos_method,
        }
    }

    pub fn duration_to_millis(&self, env: &jni::JNIEnv, obj: jni::sys::jobject) -> u64 {
        env.call_method_unchecked(obj, self.to_millis_method, JavaType::Primitive(Primitive::Long), &[]).unwrap()
            .j().unwrap() as u64
    }

    pub fn duration_from_millis(&self, env: &jni::JNIEnv, millis: u64) -> jni::sys::jobject {
        env.call_static_method_unchecked(&self.class, self.of_millis_method, JavaType::Object("java/time/Duration".to_string()), &[JValue::Long(millis as i64)]).unwrap()
            .l().unwrap().into_inner()
    }

    pub fn duration_to_seconds(&self, env: &jni::JNIEnv, obj: jni::sys::jobject) -> u64 {
        env.call_method_unchecked(obj, self.get_seconds_method, JavaType::Primitive(Primitive::Long), &[]).unwrap()
            .j().unwrap() as u64
    }

    pub fn duration_from_seconds(&self, env: &jni::JNIEnv, seconds: u64) -> jni::sys::jobject {
        env.call_static_method_unchecked(&self.class, self.of_seconds_method, JavaType::Object("java/time/Duration".to_string()), &[JValue::Long(seconds as i64)]).unwrap()
            .l().unwrap().into_inner()
    }

    pub fn duration_to_seconds_float(&self, env: &jni::JNIEnv, obj: jni::sys::jobject) -> f32 {
        let seconds = env.call_method_unchecked(obj, self.get_seconds_method, JavaType::Primitive(Primitive::Long), &[]).unwrap()
            .j().unwrap() as u64;

        let nanos = env.call_method_unchecked(obj, self.get_nano_method, JavaType::Primitive(Primitive::Int), &[]).unwrap()
            .i().unwrap() as u64;

        (seconds as f32) + ((nanos as f32) / 1_000_000_000.0)
    }

    pub fn duration_from_seconds_float(&self, env: &jni::JNIEnv, value: f32) -> jni::sys::jobject {
        let seconds = value as i64;
        let nano = ((value - value.trunc()) * 1_000_000_000.0) as i32;

        let obj = env.call_static_method_unchecked(&self.class, self.of_seconds_method, JavaType::Object("java/time/Duration".to_string()), &[JValue::Long(seconds)]).unwrap()
            .l().unwrap();

        env.call_method_unchecked(obj, self.with_nanos_method, JavaType::Object("java/time/Duration".to_string()), &[JValue::Int(nano)]).unwrap()
            .l().unwrap().into_inner()
    }
}
