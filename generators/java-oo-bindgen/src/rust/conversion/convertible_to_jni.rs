use oo_bindgen::backend::*;
use oo_bindgen::model::*;

pub(crate) trait ConvertibleToJni {
    /// Optional conversion from JNI argument type to Rust type
    fn conversion(&self) -> Option<TypeConverter>;
    /// Indicates whether a local reference cleanup is required once we are done with the type
    fn requires_local_ref_cleanup(&self) -> bool;
}

impl ConvertibleToJni for DurationType {
    fn conversion(&self) -> Option<TypeConverter> {
        Some(DurationConverter::wrap(*self))
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        true
    }
}

impl<D> ConvertibleToJni for Handle<Enum<D>>
where
    D: DocReference,
{
    fn conversion(&self) -> Option<TypeConverter> {
        Some(EnumConverter::wrap(self.clone()))
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        // We re-use a global ref here
        false
    }
}

impl ConvertibleToJni for StringType {
    fn conversion(&self) -> Option<TypeConverter> {
        Some(StringConverter::wrap())
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        true
    }
}

impl ConvertibleToJni for Primitive {
    fn conversion(&self) -> Option<TypeConverter> {
        match self {
            Self::Bool => Some(BooleanConverter::wrap()),
            Self::U8 => Some(UnsignedConverter::U8),
            Self::S8 => None,
            Self::U16 => Some(UnsignedConverter::U16),
            Self::S16 => None,
            Self::U32 => Some(UnsignedConverter::U32),
            Self::S32 => None,
            Self::U64 => Some(UnsignedConverter::U64),
            Self::S64 => None,
            Self::Float => None,
            Self::Double => None,
        }
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        // unsigned integers require cleanup since they're wrapped
        match self {
            Self::Bool => false,
            Self::U8 => true,
            Self::S8 => false,
            Self::U16 => true,
            Self::S16 => false,
            Self::U32 => true,
            Self::S32 => false,
            Self::U64 => true,
            Self::S64 => false,
            Self::Float => false,
            Self::Double => false,
        }
    }
}

impl ConvertibleToJni for BasicType {
    fn conversion(&self) -> Option<TypeConverter> {
        match self {
            Self::Primitive(x) => x.conversion(),
            Self::Duration(x) => x.conversion(),
            Self::Enum(x) => x.conversion(),
        }
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        match self {
            Self::Primitive(x) => x.requires_local_ref_cleanup(),
            Self::Duration(x) => x.requires_local_ref_cleanup(),
            Self::Enum(x) => x.requires_local_ref_cleanup(),
        }
    }
}

impl ConvertibleToJni for StructDeclarationHandle {
    fn conversion(&self) -> Option<TypeConverter> {
        Some(StructRefConverter::wrap(self.clone()))
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        true
    }
}

impl ConvertibleToJni for ClassDeclarationHandle {
    fn conversion(&self) -> Option<TypeConverter> {
        Some(ClassConverter::wrap(self.clone()))
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        true
    }
}

impl<D> ConvertibleToJni for Handle<AbstractIterator<D>>
where
    D: DocReference,
{
    fn conversion(&self) -> Option<TypeConverter> {
        Some(IteratorConverter::wrap(self.clone()))
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        true
    }
}

impl<T, D> ConvertibleToJni for Handle<Struct<T, D>>
where
    D: DocReference,
    T: StructFieldType,
{
    fn conversion(&self) -> Option<TypeConverter> {
        Some(StructConverter::wrap(self.declaration.inner.clone()))
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        true
    }
}

impl<T> ConvertibleToJni for UniversalOr<T>
where
    T: StructFieldType,
{
    fn conversion(&self) -> Option<TypeConverter> {
        match self {
            UniversalOr::Specific(x) => x.conversion(),
            UniversalOr::Universal(x) => x.conversion(),
        }
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        match self {
            UniversalOr::Specific(x) => x.requires_local_ref_cleanup(),
            UniversalOr::Universal(x) => x.requires_local_ref_cleanup(),
        }
    }
}

impl ConvertibleToJni for FunctionReturnStructField {
    fn conversion(&self) -> Option<TypeConverter> {
        match self {
            Self::Basic(x) => x.conversion(),
            Self::ClassRef(x) => x.conversion(),
            Self::Struct(x) => x.conversion(),
            Self::Iterator(x) => x.conversion(),
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
    fn conversion(&self) -> Option<TypeConverter> {
        match self {
            Self::Basic(x) => x.conversion(),
            Self::Iterator(x) => x.conversion(),
            Self::Struct(x) => x.conversion(),
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
    fn conversion(&self) -> Option<TypeConverter> {
        match self {
            Self::Basic(x) => x.conversion(),
            Self::Struct(x) => x.conversion(),
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
    fn conversion(&self) -> Option<TypeConverter> {
        match self {
            Self::Basic(x) => x.conversion(),
            Self::String(x) => x.conversion(),
            Self::Iterator(x) => x.conversion(),
            Self::Struct(x) => x.conversion(),
            Self::Class(x) => x.conversion(),
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
    fn conversion(&self) -> Option<TypeConverter> {
        match self {
            Self::Basic(x) => x.conversion(),
            Self::String(x) => x.conversion(),
            Self::ClassRef(x) => x.conversion(),
            Self::Struct(x) => x.conversion(),
            Self::StructRef(x) => x.untyped().conversion(),
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
    fn conversion(&self) -> Option<TypeConverter> {
        match self {
            IteratorItemType::Struct(x) => x.conversion(),
        }
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        match self {
            IteratorItemType::Struct(x) => x.requires_local_ref_cleanup(),
        }
    }
}

impl ConvertibleToJni for CallbackReturnValue {
    fn conversion(&self) -> Option<TypeConverter> {
        match self {
            Self::Basic(x) => x.conversion(),
            Self::Struct(x) => x.conversion(),
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
    fn conversion(&self) -> Option<TypeConverter> {
        match self.get_value() {
            None => None,
            Some(return_type) => return_type.conversion(),
        }
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        match self.get_value() {
            None => false,
            Some(return_type) => return_type.requires_local_ref_cleanup(),
        }
    }
}

trait TypeConverterTrait {
    fn convert_from_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()>;
}

pub(crate) enum TypeConverter {
    Bool(BooleanConverter),
    Unsigned(UnsignedConverter),
    String(StringConverter),
    Struct(StructConverter),
    StructRef(StructRefConverter),
    Enum(EnumConverter),
    Class(ClassConverter),
    Duration(DurationConverter),
    Iterator(IteratorConverter),
}

impl TypeConverter {
    pub(crate) fn to_jni(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        match self {
            TypeConverter::Bool(x) => x.convert_from_rust(f, from, to),
            TypeConverter::Unsigned(x) => x.convert_from_rust(f, from, to),
            TypeConverter::String(x) => x.convert_from_rust(f, from, to),
            TypeConverter::Struct(x) => x.convert_from_rust(f, from, to),
            TypeConverter::StructRef(x) => x.convert_from_rust(f, from, to),
            TypeConverter::Enum(x) => x.convert_from_rust(f, from, to),
            TypeConverter::Class(x) => x.convert_from_rust(f, from, to),
            TypeConverter::Duration(x) => x.convert_from_rust(f, from, to),
            TypeConverter::Iterator(x) => x.convert_from_rust(f, from, to),
        }
    }
}

pub(crate) struct BooleanConverter;
impl BooleanConverter {
    pub(crate) fn wrap() -> TypeConverter {
        TypeConverter::Bool(BooleanConverter {})
    }
}

impl TypeConverterTrait for BooleanConverter {
    fn convert_from_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{}{} as u8", to, from))
    }
}

pub(crate) struct UnsignedConverter {
    java_type: &'static str,
}

impl UnsignedConverter {
    const fn new(java_type: &'static str) -> Self {
        Self { java_type }
    }

    const U8: TypeConverter = TypeConverter::Unsigned(UnsignedConverter::new("ubyte"));
    const U16: TypeConverter = TypeConverter::Unsigned(UnsignedConverter::new("ushort"));
    const U32: TypeConverter = TypeConverter::Unsigned(UnsignedConverter::new("uinteger"));
    const U64: TypeConverter = TypeConverter::Unsigned(UnsignedConverter::new("ulong"));
}

impl TypeConverterTrait for UnsignedConverter {
    fn convert_from_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!(
            "{}_cache.joou.{}_from_rust(&_env, {})",
            to, self.java_type, from
        ))
    }
}

pub(crate) struct StringConverter;
impl StringConverter {
    fn wrap() -> TypeConverter {
        TypeConverter::String(StringConverter)
    }
}

impl TypeConverterTrait for StringConverter {
    fn convert_from_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(to)?;
        blocked(f, |f| {
            f.writeln(&format!(
                "_env.new_string(unsafe {{ std::ffi::CStr::from_ptr({}) }}.to_string_lossy()).unwrap().into_inner()",
                from
            ))
        })
    }
}

pub(crate) struct StructConverter {
    inner: StructDeclarationHandle,
}

impl StructConverter {
    fn wrap(inner: StructDeclarationHandle) -> TypeConverter {
        TypeConverter::Struct(Self { inner })
    }
}

impl TypeConverterTrait for StructConverter {
    fn convert_from_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!(
            "{}_cache.structs.{}.to_jni(_cache, &_env, &{})",
            to, self.inner.name, from
        ))
    }
}

pub(crate) struct StructRefConverter {
    handle: StructDeclarationHandle,
}
impl StructRefConverter {
    fn wrap(handle: StructDeclarationHandle) -> TypeConverter {
        TypeConverter::StructRef(StructRefConverter { handle })
    }
}

impl TypeConverterTrait for StructRefConverter {
    fn convert_from_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{}match unsafe {{ {}.as_ref() }}", to, from))?;
        blocked(f, |f| {
            f.writeln("None => jni::objects::JObject::null().into_inner(),")?;
            f.writeln(&format!(
                "Some(value) => _cache.structs.struct_{}.to_jni(_cache, &_env, &value),",
                self.handle.name
            ))
        })
    }
}

pub(crate) struct EnumConverter {
    name: Name,
}

impl EnumConverter {
    pub(crate) fn wrap<D: DocReference>(handle: Handle<Enum<D>>) -> TypeConverter {
        TypeConverter::Enum(EnumConverter {
            name: handle.name.clone(),
        })
    }
}
impl TypeConverterTrait for EnumConverter {
    fn convert_from_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!(
            "{}_cache.enums.enum_{}.enum_from_rust(&_env, {})",
            to, self.name, from
        ))
    }
}

pub(crate) struct ClassConverter {
    handle: ClassDeclarationHandle,
}

impl ClassConverter {
    fn wrap(handle: ClassDeclarationHandle) -> TypeConverter {
        TypeConverter::Class(ClassConverter { handle })
    }
}

impl TypeConverterTrait for ClassConverter {
    fn convert_from_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!(
            "{}_cache.classes.{}_from_rust(&_env, {})",
            to, self.handle.name, from
        ))
    }
}

pub(crate) struct IteratorConverter {
    iter_class: ClassDeclarationHandle,
}

impl IteratorConverter {
    pub(crate) fn wrap<D: DocReference>(handle: Handle<AbstractIterator<D>>) -> TypeConverter {
        TypeConverter::Iterator(Self {
            iter_class: handle.iter_class.clone(),
        })
    }
}

impl TypeConverterTrait for IteratorConverter {
    fn convert_from_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!(
            "{} crate::iterators::{}(&_env, _cache, {})",
            to, self.iter_class.name, from,
        ))
    }
}

pub(crate) struct DurationConverter {
    duration_type: DurationType,
}

impl DurationConverter {
    fn wrap(duration_type: DurationType) -> TypeConverter {
        TypeConverter::Duration(DurationConverter { duration_type })
    }
}

impl TypeConverterTrait for DurationConverter {
    fn convert_from_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        let method = match self.duration_type {
            DurationType::Milliseconds => "duration_from_millis",
            DurationType::Seconds => "duration_from_seconds",
        };

        f.writeln(&format!(
            "{}_cache.duration.{}(&_env, {})",
            to, method, from
        ))
    }
}
