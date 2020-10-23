use jni::signature::*;

#[allow(dead_code)]
pub struct Collection {
    array_list_class: jni::objects::GlobalRef,
    array_list_constructor: jni::objects::JMethodID<'static>,
    array_list_add_method: jni::objects::JMethodID<'static>,
    unmodifiable_list_constructor: jni::objects::JStaticMethodID<'static>,
}

impl Collection {
    pub fn init(env: &jni::JNIEnv) -> Self {
        let array_list_class = env.find_class("Ljava/util/ArrayList;").expect("Unable to find java/util/ArrayList class");

        let array_list_constructor = env.get_method_id(array_list_class, "<init>", "()V").map(|mid| mid.into_inner().into()).expect("Unable to find ArrayList constructor");
        let array_list_add_method = env.get_method_id(array_list_class, "add", "(Ljava/lang/Object;)Z").map(|mid| mid.into_inner().into()).expect("Unable to find ArrayList::add()");
        let unmodifiable_list_constructor = env.get_static_method_id("java/util/Collections", "unmodifiableCollection", "(Ljava/util/Collection;)Ljava/util/Collection;").map(|mid| mid.into_inner().into()).expect("Unable to find Collections::unmodifiableList()");

        Self {
            array_list_class: env.new_global_ref(array_list_class).unwrap(),
            array_list_constructor,
            array_list_add_method,
            unmodifiable_list_constructor,
        }
    }

    pub fn new_array_list<'a>(&self, env: &'a jni::JNIEnv) -> jni::objects::JObject<'a> {
        env.new_object_unchecked(&self.array_list_class, self.array_list_constructor, &[]).unwrap()
    }

    pub fn add_to_array_list(&self, env: &jni::JNIEnv, array_list: jni::objects::JObject, item: jni::objects::JObject) {
        env.call_method_unchecked(array_list, self.array_list_add_method, JavaType::Primitive(Primitive::Boolean), &[item.into()]).unwrap();
    }
}
