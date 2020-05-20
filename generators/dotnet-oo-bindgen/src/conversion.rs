use oo_bindgen::formatting::*;
use oo_bindgen::interface::*;
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
            Type::Interface(handle) => format!("{}", handle.name),
            Type::Duration(_) => "TimeSpan".to_string(),
        }
    }

    /// Return the .NET representation of the native C type
    pub fn as_native_type(&self) -> String {
        match self.0 {
            Type::Bool => "byte".to_string(),
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
            Type::Interface(handle) => format!("{}NativeAdapter", handle.name),
            Type::Duration(mapping) => match mapping {
                DurationMapping::Milliseconds|DurationMapping::Seconds => "ulong".to_string(),
                DurationMapping::SecondsFloat => "float".to_string(),
            }
        }
    }

    pub fn conversion(&self) -> Option<Box<dyn TypeConverter>> {
        match self.0 {
            Type::Bool => Some(Box::new(BoolConverter)),
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
            Type::Interface(handle) => Some(Box::new(InterfaceConverter(handle.clone()))),
            Type::Duration(mapping) => match mapping {
                DurationMapping::Milliseconds => Some(Box::new(DurationMillisecondsConverter)),
                DurationMapping::Seconds => Some(Box::new(DurationSecondsConverter)),
                DurationMapping::SecondsFloat => Some(Box::new(DurationSecondsFloatConverter)),
            }
        }
    }

    pub fn as_dotnet_arg(&self, param_name: &str) -> String {
        match self.0 {
            Type::Bool => param_name.to_string(),
            Type::Uint8 => param_name.to_string(),
            Type::Sint8 => param_name.to_string(),
            Type::Uint16 => param_name.to_string(),
            Type::Sint16 => param_name.to_string(),
            Type::Uint32 => param_name.to_string(),
            Type::Sint32 => param_name.to_string(),
            Type::Uint64 => param_name.to_string(),
            Type::Sint64 => param_name.to_string(),
            Type::Float => unimplemented!(),
            Type::Double => unimplemented!(),
            Type::String => unimplemented!(),
            Type::Struct(_) => param_name.to_string(),
            Type::StructRef(_) => format!("ref {}", param_name.to_string()),
            Type::Enum(_) => param_name.to_string(),
            Type::ClassRef(_) => format!("{}.self", param_name.to_string()),
            Type::Interface(_) => format!("_{}", param_name),
            Type::Duration(mapping) => match mapping {
                DurationMapping::Milliseconds => format!("_{}", param_name),
                DurationMapping::Seconds => format!("_{}", param_name),
                DurationMapping::SecondsFloat => format!("_{}", param_name),
            }
        }
    }
}

pub trait TypeConverter {
    fn convert_to_native(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()>;
    fn convert_from_native(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()>;
}

// By default, PInvoke transforms "bool" into a weird 4-bit value
struct BoolConverter;
impl TypeConverter for BoolConverter {
    fn convert_to_native(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{}Convert.ToByte({});", to, from))
    }

    fn convert_from_native(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{}Convert.ToBoolean({});", to, from))
    }
}

struct InterfaceConverter(InterfaceHandle);
impl TypeConverter for InterfaceConverter {
    fn convert_to_native(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{}new {}NativeAdapter({});", to, self.0.name, from))
    }

    fn convert_from_native(&self, _: &mut dyn Printer, _: &str, _: &str) -> FormattingResult<()> {
        unimplemented!("cannot return interfaces");
    }
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
