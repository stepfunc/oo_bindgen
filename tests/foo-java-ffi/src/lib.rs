use jni::sys::jint;
use jni::JNIEnv;

struct EnumDisjointEcho {
    _class: jni::objects::GlobalRef,
    enum_five: jni::objects::JObject<'static>,
    enum_one: jni::objects::JObject<'static>,
    enum_twenty: jni::objects::JObject<'static>,
    enum_four: jni::objects::JObject<'static>,
    enum_seven: jni::objects::JObject<'static>,
    enum_two: jni::objects::JObject<'static>,
}

impl EnumDisjointEcho {
    fn init(env: &JNIEnv) -> Result<Self, jni::errors::Error> {
        let class = env.find_class("Lio/stepfunc/foo/EnumDisjointEcho")?;
        
        Ok(Self {
            _class: env.new_global_ref(class)?,
            enum_five: get_enum_object(env, class, "FIVE")?,
            enum_one: get_enum_object(env, class, "ONE")?,
            enum_twenty: get_enum_object(env, class, "TWENTY")?,
            enum_four: get_enum_object(env, class, "FOUR")?,
            enum_seven: get_enum_object(env, class, "SEVEN")?,
            enum_two: get_enum_object(env, class, "TWO")?,
        })
    }
}

struct JCache {
    vm: jni::JavaVM,
    enum_disjoint_echo: EnumDisjointEcho,
}

impl JCache {
    fn init(vm: jni::JavaVM) -> Result<Self, jni::errors::Error> {
        let env = vm.get_env().unwrap();
        let enum_disjoint_echo = EnumDisjointEcho::init(&env)?;
        Ok(Self {
            vm,
            enum_disjoint_echo,
        })
    }
}

static mut JCACHE: Option<JCache> = None;

fn find_class(env: &jni::JNIEnv, class_name: &str) -> Result<jni::objects::GlobalRef, jni::errors::Error> {
    let class = env.find_class(class_name)?;
    Ok(env.new_global_ref(class)?)
}

fn get_enum_object(env: &jni::JNIEnv, class: jni::objects::JClass, field: &str) -> Result<jni::objects::JObject<'static>, jni::errors::Error> {
    env.get_static_field(class, field, "l")?.l().map(|mid| mid.into_inner().into())
}

#[no_mangle]
pub unsafe extern "C" fn JNI_OnLoad(vm: jni::JavaVM, _: *mut std::ffi::c_void) -> jint {
    // Initialize the cache
    let jcache = JCache::init(vm).unwrap();

    // Set global variables
    //JAVA_VM.replace(vm);
    JCACHE.replace(jcache);

    // We target Java 8, to JNI 1.8 is the minimum version required
    jni::JNIVersion::V8.into()
}

#[no_mangle]
pub unsafe extern "C" fn JNI_OnUnload(_vm: jni::sys::JavaVM, _reserved: *mut std::ffi::c_void) {
    // Cleanup all the static stuff
    JCACHE.take().unwrap();
}

#[no_mangle]
pub extern "C" fn Java_io_stepfunc_foo_NativeFunctions_enum_1disjoint_1echo(env: JNIEnv, value: jni::objects::JObject) -> jni::sys::jobject {
    value.into_inner()
}


