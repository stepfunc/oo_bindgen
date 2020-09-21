use oo_bindgen::formatting::*;
use oo_bindgen::native_enum::*;
use oo_bindgen::native_function::*;
use crate::formatting::*;

pub trait RustType {
    fn as_rust_type(&self) -> String;
    fn as_c_type(&self) -> String;
    fn conversion(&self) -> Option<Box<dyn TypeConverter>>;
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
            Type::StructRef(handle) => format!("*const {}", handle.name),
            Type::Enum(handle) => format!("{}", handle.name),
            Type::ClassRef(handle) => format!("*mut crate::{}", handle.name),
            Type::Interface(handle) => format!("{}", handle.name),
            Type::OneTimeCallback(handle) => format!("{}", handle.name),
            Type::Iterator(handle) => format!("*mut crate::{}", handle.name()),
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
            Type::Struct(handle) => format!("{}", handle.name()),
            Type::StructRef(handle) => format!("*const {}", handle.name),
            Type::Enum(handle) => "std::os::raw::c_int".to_string(),
            Type::ClassRef(handle) => format!("*mut crate::{}", handle.name),
            Type::Interface(handle) => format!("{}", handle.name),
            Type::OneTimeCallback(handle) => format!("{}", handle.name),
            Type::Iterator(handle) => format!("*mut crate::{}", handle.name()),
            Type::Collection(handle) => format!("*mut crate::{}", handle.name()),
            Type::Duration(mapping) => match mapping {
                DurationMapping::Milliseconds | DurationMapping::Seconds => "u64".to_string(),
                DurationMapping::SecondsFloat => "f32".to_string(),
            },
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
            Type::StructRef(_) => None,
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

pub trait TypeConverter {
    fn convert_to_c(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()>;
    fn convert_from_c(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()>;
}

struct EnumConverter(NativeEnumHandle);

impl TypeConverter for EnumConverter {
    fn convert_to_c(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{} = match {}", to, from))?;
        blocked(f, |f| {
            for variant in self.0.variants {
                f.writeln(&format!("{}::{} => {},", self.0.name, variant.name, variant.value))?;
            }
            Ok(())
        })?;
        f.write(";")
    }

    fn convert_from_c(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        f.writeln(&format!("{} = match {}", to, from))?;
        blocked(f, |f| {
            for variant in self.0.variants {
                f.writeln(&format!("{} => {}::{},", variant.value, self.0.name, variant.name, ))?;
            }
            f.writeln(&format!("_ => panic!(\"{{}} is not a variant of {}\", {}),", self.0.name, from))
        })?;
        f.write(";")
    }
}

/*pub struct StructField<'a>(&'a Type);

impl<'a> Display for StructField<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", RustType(self.0).as_rust_type())?;
        if let Type::Iterator(handle) = &self.0 {
            if handle.has_lifetime_annotation {
                f.write_str("<'a>")?
            }
        }
        Ok(())
    }
}*/