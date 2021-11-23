use std::rc::Rc;

use oo_bindgen::backend::*;
use oo_bindgen::model::*;

const JNI_SYS_JOBJECT: &str = "jni::sys::jobject";
const NULL_DEFAULT_VALUE: &str = "jni::objects::JObject::null().into_inner()";
const OBJECT_UNWRAP: &str = "l().unwrap().into_inner()";

fn jni_object_sig(lib_path: &str, object_name: &Name) -> String {
    format!("L{}/{};", lib_path, object_name.camel_case())
}

pub(crate) trait JniType {
    /// Returns raw JNI type (from jni::sys::* module)
    fn as_raw_jni_type(&self) -> &str;
    /// Returns the JNI signature of the type
    fn as_jni_sig(&self, lib_path: &str) -> String;
    /// Return the Rust FFI type
    fn as_rust_type(&self, ffi_name: &str) -> String;
    /// Convert from jni::objects::JValue to raw JNI type (by calling one of the unwrappers)
    fn convert_jvalue(&self) -> &str;
    /// Convert to Rust from a JNI JObject (even for primitives).
    ///
    /// This should call the conversion routine for objects, but implement
    /// custom conversions for primitives.
    fn convert_to_rust_from_object(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()>;

    /// Returns converter is required by the type
    fn conversion(&self) -> Option<TypeConverter>;
    /// Indicates whether a local reference cleanup is required once we are done with the type
    fn requires_local_ref_cleanup(&self) -> bool;
    /// Returns the default raw JNI type value (used when throwing exceptions). Almost always `JObject::null` except for native types.
    fn default_value(&self) -> &str;
}

impl JniType for DurationType {
    fn as_raw_jni_type(&self) -> &str {
        JNI_SYS_JOBJECT
    }

    fn as_jni_sig(&self, _: &str) -> String {
        "Ljava/time/Duration;".to_string()
    }

    fn as_rust_type(&self, _: &str) -> String {
        match self {
            DurationType::Milliseconds | DurationType::Seconds => "u64".to_string(),
        }
    }

    fn convert_jvalue(&self) -> &str {
        OBJECT_UNWRAP
    }

    fn convert_to_rust_from_object(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        DurationConverter::wrap(*self).convert_to_rust(f, from, to)
    }

    fn conversion(&self) -> Option<TypeConverter> {
        Some(DurationConverter::wrap(*self))
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        true
    }

    fn default_value(&self) -> &str {
        NULL_DEFAULT_VALUE
    }
}

impl<D> JniType for Handle<Enum<D>>
where
    D: DocReference,
{
    fn as_raw_jni_type(&self) -> &str {
        JNI_SYS_JOBJECT
    }

    fn as_jni_sig(&self, lib_path: &str) -> String {
        format!("L{}/{};", lib_path, self.name.camel_case())
    }

    fn as_rust_type(&self, _ffi_name: &str) -> String {
        "std::os::raw::c_int".to_string()
    }

    fn convert_jvalue(&self) -> &str {
        OBJECT_UNWRAP
    }

    fn convert_to_rust_from_object(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        EnumConverter {
            name: self.name.clone(),
        }
        .convert_to_rust(f, from, to)
    }

    fn conversion(&self) -> Option<TypeConverter> {
        Some(EnumConverter::wrap(self.clone()))
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        // We re-use a global ref here
        false
    }

    fn default_value(&self) -> &str {
        NULL_DEFAULT_VALUE
    }
}

impl JniType for StringType {
    fn as_raw_jni_type(&self) -> &str {
        "jni::sys::jstring"
    }

    fn as_jni_sig(&self, _lib_path: &str) -> String {
        "Ljava/lang/String;".to_string()
    }

    fn as_rust_type(&self, _ffi_name: &str) -> String {
        "*const std::os::raw::c_char".to_string()
    }

    fn convert_jvalue(&self) -> &str {
        OBJECT_UNWRAP
    }

    fn convert_to_rust_from_object(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        StringConverter.convert_to_rust(f, from, to)
    }

    fn conversion(&self) -> Option<TypeConverter> {
        Some(StringConverter::wrap())
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        true
    }

    fn default_value(&self) -> &str {
        NULL_DEFAULT_VALUE
    }
}

impl JniType for Primitive {
    fn as_raw_jni_type(&self) -> &str {
        match self {
            Self::Bool => "jni::sys::jboolean",
            Self::U8 => JNI_SYS_JOBJECT,
            Self::S8 => "jni::sys::jbyte",
            Self::U16 => JNI_SYS_JOBJECT,
            Self::S16 => "jni::sys::jshort",
            Self::U32 => JNI_SYS_JOBJECT,
            Self::S32 => "jni::sys::jint",
            Self::U64 => JNI_SYS_JOBJECT,
            Self::S64 => "jni::sys::jlong",
            Self::Float => "jni::sys::jfloat",
            Self::Double => "jni::sys::jdouble",
        }
    }

    fn as_jni_sig(&self, _lib_path: &str) -> String {
        match self {
            Self::Bool => "Z".to_string(),
            Self::U8 => "Lorg/joou/UByte;".to_string(),
            Self::S8 => "B".to_string(),
            Self::U16 => "Lorg/joou/UShort;".to_string(),
            Self::S16 => "S".to_string(),
            Self::U32 => "Lorg/joou/UInteger;".to_string(),
            Self::S32 => "I".to_string(),
            Self::U64 => "Lorg/joou/ULong;".to_string(),
            Self::S64 => "J".to_string(),
            Self::Float => "F".to_string(),
            Self::Double => "D".to_string(),
        }
    }

    fn as_rust_type(&self, _ffi_name: &str) -> String {
        self.get_c_rust_type().to_string()
    }

    fn convert_jvalue(&self) -> &str {
        match self {
            Self::Bool => "z().unwrap() as u8",
            Self::U8 => "l().unwrap().into_inner()",
            Self::S8 => "b().unwrap()",
            Self::U16 => "l().unwrap().into_inner()",
            Self::S16 => "s().unwrap()",
            Self::U32 => "l().unwrap().into_inner()",
            Self::S32 => "i().unwrap()",
            Self::U64 => "l().unwrap().into_inner()",
            Self::S64 => "j().unwrap()",
            Self::Float => "f().unwrap()",
            Self::Double => "d().unwrap()",
        }
    }

    fn convert_to_rust_from_object(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        match self {
            Self::Bool => f.writeln(&format!(
                "{}_cache.primitives.boolean_value(&_env, {})",
                to, from
            )),
            Self::U8 => UnsignedConverter::U8.convert_to_rust(f, from, to),
            Self::S8 => f.writeln(&format!(
                "{}_cache.primitives.byte_value(&_env, {})",
                to, from
            )),
            Self::U16 => UnsignedConverter::U16.convert_to_rust(f, from, to),
            Self::S16 => f.writeln(&format!(
                "{}_cache.primitives.short_value(&_env, {})",
                to, from
            )),
            Self::U32 => UnsignedConverter::U32.convert_to_rust(f, from, to),
            Self::S32 => f.writeln(&format!(
                "{}_cache.primitives.integer_value(&_env, {})",
                to, from
            )),
            Self::U64 => UnsignedConverter::U64.convert_to_rust(f, from, to),
            Self::S64 => f.writeln(&format!(
                "{}_cache.primitives.long_value(&_env, {})",
                to, from
            )),
            Self::Float => f.writeln(&format!(
                "{}_cache.primitives.float_value(&_env, {})",
                to, from
            )),
            Self::Double => f.writeln(&format!(
                "{}_cache.primitives.double_value(&_env, {})",
                to, from
            )),
        }
    }

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

    fn default_value(&self) -> &str {
        match self {
            Self::Bool => "0",
            Self::U8 => NULL_DEFAULT_VALUE,
            Self::S8 => "0",
            Self::U16 => NULL_DEFAULT_VALUE,
            Self::S16 => "0",
            Self::U32 => NULL_DEFAULT_VALUE,
            Self::S32 => "0",
            Self::U64 => NULL_DEFAULT_VALUE,
            Self::S64 => "0",
            Self::Float => "0.0",
            Self::Double => "0.0",
        }
    }
}

impl JniType for BasicType {
    fn as_raw_jni_type(&self) -> &str {
        match self {
            Self::Primitive(x) => x.as_raw_jni_type(),
            Self::Duration(x) => x.as_raw_jni_type(),
            Self::Enum(x) => x.as_raw_jni_type(),
        }
    }

    fn as_jni_sig(&self, lib_path: &str) -> String {
        match self {
            Self::Primitive(x) => x.as_jni_sig(lib_path),
            Self::Duration(x) => x.as_jni_sig(lib_path),
            Self::Enum(x) => x.as_jni_sig(lib_path),
        }
    }

    fn as_rust_type(&self, _ffi_name: &str) -> String {
        self.get_c_rust_type().to_string()
    }

    fn convert_jvalue(&self) -> &str {
        match self {
            Self::Primitive(x) => x.convert_jvalue(),
            Self::Duration(x) => x.convert_jvalue(),
            Self::Enum(x) => x.convert_jvalue(),
        }
    }

    fn convert_to_rust_from_object(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        match self {
            Self::Primitive(x) => x.convert_to_rust_from_object(f, from, to),
            Self::Duration(x) => x.convert_to_rust_from_object(f, from, to),
            Self::Enum(x) => x.convert_to_rust_from_object(f, from, to),
        }
    }

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

    fn default_value(&self) -> &str {
        match self {
            Self::Primitive(x) => x.default_value(),
            Self::Duration(x) => x.default_value(),
            Self::Enum(x) => x.default_value(),
        }
    }
}

impl JniType for StructDeclarationHandle {
    fn as_raw_jni_type(&self) -> &str {
        JNI_SYS_JOBJECT
    }

    fn as_jni_sig(&self, lib_path: &str) -> String {
        jni_object_sig(lib_path, &self.name)
    }

    fn as_rust_type(&self, ffi_name: &str) -> String {
        format!("*const {}::ffi::{}", ffi_name, self.name.camel_case())
    }

    fn convert_jvalue(&self) -> &str {
        OBJECT_UNWRAP
    }

    fn convert_to_rust_from_object(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        StructRefConverter::wrap(self.clone()).convert_to_rust(f, from, to)
    }

    fn conversion(&self) -> Option<TypeConverter> {
        Some(StructRefConverter::wrap(self.clone()))
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        true
    }

    fn default_value(&self) -> &str {
        NULL_DEFAULT_VALUE
    }
}

impl JniType for ClassDeclarationHandle {
    fn as_raw_jni_type(&self) -> &str {
        JNI_SYS_JOBJECT
    }

    fn as_jni_sig(&self, lib_path: &str) -> String {
        jni_object_sig(lib_path, &self.name)
    }

    fn as_rust_type(&self, ffi_name: &str) -> String {
        format!("*mut {}::{}", ffi_name, self.name.camel_case())
    }

    fn convert_jvalue(&self) -> &str {
        OBJECT_UNWRAP
    }

    fn convert_to_rust_from_object(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        ClassConverter::wrap(self.clone()).convert_to_rust(f, from, to)
    }

    fn conversion(&self) -> Option<TypeConverter> {
        Some(ClassConverter::wrap(self.clone()))
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        true
    }

    fn default_value(&self) -> &str {
        NULL_DEFAULT_VALUE
    }
}

impl JniType for Handle<Interface<Unvalidated>> {
    fn as_raw_jni_type(&self) -> &str {
        JNI_SYS_JOBJECT
    }

    fn as_jni_sig(&self, lib_path: &str) -> String {
        jni_object_sig(lib_path, &self.name)
    }

    fn as_rust_type(&self, ffi_name: &str) -> String {
        format!("{}::ffi::{}", ffi_name, self.name.camel_case())
    }

    fn convert_jvalue(&self) -> &str {
        OBJECT_UNWRAP
    }

    fn convert_to_rust_from_object(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        InterfaceConverter::wrap(self.clone(), self.settings.clone()).convert_to_rust(f, from, to)
    }

    fn conversion(&self) -> Option<TypeConverter> {
        Some(InterfaceConverter::wrap(
            self.clone(),
            self.settings.clone(),
        ))
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        // This is freed by Rust
        false
    }

    fn default_value(&self) -> &str {
        NULL_DEFAULT_VALUE
    }
}

impl<D> JniType for Handle<Collection<D>>
where
    D: DocReference,
{
    fn as_raw_jni_type(&self) -> &str {
        JNI_SYS_JOBJECT
    }

    fn as_jni_sig(&self, lib_path: &str) -> String {
        jni_object_sig(lib_path, self.name())
    }

    fn as_rust_type(&self, ffi_name: &str) -> String {
        format!("*mut {}::{}", ffi_name, self.name().camel_case())
    }

    fn convert_jvalue(&self) -> &str {
        OBJECT_UNWRAP
    }

    fn convert_to_rust_from_object(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        CollectionConverter::wrap(self.clone()).convert_to_rust(f, from, to)
    }

    fn conversion(&self) -> Option<TypeConverter> {
        Some(CollectionConverter::wrap(self.clone()))
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        true
    }

    fn default_value(&self) -> &str {
        NULL_DEFAULT_VALUE
    }
}

impl<D> JniType for Handle<AbstractIterator<D>>
where
    D: DocReference,
{
    fn as_raw_jni_type(&self) -> &str {
        JNI_SYS_JOBJECT
    }

    fn as_jni_sig(&self, _lib_path: &str) -> String {
        "Ljava/util/List;".to_string()
    }

    fn as_rust_type(&self, ffi_name: &str) -> String {
        format!("*mut {}::{}", ffi_name, self.name().camel_case())
    }

    fn convert_jvalue(&self) -> &str {
        OBJECT_UNWRAP
    }

    fn convert_to_rust_from_object(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        IteratorConverter::wrap(self.clone()).convert_to_rust(f, from, to)
    }

    fn conversion(&self) -> Option<TypeConverter> {
        Some(IteratorConverter::wrap(self.clone()))
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        true
    }

    fn default_value(&self) -> &str {
        NULL_DEFAULT_VALUE
    }
}

impl<T, D> JniType for Handle<Struct<T, D>>
where
    D: DocReference,
    T: StructFieldType,
{
    fn as_raw_jni_type(&self) -> &str {
        JNI_SYS_JOBJECT
    }

    fn as_jni_sig(&self, lib_path: &str) -> String {
        jni_object_sig(lib_path, self.name())
    }

    fn as_rust_type(&self, ffi_name: &str) -> String {
        format!("{}::ffi::{}", ffi_name, self.name().camel_case())
    }

    fn convert_jvalue(&self) -> &str {
        OBJECT_UNWRAP
    }

    fn convert_to_rust_from_object(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        StructConverter::wrap(self.declaration.inner.clone()).convert_to_rust(f, from, to)
    }

    fn conversion(&self) -> Option<TypeConverter> {
        Some(StructConverter::wrap(self.declaration.inner.clone()))
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        true
    }

    fn default_value(&self) -> &str {
        NULL_DEFAULT_VALUE
    }
}

impl<T> JniType for UniversalOr<T>
where
    T: StructFieldType,
{
    fn as_raw_jni_type(&self) -> &str {
        match self {
            UniversalOr::Specific(x) => x.as_raw_jni_type(),
            UniversalOr::Universal(x) => x.as_raw_jni_type(),
        }
    }

    fn as_jni_sig(&self, lib_path: &str) -> String {
        match self {
            UniversalOr::Specific(x) => x.as_jni_sig(lib_path),
            UniversalOr::Universal(x) => x.as_jni_sig(lib_path),
        }
    }

    fn as_rust_type(&self, ffi_name: &str) -> String {
        match self {
            UniversalOr::Specific(x) => x.as_rust_type(ffi_name),
            UniversalOr::Universal(x) => x.as_rust_type(ffi_name),
        }
    }

    fn convert_jvalue(&self) -> &str {
        match self {
            UniversalOr::Specific(x) => x.convert_jvalue(),
            UniversalOr::Universal(x) => x.convert_jvalue(),
        }
    }

    fn convert_to_rust_from_object(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        match self {
            UniversalOr::Specific(x) => x.convert_to_rust_from_object(f, from, to),
            UniversalOr::Universal(x) => x.convert_to_rust_from_object(f, from, to),
        }
    }

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

    fn default_value(&self) -> &str {
        match self {
            UniversalOr::Specific(x) => x.default_value(),
            UniversalOr::Universal(x) => x.default_value(),
        }
    }
}

impl JniType for FunctionArgStructField {
    fn as_raw_jni_type(&self) -> &str {
        match self {
            Self::Basic(x) => x.as_raw_jni_type(),
            Self::String(x) => x.as_raw_jni_type(),
            Self::Interface(x) => x.inner.as_raw_jni_type(),
            Self::Struct(x) => x.as_raw_jni_type(),
        }
    }

    fn as_jni_sig(&self, lib_path: &str) -> String {
        match self {
            Self::Basic(x) => x.as_jni_sig(lib_path),
            Self::String(x) => x.as_jni_sig(lib_path),
            Self::Interface(x) => x.inner.as_jni_sig(lib_path),
            Self::Struct(x) => x.as_jni_sig(lib_path),
        }
    }

    fn as_rust_type(&self, ffi_name: &str) -> String {
        match self {
            Self::Basic(x) => x.as_rust_type(ffi_name),
            Self::String(x) => x.as_rust_type(ffi_name),
            Self::Interface(x) => x.inner.as_rust_type(ffi_name),
            Self::Struct(x) => x.as_rust_type(ffi_name),
        }
    }

    fn convert_jvalue(&self) -> &str {
        match self {
            Self::Basic(x) => x.convert_jvalue(),
            Self::String(x) => x.convert_jvalue(),
            Self::Interface(x) => x.inner.convert_jvalue(),
            Self::Struct(x) => x.convert_jvalue(),
        }
    }

    fn convert_to_rust_from_object(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        match self {
            Self::Basic(x) => x.convert_to_rust_from_object(f, from, to),
            Self::String(x) => x.convert_to_rust_from_object(f, from, to),
            Self::Interface(x) => x.inner.convert_to_rust_from_object(f, from, to),
            Self::Struct(x) => x.convert_to_rust_from_object(f, from, to),
        }
    }

    fn conversion(&self) -> Option<TypeConverter> {
        match self {
            Self::Basic(x) => x.conversion(),
            Self::String(x) => x.conversion(),
            Self::Interface(x) => x.inner.conversion(),
            Self::Struct(x) => x.conversion(),
        }
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        match self {
            Self::Basic(x) => x.requires_local_ref_cleanup(),
            Self::String(x) => x.requires_local_ref_cleanup(),
            Self::Interface(x) => x.inner.requires_local_ref_cleanup(),
            Self::Struct(x) => x.requires_local_ref_cleanup(),
        }
    }

    fn default_value(&self) -> &str {
        match self {
            Self::Basic(x) => x.default_value(),
            Self::String(x) => x.default_value(),
            Self::Interface(x) => x.inner.default_value(),
            Self::Struct(x) => x.default_value(),
        }
    }
}

impl JniType for FunctionReturnStructField {
    fn as_raw_jni_type(&self) -> &str {
        match self {
            Self::Basic(x) => x.as_raw_jni_type(),
            Self::ClassRef(x) => x.as_raw_jni_type(),
            Self::Struct(x) => x.as_raw_jni_type(),
            Self::Iterator(x) => x.as_raw_jni_type(),
        }
    }

    fn as_jni_sig(&self, lib_path: &str) -> String {
        match self {
            Self::Basic(x) => x.as_jni_sig(lib_path),
            Self::ClassRef(x) => x.as_jni_sig(lib_path),
            Self::Struct(x) => x.as_jni_sig(lib_path),
            Self::Iterator(x) => x.as_jni_sig(lib_path),
        }
    }

    fn as_rust_type(&self, ffi_name: &str) -> String {
        match self {
            Self::Basic(x) => x.as_rust_type(ffi_name),
            Self::ClassRef(x) => x.as_rust_type(ffi_name),
            Self::Struct(x) => x.as_rust_type(ffi_name),
            Self::Iterator(x) => x.as_rust_type(ffi_name),
        }
    }

    fn convert_jvalue(&self) -> &str {
        match self {
            Self::Basic(x) => x.convert_jvalue(),
            Self::ClassRef(x) => x.convert_jvalue(),
            Self::Struct(x) => x.convert_jvalue(),
            Self::Iterator(x) => x.convert_jvalue(),
        }
    }

    fn convert_to_rust_from_object(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        match self {
            Self::Basic(x) => x.convert_to_rust_from_object(f, from, to),
            Self::ClassRef(x) => x.convert_to_rust_from_object(f, from, to),
            Self::Struct(x) => x.convert_to_rust_from_object(f, from, to),
            Self::Iterator(x) => x.convert_to_rust_from_object(f, from, to),
        }
    }

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

    fn default_value(&self) -> &str {
        match self {
            Self::Basic(x) => x.default_value(),
            Self::ClassRef(x) => x.default_value(),
            Self::Struct(x) => x.default_value(),
            Self::Iterator(x) => x.default_value(),
        }
    }
}

impl JniType for CallbackArgStructField {
    fn as_raw_jni_type(&self) -> &str {
        match self {
            Self::Basic(x) => x.as_raw_jni_type(),
            Self::Iterator(x) => x.as_raw_jni_type(),
            Self::Struct(x) => x.as_raw_jni_type(),
        }
    }

    fn as_jni_sig(&self, lib_path: &str) -> String {
        match self {
            Self::Basic(x) => x.as_jni_sig(lib_path),
            Self::Iterator(x) => x.as_jni_sig(lib_path),
            Self::Struct(x) => x.as_jni_sig(lib_path),
        }
    }

    fn as_rust_type(&self, ffi_name: &str) -> String {
        match self {
            Self::Basic(x) => x.as_rust_type(ffi_name),
            Self::Iterator(x) => x.as_rust_type(ffi_name),
            Self::Struct(x) => x.as_rust_type(ffi_name),
        }
    }

    fn convert_jvalue(&self) -> &str {
        match self {
            Self::Basic(x) => x.convert_jvalue(),
            Self::Iterator(x) => x.convert_jvalue(),
            Self::Struct(x) => x.convert_jvalue(),
        }
    }

    fn convert_to_rust_from_object(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        match self {
            Self::Basic(x) => x.convert_to_rust_from_object(f, from, to),
            Self::Iterator(x) => x.convert_to_rust_from_object(f, from, to),
            Self::Struct(x) => x.convert_to_rust_from_object(f, from, to),
        }
    }

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

    fn default_value(&self) -> &str {
        match self {
            Self::Basic(x) => x.default_value(),
            Self::Iterator(x) => x.default_value(),
            Self::Struct(x) => x.default_value(),
        }
    }
}

impl JniType for UniversalStructField {
    fn as_raw_jni_type(&self) -> &str {
        match self {
            Self::Basic(x) => x.as_raw_jni_type(),
            Self::Struct(x) => x.as_raw_jni_type(),
        }
    }

    fn as_jni_sig(&self, lib_path: &str) -> String {
        match self {
            Self::Basic(x) => x.as_jni_sig(lib_path),
            Self::Struct(x) => x.as_jni_sig(lib_path),
        }
    }

    fn as_rust_type(&self, ffi_name: &str) -> String {
        match self {
            Self::Basic(x) => x.as_rust_type(ffi_name),
            Self::Struct(x) => x.as_rust_type(ffi_name),
        }
    }

    fn convert_jvalue(&self) -> &str {
        match self {
            Self::Basic(x) => x.convert_jvalue(),
            Self::Struct(x) => x.convert_jvalue(),
        }
    }

    fn convert_to_rust_from_object(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        match self {
            Self::Basic(x) => x.convert_to_rust_from_object(f, from, to),
            Self::Struct(x) => x.convert_to_rust_from_object(f, from, to),
        }
    }

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

    fn default_value(&self) -> &str {
        match self {
            Self::Basic(x) => x.default_value(),
            Self::Struct(x) => x.default_value(),
        }
    }
}

impl JniType for FunctionArgument {
    fn as_raw_jni_type(&self) -> &str {
        match self {
            Self::Basic(x) => x.as_raw_jni_type(),
            Self::String(x) => x.as_raw_jni_type(),
            Self::Collection(x) => x.as_raw_jni_type(),
            Self::Struct(x) => x.as_raw_jni_type(),
            Self::StructRef(x) => x.inner.as_raw_jni_type(),
            Self::ClassRef(x) => x.as_raw_jni_type(),
            Self::Interface(x) => x.as_raw_jni_type(),
        }
    }

    fn as_jni_sig(&self, lib_path: &str) -> String {
        match self {
            Self::Basic(x) => x.as_jni_sig(lib_path),
            Self::String(x) => x.as_jni_sig(lib_path),
            Self::Collection(x) => x.as_jni_sig(lib_path),
            Self::Struct(x) => x.as_jni_sig(lib_path),
            Self::StructRef(x) => x.inner.as_jni_sig(lib_path),
            Self::ClassRef(x) => x.as_jni_sig(lib_path),
            Self::Interface(x) => x.as_jni_sig(lib_path),
        }
    }

    fn as_rust_type(&self, ffi_name: &str) -> String {
        match self {
            Self::Basic(x) => x.as_rust_type(ffi_name),
            Self::String(x) => x.as_rust_type(ffi_name),
            Self::Collection(x) => x.as_rust_type(ffi_name),
            Self::Struct(x) => x.as_rust_type(ffi_name),
            Self::StructRef(x) => x.inner.as_rust_type(ffi_name),
            Self::ClassRef(x) => x.as_rust_type(ffi_name),
            Self::Interface(x) => x.as_rust_type(ffi_name),
        }
    }

    fn convert_jvalue(&self) -> &str {
        match self {
            Self::Basic(x) => x.convert_jvalue(),
            Self::String(x) => x.convert_jvalue(),
            Self::Collection(x) => x.convert_jvalue(),
            Self::Struct(x) => x.convert_jvalue(),
            Self::StructRef(x) => x.inner.convert_jvalue(),
            Self::ClassRef(x) => x.convert_jvalue(),
            Self::Interface(x) => x.convert_jvalue(),
        }
    }

    fn convert_to_rust_from_object(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        match self {
            Self::Basic(x) => x.convert_to_rust_from_object(f, from, to),
            Self::String(x) => x.convert_to_rust_from_object(f, from, to),
            Self::Collection(x) => x.convert_to_rust_from_object(f, from, to),
            Self::Struct(x) => x.convert_to_rust_from_object(f, from, to),
            Self::StructRef(x) => x.inner.convert_to_rust_from_object(f, from, to),
            Self::ClassRef(x) => x.convert_to_rust_from_object(f, from, to),
            Self::Interface(x) => x.convert_to_rust_from_object(f, from, to),
        }
    }

    fn conversion(&self) -> Option<TypeConverter> {
        match self {
            Self::Basic(x) => x.conversion(),
            Self::String(x) => x.conversion(),
            Self::Collection(x) => x.conversion(),
            Self::Struct(x) => x.conversion(),
            Self::StructRef(x) => x.inner.conversion(),
            Self::ClassRef(x) => x.conversion(),
            Self::Interface(x) => x.conversion(),
        }
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        match self {
            Self::Basic(x) => x.requires_local_ref_cleanup(),
            Self::String(x) => x.requires_local_ref_cleanup(),
            Self::Collection(x) => x.requires_local_ref_cleanup(),
            Self::Struct(x) => x.requires_local_ref_cleanup(),
            Self::StructRef(x) => x.inner.requires_local_ref_cleanup(),
            Self::ClassRef(x) => x.requires_local_ref_cleanup(),
            Self::Interface(x) => x.requires_local_ref_cleanup(),
        }
    }

    fn default_value(&self) -> &str {
        match self {
            Self::Basic(x) => x.default_value(),
            Self::String(x) => x.default_value(),
            Self::Collection(x) => x.default_value(),
            Self::Struct(x) => x.default_value(),
            Self::StructRef(x) => x.inner.default_value(),
            Self::ClassRef(x) => x.default_value(),
            Self::Interface(x) => x.default_value(),
        }
    }
}

impl JniType for CallbackArgument {
    fn as_raw_jni_type(&self) -> &str {
        match self {
            Self::Basic(x) => x.as_raw_jni_type(),
            Self::String(x) => x.as_raw_jni_type(),
            Self::Iterator(x) => x.as_raw_jni_type(),
            Self::Struct(x) => x.as_raw_jni_type(),
            Self::Class(x) => x.as_raw_jni_type(),
        }
    }

    fn as_jni_sig(&self, lib_path: &str) -> String {
        match self {
            Self::Basic(x) => x.as_jni_sig(lib_path),
            Self::String(x) => x.as_jni_sig(lib_path),
            Self::Iterator(x) => x.as_jni_sig(lib_path),
            Self::Struct(x) => x.as_jni_sig(lib_path),
            Self::Class(x) => x.as_jni_sig(lib_path),
        }
    }

    fn as_rust_type(&self, ffi_name: &str) -> String {
        match self {
            Self::Basic(x) => x.as_rust_type(ffi_name),
            Self::String(x) => x.as_rust_type(ffi_name),
            Self::Iterator(x) => x.as_rust_type(ffi_name),
            Self::Struct(x) => x.as_rust_type(ffi_name),
            Self::Class(x) => x.as_rust_type(ffi_name),
        }
    }

    fn convert_jvalue(&self) -> &str {
        match self {
            Self::Basic(x) => x.convert_jvalue(),
            Self::String(x) => x.convert_jvalue(),
            Self::Iterator(x) => x.convert_jvalue(),
            Self::Struct(x) => x.convert_jvalue(),
            Self::Class(x) => x.convert_jvalue(),
        }
    }

    fn convert_to_rust_from_object(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        match self {
            Self::Basic(x) => x.convert_to_rust_from_object(f, from, to),
            Self::String(x) => x.convert_to_rust_from_object(f, from, to),
            Self::Iterator(x) => x.convert_to_rust_from_object(f, from, to),
            Self::Struct(x) => x.convert_to_rust_from_object(f, from, to),
            Self::Class(x) => x.convert_to_rust_from_object(f, from, to),
        }
    }

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

    fn default_value(&self) -> &str {
        match self {
            Self::Basic(x) => x.default_value(),
            Self::String(x) => x.default_value(),
            Self::Iterator(x) => x.default_value(),
            Self::Struct(x) => x.default_value(),
            Self::Class(x) => x.default_value(),
        }
    }
}

impl JniType for FunctionReturnValue {
    fn as_raw_jni_type(&self) -> &str {
        match self {
            Self::Basic(x) => x.as_raw_jni_type(),
            Self::String(x) => x.as_raw_jni_type(),
            Self::ClassRef(x) => x.as_raw_jni_type(),
            Self::Struct(x) => x.as_raw_jni_type(),
            Self::StructRef(x) => x.untyped().as_raw_jni_type(),
        }
    }

    fn as_jni_sig(&self, lib_path: &str) -> String {
        match self {
            Self::Basic(x) => x.as_jni_sig(lib_path),
            Self::String(x) => x.as_jni_sig(lib_path),
            Self::ClassRef(x) => x.as_jni_sig(lib_path),
            Self::Struct(x) => x.as_jni_sig(lib_path),
            Self::StructRef(x) => x.untyped().as_jni_sig(lib_path),
        }
    }

    fn as_rust_type(&self, ffi_name: &str) -> String {
        match self {
            Self::Basic(x) => x.as_rust_type(ffi_name),
            Self::String(x) => x.as_rust_type(ffi_name),
            Self::ClassRef(x) => x.as_rust_type(ffi_name),
            Self::Struct(x) => x.as_rust_type(ffi_name),
            Self::StructRef(x) => x.untyped().as_rust_type(ffi_name),
        }
    }

    fn convert_jvalue(&self) -> &str {
        match self {
            Self::Basic(x) => x.convert_jvalue(),
            Self::String(x) => x.convert_jvalue(),
            Self::ClassRef(x) => x.convert_jvalue(),
            Self::Struct(x) => x.convert_jvalue(),
            Self::StructRef(x) => x.untyped().convert_jvalue(),
        }
    }

    fn convert_to_rust_from_object(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        match self {
            Self::Basic(x) => x.convert_to_rust_from_object(f, from, to),
            Self::String(x) => x.convert_to_rust_from_object(f, from, to),
            Self::ClassRef(x) => x.convert_to_rust_from_object(f, from, to),
            Self::Struct(x) => x.convert_to_rust_from_object(f, from, to),
            Self::StructRef(x) => x.untyped().convert_to_rust_from_object(f, from, to),
        }
    }

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

    fn default_value(&self) -> &str {
        match self {
            Self::Basic(x) => x.default_value(),
            Self::String(x) => x.default_value(),
            Self::ClassRef(x) => x.default_value(),
            Self::Struct(x) => x.default_value(),
            Self::StructRef(x) => x.untyped().default_value(),
        }
    }
}

impl JniType for CallbackReturnValue {
    fn as_raw_jni_type(&self) -> &str {
        match self {
            Self::Basic(x) => x.as_raw_jni_type(),
            Self::Struct(x) => x.as_raw_jni_type(),
        }
    }

    fn as_jni_sig(&self, lib_path: &str) -> String {
        match self {
            Self::Basic(x) => x.as_jni_sig(lib_path),
            Self::Struct(x) => x.as_jni_sig(lib_path),
        }
    }

    fn as_rust_type(&self, ffi_name: &str) -> String {
        match self {
            Self::Basic(x) => x.as_rust_type(ffi_name),
            Self::Struct(x) => x.as_rust_type(ffi_name),
        }
    }

    fn convert_jvalue(&self) -> &str {
        match self {
            Self::Basic(x) => x.convert_jvalue(),
            Self::Struct(x) => x.convert_jvalue(),
        }
    }

    fn convert_to_rust_from_object(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        match self {
            Self::Basic(x) => x.convert_to_rust_from_object(f, from, to),
            Self::Struct(x) => x.convert_to_rust_from_object(f, from, to),
        }
    }

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

    fn default_value(&self) -> &str {
        match self {
            Self::Basic(x) => x.default_value(),
            Self::Struct(x) => x.default_value(),
        }
    }
}

impl<T, D> JniType for OptionalReturnType<T, D>
where
    D: DocReference,
    T: Clone + JniType,
{
    fn as_raw_jni_type(&self) -> &str {
        match self.get_value() {
            None => "()",
            Some(return_type) => return_type.as_raw_jni_type(),
        }
    }

    fn as_jni_sig(&self, lib_path: &str) -> String {
        match self.get_value() {
            None => "V".to_string(),
            Some(return_type) => return_type.as_jni_sig(lib_path),
        }
    }

    fn as_rust_type(&self, ffi_name: &str) -> String {
        match self.get_value() {
            None => "()".to_string(),
            Some(return_type) => return_type.as_rust_type(ffi_name),
        }
    }

    fn convert_jvalue(&self) -> &str {
        match self.get_value() {
            None => "v().unwrap()",
            Some(return_type) => return_type.convert_jvalue(),
        }
    }

    fn convert_to_rust_from_object(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        match self.get_value() {
            None => Ok(()),
            Some(return_type) => return_type.convert_to_rust_from_object(f, from, to),
        }
    }

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

    fn default_value(&self) -> &str {
        match self.get_value() {
            None => "",
            Some(return_type) => return_type.default_value(),
        }
    }
}

trait TypeConverterTrait {
    fn convert_to_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()>;
    fn convert_from_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()>;

    fn convert_to_rust_cleanup(&self, _f: &mut dyn Printer, _name: &str) -> FormattingResult<()> {
        Ok(())
    }
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
    Interface(InterfaceConverter),
    Collection(CollectionConverter),
    Iterator(IteratorConverter),
}

impl TypeConverter {
    pub(crate) fn convert_to_rust(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        match self {
            TypeConverter::Bool(x) => x.convert_to_rust(f, from, to),
            TypeConverter::Unsigned(x) => x.convert_to_rust(f, from, to),
            TypeConverter::String(x) => x.convert_to_rust(f, from, to),
            TypeConverter::Struct(x) => x.convert_to_rust(f, from, to),
            TypeConverter::StructRef(x) => x.convert_to_rust(f, from, to),
            TypeConverter::Enum(x) => x.convert_to_rust(f, from, to),
            TypeConverter::Class(x) => x.convert_to_rust(f, from, to),
            TypeConverter::Duration(x) => x.convert_to_rust(f, from, to),
            TypeConverter::Interface(x) => x.convert_to_rust(f, from, to),
            TypeConverter::Collection(x) => x.convert_to_rust(f, from, to),
            TypeConverter::Iterator(x) => x.convert_to_rust(f, from, to),
        }
    }

    pub(crate) fn convert_from_rust(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        match self {
            TypeConverter::Bool(x) => x.convert_from_rust(f, from, to),
            TypeConverter::Unsigned(x) => x.convert_from_rust(f, from, to),
            TypeConverter::String(x) => x.convert_from_rust(f, from, to),
            TypeConverter::Struct(x) => x.convert_from_rust(f, from, to),
            TypeConverter::StructRef(x) => x.convert_from_rust(f, from, to),
            TypeConverter::Enum(x) => x.convert_from_rust(f, from, to),
            TypeConverter::Class(x) => x.convert_from_rust(f, from, to),
            TypeConverter::Duration(x) => x.convert_from_rust(f, from, to),
            TypeConverter::Interface(x) => x.convert_from_rust(f, from, to),
            TypeConverter::Collection(x) => x.convert_from_rust(f, from, to),
            TypeConverter::Iterator(x) => x.convert_from_rust(f, from, to),
        }
    }

    pub(crate) fn convert_to_rust_cleanup(
        &self,
        f: &mut dyn Printer,
        name: &str,
    ) -> FormattingResult<()> {
        match self {
            TypeConverter::Bool(x) => x.convert_to_rust_cleanup(f, name),
            TypeConverter::Unsigned(x) => x.convert_to_rust_cleanup(f, name),
            TypeConverter::String(x) => x.convert_to_rust_cleanup(f, name),
            TypeConverter::Struct(x) => x.convert_to_rust_cleanup(f, name),
            TypeConverter::StructRef(x) => x.convert_to_rust_cleanup(f, name),
            TypeConverter::Enum(x) => x.convert_to_rust_cleanup(f, name),
            TypeConverter::Class(x) => x.convert_to_rust_cleanup(f, name),
            TypeConverter::Duration(x) => x.convert_to_rust_cleanup(f, name),
            TypeConverter::Interface(x) => x.convert_to_rust_cleanup(f, name),
            TypeConverter::Collection(x) => x.convert_to_rust_cleanup(f, name),
            TypeConverter::Iterator(x) => x.convert_to_rust_cleanup(f, name),
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
    fn convert_to_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{}{} != 0", to, from))
    }

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
    fn convert_to_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!(
            "{}_cache.joou.{}_to_rust(&_env, {})",
            to, self.java_type, from
        ))
    }

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
    fn convert_to_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(to)?;
        blocked(f, |f| {
            f.writeln(&format!(
                "let value = _env.get_string({}.into()).unwrap();",
                from
            ))?;
            f.writeln("(**value).to_owned().into_raw()")
        })
    }

    fn convert_from_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(to)?;
        blocked(f, |f| {
            f.writeln(&format!(
                "let string = unsafe {{ std::ffi::CStr::from_ptr({}) }}.to_string_lossy();",
                from
            ))?;
            f.writeln("_env.new_string(string).unwrap().into_inner()")
        })
    }

    fn convert_to_rust_cleanup(&self, f: &mut dyn Printer, name: &str) -> FormattingResult<()> {
        f.writeln(&format!(
            "unsafe {{ std::ffi::CString::from_raw({} as *mut _) }};",
            name
        ))
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
    fn convert_to_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!(
            "{}_cache.structs.struct_{}.struct_to_rust(_cache, &_env, {})",
            to, self.inner.name, from
        ))
    }

    fn convert_from_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!(
            "{}_cache.structs.struct_{}.struct_from_rust(_cache, &_env, &{})",
            to, self.inner.name, from
        ))
    }

    fn convert_to_rust_cleanup(&self, f: &mut dyn Printer, name: &str) -> FormattingResult<()> {
        f.writeln(&format!(
            "_cache.structs.struct_{}.struct_to_rust_cleanup(_cache, &_env, &{});",
            self.inner.name, name
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
    fn convert_to_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!(
            "{} if !_env.is_same_object({}, jni::objects::JObject::null()).unwrap()",
            to, from
        ))?;
        blocked(f, |f| {
            f.writeln(&format!(
                "let temp = Box::new(_cache.structs.struct_{}.struct_to_rust(_cache, &_env, {}));",
                self.handle.name, from
            ))?;
            f.writeln("Box::into_raw(temp)")
        })?;
        f.writeln("else")?;
        blocked(f, |f| f.writeln("std::ptr::null()"))
    }

    fn convert_from_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{}match unsafe {{ {}.as_ref() }}", to, from))?;
        blocked(f, |f| {
            f.writeln("None => jni::objects::JObject::null().into_inner(),")?;
            f.writeln(&format!(
                "Some(value) => _cache.structs.struct_{}.struct_from_rust(_cache, &_env, &value),",
                self.handle.name
            ))
        })
    }

    fn convert_to_rust_cleanup(&self, f: &mut dyn Printer, name: &str) -> FormattingResult<()> {
        f.writeln(&format!("if {}.is_null()", name))?;
        blocked(f, |f| {
            f.writeln(&format!(
                "let temp = unsafe {{ Box::from_raw({} as *mut _) }};",
                name
            ))?;
            f.writeln(&format!(
                "_cache.structs.struct_{}.struct_to_rust_cleanup(_cache, &_env, &temp)",
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
    fn convert_to_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!(
            "{}_cache.enums.enum_{}.enum_to_rust(&_env, {})",
            to, self.name, from
        ))
    }

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
    fn convert_to_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!(
            "{}_cache.classes.{}_to_rust(&_env, {})",
            to, self.handle.name, from
        ))
    }

    fn convert_from_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!(
            "{}_cache.classes.{}_from_rust(&_env, {})",
            to, self.handle.name, from
        ))
    }
}

pub(crate) struct InterfaceConverter {
    name: Name,
    settings: Rc<LibrarySettings>,
}

impl InterfaceConverter {
    pub(crate) fn wrap<D: DocReference>(
        handle: Handle<Interface<D>>,
        settings: Rc<LibrarySettings>,
    ) -> TypeConverter {
        TypeConverter::Interface(Self {
            name: handle.name.clone(),
            settings,
        })
    }
}

impl TypeConverterTrait for InterfaceConverter {
    fn convert_to_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!(
            "{}_cache.interfaces.interface_{}.interface_to_rust(&_env, {})",
            to, self.name, from
        ))
    }

    fn convert_from_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!(
            "{}if let Some(obj) = unsafe {{ ({}.{} as *mut jni::objects::GlobalRef).as_ref() }}",
            to, from, self.settings.interface.context_variable_name
        ))?;
        blocked(f, |f| f.writeln("obj.as_obj()"))?;
        f.writeln("else")?;
        blocked(f, |f| f.writeln("jni::objects::JObject::null()"))
    }
}

pub(crate) struct IteratorConverter {
    next_func: Name,
    item_type: IteratorItemType,
    settings: Rc<LibrarySettings>,
}

impl IteratorConverter {
    pub(crate) fn wrap<D: DocReference>(handle: Handle<AbstractIterator<D>>) -> TypeConverter {
        TypeConverter::Iterator(Self {
            next_func: handle.next_function.name.clone(),
            item_type: handle.item_type.clone(),
            settings: handle.settings.clone(),
        })
    }
}

impl TypeConverterTrait for IteratorConverter {
    fn convert_to_rust(&self, f: &mut dyn Printer, _from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{}std::ptr::null_mut()", to))
    }

    fn convert_from_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(to)?;
        blocked(f, |f| {
            f.writeln("let array_list = _cache.collection.new_array_list(&_env);")?;
            f.writeln(&format!(
                "while let it = unsafe {{ {}_ffi::ffi::{}_{}({}) }}",
                self.settings.c_ffi_prefix, self.settings.c_ffi_prefix, self.next_func, from
            ))?;
            blocked(f, |f| {
                f.writeln("match unsafe { it.as_ref() }")?;
                blocked(f, |f| {
                    f.writeln("None => { break; }")?;
                    f.writeln("Some(it) => ")?;
                    blocked(f, |f| {
                        StructConverter::wrap(self.item_type.declaration()).convert_from_rust(
                            f,
                            "it",
                            "let item = ",
                        )?;
                        f.write(";")?;

                        f.writeln(
                            "_cache.collection.add_to_array_list(&_env, array_list, item.into());",
                        )?;
                        f.writeln("_env.delete_local_ref(item.into()).unwrap();")
                    })?;
                    f.write(",")
                })
            })?;
            f.writeln("array_list.into_inner()")
        })
    }
}

pub(crate) struct CollectionConverter {
    has_reserve: bool,
    create_func: Name,
    add_func: Name,
    item_type: FunctionArgument,
    settings: Rc<LibrarySettings>,
}

impl CollectionConverter {
    pub(crate) fn wrap<D>(handle: Handle<Collection<D>>) -> TypeConverter
    where
        D: DocReference,
    {
        TypeConverter::Collection(Self {
            has_reserve: handle.has_reserve,
            create_func: handle.create_func.name.clone(),
            add_func: handle.add_func.name.clone(),
            item_type: handle.item_type.clone(),
            settings: handle.create_func.settings.clone(),
        })
    }
}

impl TypeConverterTrait for CollectionConverter {
    fn convert_to_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(to)?;
        blocked(f, |f| {
            if self.has_reserve {
                f.writeln(&format!(
                    "let _size = _cache.collection.get_size(&_env, {}.into());",
                    from
                ))?;
                f.writeln(&format!(
                    "let result = unsafe {{ {}_ffi::ffi::{}_{}(_size) }};",
                    self.settings.c_ffi_prefix, self.settings.c_ffi_prefix, self.create_func
                ))?;
            } else {
                f.writeln(&format!(
                    "let result = unsafe {{ {}_ffi::ffi::{}_{}() }};",
                    self.settings.c_ffi_prefix, self.settings.c_ffi_prefix, self.create_func
                ))?;
            }
            f.writeln(&format!(
                "let _it = _cache.collection.get_iterator(&_env, {}.into());",
                from
            ))?;
            f.writeln("while _cache.collection.has_next(&_env, _it)")?;
            blocked(f, |f| {
                f.writeln("let next = _cache.collection.next(&_env, _it);")?;
                f.writeln("if !_env.is_same_object(next, jni::objects::JObject::null()).unwrap()")?;
                blocked(f, |f| {
                    self.item_type.convert_to_rust_from_object(
                        f,
                        "next.into_inner()",
                        "let _next = ",
                    )?;
                    f.write(";")?;

                    f.writeln(&format!(
                        "unsafe {{ {}_ffi::ffi::{}_{}(result, _next) }};",
                        self.settings.c_ffi_prefix, self.settings.c_ffi_prefix, self.add_func
                    ))?;

                    if let Some(converter) = self.item_type.conversion() {
                        converter.convert_to_rust_cleanup(f, "_next")?;
                    }

                    f.writeln("_env.delete_local_ref(next.into()).unwrap();")?;

                    Ok(())
                })
            })?;
            f.writeln("_env.delete_local_ref(_it).unwrap();")?;
            f.writeln("result")
        })
    }

    fn convert_from_rust(
        &self,
        f: &mut dyn Printer,
        _from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        f.writeln(&format!(
            "{}jni::objects::JObject::null()::into_inner()",
            to
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
    fn convert_to_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        let method = match self.duration_type {
            DurationType::Milliseconds => "duration_to_millis",
            DurationType::Seconds => "duration_to_seconds",
        };

        f.writeln(&format!(
            "{}_cache.duration.{}(&_env, {})",
            to, method, from
        ))
    }

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
