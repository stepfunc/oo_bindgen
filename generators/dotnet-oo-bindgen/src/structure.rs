use crate::*;
use heck::{CamelCase, MixedCase};
use oo_bindgen::native_struct::*;

pub(crate) fn generate(
    f: &mut impl Printer,
    native_struct: &StructHandle,
    lib: &Library,
) -> FormattingResult<()> {
    let struct_name = native_struct.name().to_camel_case();
    let struct_native_name = format!("{}Native", struct_name);

    print_license(f, &lib.license)?;
    print_imports(f)?;
    f.newline()?;

    namespaced(f, &lib.name, |f| {
        documentation(f, |f| {
            // Print top-level documentation
            xmldoc_print(f, &native_struct.doc(), lib)
        })?;

        f.writeln(&format!("public class {}", struct_name))?;
        blocked(f, |f| {
            // Write .NET structure elements
            for el in native_struct.elements() {
                documentation(f, |f| {
                    // Print top-level documentation
                    xmldoc_print(f, &el.doc, lib)
                })?;

                f.writeln(&format!(
                    "public {} {}",
                    el.element_type.to_type().as_dotnet_type(),
                    el.name.to_camel_case()
                ))?;
                match &el.element_type {
                    StructElementType::Bool(default) => match default {
                        None => (),
                        Some(false) => f.write(" = false")?,
                        Some(true) => f.write(" = true")?,
                    },
                    StructElementType::Uint8(default) => {
                        if let Some(value) = default {
                            f.write(&format!(" = (byte){}", value))?;
                        }
                    }
                    StructElementType::Sint8(default) => {
                        if let Some(value) = default {
                            f.write(&format!(" = (sbyte){}", value))?;
                        }
                    }
                    StructElementType::Uint16(default) => {
                        if let Some(value) = default {
                            f.write(&format!(" = (ushort){}", value))?;
                        }
                    }
                    StructElementType::Sint16(default) => {
                        if let Some(value) = default {
                            f.write(&format!(" = (short){}", value))?;
                        }
                    }
                    StructElementType::Uint32(default) => {
                        if let Some(value) = default {
                            f.write(&format!(" = (uint){}", value))?;
                        }
                    }
                    StructElementType::Sint32(default) => {
                        if let Some(value) = default {
                            f.write(&format!(" = (int){}", value))?;
                        }
                    }
                    StructElementType::Uint64(default) => {
                        if let Some(value) = default {
                            f.write(&format!(" = (ulong){}", value))?;
                        }
                    }
                    StructElementType::Sint64(default) => {
                        if let Some(value) = default {
                            f.write(&format!(" = (long){}", value))?;
                        }
                    }
                    StructElementType::Float(default) => {
                        if let Some(value) = default {
                            f.write(&format!(" = {}f", value))?;
                        }
                    }
                    StructElementType::Double(default) => {
                        if let Some(value) = default {
                            f.write(&format!(" = {}", value))?;
                        }
                    }
                    StructElementType::String(default) => {
                        if let Some(value) = default {
                            f.write(&format!(" = \"{}\"", &value))?;
                        }
                    }
                    StructElementType::Struct(handle) => {
                        if handle.is_default_constructed() {
                            f.write(&format!(" = new {}()", handle.name().to_camel_case()))?;
                        }
                    }
                    StructElementType::StructRef(_) => (),
                    StructElementType::Enum(handle, default) => {
                        if let Some(value) = default {
                            match handle.find_variant_by_value(*value) {
                                Some(variant) => f.write(&format!(
                                    " = {}.{}",
                                    handle.name.to_camel_case(),
                                    variant.name.to_camel_case()
                                ))?,
                                None => panic!("Variant {} not found in {}", value, handle.name),
                            }
                        }
                    }
                    StructElementType::ClassRef(_) => (),
                    StructElementType::Interface(_) => (),
                    StructElementType::OneTimeCallback(_) => (),
                    StructElementType::Iterator(_) => (),
                    StructElementType::Collection(_) => (),
                    StructElementType::Duration(mapping, default) => {
                        if let Some(value) = default {
                            match mapping {
                                DurationMapping::Milliseconds => f.write(&format!(
                                    " = TimeSpan.FromMilliseconds({})",
                                    value.as_millis()
                                ))?,
                                DurationMapping::Seconds => f.write(&format!(
                                    " = TimeSpan.FromSeconds({})",
                                    value.as_secs()
                                ))?,
                                DurationMapping::SecondsFloat => f.write(&format!(
                                    " = TimeSpan.FromSeconds({}f)",
                                    value.as_secs_f32()
                                ))?,
                            }
                        }
                    }
                }

                f.write(";")?;
            }

            f.newline()?;

            // Write constructor
            if !native_struct.definition().is_default_constructed() {
                f.writeln(&format!("public {}(", struct_name,))?;
                f.write(
                    &native_struct
                        .elements()
                        .filter(|el| !el.element_type.has_default())
                        .map(|el| {
                            format!(
                                "{} {}",
                                el.element_type.to_type().as_dotnet_type(),
                                el.name.to_mixed_case()
                            )
                        })
                        .collect::<Vec<String>>()
                        .join(", "),
                )?;
                f.write(")")?;

                blocked(f, |f| {
                    for el in native_struct.elements() {
                        if !el.element_type.has_default() {
                            f.writeln(&format!(
                                "this.{} = {};",
                                el.name.to_camel_case(),
                                el.name.to_mixed_case()
                            ))?;
                        }
                    }
                    Ok(())
                })?;

                f.newline()?;

                // Internal parameterless constructor
                f.writeln(&format!("internal {}() {{ }}", struct_name))?;
                f.newline()?;
            }

            // Write methods
            for method in &native_struct.methods {
                documentation(f, |f| {
                    // Print top-level documentation
                    xmldoc_print(f, &method.native_function.doc, lib)?;
                    f.newline()?;

                    // Print each parameter value
                    for param in method.native_function.parameters.iter().skip(1) {
                        f.writeln(&format!("<param name=\"{}\">", param.name.to_mixed_case()))?;
                        docstring_print(f, &param.doc, lib)?;
                        f.write("</param>")?;
                    }

                    // Print return value
                    if let ReturnType::Type(_, doc) = &method.native_function.return_type {
                        f.writeln("<returns>")?;
                        docstring_print(f, doc, lib)?;
                        f.write("</returns>")?;
                    }
                    Ok(())
                })?;

                f.writeln(&format!(
                    "public {} {}(",
                    method.native_function.return_type.as_dotnet_type(),
                    method.name.to_camel_case()
                ))?;
                f.write(
                    &method
                        .native_function
                        .parameters
                        .iter()
                        .skip(1)
                        .map(|param| {
                            format!(
                                "{} {}",
                                param.param_type.as_dotnet_type(),
                                param.name.to_mixed_case()
                            )
                        })
                        .collect::<Vec<String>>()
                        .join(", "),
                )?;
                f.write(")")?;

                blocked(f, |f| {
                    call_native_function(
                        f,
                        &method.native_function,
                        "return ",
                        Some("this".to_string()),
                        false,
                    )
                })?;
            }

            f.newline()?;

            // Write static methods
            for method in &native_struct.static_methods {
                documentation(f, |f| {
                    // Print top-level documentation
                    xmldoc_print(f, &method.native_function.doc, lib)?;
                    f.newline()?;

                    // Print each parameter value
                    for param in method.native_function.parameters.iter().skip(1) {
                        f.writeln(&format!("<param name=\"{}\">", param.name.to_mixed_case()))?;
                        docstring_print(f, &param.doc, lib)?;
                        f.write("</param>")?;
                    }

                    // Print return value
                    if let ReturnType::Type(_, doc) = &method.native_function.return_type {
                        f.writeln("<returns>")?;
                        docstring_print(f, doc, lib)?;
                        f.write("</returns>")?;
                    }
                    Ok(())
                })?;

                f.writeln(&format!(
                    "public static {} {}(",
                    method.native_function.return_type.as_dotnet_type(),
                    method.name.to_camel_case()
                ))?;
                f.write(
                    &method
                        .native_function
                        .parameters
                        .iter()
                        .map(|param| {
                            format!(
                                "{} {}",
                                param.param_type.as_dotnet_type(),
                                param.name.to_mixed_case()
                            )
                        })
                        .collect::<Vec<String>>()
                        .join(", "),
                )?;
                f.write(")")?;

                blocked(f, |f| {
                    call_native_function(f, &method.native_function, "return ", None, false)
                })?;
            }

            Ok(())
        })?;

        f.newline()?;

        // Write native struct
        f.writeln("[StructLayout(LayoutKind.Sequential)]")?;
        f.writeln(&format!("internal struct {}", struct_native_name))?;
        blocked(f, |f| {
            // Write native elements
            for el in native_struct.elements() {
                f.writeln(&format!(
                    "{} {};",
                    el.element_type.to_type().as_native_type(),
                    el.name.to_camel_case()
                ))?;
            }

            f.newline()?;

            // Convert from .NET to native
            f.writeln(&format!(
                "internal static {} ToNative({} self)",
                struct_native_name, struct_name
            ))?;
            blocked(f, |f| {
                f.writeln(&format!("{} result;", struct_native_name))?;
                for el in native_struct.elements() {
                    let el_name = el.name.to_camel_case();

                    if let Some(conversion) = el.element_type.to_type().conversion() {
                        conversion.convert_to_native(
                            f,
                            &format!("self.{}", el_name),
                            &format!("result.{} = ", el_name),
                        )?;
                    } else {
                        f.writeln(&format!("result.{} = self.{};", el_name, el_name))?;
                    }
                }
                f.writeln("return result;")
            })?;

            f.newline()?;

            // Convert from native to .NET
            f.writeln(&format!(
                "internal static {} FromNative({} native)",
                struct_name, struct_native_name
            ))?;
            blocked(f, |f| {
                f.writeln(&format!("{} result = new {}();", struct_name, struct_name))?;
                for el in native_struct.elements() {
                    let el_name = el.name.to_camel_case();

                    if let Some(conversion) = el.element_type.to_type().conversion() {
                        conversion.convert_from_native(
                            f,
                            &format!("native.{}", el_name),
                            &format!("result.{} = ", el_name),
                        )?;
                    } else {
                        f.writeln(&format!("result.{} = native.{};", el_name, el_name))?;
                    }
                }
                f.writeln("return result;")
            })?;

            f.newline()?;

            // Finalizer
            f.writeln("internal void Dispose()")?;
            blocked(f, |f| {
                for el in native_struct.elements() {
                    let el_name = el.name.to_camel_case();

                    if let Some(conversion) = el.element_type.to_type().conversion() {
                        conversion.convert_to_native_cleanup(f, &format!("this.{}", el_name))?;
                    }
                }
                Ok(())
            })
        })
    })
}
