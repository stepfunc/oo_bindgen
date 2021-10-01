use super::NATIVE_FUNCTIONS_CLASSNAME;
use heck::{CamelCase, MixedCase};
use oo_bindgen::formatting::*;
use oo_bindgen::native_function::*;
use oo_bindgen::types::{AnyType, BasicType};

pub(crate) trait JavaType {
    fn as_java_primitive(&self) -> String;
    fn as_java_object(&self) -> String;
}

impl JavaType for BasicType {
    fn as_java_primitive(&self) -> String {
        match self {
            Self::Bool => "boolean".to_string(),
            Self::Uint8 => "UByte".to_string(),
            Self::Sint8 => "byte".to_string(),
            Self::Uint16 => "UShort".to_string(),
            Self::Sint16 => "short".to_string(),
            Self::Uint32 => "UInteger".to_string(),
            Self::Sint32 => "int".to_string(),
            Self::Uint64 => "ULong".to_string(),
            Self::Sint64 => "long".to_string(),
            Self::Float => "float".to_string(),
            Self::Double => "double".to_string(),
            Self::Duration(_) => "java.time.Duration".to_string(),
            Self::Enum(handle) => handle.name.to_camel_case(),
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
            Self::Duration(_) => "java.time.Duration".to_string(),
            Self::Enum(handle) => handle.name.to_camel_case(),
        }
    }
}

impl JavaType for AnyType {
    /// Return the Java primitive type
    fn as_java_primitive(&self) -> String {
        match self {
            Self::Basic(x) => x.as_java_primitive(),
            Self::String => "String".to_string(),
            Self::Struct(handle) => handle.name().to_camel_case(),
            Self::StructRef(handle) => handle.name.to_camel_case(),
            Self::ClassRef(handle) => handle.name.to_camel_case(),
            Self::Interface(handle) => handle.name.to_camel_case(),
            Self::Iterator(handle) => format!(
                "java.util.List<{}>",
                handle.item_type.name().to_camel_case()
            ),
            Self::Collection(handle) => {
                format!("java.util.List<{}>", handle.item_type.as_java_object())
            }
        }
    }

    /// Returns the Java object type (for type parameter)
    fn as_java_object(&self) -> String {
        match self {
            Self::Basic(x) => x.as_java_object(),
            Self::String => "String".to_string(),
            Self::Struct(handle) => handle.name().to_camel_case(),
            Self::StructRef(handle) => handle.name.to_camel_case(),
            Self::ClassRef(handle) => handle.name.to_camel_case(),
            Self::Interface(handle) => handle.name.to_camel_case(),
            Self::Iterator(handle) => format!(
                "java.util.List<{}>",
                handle.item_type.name().to_camel_case()
            ),
            Self::Collection(handle) => {
                format!("java.util.List<{}>", handle.item_type.as_java_object())
            }
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
    method: &Function,
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
