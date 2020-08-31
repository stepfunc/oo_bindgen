use jni::sys::jint;
use jni::JNIEnv;

struct EnumDisjointEcho<'a> {
    class: jni::objects::JClass<'a>,
}

impl<'a> EnumDisjointEcho<'a> {
    fn init(env: &mut JNIEnv<'a>) -> Result<Self, jni::errors::Error> {
        Ok(Self {
            class: env.find_class("io/stepfunc/foo/EnumDisjointEcho")?,
        })
    }
}

struct JCache<'a> {
    enum_disjoint_echo: EnumDisjointEcho<'a>,
}

impl<'a> JCache<'a> {
    fn init(env: &mut JNIEnv<'a>) -> Result<Self, jni::errors::Error> {
        Ok(Self {
            enum_disjoint_echo: EnumDisjointEcho::init(env)?
        })
    }
}

static mut javaVm: Option<jni::JavaVM> = None;
static mut jcache: Option<JCache> = None;

#[no_mangle]
pub unsafe extern "C" fn JNI_OnLoad(vm: *mut jni::sys::JavaVM, _reserved: *mut std::ffi::c_void) -> jint {
    javaVm = Some(jni::JavaVM::from_raw(vm).unwrap());
    let vm = &javaVm.unwrap();
    jcache = Some(JCache::init(&mut vm.get_env().unwrap()).unwrap());

    jni::JNIVersion::V2.into()
}

#[no_mangle]
pub unsafe extern "C" fn JNI_OnUnload(_vm: jni::sys::JavaVM, _reserved: *mut std::ffi::c_void) {
    jcache.take().unwrap();
}

#[no_mangle]
pub extern "C" fn Java_io_stepfunc_foo_NativeFunctions_enum_1disjoint_1echo(env: JNIEnv, value: jni::objects::JObject) -> jni::sys::jobject {
    value.into_inner()
}
