use jni::signature::*;

pub struct Primitives {
    _class_boolean: jni::objects::GlobalRef,
    _class_byte: jni::objects::GlobalRef,
    _class_short: jni::objects::GlobalRef,
    _class_integer: jni::objects::GlobalRef,
    _class_long: jni::objects::GlobalRef,
    _class_float: jni::objects::GlobalRef,
    _class_double: jni::objects::GlobalRef,
    boolean_value: jni::objects::JMethodID<'static>,
    byte_value: jni::objects::JMethodID<'static>,
    short_value: jni::objects::JMethodID<'static>,
    integer_value: jni::objects::JMethodID<'static>,
    long_value: jni::objects::JMethodID<'static>,
    float_value: jni::objects::JMethodID<'static>,
    double_value: jni::objects::JMethodID<'static>,
}

impl Primitives {
    pub fn init(env: &jni::JNIEnv) -> Self {
        let class_boolean = env.find_class("Ljava/lang/Boolean;").expect("Unable to find java/lang/Boolean class");
        let boolean_value = env.get_method_id(class_boolean, "booleanValue", "()Z").map(|mid| mid.into_inner().into()).expect("Unable to find Boolean::booleanValue");

        let class_byte = env.find_class("Ljava/lang/Byte;").expect("Unable to find java/lang/Byte class");
        let byte_value = env.get_method_id(class_byte, "byteValue", "()B").map(|mid| mid.into_inner().into()).expect("Unable to find Byte::byteValue");

        let class_short = env.find_class("Ljava/lang/Short;").expect("Unable to find java/lang/Short class");
        let short_value = env.get_method_id(class_short, "shortValue", "()S").map(|mid| mid.into_inner().into()).expect("Unable to find Short::shortValue");

        let class_integer = env.find_class("Ljava/lang/Integer;").expect("Unable to find java/lang/Integer class");
        let integer_value = env.get_method_id(class_integer, "intValue", "()I").map(|mid| mid.into_inner().into()).expect("Unable to find Integer::intValue");

        let class_long = env.find_class("Ljava/lang/Long;").expect("Unable to find java/lang/Long class");
        let long_value = env.get_method_id(class_long, "longValue", "()J").map(|mid| mid.into_inner().into()).expect("Unable to find Long::longValue");

        let class_float = env.find_class("Ljava/lang/Float;").expect("Unable to find java/lang/Float class");
        let float_value = env.get_method_id(class_float, "floatValue", "()F").map(|mid| mid.into_inner().into()).expect("Unable to find Float::floatValue");

        let class_double = env.find_class("Ljava/lang/Double;").expect("Unable to find java/lang/Double class");
        let double_value = env.get_method_id(class_double, "doubleValue", "()D").map(|mid| mid.into_inner().into()).expect("Unable to find Double::doubleValue");

        Self {
            _class_boolean: env.new_global_ref(class_boolean).unwrap(),
            _class_byte: env.new_global_ref(class_byte).unwrap(),
            _class_short: env.new_global_ref(class_short).unwrap(),
            _class_integer: env.new_global_ref(class_integer).unwrap(),
            _class_long: env.new_global_ref(class_long).unwrap(),
            _class_float: env.new_global_ref(class_float).unwrap(),
            _class_double: env.new_global_ref(class_double).unwrap(),
            boolean_value,
            byte_value,
            short_value,
            integer_value,
            long_value,
            float_value,
            double_value,
        }
    }

    pub fn boolean_value(&self, env: &jni::JNIEnv, obj: jni::sys::jobject) -> bool {
        env.call_method_unchecked(obj, self.boolean_value, JavaType::Primitive(Primitive::Boolean), &[]).unwrap()
            .z().unwrap()
    }

    pub fn byte_value(&self, env: &jni::JNIEnv, obj: jni::sys::jobject) -> i8 {
        env.call_method_unchecked(obj, self.byte_value, JavaType::Primitive(Primitive::Byte), &[]).unwrap()
            .b().unwrap()
    }

    pub fn short_value(&self, env: &jni::JNIEnv, obj: jni::sys::jobject) -> i16 {
        env.call_method_unchecked(obj, self.short_value, JavaType::Primitive(Primitive::Short), &[]).unwrap()
            .s().unwrap()
    }

    pub fn integer_value(&self, env: &jni::JNIEnv, obj: jni::sys::jobject) -> i32 {
        env.call_method_unchecked(obj, self.integer_value, JavaType::Primitive(Primitive::Int), &[]).unwrap()
            .i().unwrap()
    }

    pub fn long_value(&self, env: &jni::JNIEnv, obj: jni::sys::jobject) -> i64 {
        env.call_method_unchecked(obj, self.long_value, JavaType::Primitive(Primitive::Long), &[]).unwrap()
            .j().unwrap()
    }

    pub fn float_value(&self, env: &jni::JNIEnv, obj: jni::sys::jobject) -> f32 {
        env.call_method_unchecked(obj, self.float_value, JavaType::Primitive(Primitive::Float), &[]).unwrap()
            .f().unwrap()
    }

    pub fn double_value(&self, env: &jni::JNIEnv, obj: jni::sys::jobject) -> f64 {
        env.call_method_unchecked(obj, self.double_value, JavaType::Primitive(Primitive::Double), &[]).unwrap()
            .d().unwrap()
    }
}
