use heck::CamelCase;
use oo_bindgen::callback::*;
use oo_bindgen::formatting::*;
use oo_bindgen::native_enum::*;
use oo_bindgen::native_function::*;
use oo_bindgen::native_struct::*;
use oo_bindgen::types::{BasicType, DurationType};
use oo_bindgen::struct_common::NativeStructDeclarationHandle;

pub(crate) trait RustType {
    fn as_rust_type(&self) -> String;
    fn as_c_type(&self) -> String;
    fn is_copyable(&self) -> bool;
    fn rust_requires_lifetime(&self) -> bool;
    fn c_requires_lifetime(&self) -> bool;
    fn conversion(&self) -> Option<Box<dyn TypeConverter>>;
    fn has_conversion(&self) -> bool {
        self.conversion().is_some()
    }
}

impl RustType for BasicType {
    fn as_rust_type(&self) -> String {
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
            Self::Duration(_) => "std::time::Duration".to_string(),
            Self::Enum(handle) => handle.name.to_camel_case(),
        }
    }

    fn as_c_type(&self) -> String {
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

    fn is_copyable(&self) -> bool {
        true
    }

    fn rust_requires_lifetime(&self) -> bool {
        false
    }

    fn c_requires_lifetime(&self) -> bool {
        false
    }

    fn conversion(&self) -> Option<Box<dyn TypeConverter>> {
        match self {
            Self::Bool => None,
            Self::Uint8 => None,
            Self::Sint8 => None,
            Self::Uint16 => None,
            Self::Sint16 => None,
            Self::Uint32 => None,
            Self::Sint32 => None,
            Self::Uint64 => None,
            Self::Sint64 => None,
            Self::Float => None,
            Self::Double => None,
            Self::Duration(mapping) => Some(Box::new(DurationConverter(*mapping))),
            Self::Enum(handle) => Some(Box::new(EnumConverter(handle.clone()))),
        }
    }
}

impl RustType for AllTypes {
    fn as_rust_type(&self) -> String {
        match self {
            AllTypes::Basic(x) => x.as_rust_type(),
            AllTypes::String => "&'a std::ffi::CStr".to_string(),
            AllTypes::Struct(handle) => handle.name().to_camel_case(),
            AllTypes::StructRef(handle) => format!("Option<&{}>", handle.name.to_camel_case()),
            AllTypes::ClassRef(handle) => format!("*mut crate::{}", handle.name.to_camel_case()),
            AllTypes::Interface(handle) => handle.name.to_camel_case(),
            AllTypes::Iterator(handle) => {
                let lifetime = if handle.has_lifetime_annotation {
                    "<'a>"
                } else {
                    ""
                };
                format!("*mut crate::{}{}", handle.name().to_camel_case(), lifetime)
            }
            AllTypes::Collection(handle) => format!("*mut crate::{}", handle.name().to_camel_case()),
        }
    }

    fn as_c_type(&self) -> String {
        match self {
            AllTypes::Basic(x) => x.as_c_type(),
            AllTypes::String => "*const std::os::raw::c_char".to_string(),
            AllTypes::Struct(handle) => handle.name().to_camel_case(),
            AllTypes::StructRef(handle) => format!("*const {}", handle.name.to_camel_case()),
            AllTypes::ClassRef(handle) => format!("*mut crate::{}", handle.name.to_camel_case()),
            AllTypes::Interface(handle) => handle.name.to_camel_case(),
            AllTypes::Iterator(handle) => {
                let lifetime = if handle.has_lifetime_annotation {
                    "<'a>"
                } else {
                    ""
                };
                format!("*mut crate::{}{}", handle.name().to_camel_case(), lifetime)
            }
            AllTypes::Collection(handle) => format!("*mut crate::{}", handle.name().to_camel_case()),
        }
    }

    fn is_copyable(&self) -> bool {
        match self {
            AllTypes::Basic(x) => x.is_copyable(),
            AllTypes::String => true, // Just copying the reference
            AllTypes::Struct(_) => false,
            AllTypes::StructRef(_) => true,
            AllTypes::ClassRef(_) => true, // Just copying the opaque pointer
            AllTypes::Interface(_) => false,
            AllTypes::Iterator(_) => true,   // Just copying the pointer
            AllTypes::Collection(_) => true, // Just copying the pointer
        }
    }

    fn rust_requires_lifetime(&self) -> bool {
        match self {
            AllTypes::Basic(x) => x.rust_requires_lifetime(),
            AllTypes::String => true,
            AllTypes::Struct(_) => false,
            AllTypes::StructRef(_) => false,
            AllTypes::ClassRef(_) => false,
            AllTypes::Interface(_) => false,
            AllTypes::Iterator(handle) => handle.has_lifetime_annotation,
            AllTypes::Collection(_) => false,
        }
    }

    fn c_requires_lifetime(&self) -> bool {
        match self {
            AllTypes::Basic(x) => x.c_requires_lifetime(),
            AllTypes::String => false,
            AllTypes::Struct(_) => false,
            AllTypes::StructRef(_) => false,
            AllTypes::ClassRef(_) => false,
            AllTypes::Interface(_) => false,
            AllTypes::Iterator(handle) => handle.has_lifetime_annotation,
            AllTypes::Collection(_) => false,
        }
    }

    fn conversion(&self) -> Option<Box<dyn TypeConverter>> {
        match self {
            AllTypes::Basic(x) => x.conversion(),
            AllTypes::String => Some(Box::new(StringConverter)),
            AllTypes::Struct(_) => None,
            AllTypes::StructRef(handle) => Some(Box::new(StructRefConverter(handle.clone()))),
            AllTypes::ClassRef(_) => None,
            AllTypes::Interface(_) => None,
            AllTypes::Iterator(_) => None,
            AllTypes::Collection(_) => None,
        }
    }
}

impl RustType for ReturnType {
    fn as_rust_type(&self) -> String {
        if let ReturnType::Type(return_type, _) = self {
            return_type.as_rust_type()
        } else {
            "()".to_string()
        }
    }

    fn as_c_type(&self) -> String {
        if let ReturnType::Type(return_type, _) = self {
            return_type.as_c_type()
        } else {
            "()".to_string()
        }
    }

    fn is_copyable(&self) -> bool {
        if let ReturnType::Type(return_type, _) = self {
            return_type.is_copyable()
        } else {
            true
        }
    }

    fn rust_requires_lifetime(&self) -> bool {
        if let ReturnType::Type(return_type, _) = self {
            return_type.rust_requires_lifetime()
        } else {
            false
        }
    }

    fn c_requires_lifetime(&self) -> bool {
        if let ReturnType::Type(return_type, _) = self {
            return_type.c_requires_lifetime()
        } else {
            false
        }
    }

    fn conversion(&self) -> Option<Box<dyn TypeConverter>> {
        if let ReturnType::Type(return_type, _) = self {
            return_type.conversion()
        } else {
            None
        }
    }
}

pub(crate) trait TypeConverter {
    fn convert_to_c(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()>;
    fn convert_from_c(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()>;
    fn is_unsafe(&self) -> bool {
        false
    }
}

struct StringConverter;

impl TypeConverter for StringConverter {
    fn convert_to_c(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{}{}.as_ptr()", to, from))
    }

    fn convert_from_c(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{}std::ffi::CStr::from_ptr({})", to, from))
    }

    fn is_unsafe(&self) -> bool {
        true
    }
}

pub(crate) struct EnumConverter(pub(crate) EnumHandle);

impl TypeConverter for EnumConverter {
    fn convert_to_c(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{}{}.into()", to, from))
    }

    fn convert_from_c(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{}{}.into()", to, from))
    }
}

struct StructRefConverter(NativeStructDeclarationHandle);

impl TypeConverter for StructRefConverter {
    fn convert_to_c(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!(
            "{}{}.map_or(std::ptr::null(), |val| val as *const _)",
            to, from
        ))
    }

    fn convert_from_c(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{}{}.as_ref()", to, from))
    }
}

struct DurationConverter(DurationType);

impl TypeConverter for DurationConverter {
    fn convert_to_c(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        match self.0 {
            DurationType::Milliseconds => f.writeln(&format!("{}{}.as_millis() as u64", to, from)),
            DurationType::Seconds => f.writeln(&format!("{}{}.as_secs()", to, from)),
        }
    }

    fn convert_from_c(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        match self.0 {
            DurationType::Milliseconds => {
                f.writeln(&format!("{}std::time::Duration::from_millis({})", to, from))
            }
            DurationType::Seconds => {
                f.writeln(&format!("{}std::time::Duration::from_secs({})", to, from))
            }
        }
    }
}

pub(crate) trait RustStruct {
    fn rust_requires_lifetime(&self) -> bool;
    fn c_requires_lifetime(&self) -> bool;
    fn has_conversion(&self) -> bool;
}

impl RustStruct for NativeStructHandle {
    fn rust_requires_lifetime(&self) -> bool {
        self.elements.iter().any(|e| e.rust_requires_lifetime())
    }

    fn c_requires_lifetime(&self) -> bool {
        self.elements.iter().any(|e| e.c_requires_lifetime())
    }

    fn has_conversion(&self) -> bool {
        self.elements
            .iter()
            .any(|e| e.element_type.to_type().has_conversion())
    }
}

pub(crate) trait RustStructField {
    fn rust_requires_lifetime(&self) -> bool;
    fn c_requires_lifetime(&self) -> bool;
}

impl RustStructField for NativeStructElement {
    fn rust_requires_lifetime(&self) -> bool {
        self.element_type.to_type().rust_requires_lifetime()
    }

    fn c_requires_lifetime(&self) -> bool {
        self.element_type.to_type().c_requires_lifetime()
    }
}

pub(crate) trait RustCallbackFunction {
    fn rust_requires_lifetime(&self) -> bool;
    fn c_requires_lifetime(&self) -> bool;
}

impl RustCallbackFunction for CallbackFunction {
    fn rust_requires_lifetime(&self) -> bool {
        self.parameters.iter().any(|param| {
            if let CallbackParameter::Parameter(param) = param {
                param.param_type.rust_requires_lifetime()
            } else {
                false
            }
        })
    }

    fn c_requires_lifetime(&self) -> bool {
        self.parameters.iter().any(|param| {
            if let CallbackParameter::Parameter(param) = param {
                param.param_type.c_requires_lifetime()
            } else {
                false
            }
        })
    }
}
