use jni::signature::*;

pub struct Collection {
    // Iterator stuff
    array_list_class: jni::objects::GlobalRef,
    array_list_constructor: jni::objects::JMethodID<'static>,
    array_list_add_method: jni::objects::JMethodID<'static>,
    unmodifiable_list_constructor: jni::objects::JStaticMethodID<'static>,

    // Collection stuff
    _iterator_class: jni::objects::GlobalRef,
    _list_class: jni::objects::GlobalRef,
    iterator_has_next_method: jni::objects::JMethodID<'static>,
    iterator_next_method: jni::objects::JMethodID<'static>,
    list_iterator_method: jni::objects::JMethodID<'static>,
    list_size_method: jni::objects::JMethodID<'static>,
}

impl Collection {
    pub fn init(env: &jni::JNIEnv) -> Self {
        let array_list_class = env.find_class("java/util/ArrayList").expect("Unable to find java/util/ArrayList class");

        let array_list_constructor = env.get_method_id(array_list_class, "<init>", "()V").map(|mid| mid.into_inner().into()).expect("Unable to find ArrayList constructor");
        let array_list_add_method = env.get_method_id(array_list_class, "add", "(Ljava/lang/Object;)Z").map(|mid| mid.into_inner().into()).expect("Unable to find ArrayList::add()");
        let unmodifiable_list_constructor = env.get_static_method_id("java/util/Collections", "unmodifiableList", "(Ljava/util/List;)Ljava/util/List;").map(|mid| mid.into_inner().into()).expect("Unable to find Collections::unmodifiableList()");

        let iterator_class = env.find_class("java/util/Iterator").expect("Unable to find java/util/Iterator class");
        let list_class = env.find_class("java/util/List").expect("Unable to find java/util/List class");
        let iterator_has_next_method = env.get_method_id(iterator_class, "hasNext", "()Z").map(|mid| mid.into_inner().into()).expect("Unable to find Iterator::hasNext()");
        let iterator_next_method = env.get_method_id(iterator_class, "next", "()Ljava/lang/Object;").map(|mid| mid.into_inner().into()).expect("Unable to find Iterator::next()");
        let list_iterator_method = env.get_method_id(list_class, "iterator", "()Ljava/util/Iterator;").map(|mid| mid.into_inner().into()).expect("Unable to find List::iterator()");
        let list_size_method = env.get_method_id(list_class, "size", "()I").map(|mid| mid.into_inner().into()).expect("Unable to find List::size()");

        Self {
            array_list_class: env.new_global_ref(array_list_class).unwrap(),
            array_list_constructor,
            array_list_add_method,
            unmodifiable_list_constructor,
            _iterator_class: env.new_global_ref(iterator_class).unwrap(),
            _list_class: env.new_global_ref(list_class).unwrap(),
            iterator_has_next_method,
            iterator_next_method,
            list_iterator_method,
            list_size_method,
        }
    }

    pub fn new_array_list<'a>(&self, env: &'a jni::JNIEnv) -> jni::objects::JObject<'a> {
        env.new_object_unchecked(&self.array_list_class, self.array_list_constructor, &[]).unwrap()
    }

    pub fn add_to_array_list(&self, env: &jni::JNIEnv, array_list: jni::objects::JObject, item: jni::objects::JObject) {
        env.call_method_unchecked(array_list, self.array_list_add_method, JavaType::Primitive(Primitive::Boolean), &[item.into()]).unwrap();
    }

    pub fn has_next(&self, env: &jni::JNIEnv, obj: jni::objects::JObject) -> bool {
        env.call_method_unchecked(obj, self.iterator_has_next_method, JavaType::Primitive(Primitive::Boolean), &[]).unwrap().z().unwrap()
    }

    pub fn next<'a>(&self, env: &jni::JNIEnv<'a>, obj: jni::objects::JObject<'a>) -> jni::objects::JObject<'a> {
        env.call_method_unchecked(obj, self.iterator_next_method, JavaType::Object("java/lang/Object".to_string()), &[]).unwrap().l().unwrap()
    }

    pub fn get_iterator<'a>(&self, env: &jni::JNIEnv<'a>, obj: jni::objects::JObject<'a>) -> jni::objects::JObject<'a> {
        env.call_method_unchecked(obj, self.list_iterator_method, JavaType::Object("java/util/Iterator".to_string()), &[]).unwrap().l().unwrap()
    }

    pub fn get_size(&self, env: &jni::JNIEnv, obj: jni::objects::JObject) -> u32 {
        env.call_method_unchecked(obj, self.list_size_method, JavaType::Primitive(Primitive::Int), &[]).unwrap().i().unwrap() as u32
    }
}
