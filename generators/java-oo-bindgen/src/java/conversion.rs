use super::NATIVE_FUNCTIONS_CLASSNAME;
use heck::{CamelCase, MixedCase};
use oo_bindgen::formatting::*;
use oo_bindgen::function::*;
use oo_bindgen::interface::{CArgument, CReturnValue, InterfaceHandle};
use oo_bindgen::return_type::ReturnType;
use oo_bindgen::structs::callback_struct::CStructFieldType;
use oo_bindgen::structs::function_return_struct::RStructFieldType;
use oo_bindgen::structs::function_struct::FStructFieldType;
use oo_bindgen::structs::univeral_struct::UStructFieldType;
use oo_bindgen::types::{BasicType, StringType};
use oo_bindgen::collection::CollectionHandle;
use oo_bindgen::structs::common::{Struct, StructFieldType, StructDeclarationHandle};
use oo_bindgen::Handle;
use oo_bindgen::class::ClassDeclarationHandle;
use oo_bindgen::iterator::IteratorHandle;

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
    }}

impl JavaType for IteratorHandle {
    fn as_java_primitive(&self) -> String {
        self.as_java_object()
    }

    fn as_java_object(&self) -> String {
        format!(
            "java.util.List<{}>",
            self.item_type.as_java_object()
        )
    }
}

impl<T> JavaType for Handle<Struct<T>> where T: StructFieldType {
    fn as_java_primitive(&self) -> String {
        self.as_java_object()
    }

    fn as_java_object(&self) -> String {
        self.name().to_camel_case()
    }
}

impl JavaType for FStructFieldType {
    fn as_java_primitive(&self) -> String {
        match self {
            FStructFieldType::Basic(x) => x.as_java_primitive(),
            FStructFieldType::String(x) => x.as_java_primitive(),
            FStructFieldType::Interface(x) => x.as_java_primitive(),
            FStructFieldType::Collection(x) => x.as_java_primitive(),
            FStructFieldType::Struct(x) => x.as_java_primitive(),
        }
    }

    fn as_java_object(&self) -> String {
        match self {
            FStructFieldType::Basic(x) => x.as_java_object(),
            FStructFieldType::String(x) => x.as_java_object(),
            FStructFieldType::Interface(x) => x.as_java_object(),
            FStructFieldType::Collection(x) => x.as_java_object(),
            FStructFieldType::Struct(x) => x.as_java_object(),
        }
    }
}

impl JavaType for RStructFieldType {
    fn as_java_primitive(&self) -> String {
        match self {
            RStructFieldType::Basic(x) => x.as_java_primitive(),
            RStructFieldType::ClassRef(x) => x.as_java_primitive(),
            RStructFieldType::Struct(x) => x.as_java_primitive(),
        }
    }

    fn as_java_object(&self) -> String {
        match self {
            RStructFieldType::Basic(x) => x.as_java_object(),
            RStructFieldType::ClassRef(x) => x.as_java_object(),
            RStructFieldType::Struct(x) => x.as_java_object(),
        }
    }
}

impl JavaType for CStructFieldType {
    fn as_java_primitive(&self) -> String {
        match self {
            CStructFieldType::Basic(x) => x.as_java_primitive(),
            CStructFieldType::Iterator(x) => x.as_java_primitive(),
            CStructFieldType::Struct(x) => x.as_java_primitive(),
        }
    }

    fn as_java_object(&self) -> String {
        match self {
            CStructFieldType::Basic(x) => x.as_java_object(),
            CStructFieldType::Iterator(x) => x.as_java_object(),
            CStructFieldType::Struct(x) => x.as_java_object(),
        }
    }
}

impl JavaType for UStructFieldType {
    fn as_java_primitive(&self) -> String {
        match self {
            UStructFieldType::Basic(x) => x.as_java_primitive(),
            UStructFieldType::Struct(x) => x.as_java_primitive(),
        }
    }

    fn as_java_object(&self) -> String {
        match self {
            UStructFieldType::Basic(x) => x.as_java_object(),
            UStructFieldType::Struct(x) => x.as_java_object(),
        }
    }
}

impl JavaType for FArgument {
    fn as_java_primitive(&self) -> String {
        match self {
            FArgument::Basic(x) => x.as_java_primitive(),
            FArgument::String(x) => x.as_java_primitive(),
            FArgument::Collection(x) => x.as_java_primitive(),
            FArgument::Struct(x) => x.as_java_primitive(),
            FArgument::StructRef(x) => x.as_java_primitive(),
            FArgument::ClassRef(x) => x.as_java_primitive(),
            FArgument::Interface(x) => x.as_java_primitive(),
        }
    }

    fn as_java_object(&self) -> String {
        match self {
            FArgument::Basic(x) => x.as_java_object(),
            FArgument::String(x) => x.as_java_object(),
            FArgument::Collection(x) => x.as_java_object(),
            FArgument::Struct(x) => x.as_java_object(),
            FArgument::StructRef(x) => x.as_java_object(),
            FArgument::ClassRef(x) => x.as_java_object(),
            FArgument::Interface(x) => x.as_java_object(),
        }
    }
}

impl JavaType for CArgument {
    fn as_java_primitive(&self) -> String {
        match self {
            CArgument::Basic(x) => x.as_java_primitive(),
            CArgument::String(x) => x.as_java_primitive(),
            CArgument::Iterator(x) => x.as_java_primitive(),
            CArgument::Struct(x) => x.as_java_primitive(),
        }
    }

    fn as_java_object(&self) -> String {
        match self {
            CArgument::Basic(x) => x.as_java_object(),
            CArgument::String(x) => x.as_java_object(),
            CArgument::Iterator(x) => x.as_java_object(),
            CArgument::Struct(x) => x.as_java_object(),
        }
    }
}

impl JavaType for CReturnValue {
    fn as_java_primitive(&self) -> String {
        match self {
            CReturnValue::Basic(x) => x.as_java_primitive(),
            CReturnValue::Struct(x) => x.as_java_primitive(),
        }
    }

    fn as_java_object(&self) -> String {
        match self {
            CReturnValue::Basic(x) => x.as_java_object(),
            CReturnValue::Struct(x) => x.as_java_object(),
        }
    }
}

impl JavaType for FReturnValue {
    fn as_java_primitive(&self) -> String {
        match self {
            FReturnValue::Basic(x) => x.as_java_primitive(),
            FReturnValue::String(x) => x.as_java_primitive(),
            FReturnValue::ClassRef(x) => x.as_java_primitive(),
            FReturnValue::Struct(x) => x.as_java_primitive(),
            FReturnValue::StructRef(x) => x.as_java_primitive(),
        }
    }

    fn as_java_object(&self) -> String {
        match self {
            FReturnValue::Basic(x) => x.as_java_object(),
            FReturnValue::String(x) => x.as_java_object(),
            FReturnValue::ClassRef(x) => x.as_java_object(),
            FReturnValue::Struct(x) => x.as_java_object(),
            FReturnValue::StructRef(x) => x.as_java_object(),
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
