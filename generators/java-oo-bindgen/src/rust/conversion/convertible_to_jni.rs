use oo_bindgen::model::*;

pub(crate) trait ConvertibleToJni {
    /// Possible conversion from JNI argument type to Rust type
    fn maybe_convert(&self, expr: &str) -> Option<String>;
    /// Indicates whether a local reference cleanup is required once we are done with the type
    fn requires_local_ref_cleanup(&self) -> bool {
        // TODO - this will get removed
        false
    }
}

/// trait to implement when the type always has a conversion
pub(crate) trait DefiniteConversionToJni {
    /// Possible conversion from JNI argument type to Rust type
    fn convert(&self, expr: &str) -> String;
}

/// Blanket implementation for all types that have definite conversions
impl<T> ConvertibleToJni for T
where
    T: DefiniteConversionToJni,
{
    fn maybe_convert(&self, expr: &str) -> Option<String> {
        Some(self.convert(expr))
    }
}

impl DefiniteConversionToJni for DurationType {
    fn convert(&self, expr: &str) -> String {
        let method = match self {
            DurationType::Milliseconds => "duration_from_millis",
            DurationType::Seconds => "duration_from_seconds",
        };

        format!("_cache.duration.{}(&_env, {})", method, expr)
    }
}

impl<D> DefiniteConversionToJni for Handle<Enum<D>>
where
    D: DocReference,
{
    fn convert(&self, expr: &str) -> String {
        format!(
            "_cache.enums.enum_{}.enum_from_rust(&_env, {})",
            self.name, expr
        )
    }
}

impl DefiniteConversionToJni for StringType {
    fn convert(&self, expr: &str) -> String {
        format!("_env.new_string(unsafe {{ std::ffi::CStr::from_ptr({}) }}.to_string_lossy()).unwrap().into_inner()", expr)
    }
}

impl ConvertibleToJni for Primitive {
    fn maybe_convert(&self, expr: &str) -> Option<String> {
        match self {
            Self::Bool => Some(format!("{} as u8", expr)),
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

impl ConvertibleToJni for BasicType {
    fn maybe_convert(&self, expr: &str) -> Option<String> {
        match self {
            Self::Primitive(x) => x.maybe_convert(expr),
            Self::Duration(x) => x.maybe_convert(expr),
            Self::Enum(x) => x.maybe_convert(expr),
        }
    }
}

impl DefiniteConversionToJni for StructDeclarationHandle {
    fn convert(&self, expr: &str) -> String {
        format!(
            "{}.as_ref().map(|x| _cache.structs.struct_{}.to_jni(_cache, &_env, &value)).or_else(|| jni::objects::JObject::null().into_inner())",
            expr,
            self.name
        )
    }
}

impl DefiniteConversionToJni for ClassDeclarationHandle {
    fn convert(&self, expr: &str) -> String {
        format!("_cache.classes.{}_from_rust(&_env, {})", self.name, expr)
    }
}

impl<D> DefiniteConversionToJni for Handle<AbstractIterator<D>>
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

impl<T, D> DefiniteConversionToJni for Handle<Struct<T, D>>
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

impl<T> DefiniteConversionToJni for UniversalOr<T>
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

impl ConvertibleToJni for FunctionReturnStructField {
    fn maybe_convert(&self, expr: &str) -> Option<String> {
        match self {
            Self::Basic(x) => x.maybe_convert(expr),
            Self::ClassRef(x) => x.maybe_convert(expr),
            Self::Struct(x) => x.maybe_convert(expr),
            Self::Iterator(x) => x.maybe_convert(expr),
        }
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        match self {
            Self::Basic(x) => x.requires_local_ref_cleanup(),
            Self::ClassRef(x) => x.requires_local_ref_cleanup(),
            Self::Struct(x) => x.requires_local_ref_cleanup(),
            Self::Iterator(x) => x.requires_local_ref_cleanup(),
        }
    }
}

impl ConvertibleToJni for CallbackArgStructField {
    fn maybe_convert(&self, expr: &str) -> Option<String> {
        match self {
            Self::Basic(x) => x.maybe_convert(expr),
            Self::Iterator(x) => x.maybe_convert(expr),
            Self::Struct(x) => x.maybe_convert(expr),
        }
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        match self {
            Self::Basic(x) => x.requires_local_ref_cleanup(),
            Self::Iterator(x) => x.requires_local_ref_cleanup(),
            Self::Struct(x) => x.requires_local_ref_cleanup(),
        }
    }
}

impl ConvertibleToJni for UniversalStructField {
    fn maybe_convert(&self, expr: &str) -> Option<String> {
        match self {
            Self::Basic(x) => x.maybe_convert(expr),
            Self::Struct(x) => x.maybe_convert(expr),
        }
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        match self {
            Self::Basic(x) => x.requires_local_ref_cleanup(),
            Self::Struct(x) => x.requires_local_ref_cleanup(),
        }
    }
}

impl ConvertibleToJni for CallbackArgument {
    fn maybe_convert(&self, expr: &str) -> Option<String> {
        match self {
            Self::Basic(x) => x.maybe_convert(expr),
            Self::String(x) => x.maybe_convert(expr),
            Self::Iterator(x) => x.maybe_convert(expr),
            Self::Struct(x) => x.maybe_convert(expr),
            Self::Class(x) => x.maybe_convert(expr),
        }
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        match self {
            Self::Basic(x) => x.requires_local_ref_cleanup(),
            Self::String(x) => x.requires_local_ref_cleanup(),
            Self::Iterator(x) => x.requires_local_ref_cleanup(),
            Self::Struct(x) => x.requires_local_ref_cleanup(),
            Self::Class(x) => x.requires_local_ref_cleanup(),
        }
    }
}

impl ConvertibleToJni for FunctionReturnValue {
    fn maybe_convert(&self, expr: &str) -> Option<String> {
        match self {
            Self::Basic(x) => x.maybe_convert(expr),
            Self::String(x) => x.maybe_convert(expr),
            Self::ClassRef(x) => x.maybe_convert(expr),
            Self::Struct(x) => x.maybe_convert(expr),
            Self::StructRef(x) => x.untyped().maybe_convert(expr),
        }
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        match self {
            Self::Basic(x) => x.requires_local_ref_cleanup(),
            Self::String(x) => x.requires_local_ref_cleanup(),
            Self::ClassRef(x) => x.requires_local_ref_cleanup(),
            Self::Struct(x) => x.requires_local_ref_cleanup(),
            Self::StructRef(x) => x.untyped().requires_local_ref_cleanup(),
        }
    }
}

impl ConvertibleToJni for IteratorItemType {
    fn maybe_convert(&self, expr: &str) -> Option<String> {
        match self {
            IteratorItemType::StructRef(x) => x.maybe_convert(expr),
        }
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        match self {
            IteratorItemType::StructRef(x) => x.requires_local_ref_cleanup(),
        }
    }
}

impl ConvertibleToJni for CallbackReturnValue {
    fn maybe_convert(&self, expr: &str) -> Option<String> {
        match self {
            Self::Basic(x) => x.maybe_convert(expr),
            Self::Struct(x) => x.maybe_convert(expr),
        }
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        match self {
            Self::Basic(x) => x.requires_local_ref_cleanup(),
            Self::Struct(x) => x.requires_local_ref_cleanup(),
        }
    }
}

impl<T, D> ConvertibleToJni for OptionalReturnType<T, D>
where
    D: DocReference,
    T: Clone + ConvertibleToJni,
{
    fn maybe_convert(&self, expr: &str) -> Option<String> {
        match self.get_value() {
            None => None,
            Some(return_type) => return_type.maybe_convert(expr),
        }
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        match self.get_value() {
            None => false,
            Some(return_type) => return_type.requires_local_ref_cleanup(),
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

    const U8: UnsignedConverter = UnsignedConverter::new("ubyte");
    const U16: UnsignedConverter = UnsignedConverter::new("ushort");
    const U32: UnsignedConverter = UnsignedConverter::new("uinteger");
    const U64: UnsignedConverter = UnsignedConverter::new("ulong");

    fn apply(&self, expr: &str) -> String {
        format!("_cache.joou.{}_from_rust(&_env, {})", self.java_type, expr)
    }
}
