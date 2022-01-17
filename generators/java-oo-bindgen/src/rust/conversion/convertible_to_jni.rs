use oo_bindgen::model::*;

pub(crate) trait MaybeConvertibleToJni {
    /// Possible conversion from JNI argument type to Rust type
    fn maybe_convert(&self, expr: &str) -> Option<String>;
}

/// trait to implement when the type always has a conversion
pub(crate) trait ConvertibleToJni {
    /// definite conversion from JNI argument type to Rust type
    fn convert(&self, expr: &str) -> String;
}

/// Blanket implementation for all types that have definite conversions
impl<T> MaybeConvertibleToJni for T
where
    T: ConvertibleToJni,
{
    fn maybe_convert(&self, expr: &str) -> Option<String> {
        Some(self.convert(expr))
    }
}

impl ConvertibleToJni for DurationType {
    fn convert(&self, expr: &str) -> String {
        let method = match self {
            DurationType::Milliseconds => "to_jni_millis",
            DurationType::Seconds => "to_jni_seconds",
        };

        format!("_cache.duration.{}(&_env, {})", method, expr)
    }
}

impl<D> ConvertibleToJni for Handle<Enum<D>>
where
    D: DocReference,
{
    fn convert(&self, expr: &str) -> String {
        format!("_cache.enums.{}.to_jni(&_env, {})", self.name, expr)
    }
}

impl ConvertibleToJni for StringType {
    fn convert(&self, expr: &str) -> String {
        format!("_env.new_string(unsafe {{ std::ffi::CStr::from_ptr({}) }}.to_string_lossy()).unwrap().into_inner()", expr)
    }
}

impl MaybeConvertibleToJni for Primitive {
    fn maybe_convert(&self, expr: &str) -> Option<String> {
        match self {
            Self::Bool => Some(format!("u8::from({})", expr)),
            Self::U8 => Some(UnsignedConverter::U8.apply(expr)),
            Self::S8 => None,
            Self::U16 => Some(UnsignedConverter::U16.apply(expr)),
            Self::S16 => None,
            Self::U32 => Some(UnsignedConverter::U32.apply(expr)),
            Self::S32 => None,
            Self::U64 => Some(UnsignedConverter::U64.apply(expr)),
            Self::S64 => None,
            Self::Float => None,
            Self::Double => None,
        }
    }
}

impl MaybeConvertibleToJni for BasicType {
    fn maybe_convert(&self, expr: &str) -> Option<String> {
        match self {
            Self::Primitive(x) => x.maybe_convert(expr),
            Self::Duration(x) => x.maybe_convert(expr),
            Self::Enum(x) => x.maybe_convert(expr),
        }
    }
}

impl ConvertibleToJni for StructDeclarationHandle {
    fn convert(&self, expr: &str) -> String {
        format!(
            "{}.as_ref().map(|x| _cache.structs.struct_{}.to_jni(_cache, &_env, &value)).or_else(|| jni::objects::JObject::null().into_inner())",
            expr,
            self.name
        )
    }
}

impl ConvertibleToJni for ClassDeclarationHandle {
    fn convert(&self, expr: &str) -> String {
        format!("_cache.classes.{}.to_jni(&_env, {})", self.name, expr)
    }
}

impl<D> ConvertibleToJni for Handle<AbstractIterator<D>>
where
    D: DocReference,
{
    fn convert(&self, expr: &str) -> String {
        format!(
            "crate::iterators::{}(&_env, _cache, {})",
            self.iter_class.name, expr
        )
    }
}

impl<T, D> ConvertibleToJni for Handle<Struct<T, D>>
where
    D: DocReference,
    T: StructFieldType,
{
    fn convert(&self, expr: &str) -> String {
        format!(
            "_cache.structs.{}.to_jni(_cache, &_env, &{})",
            self.declaration.name(),
            expr
        )
    }
}

impl<T> ConvertibleToJni for UniversalOr<T>
where
    T: StructFieldType,
{
    fn convert(&self, expr: &str) -> String {
        match self {
            UniversalOr::Specific(x) => x.convert(expr),
            UniversalOr::Universal(x) => x.convert(expr),
        }
    }
}

impl MaybeConvertibleToJni for FunctionReturnStructField {
    fn maybe_convert(&self, expr: &str) -> Option<String> {
        match self {
            Self::Basic(x) => x.maybe_convert(expr),
            Self::ClassRef(x) => x.maybe_convert(expr),
            Self::Struct(x) => x.maybe_convert(expr),
            Self::Iterator(x) => x.maybe_convert(expr),
        }
    }
}

impl MaybeConvertibleToJni for CallbackArgStructField {
    fn maybe_convert(&self, expr: &str) -> Option<String> {
        match self {
            Self::Basic(x) => x.maybe_convert(expr),
            Self::Iterator(x) => x.maybe_convert(expr),
            Self::Struct(x) => x.maybe_convert(expr),
        }
    }
}

impl MaybeConvertibleToJni for UniversalStructField {
    fn maybe_convert(&self, expr: &str) -> Option<String> {
        match self {
            Self::Basic(x) => x.maybe_convert(expr),
            Self::Struct(x) => x.maybe_convert(expr),
        }
    }
}

impl MaybeConvertibleToJni for CallbackArgument {
    fn maybe_convert(&self, expr: &str) -> Option<String> {
        match self {
            Self::Basic(x) => x.maybe_convert(expr),
            Self::String(x) => x.maybe_convert(expr),
            Self::Iterator(x) => x.maybe_convert(expr),
            Self::Struct(x) => x.maybe_convert(expr),
            Self::Class(x) => x.maybe_convert(expr),
        }
    }
}

impl MaybeConvertibleToJni for FunctionReturnValue {
    fn maybe_convert(&self, expr: &str) -> Option<String> {
        match self {
            Self::Basic(x) => x.maybe_convert(expr),
            Self::String(x) => x.maybe_convert(expr),
            Self::ClassRef(x) => x.maybe_convert(expr),
            Self::Struct(x) => x.maybe_convert(expr),
            Self::StructRef(x) => x.untyped().maybe_convert(expr),
        }
    }
}

impl MaybeConvertibleToJni for IteratorItemType {
    fn maybe_convert(&self, expr: &str) -> Option<String> {
        match self {
            IteratorItemType::StructRef(x) => x.maybe_convert(expr),
        }
    }
}

impl MaybeConvertibleToJni for CallbackReturnValue {
    fn maybe_convert(&self, expr: &str) -> Option<String> {
        match self {
            Self::Basic(x) => x.maybe_convert(expr),
            Self::Struct(x) => x.maybe_convert(expr),
        }
    }
}

impl<T, D> MaybeConvertibleToJni for OptionalReturnType<T, D>
where
    D: DocReference,
    T: Clone + MaybeConvertibleToJni,
{
    fn maybe_convert(&self, expr: &str) -> Option<String> {
        match self.get_value() {
            None => None,
            Some(return_type) => return_type.maybe_convert(expr),
        }
    }
}

pub(crate) struct UnsignedConverter {
    java_type: &'static str,
}

impl UnsignedConverter {
    const fn new(java_type: &'static str) -> Self {
        Self { java_type }
    }

    const U8: UnsignedConverter = UnsignedConverter::new("byte");
    const U16: UnsignedConverter = UnsignedConverter::new("short");
    const U32: UnsignedConverter = UnsignedConverter::new("integer");
    const U64: UnsignedConverter = UnsignedConverter::new("long");

    fn apply(&self, expr: &str) -> String {
        format!("_cache.unsigned.{}.to_jni(&_env, {})", self.java_type, expr)
    }
}
