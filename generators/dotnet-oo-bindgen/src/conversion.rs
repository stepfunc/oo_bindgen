use oo_bindgen::formatting::*;
use oo_bindgen::native_function::*;

pub struct DotnetType<'a>(pub &'a Type);

impl<'a> DotnetType<'a> {
    /// Returns the .NET natural type
    pub fn as_dotnet_type(&self) -> String {
        match self.0 {
            Type::Bool => "bool".to_string(),
            Type::Uint8 => "byte".to_string(),
            Type::Sint8 => "sbyte".to_string(),
            Type::Uint16 => "ushort".to_string(),
            Type::Sint16 => "short".to_string(),
            Type::Uint32 => "uint".to_string(),
            Type::Sint32 => "int".to_string(),
            Type::Uint64 => "ulong".to_string(),
            Type::Sint64 => "long".to_string(),
            Type::Float => unimplemented!(),
            Type::Double => unimplemented!(),
            Type::String => unimplemented!(),
            Type::Struct(handle) => format!("{}", handle.name()),
            Type::StructRef(handle) => format!("{}", handle.name),
            Type::Enum(handle) => format!("{}", handle.name),
            Type::ClassRef(handle) => format!("{}", handle.name),
            Type::Duration(_) => "TimeSpan".to_string(),
        }
    }

    /// Return the .NET representation of the native C type
    pub fn as_native_type(&self) -> String {
        match self.0 {
            Type::Bool => "bool".to_string(),
            Type::Uint8 => "byte".to_string(),
            Type::Sint8 => "sbyte".to_string(),
            Type::Uint16 => "ushort".to_string(),
            Type::Sint16 => "short".to_string(),
            Type::Uint32 => "uint".to_string(),
            Type::Sint32 => "int".to_string(),
            Type::Uint64 => "ulong".to_string(),
            Type::Sint64 => "long".to_string(),
            Type::Float => unimplemented!(),
            Type::Double => unimplemented!(),
            Type::String => unimplemented!(),
            Type::Struct(handle) => format!("{}", handle.name()),
            Type::StructRef(handle) => format!("ref {}", handle.name),
            Type::Enum(handle) => format!("{}", handle.name),
            Type::ClassRef(_) => "IntPtr".to_string(),
            Type::Duration(mapping) => match mapping {
                DurationMapping::Milliseconds|DurationMapping::Seconds => "ulong".to_string(),
                DurationMapping::SecondsFloat => "float".to_string(),
            }
        }
    }

    pub fn conversion(&self) -> Option<Box<dyn TypeConverter>> {
        match self.0 {
            Type::Bool => None,
            Type::Uint8 => None,
            Type::Sint8 => None,
            Type::Uint16 => None,
            Type::Sint16 => None,
            Type::Uint32 => None,
            Type::Sint32 => None,
            Type::Uint64 => None,
            Type::Sint64 => None,
            Type::Float => unimplemented!(),
            Type::Double => unimplemented!(),
            Type::String => unimplemented!(),
            Type::Struct(_) => None,
            Type::StructRef(_) => None,
            Type::Enum(_) => None,
            Type::ClassRef(_) => None,
            Type::Duration(mapping) => match mapping {
                DurationMapping::Milliseconds => Some(Box::new(DurationMillisecondsConverter)),
                DurationMapping::Seconds => Some(Box::new(DurationSecondsConverter)),
                DurationMapping::SecondsFloat => Some(Box::new(DurationSecondsFloatConverter)),
            }
        }
    }
}

pub trait TypeConverter {
    fn convert_to_native(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()>;
    fn convert_from_native(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()>;
}

struct DurationMillisecondsConverter;
impl TypeConverter for DurationMillisecondsConverter {
    fn convert_to_native(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{}(ulong){}.TotalMilliseconds;", to, from))
    }

    fn convert_from_native(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{}TimeSpan.FromMilliseconds({});", to, from))
    }
}

struct DurationSecondsConverter;
impl TypeConverter for DurationSecondsConverter {
    fn convert_to_native(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{}(ulong){}.TotalSeconds;", to, from))
    }

    fn convert_from_native(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{}TimeSpan.FromSeconds({});", to, from))
    }
}

struct DurationSecondsFloatConverter;
impl TypeConverter for DurationSecondsFloatConverter {
    fn convert_to_native(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{}(float){}.TotalSeconds;", to, from))
    }

    fn convert_from_native(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{}TimeSpan.FromSeconds({});", to, from))
    }
}

pub struct DotnetReturnType<'a>(pub &'a ReturnType);

impl <'a> DotnetReturnType<'a> {
    pub fn as_dotnet_type(&self) -> String {
        match self.0 {
            ReturnType::Void => "void".to_string(),
            ReturnType::Type(return_type) => DotnetType(return_type).as_dotnet_type(),
        }
    }

    pub fn as_native_type(&self) -> String {
        match self.0 {
            ReturnType::Void => "void".to_string(),
            ReturnType::Type(return_type) => DotnetType(return_type).as_native_type(),
        }
    }
}
