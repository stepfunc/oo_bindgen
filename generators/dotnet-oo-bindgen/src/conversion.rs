use crate::formatting::blocked;
use crate::NATIVE_FUNCTIONS_CLASSNAME;
use heck::{CamelCase, MixedCase};
use oo_bindgen::formatting::*;
use oo_bindgen::function::*;
use oo_bindgen::interface::*;
use oo_bindgen::types::{AnyType, BasicType, DurationType};

pub(crate) trait DotnetType {
    /// Returns the .NET natural type
    fn as_dotnet_type(&self) -> String;
    /// Return the .NET representation of the native C type
    fn as_native_type(&self) -> String;
    fn convert_to_native(&self, from: &str) -> Option<String>;
    fn cleanup(&self, from: &str) -> Option<String>;
    fn convert_from_native(&self, from: &str) -> Option<String>;
}

impl DotnetType for BasicType {
    fn as_dotnet_type(&self) -> String {
        match self {
            Self::Bool => "bool".to_string(),
            Self::Uint8 => "byte".to_string(),
            Self::Sint8 => "sbyte".to_string(),
            Self::Uint16 => "ushort".to_string(),
            Self::Sint16 => "short".to_string(),
            Self::Uint32 => "uint".to_string(),
            Self::Sint32 => "int".to_string(),
            Self::Uint64 => "ulong".to_string(),
            Self::Sint64 => "long".to_string(),
            Self::Float => "float".to_string(),
            Self::Double => "double".to_string(),
            Self::Duration(_) => "TimeSpan".to_string(),
            Self::Enum(handle) => handle.name.to_camel_case(),
        }
    }

    fn as_native_type(&self) -> String {
        match self {
            Self::Bool => "byte".to_string(),
            Self::Uint8 => "byte".to_string(),
            Self::Sint8 => "sbyte".to_string(),
            Self::Uint16 => "ushort".to_string(),
            Self::Sint16 => "short".to_string(),
            Self::Uint32 => "uint".to_string(),
            Self::Sint32 => "int".to_string(),
            Self::Uint64 => "ulong".to_string(),
            Self::Sint64 => "long".to_string(),
            Self::Float => "float".to_string(),
            Self::Double => "double".to_string(),
            Self::Duration(_) => "ulong".to_string(),
            Self::Enum(handle) => handle.name.to_camel_case(),
        }
    }

    fn convert_to_native(&self, from: &str) -> Option<String> {
        match self {
            Self::Bool => Some(format!("Convert.ToByte({})", from)),
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
            Self::Duration(mapping) => match mapping {
                DurationType::Milliseconds => Some(format!("(ulong){}.TotalMilliseconds", from)),
                DurationType::Seconds => Some(format!("(ulong){}.TotalSeconds", from)),
            },
            Self::Enum(_) => None,
        }
    }

    fn cleanup(&self, _: &str) -> Option<String> {
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
            Self::Duration(_) => None,
            Self::Enum(_) => None,
        }
    }

    fn convert_from_native(&self, from: &str) -> Option<String> {
        match self {
            Self::Bool => Some(format!("Convert.ToBoolean({})", from)),
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
            Self::Duration(mapping) => match mapping {
                DurationType::Milliseconds => Some(format!("TimeSpan.FromMilliseconds({})", from)),
                DurationType::Seconds => Some(format!("TimeSpan.FromSeconds({})", from)),
            },
            Self::Enum(_) => None,
        }
    }
}

impl DotnetType for AnyType {
    fn as_dotnet_type(&self) -> String {
        match self {
            Self::Basic(x) => x.as_dotnet_type(),
            Self::String => "string".to_string(),
            Self::Struct(handle) => handle.name().to_camel_case(),
            Self::StructRef(handle) => handle.name.to_camel_case(),
            Self::ClassRef(handle) => handle.name.to_camel_case(),
            Self::Interface(handle) => format!("I{}", handle.name.to_camel_case()),
            Self::Iterator(handle) => format!(
                "System.Collections.Generic.ICollection<{}>",
                handle.item_type.name().to_camel_case()
            ),
            Self::Collection(handle) => format!(
                "System.Collections.Generic.ICollection<{}>",
                handle.item_type.as_dotnet_type()
            ),
        }
    }

    fn as_native_type(&self) -> String {
        match self {
            Self::Basic(x) => x.as_native_type(),
            Self::String => "IntPtr".to_string(),
            Self::Struct(handle) => format!("{}Native", handle.name().to_camel_case()),
            Self::StructRef(_) => "IntPtr".to_string(),
            Self::ClassRef(_) => "IntPtr".to_string(),
            Self::Interface(handle) => format!("I{}NativeAdapter", handle.name.to_camel_case()),
            Self::Iterator(_) => "IntPtr".to_string(),
            Self::Collection(_) => "IntPtr".to_string(),
        }
    }

    fn convert_to_native(&self, from: &str) -> Option<String> {
        match self {
            Self::Basic(x) => x.convert_to_native(from),
            Self::String => Some(format!("Helpers.RustString.ToNative({})", from)),
            Self::Struct(handle) => Some(format!(
                "{}Native.ToNative({})",
                handle.name().to_camel_case(),
                from
            )),
            Self::StructRef(handle) => Some(format!(
                "{}Native.ToNativeRef({})",
                handle.name.to_camel_case(),
                from
            )),
            Self::ClassRef(_) => Some(format!("{}.self", from)),
            Self::Interface(handle) => Some(format!(
                "new I{}NativeAdapter({})",
                handle.name.to_camel_case(),
                from
            )),
            Self::Iterator(_) => Some("IntPtr.Zero".to_string()),
            Self::Collection(handle) => Some(format!(
                "{}Helpers.ToNative({})",
                handle.collection_type.name.to_camel_case(),
                from
            )),
        }
    }

    fn cleanup(&self, from: &str) -> Option<String> {
        match self {
            Self::Basic(t) => t.cleanup(from),
            Self::String => Some(format!("Helpers.RustString.Destroy({});", from)),
            Self::Struct(_) => Some(format!("{}.Dispose();", from)),
            Self::StructRef(handle) => Some(format!(
                "{}Native.NativeRefCleanup({});",
                handle.name.to_camel_case(),
                from
            )),
            Self::ClassRef(_) => None,
            Self::Interface(_) => None,
            Self::Iterator(_) => None,
            Self::Collection(handle) => Some(format!(
                "{}Helpers.Cleanup({});",
                handle.collection_type.name.to_camel_case(),
                from
            )),
        }
    }

    fn convert_from_native(&self, from: &str) -> Option<String> {
        match self {
            Self::Basic(x) => x.convert_from_native(from),
            Self::String => Some(format!("Helpers.RustString.FromNative({})", from)),
            Self::Struct(handle) => Some(format!(
                "{}Native.FromNative({})",
                handle.name().to_camel_case(),
                from
            )),
            Self::StructRef(handle) => Some(format!(
                "{}Native.FromNativeRef({})",
                handle.name.to_camel_case(),
                from
            )),
            Self::ClassRef(handle) => Some(format!(
                "{}.FromNative({})",
                handle.name.to_camel_case(),
                from
            )),
            Self::Interface(handle) => Some(format!(
                "I{}NativeAdapter.FromNative({}.{})",
                handle.name.to_camel_case(),
                from,
                CTX_VARIABLE_NAME.to_mixed_case()
            )),
            Self::Iterator(handle) => Some(format!(
                "{}Helpers.FromNative({})",
                handle.iter_type.name.to_camel_case(),
                from
            )),
            Self::Collection(handle) => Some(format!(
                "System.Collections.Immutable.ImmutableArray<{}>.Empty",
                handle.item_type.as_dotnet_type()
            )),
        }
    }
}

impl DotnetType for CArgument {
    fn as_dotnet_type(&self) -> String {
        AnyType::from(self.clone()).as_dotnet_type()
    }

    fn as_native_type(&self) -> String {
        AnyType::from(self.clone()).as_native_type()
    }

    fn convert_to_native(&self, from: &str) -> Option<String> {
        AnyType::from(self.clone()).convert_to_native(from)
    }

    fn cleanup(&self, from: &str) -> Option<String> {
        AnyType::from(self.clone()).cleanup(from)
    }

    fn convert_from_native(&self, from: &str) -> Option<String> {
        AnyType::from(self.clone()).convert_from_native(from)
    }
}

impl DotnetType for ReturnType {
    fn as_dotnet_type(&self) -> String {
        AnyType::from(self.clone()).as_dotnet_type()
    }

    fn as_native_type(&self) -> String {
        AnyType::from(self.clone()).as_native_type()
    }

    fn convert_to_native(&self, from: &str) -> Option<String> {
        AnyType::from(self.clone()).convert_to_native(from)
    }

    fn cleanup(&self, from: &str) -> Option<String> {
        AnyType::from(self.clone()).cleanup(from)
    }

    fn convert_from_native(&self, from: &str) -> Option<String> {
        AnyType::from(self.clone()).convert_from_native(from)
    }
}

impl DotnetType for ReturnTypeInfo {
    fn as_dotnet_type(&self) -> String {
        match self {
            ReturnTypeInfo::Void => "void".to_string(),
            ReturnTypeInfo::Type(return_type, _) => return_type.as_dotnet_type(),
        }
    }

    fn as_native_type(&self) -> String {
        match self {
            ReturnTypeInfo::Void => "void".to_string(),
            ReturnTypeInfo::Type(return_type, _) => return_type.as_native_type(),
        }
    }

    fn convert_to_native(&self, from: &str) -> Option<String> {
        match self {
            ReturnTypeInfo::Void => None,
            ReturnTypeInfo::Type(return_type, _) => return_type.convert_to_native(from),
        }
    }

    fn cleanup(&self, from: &str) -> Option<String> {
        match self {
            ReturnTypeInfo::Void => None,
            ReturnTypeInfo::Type(return_type, _) => return_type.cleanup(from),
        }
    }

    fn convert_from_native(&self, from: &str) -> Option<String> {
        match self {
            ReturnTypeInfo::Void => None,
            ReturnTypeInfo::Type(return_type, _) => return_type.convert_from_native(from),
        }
    }
}

pub(crate) fn call_native_function(
    f: &mut dyn Printer,
    method: &Function,
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

        let conversion = AnyType::from(param.arg_type.clone())
            .convert_to_native(&param_name)
            .unwrap_or(param_name);
        f.writeln(&format!(
            "var _{} = {};",
            param.name.to_mixed_case(),
            conversion
        ))?;
    }

    let call_native_function = move |f: &mut dyn Printer| -> FormattingResult<()> {
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
                .map(|param| format!("_{}", param.name.to_mixed_case()))
                .collect::<Vec<String>>()
                .join(", "),
        )?;
        f.write(");")?;

        // Convert the result (if required)
        let return_name = if let ReturnTypeInfo::Type(return_type, _) = &method.return_type {
            let mut return_name = "_result";
            if let Some(conversion) = return_type.convert_from_native("_result") {
                if !is_constructor {
                    f.writeln(&format!("var __result = {};", conversion))?;
                    return_name = "__result";
                }
            }

            return_name
        } else {
            ""
        };

        // Return (if required)
        if !method.return_type.is_void() {
            f.writeln(&format!("{}{};", return_destination, return_name))?;
        }

        Ok(())
    };

    let has_cleanup = method.parameters.iter().any(|param| {
        AnyType::from(param.arg_type.clone())
            .cleanup("temp")
            .is_some()
    });

    if has_cleanup {
        f.writeln("try")?;
        blocked(f, call_native_function)?;
        f.writeln("finally")?;
        blocked(f, |f| {
            // Cleanup type conversions
            for param in method.parameters.iter() {
                if let Some(cleanup) = AnyType::from(param.arg_type.clone())
                    .cleanup(&format!("_{}", param.name.to_mixed_case()))
                {
                    f.writeln(&cleanup)?;
                }
            }
            Ok(())
        })?;
    } else {
        call_native_function(f)?;
    }

    Ok(())
}

pub(crate) fn call_dotnet_function(
    f: &mut dyn Printer,
    method: &CallbackFunction,
    return_destination: &str,
) -> FormattingResult<()> {
    // Write the type conversions
    for arg in method.arguments.iter() {
        let conversion = arg
            .arg_type
            .convert_from_native(&arg.name.to_mixed_case())
            .unwrap_or_else(|| arg.name.to_mixed_case());
        f.writeln(&format!(
            "var _{} = {};",
            arg.name.to_mixed_case(),
            conversion
        ))?;
    }

    // Call the .NET function
    f.newline()?;
    let method_name = method.name.to_camel_case();
    if let ReturnTypeInfo::Type(return_type, _) = &method.return_type {
        if return_type.convert_to_native("_result").is_some() {
            f.write(&format!("var _result = _impl.{}(", method_name))?;
        } else {
            f.write(&format!("{}_impl.{}(", return_destination, method_name))?;
        }
    } else {
        f.write(&format!("_impl.{}(", method_name))?;
    }

    f.write(
        &method
            .arguments
            .iter()
            .map(|arg| format!("_{}", arg.name.to_mixed_case()))
            .collect::<Vec<String>>()
            .join(", "),
    )?;
    f.write(");")?;

    // Convert the result (if required)
    if let ReturnTypeInfo::Type(return_type, _) = &method.return_type {
        if let Some(conversion) = return_type.convert_to_native("_result") {
            f.writeln(&format!("{}{};", return_destination, conversion))?;
        }
    }

    Ok(())
}
