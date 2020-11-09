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
    ) -> FormattingResult<()>;
    /// Returns converter is required by the type
    fn conversion(&self, lib_name: &str) -> Option<Box<dyn TypeConverter>>;
    /// Indicates whether a local reference cleanup is required once we are done with the type
    fn requires_local_ref_cleanup(&self) -> bool;
    /// Check the parameter for null value. Must return an `Err(String)` if it's the case.
    fn check_null(&self, f: &mut dyn Printer, param_name: &str) -> FormattingResult<()>;
    /// Returns the default raw JNI type value (used when throwing exceptions). Almost always `JObject::null` except for native types.
    fn default_value(&self) -> &str;
}

impl JniType for Type {
    fn as_raw_jni_type(&self) -> &str {
        match self {
            Type::Bool => "jni::sys::jboolean",
            Type::Uint8 => "jni::sys::jobject",
            Type::Sint8 => "jni::sys::jbyte",
            Type::Uint16 => "jni::sys::jobject",
            Type::Sint16 => "jni::sys::jshort",
            Type::Uint32 => "jni::sys::jobject",
            Type::Sint32 => "jni::sys::jint",
            Type::Uint64 => "jni::sys::jobject",
            Type::Sint64 => "jni::sys::jlong",
            Type::Float => "jni::sys::jfloat",
            Type::Double => "jni::sys::jdouble",
            Type::String => "jni::sys::jstring",
            Type::Struct(_) => "jni::sys::jobject",
            Type::StructRef(_) => "jni::sys::jobject",
            Type::Enum(_) => "jni::sys::jobject",
            Type::ClassRef(_) => "jni::sys::jobject",
            Type::Interface(_) => "jni::sys::jobject",
            Type::OneTimeCallback(_) => "jni::sys::jobject",
            Type::Iterator(_) => "jni::sys::jobject",
            Type::Collection(_) => "jni::sys::jobject",
            Type::Duration(_) => "jni::sys::jobject",
        }
    }

    fn as_jni_sig(&self, lib_path: &str) -> String {
        match self {
            Type::Bool => "Z".to_string(),
            Type::Uint8 => "Lorg/joou/UByte;".to_string(),
            Type::Sint8 => "B".to_string(),
            Type::Uint16 => "Lorg/joou/UShort;".to_string(),
            Type::Sint16 => "S".to_string(),
            Type::Uint32 => "Lorg/joou/UInteger;".to_string(),
            Type::Sint32 => "I".to_string(),
            Type::Uint64 => "Lorg/joou/ULong;".to_string(),
            Type::Sint64 => "J".to_string(),
            Type::Float => "F".to_string(),
            Type::Double => "D".to_string(),
            Type::String => "Ljava/lang/String;".to_string(),
            Type::Struct(handle) => format!("L{}/{};", lib_path, handle.name().to_camel_case()),
            Type::StructRef(handle) => format!("L{}/{};", lib_path, handle.name.to_camel_case()),
            Type::Enum(handle) => format!("L{}/{};", lib_path, handle.name.to_camel_case()),
            Type::ClassRef(handle) => format!("L{}/{};", lib_path, handle.name.to_camel_case()),
            Type::Interface(handle) => format!("L{}/{};", lib_path, handle.name.to_camel_case()),
            Type::OneTimeCallback(handle) => {
                format!("L{}/{};", lib_path, handle.name.to_camel_case())
            }
            Type::Iterator(_) => "Ljava/util/List;".to_string(),
            Type::Collection(_) => "Ljava/util/List;".to_string(),
            Type::Duration(_) => "Ljava/time/Duration;".to_string(),
        }
    }

    fn as_rust_type(&self, ffi_name: &str) -> String {
        match self {
            Type::Bool => "bool".to_string(),
            Type::Uint8 => "u8".to_string(),
            Type::Sint8 => "i8".to_string(),
            Type::Uint16 => "u16".to_string(),
            Type::Sint16 => "i16".to_string(),
            Type::Uint32 => "u32".to_string(),
            Type::Sint32 => "i32".to_string(),
            Type::Uint64 => "u64".to_string(),
            Type::Sint64 => "i64".to_string(),
            Type::Float => "f32".to_string(),
            Type::Double => "f64".to_string(),
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
            Type::OneTimeCallback(handle) => {
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
            Type::Bool => "z().unwrap() as u8",
            Type::Uint8 => "l().unwrap().into_inner()",
            Type::Sint8 => "b().unwrap()",
            Type::Uint16 => "l().unwrap().into_inner()",
            Type::Sint16 => "s().unwrap()",
            Type::Uint32 => "l().unwrap().into_inner()",
            Type::Sint32 => "i().unwrap()",
            Type::Uint64 => "l().unwrap().into_inner()",
            Type::Sint64 => "j().unwrap()",
            Type::Float => "f().unwrap()",
            Type::Double => "d().unwrap()",
            Type::String => "l().unwrap().into_inner()",
            Type::Struct(_) => "l().unwrap().into_inner()",
            Type::StructRef(_) => "l().unwrap().into_inner()",
            Type::Enum(_) => "l().unwrap().into_inner()",
            Type::ClassRef(_) => "l().unwrap().into_inner()",
            Type::Interface(_) => "l().unwrap().into_inner()",
            Type::OneTimeCallback(_) => "l().unwrap().into_inner()",
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
    ) -> FormattingResult<()> {
        match self {
            Type::Bool => f.writeln(&format!(
                "{}_cache.primitives.boolean_value(&_env, {})",
                to, from
            )),
            Type::Uint8 => UnsignedConverter("ubyte".to_string()).convert_to_rust(f, from, to),
            Type::Sint8 => f.writeln(&format!(
                "{}_cache.primitives.byte_value(&_env, {})",
                to, from
            )),
            Type::Uint16 => UnsignedConverter("ushort".to_string()).convert_to_rust(f, from, to),
            Type::Sint16 => f.writeln(&format!(
                "{}_cache.primitives.short_value(&_env, {})",
                to, from
            )),
            Type::Uint32 => UnsignedConverter("uinteger".to_string()).convert_to_rust(f, from, to),
            Type::Sint32 => f.writeln(&format!(
                "{}_cache.primitives.integer_value(&_env, {})",
                to, from
            )),
            Type::Uint64 => UnsignedConverter("ulong".to_string()).convert_to_rust(f, from, to),
            Type::Sint64 => f.writeln(&format!(
                "{}_cache.primitives.long_value(&_env, {})",
                to, from
            )),
            Type::Float => f.writeln(&format!(
                "{}_cache.primitives.float_value(&_env, {})",
                to, from
            )),
            Type::Double => f.writeln(&format!(
                "{}_cache.primitives.double_value(&_env, {})",
                to, from
            )),
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
            Type::OneTimeCallback(handle) => {
                OneTimeCallbackConverter(handle.clone()).convert_to_rust(f, from, to)
            }
            Type::Iterator(handle) => {
                IteratorConverter(handle.clone(), lib_name.to_string()).convert_to_rust(f, from, to)
            }
            Type::Collection(handle) => CollectionConverter(handle.clone(), lib_name.to_string())
                .convert_to_rust(f, from, to),
            Type::Duration(mapping) => DurationConverter(*mapping).convert_to_rust(f, from, to),
        }
    }

    fn conversion(&self, lib_name: &str) -> Option<Box<dyn TypeConverter>> {
        match self {
            Type::Bool => Some(Box::new(BooleanConverter)),
            Type::Uint8 => Some(Box::new(UnsignedConverter("ubyte".to_string()))),
            Type::Sint8 => None,
            Type::Uint16 => Some(Box::new(UnsignedConverter("ushort".to_string()))),
            Type::Sint16 => None,
            Type::Uint32 => Some(Box::new(UnsignedConverter("uinteger".to_string()))),
            Type::Sint32 => None,
            Type::Uint64 => Some(Box::new(UnsignedConverter("ulong".to_string()))),
            Type::Sint64 => None,
            Type::Float => None,
            Type::Double => None,
            Type::String => Some(Box::new(StringConverter)),
            Type::Struct(handle) => Some(Box::new(StructConverter(handle.clone()))),
            Type::StructRef(handle) => Some(Box::new(StructRefConverter(handle.clone()))),
            Type::Enum(handle) => Some(Box::new(EnumConverter(handle.clone()))),
            Type::ClassRef(handle) => Some(Box::new(ClassConverter(handle.clone()))),
            Type::Interface(handle) => Some(Box::new(InterfaceConverter(handle.clone()))),
            Type::OneTimeCallback(handle) => {
                Some(Box::new(OneTimeCallbackConverter(handle.clone())))
            }
            Type::Iterator(handle) => Some(Box::new(IteratorConverter(
                handle.clone(),
                lib_name.to_string(),
            ))),
            Type::Collection(handle) => Some(Box::new(CollectionConverter(
                handle.clone(),
                lib_name.to_string(),
            ))),
            Type::Duration(mapping) => Some(Box::new(DurationConverter(*mapping))),
        }
    }

    fn requires_local_ref_cleanup(&self) -> bool {
        match self {
            Type::Bool => false,
            Type::Uint8 => true,
            Type::Sint8 => false,
            Type::Uint16 => true,
            Type::Sint16 => false,
            Type::Uint32 => true,
            Type::Sint32 => false,
            Type::Uint64 => true,
            Type::Sint64 => false,
            Type::Float => false,
            Type::Double => false,
            Type::String => true,
            Type::Struct(_) => true,
            Type::StructRef(_) => true,
            Type::Enum(_) => false, // We re-use a global ref here
            Type::ClassRef(_) => true,
            Type::Interface(_) => false,       // This is freed by Rust
            Type::OneTimeCallback(_) => false, // This is freed by Rust
            Type::Iterator(_) => true,
            Type::Collection(_) => true,
            Type::Duration(_) => true,
        }
    }

    fn check_null(&self, f: &mut dyn Printer, param_name: &str) -> FormattingResult<()> {
        match self {
            Type::Bool => Ok(()),
            Type::Uint8 => f.writeln(&format!("if _env.is_same_object({}, jni::objects::JObject::null()).unwrap() {{ return Err(\"{}\".to_string()); }}", param_name, param_name)),
            Type::Sint8 => Ok(()),
            Type::Uint16 => f.writeln(&format!("if _env.is_same_object({}, jni::objects::JObject::null()).unwrap() {{ return Err(\"{}\".to_string()); }}", param_name, param_name)),
            Type::Sint16 => Ok(()),
            Type::Uint32 => f.writeln(&format!("if _env.is_same_object({}, jni::objects::JObject::null()).unwrap() {{ return Err(\"{}\".to_string()); }}", param_name, param_name)),
            Type::Sint32 => Ok(()),
            Type::Uint64 => f.writeln(&format!("if _env.is_same_object({}, jni::objects::JObject::null()).unwrap() {{ return Err(\"{}\".to_string()); }}", param_name, param_name)),
            Type::Sint64 => Ok(()),
            Type::Float => Ok(()),
            Type::Double => Ok(()),
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
            Type::OneTimeCallback(_) => f.writeln(&format!("if _env.is_same_object({}, jni::objects::JObject::null()).unwrap() {{ return Err(\"{}\".to_string()); }}", param_name, param_name)),
            Type::Iterator(_) => f.writeln(&format!("if _env.is_same_object({}, jni::objects::JObject::null()).unwrap() {{ return Err(\"{}\".to_string()); }}", param_name, param_name)),
            Type::Collection(_) => f.writeln(&format!("if _env.is_same_object({}, jni::objects::JObject::null()).unwrap() {{ return Err(\"{}\".to_string()); }}", param_name, param_name)),
            Type::Duration(_) => f.writeln(&format!("if _env.is_same_object({}, jni::objects::JObject::null()).unwrap() {{ return Err(\"{}\".to_string()); }}", param_name, param_name)),
        }
    }

    fn default_value(&self) -> &str {
        match self {
            Type::Bool => "0",
            Type::Uint8 => "jni::objects::JObject::null().into_inner()",
            Type::Sint8 => "0",
            Type::Uint16 => "jni::objects::JObject::null().into_inner()",
            Type::Sint16 => "0",
            Type::Uint32 => "jni::objects::JObject::null().into_inner()",
            Type::Sint32 => "0",
            Type::Uint64 => "jni::objects::JObject::null().into_inner()",
            Type::Sint64 => "0",
            Type::Float => "0.0",
            Type::Double => "0.0",
            Type::String => "jni::objects::JObject::null().into_inner()",
            Type::Struct(_) => "jni::objects::JObject::null().into_inner()",
            Type::StructRef(_) => "jni::objects::JObject::null().into_inner()",
            Type::Enum(_) => "jni::objects::JObject::null().into_inner()",
            Type::ClassRef(_) => "jni::objects::JObject::null().into_inner()",
            Type::Interface(_) => "jni::objects::JObject::null().into_inner()",
            Type::OneTimeCallback(_) => "jni::objects::JObject::null().into_inner()",
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
    ) -> FormattingResult<()> {
        match self {
            ReturnType::Void => Ok(()),
            ReturnType::Type(return_type, _) => {
                return_type.convert_to_rust_from_object(f, from, to, lib_name)
            }
        }
    }

    fn conversion(&self, lib_name: &str) -> Option<Box<dyn TypeConverter>> {
        match self {
            ReturnType::Void => None,
            ReturnType::Type(return_type, _) => return_type.conversion(lib_name),
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
        f.writeln(&format!("{}if {} {{ 1 }} else {{ 0 }}", to, from))
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

struct EnumConverter(NativeEnumHandle);
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

struct OneTimeCallbackConverter(OneTimeCallbackHandle);
impl TypeConverter for OneTimeCallbackConverter {
    fn convert_to_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!(
            "{}_cache.interfaces.callback_{}.interface_to_rust(&_env, {})",
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

struct IteratorConverter(IteratorHandle, String);
impl TypeConverter for IteratorConverter {
    fn convert_to_rust(&self, f: &mut dyn Printer, _from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{}std::ptr::null_mut()", to))
    }

    fn convert_from_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(to)?;
        blocked(f, |f| {
            f.writeln("let array_list = _cache.collection.new_array_list(&_env);")?;
            f.writeln(&format!(
                "while let it = unsafe {{ {}::ffi::{}({}) }}",
                self.1, self.0.native_func.name, from
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

struct CollectionConverter(CollectionHandle, String);
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
                    "let result = unsafe {{ {}::ffi::{}(_size) }};",
                    self.1, self.0.create_func.name
                ))?;
            } else {
                f.writeln(&format!(
                    "let result = unsafe {{ {}::ffi::{}() }};",
                    self.1, self.0.create_func.name
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
                    )?;
                    f.write(";")?;

                    f.writeln(&format!(
                        "unsafe {{ {}::ffi::{}(result, _next) }};",
                        self.1, self.0.add_func.name
                    ))?;

                    if let Some(converter) = self.0.item_type.conversion(&self.1) {
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
