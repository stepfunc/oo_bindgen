use crate::formatting::blocked;
use crate::NATIVE_FUNCTIONS_CLASSNAME;
use heck::{CamelCase, MixedCase};
use oo_bindgen::callback::*;
use oo_bindgen::formatting::*;
use oo_bindgen::native_function::*;

pub(crate) trait DotnetType {
    fn as_dotnet_type(&self) -> String;
    fn as_native_type(&self) -> String;
    fn convert_to_native(&self, from: &str) -> Option<String>;
    fn cleanup(&self, from: &str) -> Option<String>;
    fn convert_from_native(&self, from: &str) -> Option<String>;
}

impl DotnetType for Type {
    /// Returns the .NET natural type
    fn as_dotnet_type(&self) -> String {
        match self {
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
            Type::StructRef(handle) => handle.name.to_camel_case(),
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
                handle.item_type.as_dotnet_type()
            ),
            Type::Duration(_) => "TimeSpan".to_string(),
        }
    }

    /// Return the .NET representation of the native C type
    fn as_native_type(&self) -> String {
        match self {
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

    fn convert_to_native(&self, from: &str) -> Option<String> {
        match self {
            Type::Bool => Some(format!("Convert.ToByte({})", from)),
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
            Type::String => Some(format!("Helpers.RustString.ToNative({})", from)),
            Type::Struct(handle) => Some(format!(
                "{}Native.ToNative({})",
                handle.name().to_camel_case(),
                from
            )),
            Type::StructRef(handle) => Some(format!(
                "{}Native.ToNativeRef({})",
                handle.name.to_camel_case(),
                from
            )),
            Type::Enum(_) => None,
            Type::ClassRef(_) => Some(format!("{}.self", from)),
            Type::Interface(handle) => Some(format!(
                "new I{}NativeAdapter({})",
                handle.name.to_camel_case(),
                from
            )),
            Type::OneTimeCallback(handle) => Some(format!(
                "new I{}NativeAdapter({})",
                handle.name.to_camel_case(),
                from
            )),
            Type::Iterator(_) => Some("IntPtr.Zero".to_string()),
            Type::Collection(handle) => Some(format!(
                "{}Helpers.ToNative({})",
                handle.collection_type.name.to_camel_case(),
                from
            )),
            Type::Duration(mapping) => match mapping {
                DurationMapping::Milliseconds => Some(format!("(ulong){}.TotalMilliseconds", from)),
                DurationMapping::Seconds => Some(format!("(ulong){}.TotalSeconds", from)),
                DurationMapping::SecondsFloat => Some(format!("(float){}.TotalSeconds", from)),
            },
        }
    }

    fn cleanup(&self, from: &str) -> Option<String> {
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
            Type::String => Some(format!("Helpers.RustString.Destroy({});", from)),
            Type::Struct(_) => Some(format!("{}.Dispose();", from)),
            Type::StructRef(handle) => Some(format!(
                "{}Native.NativeRefCleanup({});",
                handle.name.to_camel_case(),
                from
            )),
            Type::Enum(_) => None,
            Type::ClassRef(_) => None,
            Type::Interface(_) => None,
            Type::OneTimeCallback(_) => None,
            Type::Iterator(_) => None,
            Type::Collection(handle) => Some(format!(
                "{}Helpers.Cleanup({});",
                handle.collection_type.name.to_camel_case(),
                from
            )),
            Type::Duration(_) => None,
        }
    }

    fn convert_from_native(&self, from: &str) -> Option<String> {
        match self {
            Type::Bool => Some(format!("Convert.ToBoolean({})", from)),
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
            Type::String => Some(format!("Helpers.RustString.FromNative({})", from)),
            Type::Struct(handle) => Some(format!(
                "{}Native.FromNative({})",
                handle.name().to_camel_case(),
                from
            )),
            Type::StructRef(handle) => Some(format!(
                "{}Native.FromNativeRef({})",
                handle.name.to_camel_case(),
                from
            )),
            Type::Enum(_) => None,
            Type::ClassRef(handle) => Some(format!(
                "{}.FromNative({})",
                handle.name.to_camel_case(),
                from
            )),
            Type::Interface(handle) => Some(format!(
                "I{}NativeAdapter.FromNative({}.{})",
                handle.name.to_camel_case(),
                from,
                handle.arg_name.to_mixed_case()
            )),
            Type::OneTimeCallback(handle) => Some(format!(
                "I{}NativeAdapter.FromNative({}.{})",
                handle.name.to_camel_case(),
                from,
                handle.arg_name.to_mixed_case()
            )),
            Type::Iterator(handle) => Some(format!(
                "{}Helpers.FromNative({})",
                handle.iter_type.name.to_camel_case(),
                from
            )),
            Type::Collection(handle) => Some(format!(
                "System.Collections.Immutable.ImmutableArray<{}>.Empty",
                handle.item_type.as_dotnet_type()
            )),
            Type::Duration(mapping) => match mapping {
                DurationMapping::Milliseconds => {
                    Some(format!("TimeSpan.FromMilliseconds({})", from))
                }
                DurationMapping::Seconds => Some(format!("TimeSpan.FromSeconds({})", from)),
                DurationMapping::SecondsFloat => Some(format!("TimeSpan.FromSeconds({})", from)),
            },
        }
    }
}

impl DotnetType for ReturnType {
    fn as_dotnet_type(&self) -> String {
        match self {
            ReturnType::Void => "void".to_string(),
            ReturnType::Type(return_type, _) => return_type.as_dotnet_type(),
        }
    }

    fn as_native_type(&self) -> String {
        match self {
            ReturnType::Void => "void".to_string(),
            ReturnType::Type(return_type, _) => return_type.as_native_type(),
        }
    }

    fn convert_to_native(&self, from: &str) -> Option<String> {
        match self {
            ReturnType::Void => None,
            ReturnType::Type(return_type, _) => return_type.convert_to_native(from),
        }
    }

    fn cleanup(&self, from: &str) -> Option<String> {
        match self {
            ReturnType::Void => None,
            ReturnType::Type(return_type, _) => return_type.cleanup(from),
        }
    }

    fn convert_from_native(&self, from: &str) -> Option<String> {
        match self {
            ReturnType::Void => None,
            ReturnType::Type(return_type, _) => return_type.convert_from_native(from),
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

        let conversion = param
            .param_type
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
        let return_name = if let ReturnType::Type(return_type, _) = &method.return_type {
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

    let has_cleanup = method
        .parameters
        .iter()
        .any(|param| param.param_type.cleanup("temp").is_some());

    if has_cleanup {
        f.writeln("try")?;
        blocked(f, call_native_function)?;
        f.writeln("finally")?;
        blocked(f, |f| {
            // Cleanup type conversions
            for param in method.parameters.iter() {
                if let Some(cleanup) = param
                    .param_type
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
    for param in method.params() {
        let conversion = param
            .param_type
            .convert_from_native(&param.name.to_mixed_case())
            .unwrap_or_else(|| param.name.to_mixed_case());
        f.writeln(&format!(
            "var _{} = {};",
            param.name.to_mixed_case(),
            conversion
        ))?;
    }

    // Call the .NET function
    f.newline()?;
    let method_name = method.name.to_camel_case();
    if let ReturnType::Type(return_type, _) = &method.return_type {
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
            .params()
            .map(|param| format!("_{}", param.name.to_mixed_case()))
            .collect::<Vec<String>>()
            .join(", "),
    )?;
    f.write(");")?;

    // Convert the result (if required)
    if let ReturnType::Type(return_type, _) = &method.return_type {
        if let Some(conversion) = return_type.convert_to_native("_result") {
            f.writeln(&format!("{}{};", return_destination, conversion))?;
        }
    }

    Ok(())
}

pub(crate) fn generate_functional_callback(
    f: &mut dyn Printer,
    interface_name: &str,
    class_name: &str,
    function: &CallbackFunction,
) -> FormattingResult<()> {
    // Build the Action<>/Func<> signature
    let param_types = function
        .params()
        .map(|param| param.param_type.as_dotnet_type())
        .collect::<Vec<_>>()
        .join(", ");
    let action_type = match &function.return_type {
        ReturnType::Type(return_type, _) => {
            if param_types.is_empty() {
                format!("Func<{}>", return_type.as_dotnet_type())
            } else {
                format!("Func<{}, {}>", param_types, return_type.as_dotnet_type())
            }
        }
        ReturnType::Void => {
            if param_types.is_empty() {
                "Action".to_string()
            } else {
                format!("Action<{}>", param_types)
            }
        }
    };

    f.writeln(&format!("public class {} : {}", class_name, interface_name))?;
    blocked(f, |f| {
        f.writeln(&format!("readonly {} action;", action_type))?;

        f.newline()?;

        // Write the constructor
        f.writeln(&format!("public {}({} action)", class_name, action_type))?;
        blocked(f, |f| f.writeln("this.action = action;"))?;

        f.newline()?;

        // Write the required method
        f.writeln(&format!(
            "public {} {}(",
            function.return_type.as_dotnet_type(),
            function.name.to_camel_case()
        ))?;
        f.write(
            &function
                .params()
                .map(|param| {
                    format!(
                        "{} {}",
                        param.param_type.as_dotnet_type(),
                        param.name.to_mixed_case()
                    )
                })
                .collect::<Vec<_>>()
                .join(", "),
        )?;
        f.write(")")?;
        blocked(f, |f| {
            f.newline()?;

            if !function.return_type.is_void() {
                f.write("return ")?;
            }

            let params = function
                .params()
                .map(|param| param.name.to_mixed_case())
                .collect::<Vec<_>>()
                .join(", ");

            f.write(&format!("this.action.Invoke({});", params))
        })
    })
}
