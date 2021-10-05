use super::formatting::*;
use heck::{CamelCase, SnakeCase};
use oo_bindgen::class::*;
use oo_bindgen::collection::*;
use oo_bindgen::enum_type::*;
use oo_bindgen::formatting::*;
use oo_bindgen::function::*;
use oo_bindgen::interface::*;
use oo_bindgen::iterator::*;
use oo_bindgen::structs::any_struct::*;
use oo_bindgen::structs::common::StructDeclarationHandle;
use oo_bindgen::types::{AnyType, BasicType, DurationType};

pub(crate) trait JniType {
    /// Returns raw JNI type (from jni::sys::* module)
    fn as_raw_jni_type(&self) -> String;
    /// Returns the JNI signature of the type
    fn as_jni_sig(&self, lib_path: &str) -> String;
    /// Return the Rust FFI type
    fn as_rust_type(&self, ffi_name: &str) -> String;
    /// Convert from jni::objects::JValue to raw JNI type (by calling one of the unwrappers)
    fn convert_jvalue(&self) -> String;
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
    fn default_value(&self) -> String;
}

impl JniType for BasicType {
    fn as_raw_jni_type(&self) -> String {
        match self {
            Self::Bool => "jni::sys::jboolean".to_string(),
            Self::Uint8 => "jni::sys::jobject".to_string(),
            Self::Sint8 => "jni::sys::jbyte".to_string(),
            Self::Uint16 => "jni::sys::jobject".to_string(),
            Self::Sint16 => "jni::sys::jshort".to_string(),
            Self::Uint32 => "jni::sys::jobject".to_string(),
            Self::Sint32 => "jni::sys::jint".to_string(),
            Self::Uint64 => "jni::sys::jobject".to_string(),
            Self::Sint64 => "jni::sys::jlong".to_string(),
            Self::Float => "jni::sys::jfloat".to_string(),
            Self::Double => "jni::sys::jdouble".to_string(),
            Self::Duration(_) => "jni::sys::jobject".to_string(),
            Self::Enum(_) => "jni::sys::jobject".to_string(),
        }
    }

    fn as_jni_sig(&self, lib_path: &str) -> String {
        match self {
            Self::Bool => "Z".to_string(),
            Self::Uint8 => "Lorg/joou/UByte;".to_string(),
            Self::Sint8 => "B".to_string(),
            Self::Uint16 => "Lorg/joou/UShort;".to_string(),
            Self::Sint16 => "S".to_string(),
            Self::Uint32 => "Lorg/joou/UInteger;".to_string(),
            Self::Sint32 => "I".to_string(),
            Self::Uint64 => "Lorg/joou/ULong;".to_string(),
            Self::Sint64 => "J".to_string(),
            Self::Float => "F".to_string(),
            Self::Double => "D".to_string(),
            Self::Duration(_) => "Ljava/time/Duration;".to_string(),
            Self::Enum(handle) => format!("L{}/{};", lib_path, handle.name.to_camel_case()),
        }
    }

    fn as_rust_type(&self, _ffi_name: &str) -> String {
        match self {
            Self::Bool => "bool".to_string(),
            Self::Uint8 => "u8".to_string(),
            Self::Sint8 => "i8".to_string(),
            Self::Uint16 => "u16".to_string(),
            Self::Sint16 => "i16".to_string(),
            Self::Uint32 => "u32".to_string(),
            Self::Sint32 => "i32".to_string(),
            Self::Uint64 => "u64".to_string(),
            Self::Sint64 => "i64".to_string(),
            Self::Float => "f32".to_string(),
            Self::Double => "f64".to_string(),
            Self::Duration(_) => "u64".to_string(),
            Self::Enum(_) => "std::os::raw::c_int".to_string(),
        }
    }

    fn convert_jvalue(&self) -> String {
        match self {
            Self::Bool => "z().unwrap() as u8".to_string(),
            Self::Uint8 => "l().unwrap().into_inner()".to_string(),
            Self::Sint8 => "b().unwrap()".to_string(),
            Self::Uint16 => "l().unwrap().into_inner()".to_string(),
            Self::Sint16 => "s().unwrap()".to_string(),
            Self::Uint32 => "l().unwrap().into_inner()".to_string(),
            Self::Sint32 => "i().unwrap()".to_string(),
            Self::Uint64 => "l().unwrap().into_inner()".to_string(),
            Self::Sint64 => "j().unwrap()".to_string(),
            Self::Float => "f().unwrap()".to_string(),
            Self::Double => "d().unwrap()".to_string(),
            Self::Duration(_) => "l().unwrap().into_inner()".to_string(),
            Self::Enum(_) => "l().unwrap().into_inner()".to_string(),
        }
    }

    fn convert_to_rust_from_object(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
        _lib_name: &str,
        _prefix: &str,
    ) -> FormattingResult<()> {
        match self {
            Self::Bool => f.writeln(&format!(
                "{}_cache.primitives.boolean_value(&_env, {})",
                to, from
            )),
            Self::Uint8 => UnsignedConverter("ubyte".to_string()).convert_to_rust(f, from, to),
            Self::Sint8 => f.writeln(&format!(
                "{}_cache.primitives.byte_value(&_env, {})",
                to, from
            )),
            Self::Uint16 => UnsignedConverter("ushort".to_string()).convert_to_rust(f, from, to),
            Self::Sint16 => f.writeln(&format!(
                "{}_cache.primitives.short_value(&_env, {})",
                to, from
            )),
            Self::Uint32 => UnsignedConverter("uinteger".to_string()).convert_to_rust(f, from, to),
            Self::Sint32 => f.writeln(&format!(
                "{}_cache.primitives.integer_value(&_env, {})",
                to, from
            )),
            Self::Uint64 => UnsignedConverter("ulong".to_string()).convert_to_rust(f, from, to),
            Self::Sint64 => f.writeln(&format!(
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
            Self::Duration(mapping) => DurationConverter(*mapping).convert_to_rust(f, from, to),
            Self::Enum(handle) => EnumConverter(handle.clone()).convert_to_rust(f, from, to),
        }
    }

    fn conversion(&self, _lib_name: &str, _prefix: &str) -> Option<Box<dyn TypeConverter>> {
        match self {
            Self::Bool => Some(Box::new(BooleanConverter)),
            Self::Uint8 => Some(Box::new(UnsignedConverter("ubyte".to_string()))),
            Self::Sint8 => None,
            Self::Uint16 => Some(Box::new(UnsignedConverter("ushort".to_string()))),
            Self::Sint16 => None,
            Self::Uint32 => Some(Box::new(UnsignedConverter("uinteger".to_string()))),
            Self::Sint32 => None,
            Self::Uint64 => Some(Box::new(UnsignedConverter("ulong".to_string()))),
            Self::Sint64 => None,
            Self::Float => None,
            Self::Double => None,
            Self::Duration(mapping) => Some(Box::new(DurationConverter(*mapping))),
            Self::Enum(handle) => Some(Box::new(EnumConverter(handle.clone()))),
        }
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        // unsigned integers require cleanup since they're wrapped
        match self {
            Self::Bool => false,
            Self::Uint8 => true,
            Self::Sint8 => false,
            Self::Uint16 => true,
            Self::Sint16 => false,
            Self::Uint32 => true,
            Self::Sint32 => false,
            Self::Uint64 => true,
            Self::Sint64 => false,
            Self::Float => false,
            Self::Double => false,
            Self::Duration(_) => true,
            Self::Enum(_) => false, // We re-use a global ref here
        }
    }

    fn check_null(&self, f: &mut dyn Printer, param_name: &str) -> FormattingResult<()> {
        fn perform_null_check(f: &mut dyn Printer, param_name: &str) -> FormattingResult<()> {
            f.writeln(&format!("if _env.is_same_object({}, jni::objects::JObject::null()).unwrap() {{ return Err(\"{}\".to_string()); }}", param_name, param_name))
        }

        match self {
            Self::Bool => Ok(()),
            Self::Uint8 => perform_null_check(f, param_name),
            Self::Sint8 => Ok(()),
            Self::Uint16 => perform_null_check(f, param_name),
            Self::Sint16 => Ok(()),
            Self::Uint32 => perform_null_check(f, param_name),
            Self::Sint32 => Ok(()),
            Self::Uint64 => perform_null_check(f, param_name),
            Self::Sint64 => Ok(()),
            Self::Float => Ok(()),
            Self::Double => Ok(()),
            Self::Duration(_) => perform_null_check(f, param_name),
            Self::Enum(_) => perform_null_check(f, param_name),
        }
    }

    fn default_value(&self) -> String {
        const NULL: &str = "jni::objects::JObject::null().into_inner()";

        match self {
            Self::Bool => "0".to_string(),
            Self::Uint8 => NULL.to_string(),
            Self::Sint8 => "0".to_string(),
            Self::Uint16 => NULL.to_string(),
            Self::Sint16 => "0".to_string(),
            Self::Uint32 => NULL.to_string(),
            Self::Sint32 => "0".to_string(),
            Self::Uint64 => NULL.to_string(),
            Self::Sint64 => "0".to_string(),
            Self::Float => "0.0".to_string(),
            Self::Double => "0.0".to_string(),
            Self::Duration(_) => NULL.to_string(),
            Self::Enum(_) => NULL.to_string(),
        }
    }
}

impl JniType for AnyType {
    fn as_raw_jni_type(&self) -> String {
        const JOBJECT: &str = "jni::sys::jobject";

        match self {
            Self::Basic(x) => x.as_raw_jni_type(),
            Self::String => "jni::sys::jstring".to_string(),
            Self::Struct(_) => JOBJECT.to_string(),
            Self::StructRef(_) => JOBJECT.to_string(),
            Self::ClassRef(_) => JOBJECT.to_string(),
            Self::Interface(_) => JOBJECT.to_string(),
            Self::Iterator(_) => JOBJECT.to_string(),
            Self::Collection(_) => JOBJECT.to_string(),
        }
    }

    fn as_jni_sig(&self, lib_path: &str) -> String {
        match self {
            Self::Basic(x) => x.as_jni_sig(lib_path),
            Self::String => "Ljava/lang/String;".to_string(),
            Self::Struct(handle) => format!("L{}/{};", lib_path, handle.name().to_camel_case()),
            Self::StructRef(handle) => format!("L{}/{};", lib_path, handle.name.to_camel_case()),
            Self::ClassRef(handle) => format!("L{}/{};", lib_path, handle.name.to_camel_case()),
            Self::Interface(handle) => format!("L{}/{};", lib_path, handle.name.to_camel_case()),
            Self::Iterator(_) => "Ljava/util/List;".to_string(),
            Self::Collection(_) => "Ljava/util/List;".to_string(),
        }
    }

    fn as_rust_type(&self, ffi_name: &str) -> String {
        match self {
            Self::Basic(x) => x.as_rust_type(ffi_name),
            Self::String => "*const std::os::raw::c_char".to_string(),
            Self::Struct(handle) => format!("{}::ffi::{}", ffi_name, handle.name().to_camel_case()),
            Self::StructRef(handle) => {
                format!("*const {}::ffi::{}", ffi_name, handle.name.to_camel_case())
            }
            Self::ClassRef(handle) => format!("*mut {}::{}", ffi_name, handle.name.to_camel_case()),
            Self::Interface(handle) => {
                format!("{}::ffi::{}", ffi_name, handle.name.to_camel_case())
            }
            Self::Iterator(handle) => {
                format!("*mut {}::{}", ffi_name, handle.name().to_camel_case())
            }
            Self::Collection(handle) => {
                format!("*mut {}::{}", ffi_name, handle.name().to_camel_case())
            }
        }
    }

    fn convert_jvalue(&self) -> String {
        match self {
            Self::Basic(x) => x.convert_jvalue(),
            Self::String => "l().unwrap().into_inner()".to_string(),
            Self::Struct(_) => "l().unwrap().into_inner()".to_string(),
            Self::StructRef(_) => "l().unwrap().into_inner()".to_string(),
            Self::ClassRef(_) => "l().unwrap().into_inner()".to_string(),
            Self::Interface(_) => "l().unwrap().into_inner()".to_string(),
            Self::Iterator(_) => "l().unwrap().into_inner()".to_string(),
            Self::Collection(_) => "l().unwrap().into_inner()".to_string(),
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
            Self::String => StringConverter.convert_to_rust(f, from, to),
            Self::Struct(handle) => StructConverter(handle.clone()).convert_to_rust(f, from, to),
            Self::StructRef(handle) => {
                StructRefConverter(handle.clone()).convert_to_rust(f, from, to)
            }
            Self::ClassRef(handle) => ClassConverter(handle.clone()).convert_to_rust(f, from, to),
            Self::Interface(handle) => {
                InterfaceConverter(handle.clone()).convert_to_rust(f, from, to)
            }
            Self::Iterator(handle) => {
                IteratorConverter(handle.clone(), lib_name.to_string(), prefix.to_string())
                    .convert_to_rust(f, from, to)
            }
            Self::Collection(handle) => {
                CollectionConverter(handle.clone(), lib_name.to_string(), prefix.to_string())
                    .convert_to_rust(f, from, to)
            }
        }
    }

    fn conversion(&self, lib_name: &str, prefix: &str) -> Option<Box<dyn TypeConverter>> {
        match self {
            Self::Basic(x) => x.conversion(lib_name, prefix),
            Self::String => Some(Box::new(StringConverter)),
            Self::Struct(handle) => Some(Box::new(StructConverter(handle.clone()))),
            Self::StructRef(handle) => Some(Box::new(StructRefConverter(handle.clone()))),
            Self::ClassRef(handle) => Some(Box::new(ClassConverter(handle.clone()))),
            Self::Interface(handle) => Some(Box::new(InterfaceConverter(handle.clone()))),
            Self::Iterator(handle) => Some(Box::new(IteratorConverter(
                handle.clone(),
                lib_name.to_string(),
                prefix.to_string(),
            ))),
            Self::Collection(handle) => Some(Box::new(CollectionConverter(
                handle.clone(),
                lib_name.to_string(),
                prefix.to_string(),
            ))),
        }
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        match self {
            Self::Basic(x) => x.requires_local_ref_cleanup(),
            Self::String => true,
            Self::Struct(_) => true,
            Self::StructRef(_) => true,
            Self::ClassRef(_) => true,
            Self::Interface(_) => false, // This is freed by Rust
            Self::Iterator(_) => true,
            Self::Collection(_) => true,
        }
    }

    fn check_null(&self, f: &mut dyn Printer, param_name: &str) -> FormattingResult<()> {
        match self {
            Self::Basic(x) => x.check_null(f, param_name),
            Self::String => f.writeln(&format!("if _env.is_same_object({}, jni::objects::JObject::null()).unwrap() {{ return Err(\"{}\".to_string()); }}", param_name, param_name)),
            Self::Struct(handle) => {
                f.writeln(&format!("if _env.is_same_object({}, jni::objects::JObject::null()).unwrap() {{ return Err(\"{}\".to_string()); }}", param_name, param_name))?;
                f.writeln(&format!("_cache.structs.struct_{}.check_null(_cache, &_env, {}).map_err(|_| \"{}\".to_string())?;", handle.name().to_snake_case(), param_name, param_name))
            },
            Self::StructRef(handle) => {
                f.writeln(&format!("if !_env.is_same_object({}, jni::objects::JObject::null()).unwrap()", param_name))?;
                blocked(f, |f| {
                    f.writeln(&format!("_cache.structs.struct_{}.check_null(_cache, &_env, {}).map_err(|_| \"{}\".to_string())?;", handle.name.to_snake_case(), param_name, param_name))
                })
            },
            Self::ClassRef(_) => f.writeln(&format!("if _env.is_same_object({}, jni::objects::JObject::null()).unwrap() {{ return Err(\"{}\".to_string()); }}", param_name, param_name)),
            Self::Interface(_) => f.writeln(&format!("if _env.is_same_object({}, jni::objects::JObject::null()).unwrap() {{ return Err(\"{}\".to_string()); }}", param_name, param_name)),
            Self::Iterator(_) => f.writeln(&format!("if _env.is_same_object({}, jni::objects::JObject::null()).unwrap() {{ return Err(\"{}\".to_string()); }}", param_name, param_name)),
            Self::Collection(_) => f.writeln(&format!("if _env.is_same_object({}, jni::objects::JObject::null()).unwrap() {{ return Err(\"{}\".to_string()); }}", param_name, param_name)),
        }
    }

    fn default_value(&self) -> String {
        const NULL: &str = "jni::objects::JObject::null().into_inner()";

        match self {
            Self::Basic(x) => x.default_value(),
            Self::String => NULL.to_string(),
            Self::Struct(_) => NULL.to_string(),
            Self::StructRef(_) => NULL.to_string(),
            Self::ClassRef(_) => NULL.to_string(),
            Self::Interface(_) => NULL.to_string(),
            Self::Iterator(_) => NULL.to_string(),
            Self::Collection(_) => NULL.to_string(),
        }
    }
}

impl JniType for CArgument {
    fn as_raw_jni_type(&self) -> String {
        AnyType::from(self.clone()).as_raw_jni_type()
    }

    fn as_jni_sig(&self, lib_path: &str) -> String {
        AnyType::from(self.clone()).as_jni_sig(lib_path)
    }

    fn as_rust_type(&self, ffi_name: &str) -> String {
        AnyType::from(self.clone()).as_rust_type(ffi_name)
    }

    fn convert_jvalue(&self) -> String {
        AnyType::from(self.clone()).convert_jvalue()
    }

    fn convert_to_rust_from_object(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
        lib_name: &str,
        prefix: &str,
    ) -> FormattingResult<()> {
        AnyType::from(self.clone()).convert_to_rust_from_object(f, from, to, lib_name, prefix)
    }

    fn conversion(&self, lib_name: &str, prefix: &str) -> Option<Box<dyn TypeConverter>> {
        AnyType::from(self.clone()).conversion(lib_name, prefix)
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        AnyType::from(self.clone()).requires_local_ref_cleanup()
    }

    fn check_null(&self, f: &mut dyn Printer, param_name: &str) -> FormattingResult<()> {
        AnyType::from(self.clone()).check_null(f, param_name)
    }

    fn default_value(&self) -> String {
        AnyType::from(self.clone()).default_value()
    }
}

impl JniType for FReturnValue {
    fn as_raw_jni_type(&self) -> String {
        AnyType::from(self.clone()).as_raw_jni_type()
    }

    fn as_jni_sig(&self, lib_path: &str) -> String {
        AnyType::from(self.clone()).as_jni_sig(lib_path)
    }

    fn as_rust_type(&self, ffi_name: &str) -> String {
        AnyType::from(self.clone()).as_rust_type(ffi_name)
    }

    fn convert_jvalue(&self) -> String {
        AnyType::from(self.clone()).convert_jvalue()
    }

    fn convert_to_rust_from_object(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
        lib_name: &str,
        prefix: &str,
    ) -> FormattingResult<()> {
        AnyType::from(self.clone()).convert_to_rust_from_object(f, from, to, lib_name, prefix)
    }

    fn conversion(&self, lib_name: &str, prefix: &str) -> Option<Box<dyn TypeConverter>> {
        AnyType::from(self.clone()).conversion(lib_name, prefix)
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        AnyType::from(self.clone()).requires_local_ref_cleanup()
    }

    fn check_null(&self, f: &mut dyn Printer, param_name: &str) -> FormattingResult<()> {
        AnyType::from(self.clone()).check_null(f, param_name)
    }

    fn default_value(&self) -> String {
        AnyType::from(self.clone()).default_value()
    }
}

impl JniType for FReturnType {
    fn as_raw_jni_type(&self) -> String {
        match self {
            Self::Void => "()".to_string(),
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

    fn convert_jvalue(&self) -> String {
        match self {
            Self::Void => "v().unwrap()".to_string(),
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

    fn default_value(&self) -> String {
        match self {
            Self::Void => "".to_string(),
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

    fn convert_to_rust_cleanup(&self, f: &mut dyn Printer, name: &str) -> FormattingResult<()> {
        f.writeln(&format!(
            "unsafe {{ std::ffi::CString::from_raw({} as *mut _) }};",
            name
        ))
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
}

struct StructConverter(AnyStructHandle);
impl TypeConverter for StructConverter {
    fn convert_to_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!(
            "{}_cache.structs.struct_{}.struct_to_rust(_cache, &_env, {})",
            to,
            self.0.name().to_snake_case(),
            from
        ))
    }

    fn convert_to_rust_cleanup(&self, f: &mut dyn Printer, name: &str) -> FormattingResult<()> {
        f.writeln(&format!(
            "_cache.structs.struct_{}.struct_to_rust_cleanup(_cache, &_env, &{});",
            self.0.name().to_snake_case(),
            name
        ))
    }

    fn convert_from_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!(
            "{}_cache.structs.struct_{}.struct_from_rust(_cache, &_env, &{})",
            to,
            self.0.name().to_snake_case(),
            from
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
                        StructConverter(self.0.item_type.to_any_struct()).convert_from_rust(
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
