pub(crate) trait CreateObject {
    fn create_object<'a>(&self, env: &'a jni::JNIEnv) -> jni::objects::JObject<'a>;

    fn create_inner_object(&self, env: &jni::JNIEnv) -> jni::sys::jobject {
        self.create_object(env).into_inner()
    }
}

impl CreateObject for *const bool {
    fn create_object<'a>(&self, env: &'a jni::JNIEnv) -> jni::objects::JObject<'a> {
        unsafe {
            self
                .as_ref()
                .map(|x| crate::get_cache().primitives.boolean.create(env, *x))
                .unwrap_or_else(|| jni::objects::JObject::null())
        }
    }
}

impl CreateObject for *const i8 {
    fn create_object<'a>(&self, env: &'a jni::JNIEnv) -> jni::objects::JObject<'a> {
        unsafe {
            self
                .as_ref()
                .map(|x| crate::get_cache().primitives.byte.create(env, *x))
                .unwrap_or_else(|| jni::objects::JObject::null())
        }
    }
}

impl CreateObject for *const u8 {
    fn create_object<'a>(&self, env: &'a jni::JNIEnv) -> jni::objects::JObject<'a> {
        unsafe {
            self
                .as_ref()
                .map(|x| crate::get_cache().unsigned.byte.to_jni(env, *x).into())
                .unwrap_or_else(|| jni::objects::JObject::null())
        }
    }
}

impl CreateObject for *const i16 {
    fn create_object<'a>(&self, env: &'a jni::JNIEnv) -> jni::objects::JObject<'a> {
        unsafe {
            self
                .as_ref()
                .map(|x| crate::get_cache().primitives.short.create(env, *x))
                .unwrap_or_else(|| jni::objects::JObject::null())
        }
    }
}

impl CreateObject for *const u16 {
    fn create_object<'a>(&self, env: &'a jni::JNIEnv) -> jni::objects::JObject<'a> {
        unsafe {
            self
                .as_ref()
                .map(|x| crate::get_cache().unsigned.short.to_jni(env, *x).into())
                .unwrap_or_else(|| jni::objects::JObject::null())
        }
    }
}

impl CreateObject for *const i32 {
    fn create_object<'a>(&self, env: &'a jni::JNIEnv) -> jni::objects::JObject<'a> {
        unsafe {
            self
                .as_ref()
                .map(|x| crate::get_cache().primitives.integer.create(env, *x))
                .unwrap_or_else(|| jni::objects::JObject::null())
        }
    }
}

impl CreateObject for *const u32 {
    fn create_object<'a>(&self, env: &'a jni::JNIEnv) -> jni::objects::JObject<'a> {
        unsafe {
            self
                .as_ref()
                .map(|x| crate::get_cache().unsigned.integer.to_jni(env, *x).into())
                .unwrap_or_else(|| jni::objects::JObject::null())
        }
    }
}

impl CreateObject for *const i64 {
    fn create_object<'a>(&self, env: &'a jni::JNIEnv) -> jni::objects::JObject<'a> {
        unsafe {
            self
                .as_ref()
                .map(|x| crate::get_cache().primitives.long.create(env, *x))
                .unwrap_or_else(|| jni::objects::JObject::null())
        }
    }
}

impl CreateObject for *const u64 {
    fn create_object<'a>(&self, env: &'a jni::JNIEnv) -> jni::objects::JObject<'a> {
        unsafe {
            self
                .as_ref()
                .map(|x| crate::get_cache().unsigned.long.to_jni(env, *x).into())
                .unwrap_or_else(|| jni::objects::JObject::null())
        }
    }
}

impl CreateObject for *const f32 {
    fn create_object<'a>(&self, env: &'a jni::JNIEnv) -> jni::objects::JObject<'a> {
        unsafe {
            self
                .as_ref()
                .map(|x| crate::get_cache().primitives.float.create(env, *x))
                .unwrap_or_else(|| jni::objects::JObject::null())
        }
    }
}

impl CreateObject for *const f64 {
    fn create_object<'a>(&self, env: &'a jni::JNIEnv) -> jni::objects::JObject<'a> {
        unsafe {
            self
                .as_ref()
                .map(|x| crate::get_cache().primitives.double.create(env, *x))
                .unwrap_or_else(|| jni::objects::JObject::null())
        }
    }
}

