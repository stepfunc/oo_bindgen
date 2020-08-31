use super::formatting::blocked;
use super::NATIVE_FUNCTIONS_CLASSNAME;
use heck::{CamelCase, MixedCase};
use oo_bindgen::class::*;
use oo_bindgen::formatting::*;
use oo_bindgen::iterator::*;
use oo_bindgen::native_function::*;

pub(crate) struct JavaType<'a>(pub(crate) &'a Type);

impl<'a> JavaType<'a> {
    /// Returns the Java natural type
    pub(crate) fn as_java_type(&self) -> String {
        match self.0 {
            Type::Bool => "Boolean".to_string(),
            Type::Uint8 => "UByte".to_string(),
            Type::Sint8 => "Byte".to_string(),
            Type::Uint16 => "UShort".to_string(),
            Type::Sint16 => "Short".to_string(),
            Type::Uint32 => "UInteger".to_string(),
            Type::Sint32 => "Integer".to_string(),
            Type::Uint64 => "ULong".to_string(),
            Type::Sint64 => "Long".to_string(),
            Type::Float => "Float".to_string(),
            Type::Double => "Double".to_string(),
            Type::String => "String".to_string(),
            Type::Struct(handle) => handle.name().to_camel_case(),
            Type::StructRef(handle) => handle.name.to_camel_case(),
            Type::Enum(handle) => handle.name.to_camel_case(),
            Type::ClassRef(handle) => handle.name.to_camel_case(),
            Type::Interface(handle) => handle.name.to_camel_case(),
            Type::OneTimeCallback(handle) => handle.name.to_camel_case(),
            Type::Iterator(handle) => format!(
                "java.util.List<{}>",
                handle.item_type.name().to_camel_case()
            ),
            Type::Duration(_) => "java.time.Duration".to_string(),
        }
    }

    /// Return the Java representation of the native C type
    pub(crate) fn as_native_type(&self) -> String {
        match self.0 {
            Type::Bool => "boolean".to_string(),
            Type::Uint8 => "byte".to_string(),
            Type::Sint8 => "byte".to_string(),
            Type::Uint16 => "short".to_string(),
            Type::Sint16 => "short".to_string(),
            Type::Uint32 => "int".to_string(),
            Type::Sint32 => "int".to_string(),
            Type::Uint64 => "long".to_string(),
            Type::Sint64 => "long".to_string(),
            Type::Float => "float".to_string(),
            Type::Double => "double".to_string(),
            Type::String => "String".to_string(),
            Type::Struct(handle) => handle.name().to_camel_case(),
            Type::StructRef(handle) => handle.name.to_camel_case(),
            Type::Enum(handle) => handle.name.to_string(),
            Type::ClassRef(_) => "long".to_string(),
            Type::Interface(handle) => handle.name.to_camel_case(),
            Type::OneTimeCallback(handle) => handle.name.to_camel_case(),
            Type::Iterator(_) => "long".to_string(),
            Type::Duration(_) => "java.time.Duration".to_string(),
        }
    }

    pub(crate) fn conversion(&self) -> Option<Box<dyn TypeConverter>> {
        match self.0 {
            Type::Bool => None,
            Type::Uint8 => Some(Box::new(UByteConverter)),
            Type::Sint8 => None,
            Type::Uint16 => Some(Box::new(UShortConverter)),
            Type::Sint16 => None,
            Type::Uint32 => Some(Box::new(UIntegerConverter)),
            Type::Sint32 => None,
            Type::Uint64 => Some(Box::new(ULongConverter)),
            Type::Sint64 => None,
            Type::Float => None,
            Type::Double => None,
            Type::String => None,
            Type::Struct(_) => None,
            Type::StructRef(_) => None,
            Type::Enum(_) => None,
            Type::ClassRef(handle) => Some(Box::new(ClassConverter(handle.clone()))),
            Type::Interface(_) => None,
            Type::OneTimeCallback(_) => None,
            Type::Iterator(handle) => Some(Box::new(IteratorConverter(handle.clone()))),
            Type::Duration(_) => None,
        }
    }

    pub(crate) fn as_java_arg(&self, param_name: &str) -> String {
        match self.0 {
            Type::Bool => param_name.to_mixed_case(),
            Type::Uint8 => format!("_{}", param_name.to_mixed_case()),
            Type::Sint8 => param_name.to_mixed_case(),
            Type::Uint16 => format!("_{}", param_name.to_mixed_case()),
            Type::Sint16 => param_name.to_mixed_case(),
            Type::Uint32 => format!("_{}", param_name.to_mixed_case()),
            Type::Sint32 => param_name.to_mixed_case(),
            Type::Uint64 => format!("_{}", param_name.to_mixed_case()),
            Type::Sint64 => param_name.to_mixed_case(),
            Type::Float => param_name.to_mixed_case(),
            Type::Double => param_name.to_mixed_case(),
            Type::String => param_name.to_mixed_case(),
            Type::Struct(_) => param_name.to_mixed_case(),
            Type::StructRef(_) => param_name.to_mixed_case(),
            Type::Enum(_) => param_name.to_mixed_case(),
            Type::ClassRef(_) => format!("_{}", param_name.to_mixed_case()),
            Type::Interface(_) => param_name.to_mixed_case(),
            Type::OneTimeCallback(_) => param_name.to_mixed_case(),
            Type::Iterator(_) => format!("_{}", param_name.to_mixed_case()),
            Type::Duration(_) => param_name.to_mixed_case(),
        }
    }
}

pub(crate) trait TypeConverter {
    fn convert_to_native(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()>;
    fn convert_from_native(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()>;
}

struct UByteConverter;
impl TypeConverter for UByteConverter {
    fn convert_to_native(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{}{}.byteValue();", to, from))
    }

    fn convert_from_native(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        f.writeln(&format!("{}UByte.valueOf({});", to, from))
    }
}

struct UShortConverter;
impl TypeConverter for UShortConverter {
    fn convert_to_native(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{}{}.shortValue();", to, from))
    }

    fn convert_from_native(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        f.writeln(&format!("{}UShort.valueOf({});", to, from))
    }
}

struct UIntegerConverter;
impl TypeConverter for UIntegerConverter {
    fn convert_to_native(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{}{}.intValue();", to, from))
    }

    fn convert_from_native(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        f.writeln(&format!("{}UInteger.valueOf({});", to, from))
    }
}

struct ULongConverter;
impl TypeConverter for ULongConverter {
    fn convert_to_native(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{}{}.longValue();", to, from))
    }

    fn convert_from_native(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        f.writeln(&format!("{}ULong.valueOf({});", to, from))
    }
}

struct ClassConverter(ClassDeclarationHandle);
impl TypeConverter for ClassConverter {
    fn convert_to_native(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{}{}.self;", to, from))
    }

    fn convert_from_native(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        let handle_name = format!("_{}_handle", from);
        f.writeln(&format!(
            "{} {} = null;",
            self.0.name.to_camel_case(),
            handle_name
        ))?;
        f.writeln(&format!("if ({} != 0L)", from))?;
        blocked(f, |f| {
            f.writeln(&format!(
                "{} = new {}({});",
                handle_name,
                self.0.name.to_camel_case(),
                from
            ))
        })?;
        f.writeln(&format!("{}{};", to, handle_name))
    }
}

struct IteratorConverter(IteratorHandle);
impl TypeConverter for IteratorConverter {
    fn convert_to_native(
        &self,
        f: &mut dyn Printer,
        _from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        f.writeln(&format!("{}0L;", to))
    }

    fn convert_from_native(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        let item_type = self.0.item_type.name().to_camel_case();
        let builder_name = format!("_{}Builder", from.replace(".", "_"));
        let next_call = format!(
            "{}.{}({})",
            NATIVE_FUNCTIONS_CLASSNAME, self.0.native_func.name, from
        );

        f.writeln(&format!(
            "java.util.List<{}> {} = new java.util.ArrayList<>();",
            item_type, builder_name,
        ))?;
        f.writeln(&format!(
            "for ({} _itValue = {}; _itValue != null; _itValue = {})",
            item_type, next_call, next_call
        ))?;
        blocked(f, |f| {
            f.writeln(&format!("{}.add(_itValue);", builder_name))
        })?;
        f.writeln(&format!(
            "{}java.util.Collections.unmodifiableList({});",
            to, builder_name
        ))
    }
}

pub(crate) struct JavaReturnType<'a>(pub(crate) &'a ReturnType);

impl<'a> JavaReturnType<'a> {
    pub(crate) fn as_java_type(&self) -> String {
        match self.0 {
            ReturnType::Void => "void".to_string(),
            ReturnType::Type(return_type, _) => JavaType(return_type).as_java_type(),
        }
    }

    pub(crate) fn as_native_type(&self) -> String {
        match self.0 {
            ReturnType::Void => "void".to_string(),
            ReturnType::Type(return_type, _) => JavaType(return_type).as_native_type(),
        }
    }
}

pub(crate) fn call_native_function(
    f: &mut dyn Printer,
    method: &NativeFunction,
    return_destination: &str,
    first_param_is_self: Option<String>,
    is_constructor: bool,
) -> FormattingResult<()> {
    // Write the type conversions
    for (idx, param) in method.parameters.iter().enumerate() {
        let mut param_name = param.name.to_mixed_case();
        if idx == 0 {
            if let Some(first_param) = first_param_is_self.clone() {
                param_name = first_param;
            }
        }

        if let Some(converter) = JavaType(&param.param_type).conversion() {
            if idx == 0 {
                if let Some(first_param) = first_param_is_self.clone() {
                    param_name = first_param;
                }
            }
            converter.convert_to_native(
                f,
                &param_name,
                &format!(
                    "{} _{} = ",
                    JavaType(&param.param_type).as_native_type(),
                    param.name.to_mixed_case()
                ),
            )?;
        } else if idx == 0 && first_param_is_self.is_some() {
            f.writeln(&format!("{} {} = this;", JavaType(&param.param_type).as_native_type(), param.name.to_mixed_case()))?;
        }
    }

    // Call the native function
    f.newline()?;
    if let ReturnType::Type(return_type, _) = &method.return_type {
        f.write(&format!(
            "{} _result = {}.{}(",
            JavaType(return_type).as_native_type(),
            NATIVE_FUNCTIONS_CLASSNAME,
            method.name
        ))?;
    } else {
        f.write(&format!("{}.{}(", NATIVE_FUNCTIONS_CLASSNAME, method.name))?;
    }

    f.write(
        &method
            .parameters
            .iter()
            .map(|param| JavaType(&param.param_type).as_java_arg(&param.name.to_mixed_case()))
            .collect::<Vec<String>>()
            .join(", "),
    )?;
    f.write(");")?;

    // Convert the result (if required) and return
    if let ReturnType::Type(return_type, _) = &method.return_type {
        if let Some(converter) = JavaType(&return_type).conversion() {
            if !is_constructor {
                return converter.convert_from_native(f, "_result", return_destination);
            }
        }

        f.writeln(&format!("{}_result;", return_destination))?;
    }

    Ok(())
}
