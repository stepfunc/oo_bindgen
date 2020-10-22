use super::formatting::*;
use heck::{CamelCase, SnakeCase};
use oo_bindgen::class::*;
use oo_bindgen::formatting::*;
use oo_bindgen::native_enum::*;
use oo_bindgen::native_function::*;
use oo_bindgen::native_struct::*;

pub(crate) trait JniType {
    fn as_raw_jni_type(&self) -> &str;
    fn as_jni_sig(&self, lib_path: &str) -> String;
    fn convert_jvalue(&self) -> &str;
    fn conversion(&self) -> Option<Box<dyn TypeConverter>>;
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
            Type::Interface(_) => todo!(),
            Type::OneTimeCallback(_) => todo!(),
            Type::Iterator(_) => todo!(),
            Type::Collection(_) => todo!(),
            Type::Duration(_) => "Ljava/time/Duration;".to_string(),
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

    fn conversion(&self) -> Option<Box<dyn TypeConverter>> {
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
            Type::Interface(_) => None,
            Type::OneTimeCallback(_) => None,
            Type::Iterator(_) => None,
            Type::Collection(_) => None,
            Type::Duration(mapping) => Some(Box::new(DurationConverter(*mapping))),
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

    fn convert_jvalue(&self) -> &str {
        match self {
            ReturnType::Void => "v().unwrap()",
            ReturnType::Type(return_type, _) => return_type.convert_jvalue(),
        }
    }

    fn conversion(&self) -> Option<Box<dyn TypeConverter>> {
        match self {
            ReturnType::Void => None,
            ReturnType::Type(return_type, _) => return_type.conversion(),
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
        f.writeln(&format!("{}_cache.structs.struct_{}.struct_to_rust(_cache, &_env, {})", to, self.0.name().to_snake_case(), from))
    }

    fn convert_to_rust_cleanup(&self, f: &mut dyn Printer, name: &str) -> FormattingResult<()> {
        f.writeln(&format!("_cache.structs.struct_{}.struct_to_rust_cleanup(_cache, &_env, &{});", self.0.name().to_snake_case(), name))
    }

    fn convert_from_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{}_cache.structs.struct_{}.struct_from_rust(_cache, &_env, &{})", to, self.0.name().to_snake_case(), from))
    }
}

struct StructRefConverter(NativeStructDeclarationHandle);
impl TypeConverter for StructRefConverter {
    fn convert_to_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{} if !_env.is_same_object({}, jni::objects::JObject::null()).unwrap()", to, from))?;
        blocked(f, |f| {
            f.writeln(&format!("let temp = Box::new(_cache.structs.struct_{}.struct_to_rust(_cache, &_env, {}));", self.0.name.to_snake_case(), from))?;
            f.writeln("Box::into_raw(temp)")
        })?;
        f.writeln("else")?;
        blocked(f, |f| {
            f.writeln("std::ptr::null()")
        })
    }

    fn convert_to_rust_cleanup(&self, f: &mut dyn Printer, name: &str) -> FormattingResult<()> {
        f.writeln(&format!("if {}.is_null()", name))?;
        blocked(f, |f| {
            f.writeln(&format!("let temp = unsafe {{ Box::from_raw({} as *mut _) }};", name))?;
            f.writeln(&format!("_cache.structs.struct_{}.struct_to_rust_cleanup(_cache, &_env, &temp)", self.0.name.to_snake_case()))
        })
    }

    fn convert_from_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{}_cache.structs.struct_{}.struct_from_rust(_cache, &_env, &{})", to, self.0.name.to_snake_case(), from))
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
