use crate::*;
use heck::{CamelCase, MixedCase};
use oo_bindgen::native_struct::*;

fn field_visibility(struct_type: NativeStructType) -> &'static str {
    match struct_type {
        NativeStructType::Opaque => "internal",
        NativeStructType::Public => "public",
    }
}

fn constructor_visibility(struct_type: NativeStructType) -> &'static str {
    match struct_type {
        NativeStructType::Opaque => "internal",
        NativeStructType::Public => "public",
    }
}

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

    let doc = match native_struct.definition.struct_type {
        NativeStructType::Public => native_struct.doc().clone(),
        NativeStructType::Opaque => native_struct
            .doc()
            .clone()
            .warning("This class is an opaque handle and cannot be constructed by user code"),
    };

    namespaced(f, &lib.name, |f| {
        documentation(f, |f| {
            // Print top-level documentation
            xmldoc_print(f, &doc, lib)
        })?;

        f.writeln(&format!("public class {}", struct_name))?;
        blocked(f, |f| {
            // Write .NET structure elements
            for el in native_struct.elements() {
                documentation(f, |f| {
                    // Print top-level documentation
                    xmldoc_print(f, &el.doc, lib)?;

                    let default_value = match &el.element_type {
                        StructElementType::Bool(default) => default.map(|x| x.to_string()),
                        StructElementType::Uint8(default) => default.map(|x| x.to_string()),
                        StructElementType::Sint8(default) => default.map(|x| x.to_string()),
                        StructElementType::Uint16(default) => default.map(|x| x.to_string()),
                        StructElementType::Sint16(default) => default.map(|x| x.to_string()),
                        StructElementType::Uint32(default) => default.map(|x| x.to_string()),
                        StructElementType::Sint32(default) => default.map(|x| x.to_string()),
                        StructElementType::Uint64(default) => default.map(|x| x.to_string()),
                        StructElementType::Sint64(default) => default.map(|x| x.to_string()),
                        StructElementType::Float(default) => default.map(|x| x.to_string()),
                        StructElementType::Double(default) => default.map(|x| x.to_string()),
                        StructElementType::String(default) => {
                            default.clone().map(|x| format!("\"{}\"", x))
                        }
                        StructElementType::Struct(_) => None,
                        StructElementType::StructRef(_) => None,
                        StructElementType::Enum(handle, default) => default.clone().map(|x| {
                            format!(
                                "<see cref=\"{}.{}\" />",
                                handle.name.to_camel_case(),
                                x.to_camel_case()
                            )
                        }),
                        StructElementType::ClassRef(_) => None,
                        StructElementType::Interface(_) => None,
                        StructElementType::Iterator(_) => None,
                        StructElementType::Collection(_) => None,
                        StructElementType::Duration(_, default) => {
                            default.map(|x| format!("{}s", x.as_secs_f32()))
                        }
                    };

                    if let Some(default_value) = default_value {
                        f.writeln(&format!(
                            "<value>Default value is {}</value>",
                            default_value
                        ))?;
                    }

                    Ok(())
                })?;

                f.writeln(&format!(
                    "{} {} {}",
                    field_visibility(native_struct.definition.struct_type),
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
                            match handle.find_variant_by_name(value) {
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
                f.writeln(&format!(
                    "{} {}(",
                    constructor_visibility(native_struct.definition.struct_type),
                    struct_name
                ))?;
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

                    // Print exception
                    if let Some(error) = &method.native_function.error_type {
                        f.writeln(&format!(
                            "<exception cref=\"{}\" />",
                            error.exception_name.to_camel_case()
                        ))?;
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

                    // Print exception
                    if let Some(error) = &method.native_function.error_type {
                        f.writeln(&format!(
                            "<exception cref=\"{}\" />",
                            error.exception_name.to_camel_case()
                        ))?;
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

                    let conversion = el
                        .element_type
                        .to_type()
                        .convert_to_native(&format!("self.{}", el_name))
                        .unwrap_or(format!("self.{}", el_name));
                    f.writeln(&format!("result.{} = {};", el_name, conversion))?;
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

                    let conversion = el
                        .element_type
                        .to_type()
                        .convert_from_native(&format!("native.{}", el_name))
                        .unwrap_or(format!("native.{}", el_name));
                    f.writeln(&format!("result.{} = {};", el_name, conversion))?;
                }
                f.writeln("return result;")
            })?;

            f.newline()?;

            // Convert from .NET to native reference
            f.writeln(&format!(
                "internal static IntPtr ToNativeRef({} self)",
                struct_name
            ))?;
            blocked(f, |f| {
                f.writeln("var handle = IntPtr.Zero;")?;
                f.writeln("if (self != null)")?;
                blocked(f, |f| {
                    f.writeln("var nativeStruct = ToNative(self);")?;
                    f.writeln("handle = Marshal.AllocHGlobal(Marshal.SizeOf(nativeStruct));")?;
                    f.writeln("Marshal.StructureToPtr(nativeStruct, handle, false);")?;
                    f.writeln("nativeStruct.Dispose();")
                })?;
                f.writeln("return handle;")
            })?;

            f.newline()?;

            // Ref cleanup
            f.writeln("internal static void NativeRefCleanup(IntPtr native)")?;
            blocked(f, |f| {
                f.writeln("if (native != IntPtr.Zero)")?;
                blocked(f, |f| f.writeln("Marshal.FreeHGlobal(native);"))
            })?;

            f.newline()?;

            // Convert from native ref to .NET
            f.writeln(&format!(
                "internal static {} FromNativeRef(IntPtr native)",
                struct_name
            ))?;
            blocked(f, |f| {
                f.writeln(&format!("{} handle = null;", struct_name))?;
                f.writeln("if (native != IntPtr.Zero)")?;
                blocked(f, |f| {
                    f.writeln(&format!(
                        "var nativeStruct = Marshal.PtrToStructure<{}>(native);",
                        struct_native_name
                    ))?;
                    f.writeln("handle = FromNative(nativeStruct);")
                })?;
                f.writeln("return handle;")
            })?;

            f.newline()?;

            // Finalizer
            f.writeln("internal void Dispose()")?;
            blocked(f, |f| {
                for el in native_struct.elements() {
                    let el_name = el.name.to_camel_case();

                    if let Some(cleanup) = el
                        .element_type
                        .to_type()
                        .cleanup(&format!("this.{}", el_name))
                    {
                        f.writeln(&cleanup)?;
                    }
                }
                Ok(())
            })
        })
    })
}
