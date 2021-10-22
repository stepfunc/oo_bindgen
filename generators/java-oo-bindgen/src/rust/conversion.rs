use super::formatting::*;
use heck::{CamelCase, SnakeCase};
use oo_bindgen::class::*;
use oo_bindgen::collection::*;
use oo_bindgen::enum_type::*;
use oo_bindgen::function::*;
use oo_bindgen::interface::*;
use oo_bindgen::iterator::*;
use oo_bindgen::return_type::ReturnType;
use oo_bindgen::structs::*;
use oo_bindgen::types::{BasicType, DurationType, StringType};
use oo_bindgen::{Handle, UniversalOr};

const JNI_SYS_JOBJECT: &str = "jni::sys::jobject";
const NULL_DEFAULT_VALUE: &str = "jni::objects::JObject::null().into_inner()";
// TODO - better name for this?
const OBJECT_UNWRAP: &str = "l().unwrap().into_inner()";

fn perform_null_check(f: &mut dyn Printer, param_name: &str) -> FormattingResult<()> {
    f.writeln(&format!("if _env.is_same_object({}, jni::objects::JObject::null()).unwrap() {{ return Err(\"{}\".to_string()); }}", param_name, param_name))
}

fn jni_object_sig(lib_path: &str, object_name: &str) -> String {
    format!("L{}/{};", lib_path, object_name.to_camel_case())
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
        lib_name: &str,
        prefix: &str,
    ) -> FormattingResult<()>;
    /// Returns converter is required by the type
    fn conversion(&self, lib_name: &str, prefix: &str) -> Option<Box<dyn TypeConverter>>;
    /// Indicates whether a local reference cleanup is required once we are done with the type
    fn requires_local_ref_cleanup(&self) -> bool;
    /// Check the parameter for null value. Must return an `Err(String)` if it's the case.
    fn check_null(&self, f: &mut dyn Printer, param_name: &str) -> FormattingResult<()>;
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
        _lib_name: &str,
        _prefix: &str,
    ) -> FormattingResult<()> {
        DurationConverter(*self).convert_to_rust(f, from, to)
    }

    fn conversion(&self, _lib_name: &str, _prefix: &str) -> Option<Box<dyn TypeConverter>> {
        Some(Box::new(DurationConverter(*self)))
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        true
    }

    fn check_null(&self, f: &mut dyn Printer, param_name: &str) -> FormattingResult<()> {
        perform_null_check(f, param_name)
    }

    fn default_value(&self) -> &str {
        NULL_DEFAULT_VALUE
    }
}

impl JniType for EnumHandle {
    fn as_raw_jni_type(&self) -> &str {
        JNI_SYS_JOBJECT
    }

    fn as_jni_sig(&self, lib_path: &str) -> String {
        format!("L{}/{};", lib_path, self.name.to_camel_case())
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
        _lib_name: &str,
        _prefix: &str,
    ) -> FormattingResult<()> {
        EnumConverter(self.clone()).convert_to_rust(f, from, to)
    }

    fn conversion(&self, _lib_name: &str, _prefix: &str) -> Option<Box<dyn TypeConverter>> {
        Some(Box::new(EnumConverter(self.clone())))
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        // We re-use a global ref here
        false
    }

    fn check_null(&self, f: &mut dyn Printer, param_name: &str) -> FormattingResult<()> {
        perform_null_check(f, param_name)
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
        _lib_name: &str,
        _prefix: &str,
    ) -> FormattingResult<()> {
        StringConverter.convert_to_rust(f, from, to)
    }

    fn conversion(&self, _lib_name: &str, _prefix: &str) -> Option<Box<dyn TypeConverter>> {
        Some(Box::new(StringConverter))
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        true
    }

    fn check_null(&self, f: &mut dyn Printer, param_name: &str) -> FormattingResult<()> {
        perform_null_check(f, param_name)
    }

    fn default_value(&self) -> &str {
        NULL_DEFAULT_VALUE
    }
}

impl JniType for BasicType {
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
            Self::Float32 => "jni::sys::jfloat",
            Self::Double64 => "jni::sys::jdouble",
            Self::Duration(x) => x.as_raw_jni_type(),
            Self::Enum(x) => x.as_raw_jni_type(),
        }
    }

    fn as_jni_sig(&self, lib_path: &str) -> String {
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
            Self::Float32 => "F".to_string(),
            Self::Double64 => "D".to_string(),
            Self::Duration(x) => x.as_jni_sig(lib_path),
            Self::Enum(x) => x.as_jni_sig(lib_path),
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
            Self::Float32 => "f().unwrap()",
            Self::Double64 => "d().unwrap()",
            Self::Duration(x) => x.convert_jvalue(),
            Self::Enum(x) => x.convert_jvalue(),
        }
    }

    fn convert_to_rust_from_object(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
        lib_name: &str,
        prefix: &str,
    ) -> FormattingResult<()> {
        match self {
            Self::Bool => f.writeln(&format!(
                "{}_cache.primitives.boolean_value(&_env, {})",
                to, from
            )),
            Self::U8 => UnsignedConverter("ubyte".to_string()).convert_to_rust(f, from, to),
            Self::S8 => f.writeln(&format!(
                "{}_cache.primitives.byte_value(&_env, {})",
                to, from
            )),
            Self::U16 => UnsignedConverter("ushort".to_string()).convert_to_rust(f, from, to),
            Self::S16 => f.writeln(&format!(
                "{}_cache.primitives.short_value(&_env, {})",
                to, from
            )),
            Self::U32 => UnsignedConverter("uinteger".to_string()).convert_to_rust(f, from, to),
            Self::S32 => f.writeln(&format!(
                "{}_cache.primitives.integer_value(&_env, {})",
                to, from
            )),
            Self::U64 => UnsignedConverter("ulong".to_string()).convert_to_rust(f, from, to),
            Self::S64 => f.writeln(&format!(
                "{}_cache.primitives.long_value(&_env, {})",
                to, from
            )),
            Self::Float32 => f.writeln(&format!(
                "{}_cache.primitives.float_value(&_env, {})",
                to, from
            )),
            Self::Double64 => f.writeln(&format!(
                "{}_cache.primitives.double_value(&_env, {})",
                to, from
            )),
            Self::Duration(x) => x.convert_to_rust_from_object(f, from, to, lib_name, prefix),
            Self::Enum(x) => x.convert_to_rust_from_object(f, from, to, lib_name, prefix),
        }
    }

    fn conversion(&self, lib_name: &str, prefix: &str) -> Option<Box<dyn TypeConverter>> {
        match self {
            Self::Bool => Some(Box::new(BooleanConverter)),
            Self::U8 => Some(Box::new(UnsignedConverter("ubyte".to_string()))),
            Self::S8 => None,
            Self::U16 => Some(Box::new(UnsignedConverter("ushort".to_string()))),
            Self::S16 => None,
            Self::U32 => Some(Box::new(UnsignedConverter("uinteger".to_string()))),
            Self::S32 => None,
            Self::U64 => Some(Box::new(UnsignedConverter("ulong".to_string()))),
            Self::S64 => None,
            Self::Float32 => None,
            Self::Double64 => None,
            Self::Duration(x) => x.conversion(lib_name, prefix),
            Self::Enum(x) => x.conversion(lib_name, prefix),
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
            Self::Float32 => false,
            Self::Double64 => false,
            Self::Duration(x) => x.requires_local_ref_cleanup(),
            Self::Enum(x) => x.requires_local_ref_cleanup(),
        }
    }

    fn check_null(&self, f: &mut dyn Printer, param_name: &str) -> FormattingResult<()> {
        match self {
            Self::Bool => Ok(()),
            Self::U8 => perform_null_check(f, param_name),
            Self::S8 => Ok(()),
            Self::U16 => perform_null_check(f, param_name),
            Self::S16 => Ok(()),
            Self::U32 => perform_null_check(f, param_name),
            Self::S32 => Ok(()),
            Self::U64 => perform_null_check(f, param_name),
            Self::S64 => Ok(()),
            Self::Float32 => Ok(()),
            Self::Double64 => Ok(()),
            Self::Duration(x) => x.check_null(f, param_name),
            Self::Enum(x) => x.check_null(f, param_name),
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
            Self::Float32 => "0.0",
            Self::Double64 => "0.0",
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
        format!("*const {}::ffi::{}", ffi_name, self.name.to_camel_case())
    }

    fn convert_jvalue(&self) -> &str {
        OBJECT_UNWRAP
    }

    fn convert_to_rust_from_object(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
        _lib_name: &str,
        _prefix: &str,
    ) -> FormattingResult<()> {
        StructRefConverter(self.clone()).convert_to_rust(f, from, to)
    }

    fn conversion(&self, _lib_name: &str, _prefix: &str) -> Option<Box<dyn TypeConverter>> {
        Some(Box::new(StructRefConverter(self.clone())))
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        true
    }

    fn check_null(&self, f: &mut dyn Printer, param_name: &str) -> FormattingResult<()> {
        perform_null_check(f, param_name)?;
        blocked(f, |f| {
            f.writeln(&format!("_cache.structs.struct_{}.check_null(_cache, &_env, {}).map_err(|_| \"{}\".to_string())?;", self.name.to_snake_case(), param_name, param_name))
        })
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
        format!("*mut {}::{}", ffi_name, self.name.to_camel_case())
    }

    fn convert_jvalue(&self) -> &str {
        OBJECT_UNWRAP
    }

    fn convert_to_rust_from_object(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
        _lib_name: &str,
        _prefix: &str,
    ) -> FormattingResult<()> {
        ClassConverter(self.clone()).convert_to_rust(f, from, to)
    }

    fn conversion(&self, _lib_name: &str, _prefix: &str) -> Option<Box<dyn TypeConverter>> {
        Some(Box::new(ClassConverter(self.clone())))
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        true
    }

    fn check_null(&self, f: &mut dyn Printer, param_name: &str) -> FormattingResult<()> {
        perform_null_check(f, param_name)
    }

    fn default_value(&self) -> &str {
        NULL_DEFAULT_VALUE
    }
}

impl JniType for InterfaceHandle {
    fn as_raw_jni_type(&self) -> &str {
        JNI_SYS_JOBJECT
    }

    fn as_jni_sig(&self, lib_path: &str) -> String {
        jni_object_sig(lib_path, &self.name)
    }

    fn as_rust_type(&self, ffi_name: &str) -> String {
        format!("{}::ffi::{}", ffi_name, self.name.to_camel_case())
    }

    fn convert_jvalue(&self) -> &str {
        OBJECT_UNWRAP
    }

    fn convert_to_rust_from_object(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
        _lib_name: &str,
        _prefix: &str,
    ) -> FormattingResult<()> {
        InterfaceConverter(self.clone()).convert_to_rust(f, from, to)
    }

    fn conversion(&self, _lib_name: &str, _prefix: &str) -> Option<Box<dyn TypeConverter>> {
        Some(Box::new(InterfaceConverter(self.clone())))
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        // This is freed by Rust
        false
    }

    fn check_null(&self, f: &mut dyn Printer, param_name: &str) -> FormattingResult<()> {
        perform_null_check(f, param_name)
    }

    fn default_value(&self) -> &str {
        NULL_DEFAULT_VALUE
    }
}

impl JniType for CollectionHandle {
    fn as_raw_jni_type(&self) -> &str {
        JNI_SYS_JOBJECT
    }

    fn as_jni_sig(&self, lib_path: &str) -> String {
        jni_object_sig(lib_path, self.name())
    }

    fn as_rust_type(&self, ffi_name: &str) -> String {
        format!("*mut {}::{}", ffi_name, self.name().to_camel_case())
    }

    fn convert_jvalue(&self) -> &str {
        OBJECT_UNWRAP
    }

    fn convert_to_rust_from_object(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
        lib_name: &str,
        prefix: &str,
    ) -> FormattingResult<()> {
        CollectionConverter(self.clone(), lib_name.to_string(), prefix.to_string())
            .convert_to_rust(f, from, to)
    }

    fn conversion(&self, lib_name: &str, prefix: &str) -> Option<Box<dyn TypeConverter>> {
        Some(Box::new(CollectionConverter(
            self.clone(),
            lib_name.to_string(),
            prefix.to_string(),
        )))
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        true
    }

    fn check_null(&self, f: &mut dyn Printer, param_name: &str) -> FormattingResult<()> {
        perform_null_check(f, param_name)
    }

    fn default_value(&self) -> &str {
        NULL_DEFAULT_VALUE
    }
}

impl JniType for IteratorHandle {
    fn as_raw_jni_type(&self) -> &str {
        JNI_SYS_JOBJECT
    }

    fn as_jni_sig(&self, _lib_path: &str) -> String {
        "Ljava/util/List;".to_string()
    }

    fn as_rust_type(&self, ffi_name: &str) -> String {
        format!("*mut {}::{}", ffi_name, self.name().to_camel_case())
    }

    fn convert_jvalue(&self) -> &str {
        OBJECT_UNWRAP
    }

    fn convert_to_rust_from_object(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
        lib_name: &str,
        prefix: &str,
    ) -> FormattingResult<()> {
        IteratorConverter(self.clone(), lib_name.to_string(), prefix.to_string())
            .convert_to_rust(f, from, to)
    }

    fn conversion(&self, lib_name: &str, prefix: &str) -> Option<Box<dyn TypeConverter>> {
        Some(Box::new(IteratorConverter(
            self.clone(),
            lib_name.to_string(),
            prefix.to_string(),
        )))
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        true
    }

    fn check_null(&self, f: &mut dyn Printer, param_name: &str) -> FormattingResult<()> {
        perform_null_check(f, param_name)
    }

    fn default_value(&self) -> &str {
        NULL_DEFAULT_VALUE
    }
}

impl<T> JniType for Handle<Struct<T>>
where
    T: StructFieldType,
{
    fn as_raw_jni_type(&self) -> &str {
        JNI_SYS_JOBJECT
    }

    fn as_jni_sig(&self, lib_path: &str) -> String {
        jni_object_sig(lib_path, self.name())
    }

    fn as_rust_type(&self, ffi_name: &str) -> String {
        format!("{}::ffi::{}", ffi_name, self.name().to_camel_case())
    }

    fn convert_jvalue(&self) -> &str {
        OBJECT_UNWRAP
    }

    fn convert_to_rust_from_object(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
        _lib_name: &str,
        _prefix: &str,
    ) -> FormattingResult<()> {
        StructConverter::new(self.declaration.clone()).convert_to_rust(f, from, to)
    }

    fn conversion(&self, _lib_name: &str, _prefix: &str) -> Option<Box<dyn TypeConverter>> {
        Some(Box::new(StructConverter::new(self.declaration.clone())))
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        true
    }

    fn check_null(&self, f: &mut dyn Printer, param_name: &str) -> FormattingResult<()> {
        perform_null_check(f, param_name)?;
        f.writeln(&format!("_cache.structs.struct_{}.check_null(_cache, &_env, {}).map_err(|_| \"{}\".to_string())?;", self.name().to_snake_case(), param_name, param_name))
    }

    fn default_value(&self) -> &str {
        NULL_DEFAULT_VALUE
    }
}

// TODO this is duplicated with Handle<Struct<T>>
impl<T> JniType for UniversalOr<T>
where
    T: StructFieldType,
{
    fn as_raw_jni_type(&self) -> &str {
        JNI_SYS_JOBJECT
    }

    fn as_jni_sig(&self, lib_path: &str) -> String {
        jni_object_sig(lib_path, self.name())
    }

    fn as_rust_type(&self, ffi_name: &str) -> String {
        format!("{}::ffi::{}", ffi_name, self.name().to_camel_case())
    }

    fn convert_jvalue(&self) -> &str {
        OBJECT_UNWRAP
    }

    fn convert_to_rust_from_object(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
        _lib_name: &str,
        _prefix: &str,
    ) -> FormattingResult<()> {
        StructConverter::new(self.declaration()).convert_to_rust(f, from, to)
    }

    fn conversion(&self, _lib_name: &str, _prefix: &str) -> Option<Box<dyn TypeConverter>> {
        Some(Box::new(StructConverter::new(self.declaration())))
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        true
    }

    fn check_null(&self, f: &mut dyn Printer, param_name: &str) -> FormattingResult<()> {
        perform_null_check(f, param_name)?;
        f.writeln(&format!("_cache.structs.struct_{}.check_null(_cache, &_env, {}).map_err(|_| \"{}\".to_string())?;", self.name().to_snake_case(), param_name, param_name))
    }

    fn default_value(&self) -> &str {
        NULL_DEFAULT_VALUE
    }
}

impl JniType for FunctionArgStructField {
    fn as_raw_jni_type(&self) -> &str {
        match self {
            Self::Basic(x) => x.as_raw_jni_type(),
            Self::String(x) => x.as_raw_jni_type(),
            Self::Interface(x) => x.as_raw_jni_type(),
            Self::Collection(x) => x.as_raw_jni_type(),
            Self::Struct(x) => x.as_raw_jni_type(),
        }
    }

    fn as_jni_sig(&self, lib_path: &str) -> String {
        match self {
            Self::Basic(x) => x.as_jni_sig(lib_path),
            Self::String(x) => x.as_jni_sig(lib_path),
            Self::Interface(x) => x.as_jni_sig(lib_path),
            Self::Collection(x) => x.as_jni_sig(lib_path),
            Self::Struct(x) => x.as_jni_sig(lib_path),
        }
    }

    fn as_rust_type(&self, ffi_name: &str) -> String {
        match self {
            Self::Basic(x) => x.as_rust_type(ffi_name),
            Self::String(x) => x.as_rust_type(ffi_name),
            Self::Interface(x) => x.as_rust_type(ffi_name),
            Self::Collection(x) => x.as_rust_type(ffi_name),
            Self::Struct(x) => x.as_rust_type(ffi_name),
        }
    }

    fn convert_jvalue(&self) -> &str {
        match self {
            Self::Basic(x) => x.convert_jvalue(),
            Self::String(x) => x.convert_jvalue(),
            Self::Interface(x) => x.convert_jvalue(),
            Self::Collection(x) => x.convert_jvalue(),
            Self::Struct(x) => x.convert_jvalue(),
        }
    }

    fn convert_to_rust_from_object(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
        lib_name: &str,
        prefix: &str,
    ) -> FormattingResult<()> {
        match self {
            Self::Basic(x) => x.convert_to_rust_from_object(f, from, to, lib_name, prefix),
            Self::String(x) => x.convert_to_rust_from_object(f, from, to, lib_name, prefix),
            Self::Interface(x) => x.convert_to_rust_from_object(f, from, to, lib_name, prefix),
            Self::Collection(x) => x.convert_to_rust_from_object(f, from, to, lib_name, prefix),
            Self::Struct(x) => x.convert_to_rust_from_object(f, from, to, lib_name, prefix),
        }
    }

    fn conversion(&self, lib_name: &str, prefix: &str) -> Option<Box<dyn TypeConverter>> {
        match self {
            Self::Basic(x) => x.conversion(lib_name, prefix),
            Self::String(x) => x.conversion(lib_name, prefix),
            Self::Interface(x) => x.conversion(lib_name, prefix),
            Self::Collection(x) => x.conversion(lib_name, prefix),
            Self::Struct(x) => x.conversion(lib_name, prefix),
        }
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        match self {
            Self::Basic(x) => x.requires_local_ref_cleanup(),
            Self::String(x) => x.requires_local_ref_cleanup(),
            Self::Interface(x) => x.requires_local_ref_cleanup(),
            Self::Collection(x) => x.requires_local_ref_cleanup(),
            Self::Struct(x) => x.requires_local_ref_cleanup(),
        }
    }

    fn check_null(&self, f: &mut dyn Printer, param_name: &str) -> FormattingResult<()> {
        match self {
            Self::Basic(x) => x.check_null(f, param_name),
            Self::String(x) => x.check_null(f, param_name),
            Self::Interface(x) => x.check_null(f, param_name),
            Self::Collection(x) => x.check_null(f, param_name),
            Self::Struct(x) => x.check_null(f, param_name),
        }
    }

    fn default_value(&self) -> &str {
        match self {
            Self::Basic(x) => x.default_value(),
            Self::String(x) => x.default_value(),
            Self::Interface(x) => x.default_value(),
            Self::Collection(x) => x.default_value(),
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
        lib_name: &str,
        prefix: &str,
    ) -> FormattingResult<()> {
        match self {
            Self::Basic(x) => x.convert_to_rust_from_object(f, from, to, lib_name, prefix),
            Self::ClassRef(x) => x.convert_to_rust_from_object(f, from, to, lib_name, prefix),
            Self::Struct(x) => x.convert_to_rust_from_object(f, from, to, lib_name, prefix),
            Self::Iterator(x) => x.convert_to_rust_from_object(f, from, to, lib_name, prefix),
        }
    }

    fn conversion(&self, lib_name: &str, prefix: &str) -> Option<Box<dyn TypeConverter>> {
        match self {
            Self::Basic(x) => x.conversion(lib_name, prefix),
            Self::ClassRef(x) => x.conversion(lib_name, prefix),
            Self::Struct(x) => x.conversion(lib_name, prefix),
            Self::Iterator(x) => x.conversion(lib_name, prefix),
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

    fn check_null(&self, f: &mut dyn Printer, param_name: &str) -> FormattingResult<()> {
        match self {
            Self::Basic(x) => x.check_null(f, param_name),
            Self::ClassRef(x) => x.check_null(f, param_name),
            Self::Struct(x) => x.check_null(f, param_name),
            Self::Iterator(x) => x.check_null(f, param_name),
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
        lib_name: &str,
        prefix: &str,
    ) -> FormattingResult<()> {
        match self {
            Self::Basic(x) => x.convert_to_rust_from_object(f, from, to, lib_name, prefix),
            Self::Iterator(x) => x.convert_to_rust_from_object(f, from, to, lib_name, prefix),
            Self::Struct(x) => x.convert_to_rust_from_object(f, from, to, lib_name, prefix),
        }
    }

    fn conversion(&self, lib_name: &str, prefix: &str) -> Option<Box<dyn TypeConverter>> {
        match self {
            Self::Basic(x) => x.conversion(lib_name, prefix),
            Self::Iterator(x) => x.conversion(lib_name, prefix),
            Self::Struct(x) => x.conversion(lib_name, prefix),
        }
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        match self {
            Self::Basic(x) => x.requires_local_ref_cleanup(),
            Self::Iterator(x) => x.requires_local_ref_cleanup(),
            Self::Struct(x) => x.requires_local_ref_cleanup(),
        }
    }

    fn check_null(&self, f: &mut dyn Printer, param_name: &str) -> FormattingResult<()> {
        match self {
            Self::Basic(x) => x.check_null(f, param_name),
            Self::Iterator(x) => x.check_null(f, param_name),
            Self::Struct(x) => x.check_null(f, param_name),
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
        lib_name: &str,
        prefix: &str,
    ) -> FormattingResult<()> {
        match self {
            Self::Basic(x) => x.convert_to_rust_from_object(f, from, to, lib_name, prefix),
            Self::Struct(x) => x.convert_to_rust_from_object(f, from, to, lib_name, prefix),
        }
    }

    fn conversion(&self, lib_name: &str, prefix: &str) -> Option<Box<dyn TypeConverter>> {
        match self {
            Self::Basic(x) => x.conversion(lib_name, prefix),
            Self::Struct(x) => x.conversion(lib_name, prefix),
        }
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        match self {
            Self::Basic(x) => x.requires_local_ref_cleanup(),
            Self::Struct(x) => x.requires_local_ref_cleanup(),
        }
    }

    fn check_null(&self, f: &mut dyn Printer, param_name: &str) -> FormattingResult<()> {
        match self {
            Self::Basic(x) => x.check_null(f, param_name),
            Self::Struct(x) => x.check_null(f, param_name),
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
            Self::StructRef(x) => x.as_raw_jni_type(),
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
            Self::StructRef(x) => x.as_jni_sig(lib_path),
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
            Self::StructRef(x) => x.as_rust_type(ffi_name),
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
            Self::StructRef(x) => x.convert_jvalue(),
            Self::ClassRef(x) => x.convert_jvalue(),
            Self::Interface(x) => x.convert_jvalue(),
        }
    }

    fn convert_to_rust_from_object(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
        lib_name: &str,
        prefix: &str,
    ) -> FormattingResult<()> {
        match self {
            Self::Basic(x) => x.convert_to_rust_from_object(f, from, to, lib_name, prefix),
            Self::String(x) => x.convert_to_rust_from_object(f, from, to, lib_name, prefix),
            Self::Collection(x) => x.convert_to_rust_from_object(f, from, to, lib_name, prefix),
            Self::Struct(x) => x.convert_to_rust_from_object(f, from, to, lib_name, prefix),
            Self::StructRef(x) => x.convert_to_rust_from_object(f, from, to, lib_name, prefix),
            Self::ClassRef(x) => x.convert_to_rust_from_object(f, from, to, lib_name, prefix),
            Self::Interface(x) => x.convert_to_rust_from_object(f, from, to, lib_name, prefix),
        }
    }

    fn conversion(&self, lib_name: &str, prefix: &str) -> Option<Box<dyn TypeConverter>> {
        match self {
            Self::Basic(x) => x.conversion(lib_name, prefix),
            Self::String(x) => x.conversion(lib_name, prefix),
            Self::Collection(x) => x.conversion(lib_name, prefix),
            Self::Struct(x) => x.conversion(lib_name, prefix),
            Self::StructRef(x) => x.conversion(lib_name, prefix),
            Self::ClassRef(x) => x.conversion(lib_name, prefix),
            Self::Interface(x) => x.conversion(lib_name, prefix),
        }
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        match self {
            Self::Basic(x) => x.requires_local_ref_cleanup(),
            Self::String(x) => x.requires_local_ref_cleanup(),
            Self::Collection(x) => x.requires_local_ref_cleanup(),
            Self::Struct(x) => x.requires_local_ref_cleanup(),
            Self::StructRef(x) => x.requires_local_ref_cleanup(),
            Self::ClassRef(x) => x.requires_local_ref_cleanup(),
            Self::Interface(x) => x.requires_local_ref_cleanup(),
        }
    }

    fn check_null(&self, f: &mut dyn Printer, param_name: &str) -> FormattingResult<()> {
        match self {
            Self::Basic(x) => x.check_null(f, param_name),
            Self::String(x) => x.check_null(f, param_name),
            Self::Collection(x) => x.check_null(f, param_name),
            Self::Struct(x) => x.check_null(f, param_name),
            Self::StructRef(x) => x.check_null(f, param_name),
            Self::ClassRef(x) => x.check_null(f, param_name),
            Self::Interface(x) => x.check_null(f, param_name),
        }
    }

    fn default_value(&self) -> &str {
        match self {
            Self::Basic(x) => x.default_value(),
            Self::String(x) => x.default_value(),
            Self::Collection(x) => x.default_value(),
            Self::Struct(x) => x.default_value(),
            Self::StructRef(x) => x.default_value(),
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
        lib_name: &str,
        prefix: &str,
    ) -> FormattingResult<()> {
        match self {
            Self::Basic(x) => x.convert_to_rust_from_object(f, from, to, lib_name, prefix),
            Self::String(x) => x.convert_to_rust_from_object(f, from, to, lib_name, prefix),
            Self::Iterator(x) => x.convert_to_rust_from_object(f, from, to, lib_name, prefix),
            Self::Struct(x) => x.convert_to_rust_from_object(f, from, to, lib_name, prefix),
            Self::Class(x) => x.convert_to_rust_from_object(f, from, to, lib_name, prefix),
        }
    }

    fn conversion(&self, lib_name: &str, prefix: &str) -> Option<Box<dyn TypeConverter>> {
        match self {
            Self::Basic(x) => x.conversion(lib_name, prefix),
            Self::String(x) => x.conversion(lib_name, prefix),
            Self::Iterator(x) => x.conversion(lib_name, prefix),
            Self::Struct(x) => x.conversion(lib_name, prefix),
            Self::Class(x) => x.conversion(lib_name, prefix),
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

    fn check_null(&self, f: &mut dyn Printer, param_name: &str) -> FormattingResult<()> {
        match self {
            Self::Basic(x) => x.check_null(f, param_name),
            Self::String(x) => x.check_null(f, param_name),
            Self::Iterator(x) => x.check_null(f, param_name),
            Self::Struct(x) => x.check_null(f, param_name),
            Self::Class(x) => x.check_null(f, param_name),
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
            Self::StructRef(x) => x.as_raw_jni_type(),
        }
    }

    fn as_jni_sig(&self, lib_path: &str) -> String {
        match self {
            Self::Basic(x) => x.as_jni_sig(lib_path),
            Self::String(x) => x.as_jni_sig(lib_path),
            Self::ClassRef(x) => x.as_jni_sig(lib_path),
            Self::Struct(x) => x.as_jni_sig(lib_path),
            Self::StructRef(x) => x.as_jni_sig(lib_path),
        }
    }

    fn as_rust_type(&self, ffi_name: &str) -> String {
        match self {
            Self::Basic(x) => x.as_rust_type(ffi_name),
            Self::String(x) => x.as_rust_type(ffi_name),
            Self::ClassRef(x) => x.as_rust_type(ffi_name),
            Self::Struct(x) => x.as_rust_type(ffi_name),
            Self::StructRef(x) => x.as_rust_type(ffi_name),
        }
    }

    fn convert_jvalue(&self) -> &str {
        match self {
            Self::Basic(x) => x.convert_jvalue(),
            Self::String(x) => x.convert_jvalue(),
            Self::ClassRef(x) => x.convert_jvalue(),
            Self::Struct(x) => x.convert_jvalue(),
            Self::StructRef(x) => x.convert_jvalue(),
        }
    }

    fn convert_to_rust_from_object(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
        lib_name: &str,
        prefix: &str,
    ) -> FormattingResult<()> {
        match self {
            Self::Basic(x) => x.convert_to_rust_from_object(f, from, to, lib_name, prefix),
            Self::String(x) => x.convert_to_rust_from_object(f, from, to, lib_name, prefix),
            Self::ClassRef(x) => x.convert_to_rust_from_object(f, from, to, lib_name, prefix),
            Self::Struct(x) => x.convert_to_rust_from_object(f, from, to, lib_name, prefix),
            Self::StructRef(x) => x.convert_to_rust_from_object(f, from, to, lib_name, prefix),
        }
    }

    fn conversion(&self, lib_name: &str, prefix: &str) -> Option<Box<dyn TypeConverter>> {
        match self {
            Self::Basic(x) => x.conversion(lib_name, prefix),
            Self::String(x) => x.conversion(lib_name, prefix),
            Self::ClassRef(x) => x.conversion(lib_name, prefix),
            Self::Struct(x) => x.conversion(lib_name, prefix),
            Self::StructRef(x) => x.conversion(lib_name, prefix),
        }
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        match self {
            Self::Basic(x) => x.requires_local_ref_cleanup(),
            Self::String(x) => x.requires_local_ref_cleanup(),
            Self::ClassRef(x) => x.requires_local_ref_cleanup(),
            Self::Struct(x) => x.requires_local_ref_cleanup(),
            Self::StructRef(x) => x.requires_local_ref_cleanup(),
        }
    }

    fn check_null(&self, f: &mut dyn Printer, param_name: &str) -> FormattingResult<()> {
        match self {
            Self::Basic(x) => x.check_null(f, param_name),
            Self::String(x) => x.check_null(f, param_name),
            Self::ClassRef(x) => x.check_null(f, param_name),
            Self::Struct(x) => x.check_null(f, param_name),
            Self::StructRef(x) => x.check_null(f, param_name),
        }
    }

    fn default_value(&self) -> &str {
        match self {
            Self::Basic(x) => x.default_value(),
            Self::String(x) => x.default_value(),
            Self::ClassRef(x) => x.default_value(),
            Self::Struct(x) => x.default_value(),
            Self::StructRef(x) => x.default_value(),
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
        lib_name: &str,
        prefix: &str,
    ) -> FormattingResult<()> {
        match self {
            Self::Basic(x) => x.convert_to_rust_from_object(f, from, to, lib_name, prefix),
            Self::Struct(x) => x.convert_to_rust_from_object(f, from, to, lib_name, prefix),
        }
    }

    fn conversion(&self, lib_name: &str, prefix: &str) -> Option<Box<dyn TypeConverter>> {
        match self {
            Self::Basic(x) => x.conversion(lib_name, prefix),
            Self::Struct(x) => x.conversion(lib_name, prefix),
        }
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        match self {
            Self::Basic(x) => x.requires_local_ref_cleanup(),
            Self::Struct(x) => x.requires_local_ref_cleanup(),
        }
    }

    fn check_null(&self, f: &mut dyn Printer, param_name: &str) -> FormattingResult<()> {
        match self {
            Self::Basic(x) => x.check_null(f, param_name),
            Self::Struct(x) => x.check_null(f, param_name),
        }
    }

    fn default_value(&self) -> &str {
        match self {
            Self::Basic(x) => x.default_value(),
            Self::Struct(x) => x.default_value(),
        }
    }
}

impl<T> JniType for ReturnType<T>
where
    T: JniType,
{
    fn as_raw_jni_type(&self) -> &str {
        match self {
            Self::Void => "()",
            Self::Type(return_type, _) => return_type.as_raw_jni_type(),
        }
    }

    fn as_jni_sig(&self, lib_path: &str) -> String {
        match self {
            Self::Void => "V".to_string(),
            Self::Type(return_type, _) => return_type.as_jni_sig(lib_path),
        }
    }

    fn as_rust_type(&self, ffi_name: &str) -> String {
        match self {
            Self::Void => "()".to_string(),
            Self::Type(return_type, _) => return_type.as_rust_type(ffi_name),
        }
    }

    fn convert_jvalue(&self) -> &str {
        match self {
            Self::Void => "v().unwrap()",
            Self::Type(return_type, _) => return_type.convert_jvalue(),
        }
    }

    fn convert_to_rust_from_object(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
        lib_name: &str,
        prefix: &str,
    ) -> FormattingResult<()> {
        match self {
            Self::Void => Ok(()),
            Self::Type(return_type, _) => {
                return_type.convert_to_rust_from_object(f, from, to, lib_name, prefix)
            }
        }
    }

    fn conversion(&self, lib_name: &str, prefix: &str) -> Option<Box<dyn TypeConverter>> {
        match self {
            Self::Void => None,
            Self::Type(return_type, _) => return_type.conversion(lib_name, prefix),
        }
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        match self {
            Self::Void => false,
            Self::Type(return_type, _) => return_type.requires_local_ref_cleanup(),
        }
    }

    fn check_null(&self, f: &mut dyn Printer, param_name: &str) -> FormattingResult<()> {
        match self {
            Self::Void => Ok(()),
            Self::Type(return_type, _) => return_type.check_null(f, param_name),
        }
    }

    fn default_value(&self) -> &str {
        match self {
            Self::Void => "",
            Self::Type(return_type, _) => return_type.default_value(),
        }
    }
}

pub(crate) trait TypeConverter {
    fn convert_to_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()>;
    fn convert_from_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()>;

    fn convert_to_rust_cleanup(&self, _f: &mut dyn Printer, _name: &str) -> FormattingResult<()> {
        Ok(())
    }
}

struct BooleanConverter;
impl TypeConverter for BooleanConverter {
    fn convert_to_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{}{} != 0", to, from))
    }

    fn convert_from_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{}{} as u8", to, from))
    }
}

struct UnsignedConverter(String);
impl TypeConverter for UnsignedConverter {
    fn convert_to_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!(
            "{}_cache.joou.{}_to_rust(&_env, {})",
            to, self.0, from
        ))
    }

    fn convert_from_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!(
            "{}_cache.joou.{}_from_rust(&_env, {})",
            to, self.0, from
        ))
    }
}

struct StringConverter;
impl TypeConverter for StringConverter {
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
    pub(crate) fn new(inner: StructDeclarationHandle) -> Self {
        Self { inner }
    }
}

impl TypeConverter for StructConverter {
    fn convert_to_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!(
            "{}_cache.structs.struct_{}.struct_to_rust(_cache, &_env, {})",
            to,
            self.inner.name.to_snake_case(),
            from
        ))
    }

    fn convert_from_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!(
            "{}_cache.structs.struct_{}.struct_from_rust(_cache, &_env, &{})",
            to,
            self.inner.name.to_snake_case(),
            from
        ))
    }

    fn convert_to_rust_cleanup(&self, f: &mut dyn Printer, name: &str) -> FormattingResult<()> {
        f.writeln(&format!(
            "_cache.structs.struct_{}.struct_to_rust_cleanup(_cache, &_env, &{});",
            self.inner.name.to_snake_case(),
            name
        ))
    }
}

struct StructRefConverter(StructDeclarationHandle);
impl TypeConverter for StructRefConverter {
    fn convert_to_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!(
            "{} if !_env.is_same_object({}, jni::objects::JObject::null()).unwrap()",
            to, from
        ))?;
        blocked(f, |f| {
            f.writeln(&format!(
                "let temp = Box::new(_cache.structs.struct_{}.struct_to_rust(_cache, &_env, {}));",
                self.0.name.to_snake_case(),
                from
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
                self.0.name.to_snake_case()
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
                self.0.name.to_snake_case()
            ))
        })
    }
}

pub(crate) struct EnumConverter(pub(crate) EnumHandle);
impl TypeConverter for EnumConverter {
    fn convert_to_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!(
            "{}_cache.enums.enum_{}.enum_to_rust(&_env, {})",
            to,
            self.0.name.to_snake_case(),
            from
        ))
    }

    fn convert_from_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!(
            "{}_cache.enums.enum_{}.enum_from_rust(&_env, {})",
            to,
            self.0.name.to_snake_case(),
            from
        ))
    }
}

struct ClassConverter(ClassDeclarationHandle);
impl TypeConverter for ClassConverter {
    fn convert_to_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!(
            "{}_cache.classes.{}_to_rust(&_env, {})",
            to,
            self.0.name.to_snake_case(),
            from
        ))
    }

    fn convert_from_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!(
            "{}_cache.classes.{}_from_rust(&_env, {})",
            to,
            self.0.name.to_snake_case(),
            from
        ))
    }
}

struct InterfaceConverter(InterfaceHandle);
impl TypeConverter for InterfaceConverter {
    fn convert_to_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!(
            "{}_cache.interfaces.interface_{}.interface_to_rust(&_env, {})",
            to,
            self.0.name.to_snake_case(),
            from
        ))
    }

    fn convert_from_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!(
            "{}if let Some(obj) = unsafe {{ ({}.{} as *mut jni::objects::GlobalRef).as_ref() }}",
            to,
            from,
            CTX_VARIABLE_NAME.to_snake_case()
        ))?;
        blocked(f, |f| f.writeln("obj.as_obj()"))?;
        f.writeln("else")?;
        blocked(f, |f| f.writeln("jni::objects::JObject::null()"))
    }
}

struct IteratorConverter(IteratorHandle, String, String);
impl TypeConverter for IteratorConverter {
    fn convert_to_rust(&self, f: &mut dyn Printer, _from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{}std::ptr::null_mut()", to))
    }

    fn convert_from_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(to)?;
        blocked(f, |f| {
            f.writeln("let array_list = _cache.collection.new_array_list(&_env);")?;
            f.writeln(&format!(
                "while let it = unsafe {{ {}::ffi::{}_{}({}) }}",
                self.1, self.2, self.0.function.name, from
            ))?;
            blocked(f, |f| {
                f.writeln("match unsafe { it.as_ref() }")?;
                blocked(f, |f| {
                    f.writeln("None => { break; }")?;
                    f.writeln("Some(it) => ")?;
                    blocked(f, |f| {
                        StructConverter::new(self.0.item_type.declaration()).convert_from_rust(
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

struct CollectionConverter(CollectionHandle, String, String);
impl TypeConverter for CollectionConverter {
    fn convert_to_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(to)?;
        blocked(f, |f| {
            if self.0.has_reserve {
                f.writeln(&format!(
                    "let _size = _cache.collection.get_size(&_env, {}.into());",
                    from
                ))?;
                f.writeln(&format!(
                    "let result = unsafe {{ {}::ffi::{}_{}(_size) }};",
                    self.1, self.2, self.0.create_func.name
                ))?;
            } else {
                f.writeln(&format!(
                    "let result = unsafe {{ {}::ffi::{}_{}() }};",
                    self.1, self.2, self.0.create_func.name
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
                    self.0.item_type.convert_to_rust_from_object(
                        f,
                        "next.into_inner()",
                        "let _next = ",
                        &self.1,
                        &self.2,
                    )?;
                    f.write(";")?;

                    f.writeln(&format!(
                        "unsafe {{ {}::ffi::{}_{}(result, _next) }};",
                        self.1, self.2, self.0.add_func.name
                    ))?;

                    if let Some(converter) = self.0.item_type.conversion(&self.1, &self.2) {
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

struct DurationConverter(DurationType);
impl TypeConverter for DurationConverter {
    fn convert_to_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        let method = match self.0 {
            DurationType::Milliseconds => "duration_to_millis",
            DurationType::Seconds => "duration_to_seconds",
        };

        f.writeln(&format!(
            "{}_cache.duration.{}(&_env, {})",
            to, method, from
        ))
    }

    fn convert_from_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        let method = match self.0 {
            DurationType::Milliseconds => "duration_from_millis",
            DurationType::Seconds => "duration_from_seconds",
        };

        f.writeln(&format!(
            "{}_cache.duration.{}(&_env, {})",
            to, method, from
        ))
    }
}
