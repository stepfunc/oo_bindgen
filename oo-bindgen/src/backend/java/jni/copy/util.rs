pub(crate) struct LocalFrameGuard<'a> {
    env: jni::JNIEnv<'a>
}

impl<'a> LocalFrameGuard<'a> {
    fn new(env: jni::JNIEnv<'a>, count: i32) ->  jni::errors::Result<Self> {
        env.push_local_frame(count)?;
        Ok(Self { env })
    }
}

impl Drop for LocalFrameGuard<'_> {
    fn drop(&mut self) {
        let _ = self.env.pop_local_frame(jni::objects::JObject::null());
    }
}

pub(crate) fn local_frame(env: jni::JNIEnv, count: i32) -> jni::errors::Result<LocalFrameGuard> {
    LocalFrameGuard::new(env, count)
}