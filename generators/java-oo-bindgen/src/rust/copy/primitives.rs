use jni::signature::*;

pub(crate) struct Boolean {
    _class: jni::objects::GlobalRef,
    value_method: jni::objects::JMethodID<'static>,
}

impl Boolean {
    fn init(env: &jni::JNIEnv) -> Self {
        let class = env.find_class("Ljava/lang/Boolean;").expect("Unable to find java/lang/Boolean class");
        Self {
            _class: env.new_global_ref(class).unwrap(),
            value_method: env.get_method_id(class, "booleanValue", "()Z").map(|mid| mid.into_inner().into()).expect("Unable to find Boolean::booleanValue"),
        }
    }

    pub(crate) fn create<'a>(&self, env: &'a jni::JNIEnv, value: bool) -> jni::objects::JObject<'a> {
        unimplemented!()
    }

    pub(crate) fn value(&self, env: &jni::JNIEnv, obj: jni::sys::jobject) -> bool {
        env.call_method_unchecked(obj, self.value_method, JavaType::Primitive(Primitive::Boolean), &[]).unwrap()
            .z().unwrap()
    }
}

pub(crate) struct Byte {
    _class: jni::objects::GlobalRef,
    value_method: jni::objects::JMethodID<'static>,
}

impl Byte {
    fn init(env: &jni::JNIEnv) -> Self {
        let class = env.find_class("Ljava/lang/Byte;").expect("Unable to find java/lang/Byte class");
        Self {
            _class: env.new_global_ref(class).unwrap(),
            value_method:env.get_method_id(class, "byteValue", "()B").map(|mid| mid.into_inner().into()).expect("Unable to find Byte::byteValue"),
        }
    }

    pub(crate) fn create<'a>(&self, env: &'a jni::JNIEnv, value: i8) -> jni::objects::JObject<'a> {
        unimplemented!()
    }

    pub(crate) fn value(&self, env: &jni::JNIEnv, obj: jni::sys::jobject) -> i8 {
        env.call_method_unchecked(obj, self.value_method, JavaType::Primitive(Primitive::Byte), &[]).unwrap()
            .b().unwrap()
    }
}

pub(crate) struct Short {
    _class: jni::objects::GlobalRef,
    value_method: jni::objects::JMethodID<'static>,
}

impl Short {
    fn init(env: &jni::JNIEnv) -> Self {
        let class = env.find_class("Ljava/lang/Short;").expect("Unable to find java/lang/Short class");
        Self {
            _class: env.new_global_ref(class).unwrap(),
            value_method: env.get_method_id(class, "shortValue", "()S").map(|mid| mid.into_inner().into()).expect("Unable to find Short::shortValue"),
        }
    }

    pub(crate) fn create<'a>(&self, env: &'a jni::JNIEnv, value: i16) -> jni::objects::JObject<'a> {
        unimplemented!()
    }

    pub(crate) fn value(&self, env: &jni::JNIEnv, obj: jni::sys::jobject) -> i16 {
        env.call_method_unchecked(obj, self.value_method, JavaType::Primitive(Primitive::Short), &[]).unwrap()
            .s().unwrap()
    }
}

pub(crate) struct Integer {
    class: jni::objects::GlobalRef,
    value_method: jni::objects::JMethodID<'static>,
    constructor: jni::objects::JMethodID<'static>,
}

impl Integer {
    fn init(env: &jni::JNIEnv) -> Self {
        let class = env.find_class("Ljava/lang/Integer;").expect("Unable to find java/lang/Integer class");
        Self {
            class: env.new_global_ref(class).unwrap(),
            value_method: env.get_method_id(class, "intValue", "()I").map(|mid| mid.into_inner().into()).expect("Unable to find Integer::intValue"),
            constructor: env.get_method_id(class, "<init>", "(I)V").map(|mid| mid.into_inner().into()).expect("Unable to find Integer::<init>"),
        }
    }

    pub(crate) fn create<'a>(&self, env: &'a jni::JNIEnv, value: i32) -> jni::objects::JObject<'a> {
        env.new_object_unchecked(&self.class, self.constructor.into(), &[value.into()]).unwrap()
    }

    pub(crate) fn value(&self, env: &jni::JNIEnv, obj: jni::sys::jobject) -> i32 {
        env.call_method_unchecked(obj, self.value_method, JavaType::Primitive(Primitive::Int), &[]).unwrap()
            .i().unwrap()
    }
}

pub(crate) struct Long {
    _class: jni::objects::GlobalRef,
    value_method: jni::objects::JMethodID<'static>,
}


impl Long {
    fn init(env: &jni::JNIEnv) -> Self {
        let class = env.find_class("Ljava/lang/Long;").expect("Unable to find java/lang/Long class");
        Self {
            _class: env.new_global_ref(class).unwrap(),
            value_method: env.get_method_id(class, "longValue", "()J").map(|mid| mid.into_inner().into()).expect("Unable to find Long::longValue"),
        }
    }

    pub(crate) fn create<'a>(&self, env: &'a jni::JNIEnv, value: i64) -> jni::objects::JObject<'a> {
        unimplemented!()
    }

    pub(crate) fn value(&self, env: &jni::JNIEnv, obj: jni::sys::jobject) -> i64 {
        env.call_method_unchecked(obj, self.value_method, JavaType::Primitive(Primitive::Long), &[]).unwrap()
            .j().unwrap()
    }
}

pub(crate) struct Float {
    _class: jni::objects::GlobalRef,
    value_method: jni::objects::JMethodID<'static>,
}


impl Float {
    fn init(env: &jni::JNIEnv) -> Self {
        let class = env.find_class("Ljava/lang/Float;").expect("Unable to find java/lang/Float class");
        Self {
            _class: env.new_global_ref(class).unwrap(),
            value_method: env.get_method_id(class, "floatValue", "()F").map(|mid| mid.into_inner().into()).expect("Unable to find Float::floatValue"),
        }
    }

    pub(crate) fn create<'a>(&self, env: &'a jni::JNIEnv, value: f32) -> jni::objects::JObject<'a> {
        unimplemented!()
    }

    pub(crate) fn value(&self, env: &jni::JNIEnv, obj: jni::sys::jobject) -> f32 {
        env.call_method_unchecked(obj, self.value_method, JavaType::Primitive(Primitive::Float), &[]).unwrap()
            .f().unwrap()
    }
}

pub(crate) struct Double {
    _class: jni::objects::GlobalRef,
    value_method: jni::objects::JMethodID<'static>,
}


impl Double {
    fn init(env: &jni::JNIEnv) -> Self {
        let class = env.find_class("Ljava/lang/Double;").expect("Unable to find java/lang/Double class");
        Self {
            _class: env.new_global_ref(class).unwrap(),
            value_method: env.get_method_id(class, "doubleValue", "()D").map(|mid| mid.into_inner().into()).expect("Unable to find Double::doubleValue"),
        }
    }

    pub(crate) fn create<'a>(&self, env: &'a jni::JNIEnv, value: f64) -> jni::objects::JObject<'a> {
        unimplemented!()
    }

    pub(crate) fn value(&self, env: &jni::JNIEnv, obj: jni::sys::jobject) -> f64 {
        env.call_method_unchecked(obj, self.value_method, JavaType::Primitive(Primitive::Double), &[]).unwrap()
            .d().unwrap()
    }
}


pub(crate) struct Primitives {
    pub(crate) boolean: Boolean,
    pub(crate) byte: Byte,
    pub(crate) short: Short,
    pub(crate) integer: Integer,
    pub(crate) long: Long,
    pub(crate) float: Float,
    pub(crate) double: Double,
}

impl Primitives {
    pub(crate) fn init(env: &jni::JNIEnv) -> Self {
        Self {
            boolean: Boolean::init(env),
            byte: Byte::init(env),
            short: Short::init(env),
            integer: Integer::init(env),
            long: Long::init(env),
            float: Float::init(env),
            double: Double::init(env),
        }
    }
}
