use heck::{CamelCase, MixedCase};
use oo_bindgen::callback::*;
use oo_bindgen::class::*;
use oo_bindgen::collection::*;
use oo_bindgen::formatting::*;
use oo_bindgen::iterator::*;
use oo_bindgen::native_function::*;
use oo_bindgen::native_struct::*;

pub(crate) trait JniType {
    fn as_raw_jni_type(&self) -> &str;
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
            Type::String => None,
            Type::Struct(_) => None,
            Type::StructRef(_) => None,
            Type::Enum(_) => None,
            Type::ClassRef(_) => None,
            Type::Interface(_) => None,
            Type::OneTimeCallback(_) => None,
            Type::Iterator(_) => None,
            Type::Collection(_) => None,
            Type::Duration(_) => None,
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

    fn conversion(&self) -> Option<Box<dyn TypeConverter>> {
        match self {
            ReturnType::Void => None,
            ReturnType::Type(return_type, _) => return_type.conversion(),
        }
    }
}

pub(crate) trait TypeConverter {
    fn convert_to_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()>;
    fn convert_from_rust(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()>;

    fn convert_to_rust_cleanup(&self, _f: &mut dyn Printer, _name: &str) -> FormattingResult<()> {
        Ok(())
    }
}

struct BooleanConverter;
impl TypeConverter for BooleanConverter {
    fn convert_to_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{}{} != 0", to, from))
    }

    fn convert_from_rust(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        f.writeln(&format!("{}if {} {{ 0 }} else {{ 0xFF }}", to, from))
    }
}

struct UnsignedConverter(String);
impl TypeConverter for UnsignedConverter {
    fn convert_to_rust(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{}_cache.joou.{}_to_rust(&_env, {})", to, self.0, from))
    }

    fn convert_from_rust(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        f.writeln(&format!("{}_cache.joou.{}_from_rust(&_env, {})", to, self.0, from))
    }
}

/*
pub(crate) fn call_native_function(
    f: &mut dyn Printer,
    method: &NativeFunction,
    return_destination: &str,
    first_param_is_self: Option<String>,
    is_constructor: bool,
) -> FormattingResult<()> {
    // Write the type conversions
    for (idx, param) in method.parameters.iter().enumerate() {
        if let Some(converter) = param.param_type.conversion() {
            let mut param_name = param.name.to_mixed_case();
            if idx == 0 {
                if let Some(first_param) = first_param_is_self.clone() {
                    param_name = first_param;
                }
            }
            converter.convert_to_native(
                f,
                &param_name,
                &format!("var _{} = ", param.name.to_mixed_case()),
            )?;
        }
    }

    // Call the native function
    f.newline()?;
    if !method.return_type.is_void() {
        f.write(&format!(
            "var _result = {}.{}(",
            NATIVE_FUNCTIONS_CLASSNAME, method.name
        ))?;
    } else {
        f.write(&format!("{}.{}(", NATIVE_FUNCTIONS_CLASSNAME, method.name))?;
    }

    f.write(
        &method
            .parameters
            .iter()
            .map(|param| param.param_type.as_dotnet_arg(&param.name.to_mixed_case()))
            .collect::<Vec<String>>()
            .join(", "),
    )?;
    f.write(");")?;

    // Convert the result (if required)
    let return_name = if let ReturnType::Type(return_type, _) = &method.return_type {
        let mut return_name = "_result";
        if let Some(converter) = return_type.conversion() {
            if !is_constructor {
                converter.convert_from_native(f, "_result", "var __result = ")?;
                return_name = "__result";
            }
        }

        return_name
    } else {
        ""
    };

    //Cleanup type conversions
    for param in method.parameters.iter() {
        if let Some(converter) = param.param_type.conversion() {
            converter.convert_to_native_cleanup(f, &format!("_{}", param.name.to_mixed_case()))?;
        }
    }

    // Return (if required)
    if !method.return_type.is_void() {
        f.writeln(&format!("{}{};", return_destination, return_name))?;
    }

    Ok(())
}

pub(crate) fn call_dotnet_function(
    f: &mut dyn Printer,
    method: &CallbackFunction,
    return_destination: &str,
) -> FormattingResult<()> {
    // Write the type conversions
    for param in method.params() {
        if let Some(converter) = param.param_type.conversion() {
            converter.convert_from_native(
                f,
                &param.name,
                &format!("var _{} = ", param.name.to_mixed_case()),
            )?;
        }
    }

    // Call the .NET function
    f.newline()?;
    let method_name = method.name.to_camel_case();
    if let ReturnType::Type(return_type, _) = &method.return_type {
        if return_type.conversion().is_some() {
            f.write(&format!("var _result = _impl.{}(", method_name))?;
        } else {
            f.write(&format!("{}_impl.{}(", return_destination, method_name))?;
        }
    } else {
        f.write(&format!("_impl.{}(", method_name))?;
    }

    f.write(
        &method
            .params()
            .map(|param| param.param_type.as_dotnet_arg(&param.name))
            .collect::<Vec<String>>()
            .join(", "),
    )?;
    f.write(");")?;

    // Convert the result (if required)
    if let ReturnType::Type(return_type, _) = &method.return_type {
        if let Some(converter) = return_type.conversion() {
            converter.convert_to_native(f, "_result", return_destination)?;
        }
    }

    Ok(())
}
*/