use oo_bindgen::formatting::*;
use oo_bindgen::native_enum::*;
use oo_bindgen::native_function::*;
use oo_bindgen::native_struct::*;

pub(crate) trait RustType {
    fn as_rust_type(&self) -> String;
    fn as_c_type(&self) -> String;
    fn is_copyable(&self) -> bool;
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
            Type::String => "*const std::os::raw::c_char".to_string(),
            Type::Struct(handle) => format!("{}", handle.name()),
            Type::StructRef(handle) => format!("Option<&{}>", handle.name),
            Type::Enum(handle) => format!("{}", handle.name),
            Type::ClassRef(handle) => format!("*mut crate::{}", handle.name),
            Type::Interface(handle) => format!("{}", handle.name),
            Type::OneTimeCallback(handle) => format!("{}", handle.name),
            Type::Iterator(handle) => {
                let lifetime = if handle.has_lifetime_annotation {
                    "<'a>"
                } else {
                    ""
                };
                format!("*mut crate::{}{}", handle.name(), lifetime)
            }
            Type::Collection(handle) => format!("*mut crate::{}", handle.name()),
            Type::Duration(mapping) => match mapping {
                DurationMapping::Milliseconds | DurationMapping::Seconds => "u64".to_string(),
                DurationMapping::SecondsFloat => "f32".to_string(),
            },
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
            Type::Struct(handle) => handle.name().to_string(),
            Type::StructRef(handle) => format!("*const {}", handle.name),
            Type::Enum(_) => "std::os::raw::c_int".to_string(),
            Type::ClassRef(handle) => format!("*mut crate::{}", handle.name),
            Type::Interface(handle) => handle.name.to_string(),
            Type::OneTimeCallback(handle) => handle.name.to_string(),
            Type::Iterator(handle) => {
                let lifetime = if handle.has_lifetime_annotation {
                    "<'a>"
                } else {
                    ""
                };
                format!("*mut crate::{}{}", handle.name(), lifetime)
            }
            Type::Collection(handle) => format!("*mut crate::{}", handle.name()),
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
            Type::String => true, // Just copying the pointer
            Type::Struct(_) => false,
            Type::StructRef(_) => true,
            Type::Enum(_) => true,
            Type::ClassRef(_) => true, // Just copying the opaque pointer
            Type::Interface(_) => false,
            Type::OneTimeCallback(_) => false,
            Type::Iterator(_) => true,   // Just copying the pointer
            Type::Collection(_) => true, // Just copying the pointer
            Type::Duration(_) => true,
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
            Type::String => None,
            Type::Struct(_) => None,
            Type::StructRef(handle) => Some(Box::new(StructRefConverter(handle.clone()))),
            Type::Enum(handle) => Some(Box::new(EnumConverter(handle.clone()))),
            Type::ClassRef(_) => None,
            Type::Interface(_) => None,
            Type::OneTimeCallback(_) => None,
            Type::Iterator(_) => None,
            Type::Collection(_) => None,
            Type::Duration(_) => None,
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
}

struct EnumConverter(NativeEnumHandle);

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

pub(crate) trait RustStruct {
    fn requires_lifetime_annotation(&self) -> bool;
    fn has_conversion(&self) -> bool;
}

impl RustStruct for NativeStructHandle {
    fn requires_lifetime_annotation(&self) -> bool {
        self.elements
            .iter()
            .any(|e| e.requires_lifetime_annotation())
    }

    fn has_conversion(&self) -> bool {
        for element in &self.elements {
            if element.element_type.has_conversion() {
                return true;
            }
        }

        false
    }
}

pub(crate) trait RustStructField {
    /*fn as_rust_type(&self) -> String;
    fn as_c_type(&self) -> String;*/
    fn requires_lifetime_annotation(&self) -> bool;
}

impl RustStructField for NativeStructElement {
    /*fn as_rust_type(&self) -> String {
        let mut result = format!("{}", self.element_type.as_rust_type());
        if let Type::Iterator(handle) = &self.element_type {
            if handle.has_lifetime_annotation {
                result.push_str("<'a>");
            }
        }
        result
    }

    fn as_c_type(&self) -> String {
        let mut result = format!("{}", self.element_type.as_c_type());
        if let Type::Iterator(handle) = &self.element_type {
            if handle.has_lifetime_annotation {
                result.push_str("<'a>");
            }
        }
        result
    }*/

    fn requires_lifetime_annotation(&self) -> bool {
        if let Type::Iterator(handle) = &self.element_type {
            handle.has_lifetime_annotation
        } else {
            false
        }
    }
}
