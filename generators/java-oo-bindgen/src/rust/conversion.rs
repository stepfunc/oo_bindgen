use super::formatting::*;
use heck::{CamelCase, SnakeCase};
use oo_bindgen::callback::*;
use oo_bindgen::class::*;
use oo_bindgen::collection::*;
use oo_bindgen::formatting::*;
use oo_bindgen::iterator::*;
use oo_bindgen::native_enum::*;
use oo_bindgen::native_function::*;
use oo_bindgen::native_struct::*;
use oo_bindgen::types::BasicType;

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

impl JniType for BasicType {
    fn as_raw_jni_type(&self) -> &str {
        match self {
            Self::Bool => "jni::sys::jboolean",
            Self::Uint8 => "jni::sys::jobject",
            Self::Sint8 => "jni::sys::jbyte",
            Self::Uint16 => "jni::sys::jobject",
            Self::Sint16 => "jni::sys::jshort",
            Self::Uint32 => "jni::sys::jobject",
            Self::Sint32 => "jni::sys::jint",
            Self::Uint64 => "jni::sys::jobject",
            Self::Sint64 => "jni::sys::jlong",
            Self::Float => "jni::sys::jfloat",
            Self::Double => "jni::sys::jdouble",
        }
    }

    fn as_jni_sig(&self, _lib_path: &str) -> String {
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
        }
    }

    fn convert_jvalue(&self) -> &str {
        match self {
            Self::Bool => "z().unwrap() as u8",
            Self::Uint8 => "l().unwrap().into_inner()",
            Self::Sint8 => "b().unwrap()",
            Self::Uint16 => "l().unwrap().into_inner()",
            Self::Sint16 => "s().unwrap()",
            Self::Uint32 => "l().unwrap().into_inner()",
            Self::Sint32 => "i().unwrap()",
            Self::Uint64 => "l().unwrap().into_inner()",
            Self::Sint64 => "j().unwrap()",
            Self::Float => "f().unwrap()",
            Self::Double => "d().unwrap()",
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
        }
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        // only unsigned integers require special cleanup since they're wrapped
        self.is_unsigned_integer()
    }

    fn check_null(&self, f: &mut dyn Printer, param_name: &str) -> FormattingResult<()> {
        match self {
            Self::Bool => Ok(()),
            Self::Uint8 => f.writeln(&format!("if _env.is_same_object({}, jni::objects::JObject::null()).unwrap() {{ return Err(\"{}\".to_string()); }}", param_name, param_name)),
            Self::Sint8 => Ok(()),
            Self::Uint16 => f.writeln(&format!("if _env.is_same_object({}, jni::objects::JObject::null()).unwrap() {{ return Err(\"{}\".to_string()); }}", param_name, param_name)),
            Self::Sint16 => Ok(()),
            Self::Uint32 => f.writeln(&format!("if _env.is_same_object({}, jni::objects::JObject::null()).unwrap() {{ return Err(\"{}\".to_string()); }}", param_name, param_name)),
            Self::Sint32 => Ok(()),
            Self::Uint64 => f.writeln(&format!("if _env.is_same_object({}, jni::objects::JObject::null()).unwrap() {{ return Err(\"{}\".to_string()); }}", param_name, param_name)),
            Self::Sint64 => Ok(()),
            Self::Float => Ok(()),
            Self::Double => Ok(()),
        }
    }

    fn default_value(&self) -> &str {
        match self {
            Self::Bool => "0",
            Self::Uint8 => "jni::objects::JObject::null().into_inner()",
            Self::Sint8 => "0",
            Self::Uint16 => "jni::objects::JObject::null().into_inner()",
            Self::Sint16 => "0",
            Self::Uint32 => "jni::objects::JObject::null().into_inner()",
            Self::Sint32 => "0",
            Self::Uint64 => "jni::objects::JObject::null().into_inner()",
            Self::Sint64 => "0",
            Self::Float => "0.0",
            Self::Double => "0.0",
        }
    }
}

impl JniType for Type {
    fn as_raw_jni_type(&self) -> &str {
        match self {
            Type::Basic(x) => x.as_raw_jni_type(),
            Type::String => "jni::sys::jstring",
            Type::Struct(_) => "jni::sys::jobject",
            Type::StructRef(_) => "jni::sys::jobject",
            Type::Enum(_) => "jni::sys::jobject",
            Type::ClassRef(_) => "jni::sys::jobject",
            Type::Interface(_) => "jni::sys::jobject",
            Type::Iterator(_) => "jni::sys::jobject",
            Type::Collection(_) => "jni::sys::jobject",
            Type::Duration(_) => "jni::sys::jobject",
        }
    }

    fn as_jni_sig(&self, lib_path: &str) -> String {
        match self {
            Type::Basic(x) => x.as_jni_sig(lib_path),
            Type::String => "Ljava/lang/String;".to_string(),
            Type::Struct(handle) => format!("L{}/{};", lib_path, handle.name().to_camel_case()),
            Type::StructRef(handle) => format!("L{}/{};", lib_path, handle.name.to_camel_case()),
            Type::Enum(handle) => format!("L{}/{};", lib_path, handle.name.to_camel_case()),
            Type::ClassRef(handle) => format!("L{}/{};", lib_path, handle.name.to_camel_case()),
            Type::Interface(handle) => format!("L{}/{};", lib_path, handle.name.to_camel_case()),
            Type::Iterator(_) => "Ljava/util/List;".to_string(),
            Type::Collection(_) => "Ljava/util/List;".to_string(),
            Type::Duration(_) => "Ljava/time/Duration;".to_string(),
        }
    }

    fn as_rust_type(&self, ffi_name: &str) -> String {
        match self {
            Type::Basic(x) => x.as_rust_type(ffi_name),
            Type::String => "*const std::os::raw::c_char".to_string(),
            Type::Struct(handle) => format!("{}::ffi::{}", ffi_name, handle.name().to_camel_case()),
            Type::StructRef(handle) => {
                format!("*const {}::ffi::{}", ffi_name, handle.name.to_camel_case())
            }
            Type::Enum(_) => "std::os::raw::c_int".to_string(),
            Type::ClassRef(handle) => format!("*mut {}::{}", ffi_name, handle.name.to_camel_case()),
            Type::Interface(handle) => {
                format!("{}::ffi::{}", ffi_name, handle.name.to_camel_case())
            }
            Type::Iterator(handle) => {
                format!("*mut {}::{}", ffi_name, handle.name().to_camel_case())
            }
            Type::Collection(handle) => {
                format!("*mut {}::{}", ffi_name, handle.name().to_camel_case())
            }
            Type::Duration(mapping) => match mapping {
                DurationMapping::Milliseconds | DurationMapping::Seconds => "u64".to_string(),
                DurationMapping::SecondsFloat => "f32".to_string(),
            },
        }
    }

    fn convert_jvalue(&self) -> &str {
        match self {
            Type::Basic(x) => x.convert_jvalue(),
            Type::String => "l().unwrap().into_inner()",
            Type::Struct(_) => "l().unwrap().into_inner()",
            Type::StructRef(_) => "l().unwrap().into_inner()",
            Type::Enum(_) => "l().unwrap().into_inner()",
            Type::ClassRef(_) => "l().unwrap().into_inner()",
            Type::Interface(_) => "l().unwrap().into_inner()",
            Type::Iterator(_) => "l().unwrap().into_inner()",
            Type::Collection(_) => "l().unwrap().into_inner()",
            Type::Duration(_) => "l().unwrap().into_inner()",
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
            Type::Basic(x) => x.convert_to_rust_from_object(f, from, to, lib_name, prefix),
            Type::String => StringConverter.convert_to_rust(f, from, to),
            Type::Struct(handle) => StructConverter(handle.clone()).convert_to_rust(f, from, to),
            Type::StructRef(handle) => {
                StructRefConverter(handle.clone()).convert_to_rust(f, from, to)
            }
            Type::Enum(handle) => EnumConverter(handle.clone()).convert_to_rust(f, from, to),
            Type::ClassRef(handle) => ClassConverter(handle.clone()).convert_to_rust(f, from, to),
            Type::Interface(handle) => {
                InterfaceConverter(handle.clone()).convert_to_rust(f, from, to)
            }
            Type::Iterator(handle) => {
                IteratorConverter(handle.clone(), lib_name.to_string(), prefix.to_string())
                    .convert_to_rust(f, from, to)
            }
            Type::Collection(handle) => {
                CollectionConverter(handle.clone(), lib_name.to_string(), prefix.to_string())
                    .convert_to_rust(f, from, to)
            }
            Type::Duration(mapping) => DurationConverter(*mapping).convert_to_rust(f, from, to),
        }
    }

    fn conversion(&self, lib_name: &str, prefix: &str) -> Option<Box<dyn TypeConverter>> {
        match self {
            Type::Basic(x) => x.conversion(lib_name, prefix),
            Type::String => Some(Box::new(StringConverter)),
            Type::Struct(handle) => Some(Box::new(StructConverter(handle.clone()))),
            Type::StructRef(handle) => Some(Box::new(StructRefConverter(handle.clone()))),
            Type::Enum(handle) => Some(Box::new(EnumConverter(handle.clone()))),
            Type::ClassRef(handle) => Some(Box::new(ClassConverter(handle.clone()))),
            Type::Interface(handle) => Some(Box::new(InterfaceConverter(handle.clone()))),
            Type::Iterator(handle) => Some(Box::new(IteratorConverter(
                handle.clone(),
                lib_name.to_string(),
                prefix.to_string(),
            ))),
            Type::Collection(handle) => Some(Box::new(CollectionConverter(
                handle.clone(),
                lib_name.to_string(),
                prefix.to_string(),
            ))),
            Type::Duration(mapping) => Some(Box::new(DurationConverter(*mapping))),
        }
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        match self {
            Type::Basic(x) => x.requires_local_ref_cleanup(),
            Type::String => true,
            Type::Struct(_) => true,
            Type::StructRef(_) => true,
            Type::Enum(_) => false, // We re-use a global ref here
            Type::ClassRef(_) => true,
            Type::Interface(_) => false, // This is freed by Rust
            Type::Iterator(_) => true,
            Type::Collection(_) => true,
            Type::Duration(_) => true,
        }
    }

    fn check_null(&self, f: &mut dyn Printer, param_name: &str) -> FormattingResult<()> {
        match self {
            Type::Basic(x) => x.check_null(f, param_name),
            Type::String => f.writeln(&format!("if _env.is_same_object({}, jni::objects::JObject::null()).unwrap() {{ return Err(\"{}\".to_string()); }}", param_name, param_name)),
            Type::Struct(handle) => {
                f.writeln(&format!("if _env.is_same_object({}, jni::objects::JObject::null()).unwrap() {{ return Err(\"{}\".to_string()); }}", param_name, param_name))?;
                f.writeln(&format!("_cache.structs.struct_{}.check_null(_cache, &_env, {}).map_err(|_| \"{}\".to_string())?;", handle.name().to_snake_case(), param_name, param_name))
            },
            Type::StructRef(handle) => {
                f.writeln(&format!("if !_env.is_same_object({}, jni::objects::JObject::null()).unwrap()", param_name))?;
                blocked(f, |f| {
                    f.writeln(&format!("_cache.structs.struct_{}.check_null(_cache, &_env, {}).map_err(|_| \"{}\".to_string())?;", handle.name.to_snake_case(), param_name, param_name))
                })
            },
            Type::Enum(_) => f.writeln(&format!("if _env.is_same_object({}, jni::objects::JObject::null()).unwrap() {{ return Err(\"{}\".to_string()); }}", param_name, param_name)),
            Type::ClassRef(_) => f.writeln(&format!("if _env.is_same_object({}, jni::objects::JObject::null()).unwrap() {{ return Err(\"{}\".to_string()); }}", param_name, param_name)),
            Type::Interface(_) => f.writeln(&format!("if _env.is_same_object({}, jni::objects::JObject::null()).unwrap() {{ return Err(\"{}\".to_string()); }}", param_name, param_name)),
            Type::Iterator(_) => f.writeln(&format!("if _env.is_same_object({}, jni::objects::JObject::null()).unwrap() {{ return Err(\"{}\".to_string()); }}", param_name, param_name)),
            Type::Collection(_) => f.writeln(&format!("if _env.is_same_object({}, jni::objects::JObject::null()).unwrap() {{ return Err(\"{}\".to_string()); }}", param_name, param_name)),
            Type::Duration(_) => f.writeln(&format!("if _env.is_same_object({}, jni::objects::JObject::null()).unwrap() {{ return Err(\"{}\".to_string()); }}", param_name, param_name)),
        }
    }

    fn default_value(&self) -> &str {
        match self {
            Type::Basic(x) => x.default_value(),
            Type::String => "jni::objects::JObject::null().into_inner()",
            Type::Struct(_) => "jni::objects::JObject::null().into_inner()",
            Type::StructRef(_) => "jni::objects::JObject::null().into_inner()",
            Type::Enum(_) => "jni::objects::JObject::null().into_inner()",
            Type::ClassRef(_) => "jni::objects::JObject::null().into_inner()",
            Type::Interface(_) => "jni::objects::JObject::null().into_inner()",
            Type::Iterator(_) => "jni::objects::JObject::null().into_inner()",
            Type::Collection(_) => "jni::objects::JObject::null().into_inner()",
            Type::Duration(_) => "jni::objects::JObject::null().into_inner()",
        }
    }
}

impl JniType for ReturnType {
    fn as_raw_jni_type(&self) -> &str {
        match self {
            ReturnType::Void => "()",
            ReturnType::Type(return_type, _) => return_type.as_raw_jni_type(),
        }
    }

    fn as_jni_sig(&self, lib_path: &str) -> String {
        match self {
            ReturnType::Void => "V".to_string(),
            ReturnType::Type(return_type, _) => return_type.as_jni_sig(lib_path),
        }
    }

    fn as_rust_type(&self, ffi_name: &str) -> String {
        match self {
            ReturnType::Void => "()".to_string(),
            ReturnType::Type(return_type, _) => return_type.as_rust_type(ffi_name),
        }
    }

    fn convert_jvalue(&self) -> &str {
        match self {
            ReturnType::Void => "v().unwrap()",
            ReturnType::Type(return_type, _) => return_type.convert_jvalue(),
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
            ReturnType::Void => Ok(()),
            ReturnType::Type(return_type, _) => {
                return_type.convert_to_rust_from_object(f, from, to, lib_name, prefix)
            }
        }
    }

    fn conversion(&self, lib_name: &str, prefix: &str) -> Option<Box<dyn TypeConverter>> {
        match self {
            ReturnType::Void => None,
            ReturnType::Type(return_type, _) => return_type.conversion(lib_name, prefix),
        }
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        match self {
            ReturnType::Void => false,
            ReturnType::Type(return_type, _) => return_type.requires_local_ref_cleanup(),
        }
    }

    fn check_null(&self, f: &mut dyn Printer, param_name: &str) -> FormattingResult<()> {
        match self {
            ReturnType::Void => Ok(()),
            ReturnType::Type(return_type, _) => return_type.check_null(f, param_name),
        }
    }

    fn default_value(&self) -> &str {
        match self {
            ReturnType::Void => "",
            ReturnType::Type(return_type, _) => return_type.default_value(),
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

struct StructConverter(NativeStructHandle);
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

struct StructRefConverter(NativeStructDeclarationHandle);
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

pub(crate) struct EnumConverter(pub(crate) NativeEnumHandle);
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
            self.0.arg_name.to_snake_case()
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
                self.1, self.2, self.0.native_func.name, from
            ))?;
            blocked(f, |f| {
                f.writeln("match unsafe { it.as_ref() }")?;
                blocked(f, |f| {
                    f.writeln("None => { break; }")?;
                    f.writeln("Some(it) => ")?;
                    blocked(f, |f| {
                        StructConverter(self.0.item_type.clone()).convert_from_rust(
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

struct DurationConverter(DurationMapping);
impl TypeConverter for DurationConverter {
    fn convert_to_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        let method = match self.0 {
            DurationMapping::Milliseconds => "duration_to_millis",
            DurationMapping::Seconds => "duration_to_seconds",
            DurationMapping::SecondsFloat => "duration_to_seconds_float",
        };

        f.writeln(&format!(
            "{}_cache.duration.{}(&_env, {})",
            to, method, from
        ))
    }

    fn convert_from_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        let method = match self.0 {
            DurationMapping::Milliseconds => "duration_from_millis",
            DurationMapping::Seconds => "duration_from_seconds",
            DurationMapping::SecondsFloat => "duration_from_seconds_float",
        };

        f.writeln(&format!(
            "{}_cache.duration.{}(&_env, {})",
            to, method, from
        ))
    }
}
