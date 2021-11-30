pub(crate) struct LocalRefGuard<'a> {
    env: jni::JNIEnv<'a>,
    pub(crate) value: jni::objects::JObject<'a>,
}

impl<'a> LocalRefGuard<'a> {
    pub(crate) fn new(env: jni::JNIEnv<'a>, value: jni::objects::JObject<'a>) -> Self {
        Self {
            env,
            value
        }
    }
}

impl<'a> Drop for LocalRefGuard<'a> {
    fn drop(&mut self) {
        let _ = self.env.delete_local_ref(self.value);
    }
}