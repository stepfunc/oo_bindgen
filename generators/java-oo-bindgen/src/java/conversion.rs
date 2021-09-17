use super::NATIVE_FUNCTIONS_CLASSNAME;
use heck::{CamelCase, MixedCase};
use oo_bindgen::formatting::*;
use oo_bindgen::native_function::*;

pub(crate) trait JavaType {
    fn as_java_primitive(&self) -> String;
    fn as_java_object(&self) -> String;
}

impl JavaType for BasicType {
    fn as_java_primitive(&self) -> String {
        match self {
            BasicType::Bool => "boolean".to_string(),
            BasicType::Uint8 => "UByte".to_string(),
            BasicType::Sint8 => "byte".to_string(),
            BasicType::Uint16 => "UShort".to_string(),
            BasicType::Sint16 => "short".to_string(),
            BasicType::Uint32 => "UInteger".to_string(),
            BasicType::Sint32 => "int".to_string(),
            BasicType::Uint64 => "ULong".to_string(),
            BasicType::Sint64 => "long".to_string(),
            BasicType::Float => "float".to_string(),
            BasicType::Double => "double".to_string(),
        }
    }

    fn as_java_object(&self) -> String {
        match self {
            Self::Bool => "Boolean".to_string(),
            Self::Uint8 => "UByte".to_string(),
            Self::Sint8 => "Byte".to_string(),
            Self::Uint16 => "UShort".to_string(),
            Self::Sint16 => "Short".to_string(),
            Self::Uint32 => "UInteger".to_string(),
            Self::Sint32 => "Integer".to_string(),
            Self::Uint64 => "ULong".to_string(),
            Self::Sint64 => "Long".to_string(),
            Self::Float => "Float".to_string(),
            Self::Double => "Double".to_string(),
        }
    }
}

impl JavaType for Type {
    /// Return the Java primitive type
    fn as_java_primitive(&self) -> String {
        match self {
            Type::Basic(x) => x.as_java_primitive(),
            Type::String => "String".to_string(),
            Type::Struct(handle) => handle.name().to_camel_case(),
            Type::StructRef(handle) => handle.name.to_camel_case(),
            Type::Enum(handle) => handle.name.to_camel_case(),
            Type::ClassRef(handle) => handle.name.to_camel_case(),
            Type::Interface(handle) => handle.name.to_camel_case(),
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
            Type::Basic(x) => x.as_java_object(),
            Type::String => "String".to_string(),
            Type::Struct(handle) => handle.name().to_camel_case(),
            Type::StructRef(handle) => handle.name.to_camel_case(),
            Type::Enum(handle) => handle.name.to_camel_case(),
            Type::ClassRef(handle) => handle.name.to_camel_case(),
            Type::Interface(handle) => handle.name.to_camel_case(),
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
