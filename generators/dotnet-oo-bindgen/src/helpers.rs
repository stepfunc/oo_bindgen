use oo_bindgen::model::*;

use crate::conversion::TypeInfo;
use crate::*;

pub(crate) fn generate_collection_helpers(
    f: &mut dyn Printer,
    coll: &Handle<Collection<Validated>>,
    lib: &Library,
) -> FormattingResult<()> {
    print_license(f, &lib.info.license_description)?;
    print_imports(f)?;
    f.newline()?;

    namespaced(f, &lib.settings.name, |f| {
        f.writeln(&format!(
            "internal static class {}Helpers",
            coll.name().camel_case()
        ))?;
        blocked(f, |f| {
            // ToNative function
            f.writeln(&format!(
                "internal static IntPtr ToNative(System.Collections.Generic.ICollection<{}> value)",
                coll.item_type.get_dotnet_type()
            ))?;
            blocked(f, |f| {
                if coll.has_reserve {
                    f.writeln(&format!(
                        "var builder = {}.{}((uint)value.Count);",
                        NATIVE_FUNCTIONS_CLASSNAME,
                        coll.create_func.name.camel_case()
                    ))?;
                } else {
                    f.writeln(&format!(
                        "var builder = {}.{}();",
                        NATIVE_FUNCTIONS_CLASSNAME,
                        coll.create_func.name.camel_case()
                    ))?;
                }

                f.writeln("foreach (var el in value)")?;
                blocked(f, |f| {
                    let conversion = coll
                        .item_type
                        .convert_to_native("el")
                        .unwrap_or_else(|| "el".to_string());
                    f.writeln(&format!("var convertedEl = {};", conversion))?;

                    f.writeln(&format!(
                        "{}.{}(builder, convertedEl);",
                        NATIVE_FUNCTIONS_CLASSNAME,
                        coll.add_func.name.camel_case()
                    ))?;

                    if let Some(cleanup) = &coll.item_type.cleanup_native("convertedEl") {
                        f.writeln(cleanup)?;
                    }

                    Ok(())
                })?;

                f.writeln("return builder;")
            })?;

            // Cleanup function
            f.writeln("internal static void Cleanup(IntPtr value)")?;
            blocked(f, |f| {
                f.writeln(&format!(
                    "{}.{}(value);",
                    NATIVE_FUNCTIONS_CLASSNAME,
                    coll.delete_func.name.camel_case()
                ))
            })?;

            Ok(())
        })
    })
}

pub(crate) fn generate_iterator_helpers(
    f: &mut dyn Printer,
    iter: &Handle<AbstractIterator<Validated>>,
    lib: &Library,
) -> FormattingResult<()> {
    print_license(f, &lib.info.license_description)?;
    print_imports(f)?;
    f.newline()?;

    namespaced(f, &lib.settings.name, |f| {
        f.writeln(&format!(
            "internal static class {}Helpers",
            iter.name().camel_case()
        ))?;
        blocked(f, |f| {
            let value_type = match &iter.item_type {
                IteratorItemType::PrimitiveRef(x) => x.get_dotnet_type(),
                IteratorItemType::StructRef(x) => x.get_dotnet_type(),
            };

            // ToNative function
            f.writeln(&format!("internal static System.Collections.Generic.ICollection<{}> FromNative(IntPtr value)", value_type))?;
            blocked(f, |f| {
                let next_call = format!(
                    "{}.{}(value)",
                    NATIVE_FUNCTIONS_CLASSNAME,
                    iter.next_function.name.camel_case()
                );

                f.writeln(&format!(
                    "var builder = ImmutableArray.CreateBuilder<{}>();",
                    value_type
                ))?;
                f.writeln(&format!(
                    "for (var itRawValue = {}; itRawValue != IntPtr.Zero; itRawValue = {})",
                    next_call, next_call
                ))?;
                blocked(f, |f| {
                    f.writeln(&format!("{} itValue = null;", value_type))?;
                    // convert the value
                    match &iter.item_type {
                        IteratorItemType::PrimitiveRef(_x) => {
                            todo!()
                        }
                        IteratorItemType::StructRef(x) => {
                            f.writeln(&format!(
                                "itValue = {};",
                                x.declaration().convert_to_dotnet("itRawValue").unwrap()
                            ))?;
                        }
                    }
                    f.writeln("builder.Add(itValue);")
                })?;
                f.writeln("return builder.ToImmutable();")
            })
        })
    })
}

pub(crate) fn call_native_function(
    f: &mut dyn Printer,
    method: &Function<Validated>,
    return_destination: &str,
    first_param_is_self: Option<String>,
    is_constructor: bool,
) -> FormattingResult<()> {
    // Write the type conversions
    for (idx, param) in method.arguments.iter().enumerate() {
        let mut param_name = param.name.mixed_case();
        if idx == 0 {
            if let Some(first_param) = first_param_is_self.clone() {
                param_name = first_param;
            }
        }

        let conversion = param
            .arg_type
            .convert_to_native(&param_name)
            .unwrap_or(param_name);
        f.writeln(&format!(
            "var _{} = {};",
            param.name.mixed_case(),
            conversion
        ))?;
    }

    let call_native_function = move |f: &mut dyn Printer| -> FormattingResult<()> {
        // Call the native function
        f.newline()?;
        if !method.return_type.is_none() {
            f.write(&format!(
                "var _result = {}.{}(",
                NATIVE_FUNCTIONS_CLASSNAME,
                method.name.camel_case()
            ))?;
        } else {
            f.write(&format!(
                "{}.{}(",
                NATIVE_FUNCTIONS_CLASSNAME,
                method.name.camel_case()
            ))?;
        }

        f.write(
            &method
                .arguments
                .iter()
                .map(|param| format!("_{}", param.name.mixed_case()))
                .collect::<Vec<String>>()
                .join(", "),
        )?;
        f.write(");")?;

        // Convert the result (if required)
        let return_name = if let Some(return_type) = &method.return_type.get_value() {
            let mut return_name = "_result";
            if let Some(conversion) = return_type.convert_to_dotnet("_result") {
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
        if !method.return_type.is_none() {
            f.writeln(&format!("{}{};", return_destination, return_name))?;
        }

        Ok(())
    };

    let has_cleanup = method
        .arguments
        .iter()
        .any(|param| param.arg_type.cleanup_native("temp").is_some());

    if has_cleanup {
        f.writeln("try")?;
        blocked(f, call_native_function)?;
        f.writeln("finally")?;
        blocked(f, |f| {
            // Cleanup type conversions
            for param in method.arguments.iter() {
                if let Some(cleanup) = param
                    .arg_type
                    .cleanup_native(&format!("_{}", param.name.mixed_case()))
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
    method: &CallbackFunction<Validated>,
    return_destination: &str,
) -> FormattingResult<()> {
    // Write the type conversions
    for arg in method.arguments.iter() {
        let conversion = arg
            .arg_type
            .convert_to_dotnet(&arg.name.mixed_case())
            .unwrap_or_else(|| arg.name.mixed_case());
        f.writeln(&format!("var _{} = {};", arg.name.mixed_case(), conversion))?;
    }

    // Call the .NET function
    f.newline()?;
    let method_name = method.name.camel_case();
    if let Some(return_type) = &method.return_type.get_value() {
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
            .map(|arg| format!("_{}", arg.name.mixed_case()))
            .collect::<Vec<String>>()
            .join(", "),
    )?;
    f.write(");")?;

    // Convert the result (if required)
    if let Some(return_type) = &method.return_type.get_value() {
        if let Some(conversion) = return_type.convert_to_native("_result") {
            f.writeln(&format!("{}{};", return_destination, conversion))?;
        }
    }

    Ok(())
}
