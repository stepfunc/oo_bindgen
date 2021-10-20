use super::NATIVE_FUNCTIONS_CLASSNAME;
use heck::{CamelCase, MixedCase};
use oo_bindgen::class::ClassDeclarationHandle;
use oo_bindgen::collection::CollectionHandle;
use oo_bindgen::formatting::*;
use oo_bindgen::function::*;
use oo_bindgen::interface::{CallbackArgument, CallbackReturnValue, InterfaceHandle};
use oo_bindgen::iterator::IteratorHandle;
use oo_bindgen::return_type::ReturnType;
use oo_bindgen::structs::*;
use oo_bindgen::types::{BasicType, StringType};
use oo_bindgen::{Handle, UniversalOr};

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
            Self::Float32 => "float".to_string(),
            Self::Double64 => "double".to_string(),
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
            Self::Float32 => "Float".to_string(),
            Self::Double64 => "Double".to_string(),
            Self::Duration(_) => "java.time.Duration".to_string(),
            Self::Enum(handle) => handle.name.to_camel_case(),
        }
    }
}

impl JavaType for StringType {
    fn as_java_primitive(&self) -> String {
        "String".to_string()
    }

    fn as_java_object(&self) -> String {
        "String".to_string()
    }
}

impl JavaType for InterfaceHandle {
    fn as_java_primitive(&self) -> String {
        self.as_java_object()
    }

    fn as_java_object(&self) -> String {
        self.name.to_camel_case()
    }
}

impl JavaType for CollectionHandle {
    fn as_java_primitive(&self) -> String {
        self.as_java_object()
    }

    fn as_java_object(&self) -> String {
        format!("java.util.List<{}>", self.item_type.as_java_object())
    }
}

impl JavaType for ClassDeclarationHandle {
    fn as_java_primitive(&self) -> String {
        self.as_java_object()
    }

    fn as_java_object(&self) -> String {
        self.name.to_camel_case()
    }
}

impl JavaType for StructDeclarationHandle {
    fn as_java_primitive(&self) -> String {
        self.as_java_object()
    }

    fn as_java_object(&self) -> String {
        self.name.to_camel_case()
    }
}

impl JavaType for IteratorHandle {
    fn as_java_primitive(&self) -> String {
        self.as_java_object()
    }

    fn as_java_object(&self) -> String {
        format!("java.util.List<{}>", self.item_type.as_java_object())
    }
}

impl<T> JavaType for Handle<Struct<T>>
where
    T: StructFieldType,
{
    fn as_java_primitive(&self) -> String {
        self.as_java_object()
    }

    fn as_java_object(&self) -> String {
        self.name().to_camel_case()
    }
}

impl<T> JavaType for UniversalOr<T>
where
    T: StructFieldType,
{
    fn as_java_primitive(&self) -> String {
        match self {
            UniversalOr::Specific(x) => x.as_java_primitive(),
            UniversalOr::Universal(x) => x.as_java_primitive(),
        }
    }

    fn as_java_object(&self) -> String {
        match self {
            UniversalOr::Specific(x) => x.as_java_object(),
            UniversalOr::Universal(x) => x.as_java_object(),
        }
    }
}

impl JavaType for FunctionArgStructField {
    fn as_java_primitive(&self) -> String {
        match self {
            FunctionArgStructField::Basic(x) => x.as_java_primitive(),
            FunctionArgStructField::String(x) => x.as_java_primitive(),
            FunctionArgStructField::Interface(x) => x.as_java_primitive(),
            FunctionArgStructField::Collection(x) => x.as_java_primitive(),
            FunctionArgStructField::Struct(x) => x.as_java_primitive(),
        }
    }

    fn as_java_object(&self) -> String {
        match self {
            FunctionArgStructField::Basic(x) => x.as_java_object(),
            FunctionArgStructField::String(x) => x.as_java_object(),
            FunctionArgStructField::Interface(x) => x.as_java_object(),
            FunctionArgStructField::Collection(x) => x.as_java_object(),
            FunctionArgStructField::Struct(x) => x.as_java_object(),
        }
    }
}

impl JavaType for FunctionReturnStructField {
    fn as_java_primitive(&self) -> String {
        match self {
            FunctionReturnStructField::Basic(x) => x.as_java_primitive(),
            FunctionReturnStructField::ClassRef(x) => x.as_java_primitive(),
            FunctionReturnStructField::Struct(x) => x.as_java_primitive(),
            FunctionReturnStructField::Iterator(x) => x.as_java_primitive(),
        }
    }

    fn as_java_object(&self) -> String {
        match self {
            FunctionReturnStructField::Basic(x) => x.as_java_object(),
            FunctionReturnStructField::ClassRef(x) => x.as_java_object(),
            FunctionReturnStructField::Struct(x) => x.as_java_object(),
            FunctionReturnStructField::Iterator(x) => x.as_java_object(),
        }
    }
}

impl JavaType for CallbackArgStructField {
    fn as_java_primitive(&self) -> String {
        match self {
            CallbackArgStructField::Basic(x) => x.as_java_primitive(),
            CallbackArgStructField::Iterator(x) => x.as_java_primitive(),
            CallbackArgStructField::Struct(x) => x.as_java_primitive(),
        }
    }

    fn as_java_object(&self) -> String {
        match self {
            CallbackArgStructField::Basic(x) => x.as_java_object(),
            CallbackArgStructField::Iterator(x) => x.as_java_object(),
            CallbackArgStructField::Struct(x) => x.as_java_object(),
        }
    }
}

impl JavaType for UniversalStructField {
    fn as_java_primitive(&self) -> String {
        match self {
            UniversalStructField::Basic(x) => x.as_java_primitive(),
            UniversalStructField::Struct(x) => x.as_java_primitive(),
        }
    }

    fn as_java_object(&self) -> String {
        match self {
            UniversalStructField::Basic(x) => x.as_java_object(),
            UniversalStructField::Struct(x) => x.as_java_object(),
        }
    }
}

impl JavaType for FArgument {
    fn as_java_primitive(&self) -> String {
        match self {
            Self::Basic(x) => x.as_java_primitive(),
            Self::String(x) => x.as_java_primitive(),
            Self::Collection(x) => x.as_java_primitive(),
            Self::Struct(x) => x.as_java_primitive(),
            Self::StructRef(x) => x.as_java_primitive(),
            Self::ClassRef(x) => x.as_java_primitive(),
            Self::Interface(x) => x.as_java_primitive(),
        }
    }

    fn as_java_object(&self) -> String {
        match self {
            Self::Basic(x) => x.as_java_object(),
            Self::String(x) => x.as_java_object(),
            Self::Collection(x) => x.as_java_object(),
            Self::Struct(x) => x.as_java_object(),
            Self::StructRef(x) => x.as_java_object(),
            Self::ClassRef(x) => x.as_java_object(),
            Self::Interface(x) => x.as_java_object(),
        }
    }
}

impl JavaType for CallbackArgument {
    fn as_java_primitive(&self) -> String {
        match self {
            Self::Basic(x) => x.as_java_primitive(),
            Self::String(x) => x.as_java_primitive(),
            Self::Iterator(x) => x.as_java_primitive(),
            Self::Struct(x) => x.as_java_primitive(),
            Self::Class(x) => x.as_java_primitive(),
        }
    }

    fn as_java_object(&self) -> String {
        match self {
            Self::Basic(x) => x.as_java_object(),
            Self::String(x) => x.as_java_object(),
            Self::Iterator(x) => x.as_java_object(),
            Self::Struct(x) => x.as_java_object(),
            Self::Class(x) => x.as_java_object(),
        }
    }
}

impl JavaType for CallbackReturnValue {
    fn as_java_primitive(&self) -> String {
        match self {
            Self::Basic(x) => x.as_java_primitive(),
            Self::Struct(x) => x.as_java_primitive(),
        }
    }

    fn as_java_object(&self) -> String {
        match self {
            Self::Basic(x) => x.as_java_object(),
            Self::Struct(x) => x.as_java_object(),
        }
    }
}

impl JavaType for FunctionReturnValue {
    fn as_java_primitive(&self) -> String {
        match self {
            Self::Basic(x) => x.as_java_primitive(),
            Self::String(x) => x.as_java_primitive(),
            Self::ClassRef(x) => x.as_java_primitive(),
            Self::Struct(x) => x.as_java_primitive(),
            Self::StructRef(x) => x.as_java_primitive(),
        }
    }

    fn as_java_object(&self) -> String {
        match self {
            Self::Basic(x) => x.as_java_object(),
            Self::String(x) => x.as_java_object(),
            Self::ClassRef(x) => x.as_java_object(),
            Self::Struct(x) => x.as_java_object(),
            Self::StructRef(x) => x.as_java_object(),
        }
    }
}

impl<T> JavaType for ReturnType<T>
where
    T: JavaType,
{
    fn as_java_primitive(&self) -> String {
        match self {
            Self::Void => "void".to_string(),
            Self::Type(return_type, _) => return_type.as_java_primitive(),
        }
    }

    fn as_java_object(&self) -> String {
        match self {
            Self::Void => "Void".to_string(),
            Self::Type(return_type, _) => return_type.as_java_object(),
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
