use crate::formatting::blocked;
use crate::NATIVE_FUNCTIONS_CLASSNAME;
use heck::{CamelCase, MixedCase};
use oo_bindgen::callback::*;
use oo_bindgen::class::*;
use oo_bindgen::collection::*;
use oo_bindgen::formatting::*;
use oo_bindgen::iterator::*;
use oo_bindgen::native_function::*;
use oo_bindgen::native_struct::*;

pub(crate) struct DotnetType<'a>(pub(crate) &'a Type);

impl<'a> DotnetType<'a> {
    /// Returns the .NET natural type
    pub(crate) fn as_dotnet_type(&self) -> String {
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
            Type::Float => "float".to_string(),
            Type::Double => "double".to_string(),
            Type::String => "string".to_string(),
            Type::Struct(handle) => handle.name().to_camel_case(),
            Type::StructRef(handle) => format!("{}?", handle.name.to_camel_case()),
            Type::Enum(handle) => handle.name.to_camel_case(),
            Type::ClassRef(handle) => handle.name.to_camel_case(),
            Type::Interface(handle) => format!("I{}", handle.name.to_camel_case()),
            Type::OneTimeCallback(handle) => format!("I{}", handle.name.to_camel_case()),
            Type::Iterator(handle) => format!(
                "System.Collections.Generic.ICollection<{}>",
                handle.item_type.name().to_camel_case()
            ),
            Type::Collection(handle) => format!(
                "System.Collections.Generic.ICollection<{}>",
                DotnetType(&handle.item_type).as_dotnet_type()
            ),
            Type::Duration(_) => "TimeSpan".to_string(),
        }
    }

    /// Return the .NET representation of the native C type
    pub(crate) fn as_native_type(&self) -> String {
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
            Type::Float => "float".to_string(),
            Type::Double => "double".to_string(),
            Type::String => "IntPtr".to_string(),
            Type::Struct(handle) => format!("{}Native", handle.name().to_camel_case()),
            Type::StructRef(_) => "IntPtr".to_string(),
            Type::Enum(handle) => handle.name.to_camel_case(),
            Type::ClassRef(_) => "IntPtr".to_string(),
            Type::Interface(handle) => format!("I{}NativeAdapter", handle.name.to_camel_case()),
            Type::OneTimeCallback(handle) => {
                format!("I{}NativeAdapter", handle.name.to_camel_case())
            }
            Type::Iterator(_) => "IntPtr".to_string(),
            Type::Collection(_) => "IntPtr".to_string(),
            Type::Duration(mapping) => match mapping {
                DurationMapping::Milliseconds | DurationMapping::Seconds => "ulong".to_string(),
                DurationMapping::SecondsFloat => "float".to_string(),
            },
        }
    }

    pub(crate) fn conversion(&self) -> Option<Box<dyn TypeConverter>> {
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
            Type::Float => None,
            Type::Double => None,
            Type::String => Some(Box::new(StringConverter)),
            Type::Struct(handle) => Some(Box::new(StructConverter(handle.clone()))),
            Type::StructRef(handle) => Some(Box::new(StructRefConverter(handle.clone()))),
            Type::Enum(_) => None,
            Type::ClassRef(handle) => Some(Box::new(ClassConverter(handle.clone()))),
            Type::Interface(handle) => Some(Box::new(InterfaceConverter(handle.clone()))),
            Type::OneTimeCallback(handle) => {
                Some(Box::new(OneTimeCallbackConverter(handle.clone())))
            }
            Type::Iterator(handle) => Some(Box::new(IteratorConverter(handle.clone()))),
            Type::Collection(handle) => Some(Box::new(CollectionConverter(handle.clone()))),
            Type::Duration(mapping) => match mapping {
                DurationMapping::Milliseconds => Some(Box::new(DurationMillisecondsConverter)),
                DurationMapping::Seconds => Some(Box::new(DurationSecondsConverter)),
                DurationMapping::SecondsFloat => Some(Box::new(DurationSecondsFloatConverter)),
            },
        }
    }

    pub(crate) fn as_dotnet_arg(&self, param_name: &str) -> String {
        match self.0 {
            Type::Bool => format!("_{}", param_name.to_mixed_case()),
            Type::Uint8 => param_name.to_mixed_case(),
            Type::Sint8 => param_name.to_mixed_case(),
            Type::Uint16 => param_name.to_mixed_case(),
            Type::Sint16 => param_name.to_mixed_case(),
            Type::Uint32 => param_name.to_mixed_case(),
            Type::Sint32 => param_name.to_mixed_case(),
            Type::Uint64 => param_name.to_mixed_case(),
            Type::Sint64 => param_name.to_mixed_case(),
            Type::Float => param_name.to_mixed_case(),
            Type::Double => param_name.to_mixed_case(),
            Type::String => format!("_{}", param_name.to_mixed_case()),
            Type::Struct(_) => format!("_{}", param_name.to_mixed_case()),
            Type::StructRef(_) => format!("_{}", param_name.to_mixed_case()),
            Type::Enum(_) => param_name.to_mixed_case(),
            Type::ClassRef(_) => format!("_{}", param_name.to_mixed_case()),
            Type::Interface(_) => format!("_{}", param_name.to_mixed_case()),
            Type::OneTimeCallback(_) => format!("_{}", param_name.to_mixed_case()),
            Type::Iterator(_) => format!("_{}", param_name.to_mixed_case()),
            Type::Collection(_) => format!("_{}", param_name.to_mixed_case()),
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

// By default, PInvoke transforms "bool" into a weird 4-bit value
struct BoolConverter;
impl TypeConverter for BoolConverter {
    fn convert_to_native(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{}Convert.ToByte({});", to, from))
    }

    fn convert_from_native(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        f.writeln(&format!("{}Convert.ToBoolean({});", to, from))
    }
}

struct StringConverter;
impl TypeConverter for StringConverter {
    fn convert_to_native(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        let bytes_name = format!("{}Bytes", from.replace(".", "_"));
        let handle_name = format!("{}Handle", from.replace(".", "_"));

        f.writeln(&format!(
            "var {} = System.Text.Encoding.UTF8.GetBytes({});",
            bytes_name, from
        ))?;
        f.writeln(&format!(
            "var {} = Marshal.AllocHGlobal({}.Length + 1);",
            handle_name, bytes_name
        ))?;
        f.writeln(&format!(
            "Marshal.Copy({}, 0, {}, {}.Length);",
            bytes_name, handle_name, bytes_name
        ))?;
        f.writeln(&format!(
            "Marshal.WriteByte({}, {}.Length, 0);",
            handle_name, bytes_name
        ))?;
        f.writeln(&format!("{}{};", to, handle_name))
    }

    fn convert_to_native_cleanup(&self, f: &mut dyn Printer, name: &str) -> FormattingResult<()> {
        f.writeln(&format!("Marshal.FreeHGlobal({});", name))
    }

    fn convert_from_native(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        let len_name = format!("{}Length", from.replace(".", "_"));
        let buffer_name = format!("{}Buffer", from.replace(".", "_"));

        f.writeln(&format!("int {} = 0;", len_name))?;
        f.writeln(&format!(
            "while (Marshal.ReadByte({}, {}) != 0) ++{};",
            from, len_name, len_name
        ))?;
        f.writeln(&format!("byte[] {} = new byte[{}];", buffer_name, len_name))?;
        f.writeln(&format!(
            "Marshal.Copy({}, {}, 0, {});",
            from, buffer_name, len_name
        ))?;
        f.writeln(&format!(
            "{}System.Text.Encoding.UTF8.GetString({});",
            to, buffer_name
        ))
    }
}

struct InterfaceConverter(InterfaceHandle);
impl TypeConverter for InterfaceConverter {
    fn convert_to_native(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!(
            "{}new I{}NativeAdapter({});",
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
            "if ({}.{} != IntPtr.Zero)",
            from,
            self.0.arg_name.to_mixed_case()
        ))?;
        blocked(f, |f| {
            f.writeln(&format!(
                "var _handle = GCHandle.FromIntPtr({}.{});",
                from,
                self.0.arg_name.to_mixed_case()
            ))?;
            f.writeln(&format!(
                "{}_handle.Target as I{};",
                to,
                self.0.name.to_camel_case()
            ))
        })?;
        f.writeln("else")?;
        blocked(f, |f| f.writeln(&format!("{}null;", to)))
    }
}

struct OneTimeCallbackConverter(OneTimeCallbackHandle);
impl TypeConverter for OneTimeCallbackConverter {
    fn convert_to_native(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!(
            "{}new I{}NativeAdapter({});",
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
            "if ({}.{} != IntPtr.Zero)",
            from,
            self.0.arg_name.to_mixed_case()
        ))?;
        blocked(f, |f| {
            f.writeln(&format!(
                "var _handle = GCHandle.FromIntPtr({}.{});",
                from,
                self.0.arg_name.to_mixed_case()
            ))?;
            f.writeln(&format!(
                "{}_handle.Target as I{};",
                to,
                self.0.name.to_camel_case()
            ))
        })?;
        f.writeln("else")?;
        blocked(f, |f| f.writeln(&format!("{}null;", to)))
    }
}

struct StructConverter(NativeStructHandle);
impl TypeConverter for StructConverter {
    fn convert_to_native(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!(
            "{}{}Native.ToNative({});",
            to,
            self.0.name().to_camel_case(),
            from
        ))
    }

    fn convert_to_native_cleanup(&self, f: &mut dyn Printer, name: &str) -> FormattingResult<()> {
        f.writeln(&format!("{}.Dispose();", name))
    }

    fn convert_from_native(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        f.writeln(&format!(
            "{}{}Native.FromNative({});",
            to,
            self.0.name().to_camel_case(),
            from
        ))
    }
}

struct StructRefConverter(NativeStructDeclarationHandle);
impl TypeConverter for StructRefConverter {
    fn convert_to_native(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        let handle_name = format!("_{}Handle", from);
        f.writeln(&format!("var {} = IntPtr.Zero;", handle_name))?;
        f.writeln(&format!("if ({} != null)", from))?;
        blocked(f, |f| {
            let struct_name = self.0.name.to_camel_case();
            let native_name = format!("_{}Native", from);
            f.writeln(&format!(
                "var {} = {}Native.ToNative(({}){});",
                native_name, struct_name, struct_name, from
            ))?;
            f.writeln(&format!(
                "{} = Marshal.AllocHGlobal(Marshal.SizeOf({}));",
                handle_name, native_name
            ))?;
            f.writeln(&format!(
                "Marshal.StructureToPtr({}, {}, false);",
                native_name, handle_name
            ))?;
            f.writeln(&format!("{}.Dispose();", native_name))
        })?;
        f.writeln(&format!("{}{};", to, handle_name))
    }

    fn convert_to_native_cleanup(&self, f: &mut dyn Printer, name: &str) -> FormattingResult<()> {
        f.writeln(&format!("Marshal.FreeHGlobal({});", name))
    }

    fn convert_from_native(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        let handle_name = format!("_{}Handle", from.replace(".", "_"));
        f.writeln(&format!(
            "{}? {} = null;",
            self.0.name.to_camel_case(),
            handle_name
        ))?;
        f.writeln(&format!("if ({} != IntPtr.Zero)", from))?;
        blocked(f, |f| {
            let native_name = format!("_{}Native", from);
            f.writeln(&format!(
                "var {} = Marshal.PtrToStructure<{}Native>({});",
                native_name,
                self.0.name.to_camel_case(),
                from
            ))?;
            f.writeln(&format!(
                "{} = {}Native.FromNative({});",
                handle_name,
                self.0.name.to_camel_case(),
                native_name
            ))
        })?;
        f.writeln(&format!("{}{};", to, handle_name))
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
        f.writeln(&format!("if ({} != IntPtr.Zero)", from))?;
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
        f.writeln(&format!("{}IntPtr.Zero;", to))
    }

    fn convert_from_native(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        let builder_name = format!("_{}Builder", from.replace(".", "_"));
        let next_call = format!(
            "{}.{}({})",
            NATIVE_FUNCTIONS_CLASSNAME, self.0.native_func.name, from
        );

        f.writeln(&format!(
            "var {} = ImmutableArray.CreateBuilder<{}>();",
            builder_name,
            self.0.item_type.name().to_camel_case()
        ))?;
        f.writeln(&format!(
            "for (var _itRawValue = {}; _itRawValue != IntPtr.Zero; _itRawValue = {})",
            next_call, next_call
        ))?;
        blocked(f, |f| {
            f.writeln(&format!(
                "{}? _itValue = null;",
                self.0.item_type.name().to_camel_case()
            ))?;
            StructRefConverter(self.0.item_type.declaration()).convert_from_native(
                f,
                "_itRawValue",
                "_itValue = ",
            )?;
            f.writeln(&format!("{}.Add(_itValue.Value);", builder_name))
        })?;
        f.writeln(&format!("{}{}.ToImmutable();", to, builder_name))
    }
}

struct CollectionConverter(CollectionHandle);
impl TypeConverter for CollectionConverter {
    fn convert_to_native(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        let builder_name = format!("_{}Builder", from.replace(".", "_"));

        if self.0.has_reserve {
            f.writeln(&format!(
                "var {} = {}.{}((uint){}.Count);",
                builder_name, NATIVE_FUNCTIONS_CLASSNAME, self.0.create_func.name, from
            ))?;
        } else {
            f.writeln(&format!(
                "var {} = {}.{}();",
                builder_name, NATIVE_FUNCTIONS_CLASSNAME, self.0.create_func.name
            ))?;
        }
        f.writeln(&format!("foreach (var __value in {})", from))?;
        blocked(f, |f| {
            let dotnet_type = DotnetType(&self.0.item_type);
            let converter = dotnet_type.conversion();
            let value_name = if let Some(converter) = &converter {
                converter.convert_to_native(f, "__value", "var ___value = ")?;
                "___value"
            } else {
                "__value"
            };

            f.writeln(&format!(
                "{}.{}({}, {});",
                NATIVE_FUNCTIONS_CLASSNAME, self.0.add_func.name, builder_name, value_name
            ))?;

            if let Some(converter) = &converter {
                converter.convert_to_native_cleanup(f, "___value")?;
            }

            Ok(())
        })?;
        f.writeln(&format!("{}{};", to, builder_name))
    }

    fn convert_to_native_cleanup(&self, f: &mut dyn Printer, name: &str) -> FormattingResult<()> {
        f.writeln(&format!(
            "{}.{}({});",
            NATIVE_FUNCTIONS_CLASSNAME, self.0.delete_func.name, name
        ))
    }

    fn convert_from_native(
        &self,
        f: &mut dyn Printer,
        _from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        f.writeln(&format!(
            "{}System.Collections.Immutable.ImmutableArray<{}>.Empty;",
            to,
            DotnetType(&self.0.item_type).as_dotnet_type()
        ))
    }
}

struct DurationMillisecondsConverter;
impl TypeConverter for DurationMillisecondsConverter {
    fn convert_to_native(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{}(ulong){}.TotalMilliseconds;", to, from))
    }

    fn convert_from_native(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        f.writeln(&format!("{}TimeSpan.FromMilliseconds({});", to, from))
    }
}

struct DurationSecondsConverter;
impl TypeConverter for DurationSecondsConverter {
    fn convert_to_native(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{}(ulong){}.TotalSeconds;", to, from))
    }

    fn convert_from_native(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        f.writeln(&format!("{}TimeSpan.FromSeconds({});", to, from))
    }
}

struct DurationSecondsFloatConverter;
impl TypeConverter for DurationSecondsFloatConverter {
    fn convert_to_native(&self, f: &mut dyn Printer, from: &str, to: &str) -> FormattingResult<()> {
        f.writeln(&format!("{}(float){}.TotalSeconds;", to, from))
    }

    fn convert_from_native(
        &self,
        f: &mut dyn Printer,
        from: &str,
        to: &str,
    ) -> FormattingResult<()> {
        f.writeln(&format!("{}TimeSpan.FromSeconds({});", to, from))
    }
}

pub(crate) struct DotnetReturnType<'a>(pub(crate) &'a ReturnType);

impl<'a> DotnetReturnType<'a> {
    pub(crate) fn as_dotnet_type(&self) -> String {
        match self.0 {
            ReturnType::Void => "void".to_string(),
            ReturnType::Type(return_type, _) => DotnetType(return_type).as_dotnet_type(),
        }
    }

    pub(crate) fn as_native_type(&self) -> String {
        match self.0 {
            ReturnType::Void => "void".to_string(),
            ReturnType::Type(return_type, _) => DotnetType(return_type).as_native_type(),
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
        if let Some(converter) = DotnetType(&param.param_type).conversion() {
            let mut param_name = param.name.to_mixed_case();
            if idx == 0 {
                if let Some(first_param) = first_param_is_self.clone() {
                    param_name = first_param;
                }
            }
            converter.convert_to_native(
                f,
                &param_name,
                &format!("var _{} = ", param.name.to_mixed_case()),
            )?;
        }
    }

    // Call the native function
    f.newline()?;
    if !method.return_type.is_void() {
        f.write(&format!(
            "var _result = {}.{}(",
            NATIVE_FUNCTIONS_CLASSNAME, method.name
        ))?;
    } else {
        f.write(&format!("{}.{}(", NATIVE_FUNCTIONS_CLASSNAME, method.name))?;
    }

    f.write(
        &method
            .parameters
            .iter()
            .map(|param| DotnetType(&param.param_type).as_dotnet_arg(&param.name.to_mixed_case()))
            .collect::<Vec<String>>()
            .join(", "),
    )?;
    f.write(");")?;

    // Convert the result (if required)
    let return_name = if let ReturnType::Type(return_type, _) = &method.return_type {
        let mut return_name = "_result";
        if let Some(converter) = DotnetType(&return_type).conversion() {
            if !is_constructor {
                converter.convert_from_native(f, "_result", "var __result = ")?;
                return_name = "__result";
            }
        }

        return_name
    } else {
        ""
    };

    //Cleanup type conversions
    for param in method.parameters.iter() {
        if let Some(converter) = DotnetType(&param.param_type).conversion() {
            converter.convert_to_native_cleanup(f, &format!("_{}", param.name.to_mixed_case()))?;
        }
    }

    // Return (if required)
    if !method.return_type.is_void() {
        f.writeln(&format!("{}{};", return_destination, return_name))?;
    }

    Ok(())
}

pub(crate) fn call_dotnet_function(
    f: &mut dyn Printer,
    method: &CallbackFunction,
    return_destination: &str,
) -> FormattingResult<()> {
    // Write the type conversions
    for param in method.params() {
        if let Some(converter) = DotnetType(&param.param_type).conversion() {
            converter.convert_from_native(
                f,
                &param.name,
                &format!("var _{} = ", param.name.to_mixed_case()),
            )?;
        }
    }

    // Call the .NET function
    f.newline()?;
    let method_name = method.name.to_camel_case();
    if let ReturnType::Type(return_type, _) = &method.return_type {
        if DotnetType(&return_type).conversion().is_some() {
            f.write(&format!("var _result = _impl.{}(", method_name))?;
        } else {
            f.write(&format!("{}_impl.{}(", return_destination, method_name))?;
        }
    } else {
        f.write(&format!("_impl.{}(", method_name))?;
    }

    f.write(
        &method
            .params()
            .map(|param| DotnetType(&param.param_type).as_dotnet_arg(&param.name))
            .collect::<Vec<String>>()
            .join(", "),
    )?;
    f.write(");")?;

    // Convert the result (if required)
    if let ReturnType::Type(return_type, _) = &method.return_type {
        if let Some(converter) = DotnetType(&return_type).conversion() {
            converter.convert_to_native(f, "_result", return_destination)?;
        }
    }

    Ok(())
}
