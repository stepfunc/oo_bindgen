use heck::CamelCase;
use oo_bindgen::callback::*;
use oo_bindgen::formatting::*;
use oo_bindgen::native_enum::*;
use oo_bindgen::native_function::*;
use oo_bindgen::native_struct::*;

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

impl RustType for Type {
    fn as_rust_type(&self) -> String {
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
            Type::String => "&'a std::ffi::CStr".to_string(),
            Type::Struct(handle) => handle.name().to_camel_case(),
            Type::StructRef(handle) => format!("Option<&{}>", handle.name.to_camel_case()),
            Type::Enum(handle) => handle.name.to_camel_case(),
            Type::ClassRef(handle) => format!("*mut crate::{}", handle.name.to_camel_case()),
            Type::Interface(handle) => handle.name.to_camel_case(),
            Type::Iterator(handle) => {
                let lifetime = if handle.has_lifetime_annotation {
                    "<'a>"
                } else {
                    ""
                };
                format!("*mut crate::{}{}", handle.name().to_camel_case(), lifetime)
            }
            Type::Collection(handle) => format!("*mut crate::{}", handle.name().to_camel_case()),
            Type::Duration(_) => "std::time::Duration".to_string(),
        }
    }

    fn as_c_type(&self) -> String {
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
            Type::Struct(handle) => handle.name().to_camel_case(),
            Type::StructRef(handle) => format!("*const {}", handle.name.to_camel_case()),
            Type::Enum(_) => "std::os::raw::c_int".to_string(),
            Type::ClassRef(handle) => format!("*mut crate::{}", handle.name.to_camel_case()),
            Type::Interface(handle) => handle.name.to_camel_case(),
            Type::Iterator(handle) => {
                let lifetime = if handle.has_lifetime_annotation {
                    "<'a>"
                } else {
                    ""
                };
                format!("*mut crate::{}{}", handle.name().to_camel_case(), lifetime)
            }
            Type::Collection(handle) => format!("*mut crate::{}", handle.name().to_camel_case()),
            Type::Duration(mapping) => match mapping {
                DurationMapping::Milliseconds | DurationMapping::Seconds => "u64".to_string(),
                DurationMapping::SecondsFloat => "f32".to_string(),
            },
        }
    }

    fn is_copyable(&self) -> bool {
        match self {
            Type::Bool => true,
            Type::Uint8 => true,
            Type::Sint8 => true,
            Type::Uint16 => true,
            Type::Sint16 => true,
            Type::Uint32 => true,
            Type::Sint32 => true,
            Type::Uint64 => true,
            Type::Sint64 => true,
            Type::Float => true,
            Type::Double => true,
            Type::String => true, // Just copying the reference
            Type::Struct(_) => false,
            Type::StructRef(_) => true,
            Type::Enum(_) => true,
            Type::ClassRef(_) => true, // Just copying the opaque pointer
            Type::Interface(_) => false,
            Type::Iterator(_) => true,   // Just copying the pointer
            Type::Collection(_) => true, // Just copying the pointer
            Type::Duration(_) => true,
        }
    }

    fn rust_requires_lifetime(&self) -> bool {
        match self {
            Type::Bool => false,
            Type::Uint8 => false,
            Type::Sint8 => false,
            Type::Uint16 => false,
            Type::Sint16 => false,
            Type::Uint32 => false,
            Type::Sint32 => false,
            Type::Uint64 => false,
            Type::Sint64 => false,
            Type::Float => false,
            Type::Double => false,
            Type::String => true,
            Type::Struct(_) => false,
            Type::StructRef(_) => false,
            Type::Enum(_) => false,
            Type::ClassRef(_) => false,
            Type::Interface(_) => false,
            Type::Iterator(handle) => handle.has_lifetime_annotation,
            Type::Collection(_) => false,
            Type::Duration(_) => false,
        }
    }

    fn c_requires_lifetime(&self) -> bool {
        match self {
            Type::Bool => false,
            Type::Uint8 => false,
            Type::Sint8 => false,
            Type::Uint16 => false,
            Type::Sint16 => false,
            Type::Uint32 => false,
            Type::Sint32 => false,
            Type::Uint64 => false,
            Type::Sint64 => false,
            Type::Float => false,
            Type::Double => false,
            Type::String => false,
            Type::Struct(_) => false,
            Type::StructRef(_) => false,
            Type::Enum(_) => false,
            Type::ClassRef(_) => false,
            Type::Interface(_) => false,
            Type::Iterator(handle) => handle.has_lifetime_annotation,
            Type::Collection(_) => false,
            Type::Duration(_) => false,
        }
    }

    fn conversion(&self) -> Option<Box<dyn TypeConverter>> {
        match self {
            Type::Bool => None,
            Type::Uint8 => None,
            Type::Sint8 => None,
            Type::Uint16 => None,
            Type::Sint16 => None,
            Type::Uint32 => None,
            Type::Sint32 => None,
            Type::Uint64 => None,
            Type::Sint64 => None,
            Type::Float => None,
            Type::Double => None,
            Type::String => Some(Box::new(StringConverter)),
            Type::Struct(_) => None,
            Type::StructRef(handle) => Some(Box::new(StructRefConverter(handle.clone()))),
            Type::Enum(handle) => Some(Box::new(EnumConverter(handle.clone()))),
            Type::ClassRef(_) => None,
            Type::Interface(_) => None,
            Type::Iterator(_) => None,
            Type::Collection(_) => None,
            Type::Duration(mapping) => Some(Box::new(DurationConverter(*mapping))),
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

pub(crate) struct EnumConverter(pub(crate) NativeEnumHandle);

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

struct DurationConverter(DurationMapping);

impl TypeConverter for DurationConverter {
    fn convert_to_c(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        match self.0 {
            DurationMapping::Milliseconds => {
                f.writeln(&format!("{}{}.as_millis() as u64", to, from))
            }
            DurationMapping::Seconds => f.writeln(&format!("{}{}.as_secs()", to, from)),
            DurationMapping::SecondsFloat => f.writeln(&format!("{}{}.as_secs_f32()", to, from)),
        }
    }

    fn convert_from_c(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        match self.0 {
            DurationMapping::Milliseconds => {
                f.writeln(&format!("{}std::time::Duration::from_millis({})", to, from))
            }
            DurationMapping::Seconds => {
                f.writeln(&format!("{}std::time::Duration::from_secs({})", to, from))
            }
            DurationMapping::SecondsFloat => f.writeln(&format!(
                "{}std::time::Duration::from_secs_f32({})",
                to, from
            )),
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
