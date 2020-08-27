use crate::formatting::blocked;
use crate::NATIVE_FUNCTIONS_CLASSNAME;
use heck::{CamelCase, MixedCase};
use oo_bindgen::callback::*;
use oo_bindgen::class::*;
use oo_bindgen::formatting::*;
use oo_bindgen::iterator::*;
use oo_bindgen::native_enum::*;
use oo_bindgen::native_function::*;
use oo_bindgen::native_struct::*;

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
            Type::Bool => "byte".to_string(),
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
            Type::Struct(handle) => format!("{}.Native.ByValue", handle.name().to_camel_case()),
            Type::StructRef(handle) => {
                format!("{}.Native.ByReference", handle.name.to_camel_case())
            }
            Type::Enum(_) => "int".to_string(),
            Type::ClassRef(_) => "com.sun.jna.Pointer".to_string(),
            Type::Interface(handle) => {
                format!("{}.NativeAdapter.ByValue", handle.name.to_camel_case())
            }
            Type::OneTimeCallback(handle) => {
                format!("{}.NativeAdapter.ByValue", handle.name.to_camel_case())
            }
            Type::Iterator(_) => "com.sun.jna.Pointer".to_string(),
            Type::Duration(mapping) => match mapping {
                DurationMapping::Milliseconds | DurationMapping::Seconds => "long".to_string(),
                DurationMapping::SecondsFloat => "float".to_string(),
            },
        }
    }

    pub(crate) fn conversion(&self) -> Option<Box<dyn TypeConverter>> {
        match self.0 {
            Type::Bool => Some(Box::new(BoolConverter)),
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
            Type::Struct(handle) => Some(Box::new(StructConverter(handle.clone()))),
            Type::StructRef(handle) => Some(Box::new(StructRefConverter(handle.clone()))),
            Type::Enum(handle) => Some(Box::new(EnumConverter(handle.clone()))),
            Type::ClassRef(handle) => Some(Box::new(ClassConverter(handle.clone()))),
            Type::Interface(handle) => Some(Box::new(InterfaceConverter(handle.clone()))),
            Type::OneTimeCallback(handle) => {
                Some(Box::new(OneTimeCallbackConverter(handle.clone())))
            }
            Type::Iterator(handle) => Some(Box::new(IteratorConverter(handle.clone()))),
            Type::Duration(mapping) => match mapping {
                DurationMapping::Milliseconds => Some(Box::new(DurationMillisecondsConverter)),
                DurationMapping::Seconds => Some(Box::new(DurationSecondsConverter)),
                DurationMapping::SecondsFloat => Some(Box::new(DurationSecondsFloatConverter)),
            },
        }
    }

    pub(crate) fn as_java_arg(&self, param_name: &str) -> String {
        match self.0 {
            Type::Bool => format!("_{}", param_name.to_mixed_case()),
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
            Type::Struct(_) => format!("_{}", param_name.to_mixed_case()),
            Type::StructRef(_) => format!("_{}", param_name.to_mixed_case()),
            Type::Enum(_) => format!("_{}", param_name.to_mixed_case()),
            Type::ClassRef(_) => format!("_{}", param_name.to_mixed_case()),
            Type::Interface(_) => format!("_{}", param_name.to_mixed_case()),
            Type::OneTimeCallback(_) => format!("_{}", param_name.to_mixed_case()),
            Type::Iterator(_) => format!("_{}", param_name.to_mixed_case()),
            Type::Duration(mapping) => match mapping {
                DurationMapping::Milliseconds => format!("_{}", param_name.to_mixed_case()),
                DurationMapping::Seconds => format!("_{}", param_name.to_mixed_case()),
                DurationMapping::SecondsFloat => format!("_{}", param_name.to_mixed_case()),
            },
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

    fn convert_to_native_cleanup(&self, _f: &mut dyn Printer, _name: &str) -> FormattingResult<()> {
        Ok(())
    }
}

struct BoolConverter;
impl TypeConverter for BoolConverter {
    fn convert_to_native(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{}{} ? (byte)1 : 0;", to, from))
    }

    fn convert_from_native(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        f.writeln(&format!("{}{} != 0;", to, from))
    }
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

struct EnumConverter(NativeEnumHandle);
impl TypeConverter for EnumConverter {
    fn convert_to_native(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!(
            "{}{}.toNative({});",
            to,
            self.0.name.to_camel_case(),
            from
        ))
    }

    fn convert_from_native(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        f.writeln(&format!(
            "{}{}.fromNative({});",
            to,
            self.0.name.to_camel_case(),
            from
        ))
    }
}

struct InterfaceConverter(InterfaceHandle);
impl TypeConverter for InterfaceConverter {
    fn convert_to_native(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!(
            "{}new {}.NativeAdapter.ByValue({});",
            to,
            self.0.name.to_camel_case(),
            from
        ))
    }

    fn convert_from_native(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        f.writeln(&format!("{}{}.NativeAdapter._impls.containsKey({}.{}) ? {}.NativeAdapter._impls.get({}.{})._impl : null;", to, self.0.name.to_camel_case(), from, self.0.arg_name.to_mixed_case(), self.0.name.to_camel_case(), from, self.0.arg_name.to_mixed_case()))
    }
}

struct OneTimeCallbackConverter(OneTimeCallbackHandle);
impl TypeConverter for OneTimeCallbackConverter {
    fn convert_to_native(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!(
            "{}new {}.NativeAdapter.ByValue({});",
            to,
            self.0.name.to_camel_case(),
            from
        ))
    }

    fn convert_from_native(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        f.writeln(&format!(
            "{}{}.NativeAdapter._impls.get({}.{})._impl;",
            to,
            self.0.name.to_camel_case(),
            from,
            self.0.arg_name.to_mixed_case()
        ))
    }
}

struct StructConverter(NativeStructHandle);
impl TypeConverter for StructConverter {
    fn convert_to_native(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!(
            "{}new {}.Native.ByValue({});",
            to,
            self.0.name().to_camel_case(),
            from
        ))
    }

    /*fn convert_to_native_cleanup(&self, f: &mut dyn Printer, name: &str) -> FormattingResult<()> {
        f.writeln(&format!("{}.finalize();", name))
    }*/

    fn convert_from_native(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        f.writeln(&format!(
            "{}{}.Native.fromNative({});",
            to,
            self.0.name().to_camel_case(),
            from
        ))
    }
}

struct StructRefConverter(NativeStructDeclarationHandle);
impl TypeConverter for StructRefConverter {
    fn convert_to_native(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!(
            "{}new {}.Native.ByReference({});",
            to,
            self.0.name.to_camel_case(),
            from
        ))
    }

    fn convert_from_native(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        f.writeln(&format!(
            "{}{}.Native.fromNative({});",
            to,
            self.0.name.to_camel_case(),
            from
        ))
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
        f.writeln(&format!("if ({} != com.sun.jna.Pointer.NULL)", from))?;
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
        f.writeln(&format!("{}com.sun.jna.Pointer.NULL;", to))
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
            "for ({}.Native _itRawValue = {}; _itRawValue != null; _itRawValue = {})",
            item_type, next_call, next_call
        ))?;
        blocked(f, |f| {
            f.writeln(&format!("{} _itValue = null;", item_type))?;
            StructRefConverter(self.0.item_type.declaration()).convert_from_native(
                f,
                "_itRawValue",
                "_itValue = ",
            )?;
            f.writeln(&format!("{}.add(_itValue);", builder_name))
        })?;
        f.writeln(&format!(
            "{}java.util.Collections.unmodifiableList({});",
            to, builder_name
        ))
    }
}

struct DurationMillisecondsConverter;
impl TypeConverter for DurationMillisecondsConverter {
    fn convert_to_native(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{}{}.toMillis();", to, from))
    }

    fn convert_from_native(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        f.writeln(&format!("{}java.time.Duration.ofMillis({});", to, from))
    }
}

struct DurationSecondsConverter;
impl TypeConverter for DurationSecondsConverter {
    fn convert_to_native(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{}{}.getSeconds();", to, from))
    }

    fn convert_from_native(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        f.writeln(&format!("{}java.time.Duration.ofSeconds({});", to, from))
    }
}

struct DurationSecondsFloatConverter;
impl TypeConverter for DurationSecondsFloatConverter {
    fn convert_to_native(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!(
            "{}{}.getSeconds() + {}.getNano() / 1000000000.0f;",
            to, from, from
        ))
    }

    fn convert_from_native(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        f.writeln(&format!("{}java.time.Duration.ofSeconds((long){}).plusNanos((long)(({} - (long){}) * 1000000000));", to, from, from, from))
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
        if let Some(converter) = JavaType(&param.param_type).conversion() {
            let mut param_name = param.name.to_mixed_case();
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

    //Cleanup type conversions
    for param in method.parameters.iter() {
        if let Some(converter) = JavaType(&param.param_type).conversion() {
            converter.convert_to_native_cleanup(f, &format!("_{}", param.name.to_mixed_case()))?;
        }
    }

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

pub(crate) fn call_java_function(
    f: &mut dyn Printer,
    method: &CallbackFunction,
    return_destination: &str,
) -> FormattingResult<()> {
    // Write the type conversions
    for param in method.params() {
        if let Some(converter) = JavaType(&param.param_type).conversion() {
            converter.convert_from_native(
                f,
                &param.name,
                &format!(
                    "{} _{} = ",
                    JavaType(&param.param_type).as_java_type(),
                    param.name.to_mixed_case()
                ),
            )?;
        }
    }

    // Call the Java function
    f.newline()?;
    let method_name = method.name.to_mixed_case();
    if let ReturnType::Type(return_type, _) = &method.return_type {
        if JavaType(&return_type).conversion().is_some() {
            f.write(&format!(
                "{} _result = _arg._impl.{}(",
                JavaType(&return_type).as_java_type(),
                method_name
            ))?;
        } else {
            f.write(&format!(
                "{}_arg._impl.{}(",
                return_destination, method_name
            ))?;
        }
    } else {
        f.write(&format!("_arg._impl.{}(", method_name))?;
    }

    f.write(
        &method
            .params()
            .map(|param| JavaType(&param.param_type).as_java_arg(&param.name))
            .collect::<Vec<String>>()
            .join(", "),
    )?;
    f.write(");")?;

    // Convert the result (if required)
    if let ReturnType::Type(return_type, _) = &method.return_type {
        if let Some(converter) = JavaType(&return_type).conversion() {
            converter.convert_to_native(f, "_result", return_destination)?;
        }
    }

    Ok(())
}
