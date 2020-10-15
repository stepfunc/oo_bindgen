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
        let class_name = "Lio/stepfunc/foo/EnumDisjoint;";
        let class = env.find_class(class_name)?;

        Ok(Self {
            _class: env.new_global_ref(class)?,
            enum_five: get_enum_object(env, class, class_name, "FIVE")?,
            enum_one: get_enum_object(env, class, class_name, "ONE")?,
            enum_twenty: get_enum_object(env, class, class_name, "TWENTY")?,
            enum_four: get_enum_object(env, class, class_name, "FOUR")?,
            enum_seven: get_enum_object(env, class, class_name, "SEVEN")?,
            enum_two: get_enum_object(env, class, class_name, "TWO")?,
        })
    }

    fn to_ffi(&self, env: &jni::JNIEnv, obj: jni::objects::JObject) -> foo_ffi::ffi::EnumDisjoint {
        if env.is_same_object(obj, self.enum_five).unwrap() {
            return foo_ffi::ffi::EnumDisjoint::Five;
        }
        if env.is_same_object(obj, self.enum_one).unwrap() {
            return foo_ffi::ffi::EnumDisjoint::One;
        }
        if env.is_same_object(obj, self.enum_twenty).unwrap() {
            return foo_ffi::ffi::EnumDisjoint::Twenty;
        }
        if env.is_same_object(obj, self.enum_four).unwrap() {
            return foo_ffi::ffi::EnumDisjoint::Four;
        }
        if env.is_same_object(obj, self.enum_seven).unwrap() {
            return foo_ffi::ffi::EnumDisjoint::Seven;
        }
        if env.is_same_object(obj, self.enum_two).unwrap() {
            return foo_ffi::ffi::EnumDisjoint::Two;
        }

        panic!("Unrecognized EnumDisjoint enum variant");
    }

    fn from_ffi(&self, value: foo_ffi::ffi::EnumDisjoint) -> jni::objects::JObject<'static> {
        match value {
            foo_ffi::ffi::EnumDisjoint::Five => self.enum_five,
            foo_ffi::ffi::EnumDisjoint::One => self.enum_one,
            foo_ffi::ffi::EnumDisjoint::Twenty => self.enum_twenty,
            foo_ffi::ffi::EnumDisjoint::Four => self.enum_four,
            foo_ffi::ffi::EnumDisjoint::Seven => self.enum_seven,
            foo_ffi::ffi::EnumDisjoint::Two => self.enum_two,
        }
    }
}

struct JCache {
    _vm: jni::JavaVM,
    enum_disjoint_echo: EnumDisjointEcho,
}

impl JCache {
    fn init(vm: jni::JavaVM) -> Result<Self, jni::errors::Error> {
        let env = vm.get_env().unwrap();
        let enum_disjoint_echo = EnumDisjointEcho::init(&env)?;
        Ok(Self {
            _vm: vm,
            enum_disjoint_echo,
        })
    }
}

static mut JCACHE: Option<JCache> = None;

/*fn get_method_id(
    env: &jni::JNIEnv,
    class: jni::objects::JClass,
    name: &str,
    sig: &str,
) -> Result<jni::objects::JMethodID<'static>, jni::errors::Error> {
    env.get_method_id(class, name, sig)
        .map(|mid| mid.into_inner().into())
}*/

fn get_enum_object(
    env: &jni::JNIEnv,
    class: jni::objects::JClass,
    class_name: &str,
    field: &str,
) -> Result<jni::objects::JObject<'static>, jni::errors::Error> {
    env.get_static_field(class, field, class_name)?
        .l()
        .map(|mid| mid.into_inner().into())
}

#[no_mangle]
pub unsafe extern "C" fn JNI_OnLoad(vm: *mut jni::sys::JavaVM, _: *mut std::ffi::c_void) -> jint {
    // Initialize the cache
    let vm = jni::JavaVM::from_raw(vm).unwrap();
    let jcache = JCache::init(vm).unwrap();

    // Set global variables
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
pub unsafe extern "C" fn Java_io_stepfunc_foo_NativeFunctions_enum_1disjoint_1echo(
    env: JNIEnv,
    _: jni::objects::JObject,
    value: jni::objects::JObject,
) -> jni::sys::jobject {
    let cache = JCACHE.as_ref().unwrap();
    let value = cache.enum_disjoint_echo.to_ffi(&env, value);

    let result = foo_ffi::ffi::enum_disjoint_echo(value.into()).into();

    let result = cache.enum_disjoint_echo.from_ffi(result);

    result.into_inner()
}
