use super::NATIVE_FUNCTIONS_CLASSNAME;
use heck::{CamelCase, MixedCase};
use oo_bindgen::formatting::*;
use oo_bindgen::native_function::*;

pub(crate) trait JavaType {
    fn as_java_primitive(&self) -> String;
    fn as_java_object(&self) -> String;
}

impl JavaType for Type {
    /// Return the Java primitive type
    fn as_java_primitive(&self) -> String {
        match self {
            Type::Bool => "boolean".to_string(),
            Type::Uint8 => "UByte".to_string(),
            Type::Sint8 => "byte".to_string(),
            Type::Uint16 => "UShort".to_string(),
            Type::Sint16 => "short".to_string(),
            Type::Uint32 => "UInteger".to_string(),
            Type::Sint32 => "int".to_string(),
            Type::Uint64 => "ULong".to_string(),
            Type::Sint64 => "long".to_string(),
            Type::Float => "float".to_string(),
            Type::Double => "double".to_string(),
            Type::String => "String".to_string(),
            Type::Struct(handle) => handle.name().to_camel_case(),
            Type::StructRef(handle) => handle.name.to_camel_case(),
            Type::Enum(handle) => handle.name.to_camel_case(),
            Type::ClassRef(handle) => handle.name.to_camel_case(),
            Type::Interface(handle) => handle.name.to_camel_case(),
            Type::OneTimeCallback(handle) => handle.name.to_camel_case(),
            Type::Iterator(handle) => format!(
                "java.util.List<{}>",
                handle.item_type.name().to_camel_case()
            ),
            Type::Collection(handle) => {
                format!("java.util.List<{}>", handle.item_type.as_java_object())
            }
            Type::Duration(_) => "java.time.Duration".to_string(),
        }
    }

    /// Returns the Java object type (for type parameter)
    fn as_java_object(&self) -> String {
        match self {
            Type::Bool => "Boolean".to_string(),
            Type::Uint8 => "UByte".to_string(),
            Type::Sint8 => "Byte".to_string(),
            Type::Uint16 => "UShort".to_string(),
            Type::Sint16 => "Short".to_string(),
            Type::Uint32 => "UInteger".to_string(),
            Type::Sint32 => "Integer".to_string(),
            Type::Uint64 => "ULong".to_string(),
            Type::Sint64 => "Long".to_string(),
            Type::Float => "Float".to_string(),
            Type::Double => "Double".to_string(),
            Type::String => "String".to_string(),
            Type::Struct(handle) => handle.name().to_camel_case(),
            Type::StructRef(handle) => handle.name.to_camel_case(),
            Type::Enum(handle) => handle.name.to_camel_case(),
            Type::ClassRef(handle) => handle.name.to_camel_case(),
            Type::Interface(handle) => handle.name.to_camel_case(),
            Type::OneTimeCallback(handle) => handle.name.to_camel_case(),
            Type::Iterator(handle) => format!(
                "java.util.List<{}>",
                handle.item_type.name().to_camel_case()
            ),
            Type::Collection(handle) => {
                format!("java.util.List<{}>", handle.item_type.as_java_object())
            }
            Type::Duration(_) => "java.time.Duration".to_string(),
        }
    }
}

impl JavaType for ReturnType {
    fn as_java_primitive(&self) -> String {
        match self {
            ReturnType::Void => "void".to_string(),
            ReturnType::Type(return_type, _) => return_type.as_java_primitive(),
        }
    }

    fn as_java_object(&self) -> String {
        match self {
            ReturnType::Void => "Void".to_string(),
            ReturnType::Type(return_type, _) => return_type.as_java_object(),
        }
    }
}

pub(crate) fn call_native_function(
    f: &mut dyn Printer,
    method: &NativeFunction,
    return_destination: &str,
    first_param_is_this: bool,
) -> FormattingResult<()> {
    let params = method.parameters.iter().enumerate().map(|(idx, param)| {
        if first_param_is_this && idx == 0 {
            let mut new_param = param.clone();
            new_param.name = "this".to_string();
            new_param
        } else {
            param.clone()
        }
    });

    f.newline()?;
    if !method.return_type.is_void() {
        f.write(return_destination)?;
    }

    f.write(&format!("{}.{}(", NATIVE_FUNCTIONS_CLASSNAME, method.name))?;

    f.write(
        &params
            .map(|param| param.name.to_mixed_case())
            .collect::<Vec<String>>()
            .join(", "),
    )?;
    f.write(");")?;

    Ok(())
}
