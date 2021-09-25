use crate::*;
use heck::{CamelCase, MixedCase};
use oo_bindgen::native_struct::*;
use oo_bindgen::struct_common::Visibility;
use oo_bindgen::types::DurationType;

fn field_visibility(struct_type: Visibility) -> &'static str {
    match struct_type {
        Visibility::Private => "internal",
        Visibility::Public => "public",
    }
}

fn constructor_visibility(struct_type: Visibility) -> &'static str {
    match struct_type {
        Visibility::Private => "internal",
        Visibility::Public => "public",
    }
}

pub(crate) fn generate(
    f: &mut impl Printer,
    native_struct: &StructHandle,
    lib: &Library,
) -> FormattingResult<()> {
    let struct_name = native_struct.name().to_camel_case();
    let struct_native_name = format!("{}Native", struct_name);

    print_license(f, &lib.info.license_description)?;
    print_imports(f)?;
    f.newline()?;

    let doc = match native_struct.definition.visibility() {
        Visibility::Public => native_struct.doc().clone(),
        Visibility::Private => native_struct
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

                    let default_value = match &el.field_type {
                        AllStructFieldType::Bool(default) => default.map(|x| x.to_string()),
                        AllStructFieldType::Uint8(default) => default.map(|x| x.to_string()),
                        AllStructFieldType::Sint8(default) => default.map(|x| x.to_string()),
                        AllStructFieldType::Uint16(default) => default.map(|x| x.to_string()),
                        AllStructFieldType::Sint16(default) => default.map(|x| x.to_string()),
                        AllStructFieldType::Uint32(default) => default.map(|x| x.to_string()),
                        AllStructFieldType::Sint32(default) => default.map(|x| x.to_string()),
                        AllStructFieldType::Uint64(default) => default.map(|x| x.to_string()),
                        AllStructFieldType::Sint64(default) => default.map(|x| x.to_string()),
                        AllStructFieldType::Float(default) => default.map(|x| x.to_string()),
                        AllStructFieldType::Double(default) => default.map(|x| x.to_string()),
                        AllStructFieldType::String(default) => {
                            default.clone().map(|x| format!("\"{}\"", x))
                        }
                        AllStructFieldType::Struct(_) => None,
                        AllStructFieldType::StructRef(_) => None,
                        AllStructFieldType::Enum(handle, default) => default.clone().map(|x| {
                            format!(
                                "<see cref=\"{}.{}\" />",
                                handle.name.to_camel_case(),
                                x.to_camel_case()
                            )
                        }),
                        AllStructFieldType::ClassRef(_) => None,
                        AllStructFieldType::Interface(_) => None,
                        AllStructFieldType::Iterator(_) => None,
                        AllStructFieldType::Collection(_) => None,
                        AllStructFieldType::Duration(_, default) => {
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
                    field_visibility(native_struct.definition.visibility()),
                    el.field_type.to_all_types().as_dotnet_type(),
                    el.name.to_camel_case()
                ))?;
                match &el.field_type {
                    AllStructFieldType::Bool(default) => match default {
                        None => (),
                        Some(false) => f.write(" = false")?,
                        Some(true) => f.write(" = true")?,
                    },
                    AllStructFieldType::Uint8(default) => {
                        if let Some(value) = default {
                            f.write(&format!(" = (byte){}", value))?;
                        }
                    }
                    AllStructFieldType::Sint8(default) => {
                        if let Some(value) = default {
                            f.write(&format!(" = (sbyte){}", value))?;
                        }
                    }
                    AllStructFieldType::Uint16(default) => {
                        if let Some(value) = default {
                            f.write(&format!(" = (ushort){}", value))?;
                        }
                    }
                    AllStructFieldType::Sint16(default) => {
                        if let Some(value) = default {
                            f.write(&format!(" = (short){}", value))?;
                        }
                    }
                    AllStructFieldType::Uint32(default) => {
                        if let Some(value) = default {
                            f.write(&format!(" = (uint){}", value))?;
                        }
                    }
                    AllStructFieldType::Sint32(default) => {
                        if let Some(value) = default {
                            f.write(&format!(" = (int){}", value))?;
                        }
                    }
                    AllStructFieldType::Uint64(default) => {
                        if let Some(value) = default {
                            f.write(&format!(" = (ulong){}", value))?;
                        }
                    }
                    AllStructFieldType::Sint64(default) => {
                        if let Some(value) = default {
                            f.write(&format!(" = (long){}", value))?;
                        }
                    }
                    AllStructFieldType::Float(default) => {
                        if let Some(value) = default {
                            f.write(&format!(" = {}f", value))?;
                        }
                    }
                    AllStructFieldType::Double(default) => {
                        if let Some(value) = default {
                            f.write(&format!(" = {}", value))?;
                        }
                    }
                    AllStructFieldType::String(default) => {
                        if let Some(value) = default {
                            f.write(&format!(" = \"{}\"", &value))?;
                        }
                    }
                    AllStructFieldType::Struct(handle) => {
                        if handle.all_fields_have_defaults() {
                            f.write(&format!(" = new {}()", handle.name().to_camel_case()))?;
                        }
                    }
                    AllStructFieldType::StructRef(_) => (),
                    AllStructFieldType::Enum(handle, default) => {
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
                    AllStructFieldType::ClassRef(_) => (),
                    AllStructFieldType::Interface(_) => (),
                    AllStructFieldType::Iterator(_) => (),
                    AllStructFieldType::Collection(_) => (),
                    AllStructFieldType::Duration(mapping, default) => {
                        if let Some(value) = default {
                            match mapping {
                                DurationType::Milliseconds => f.write(&format!(
                                    " = TimeSpan.FromMilliseconds({})",
                                    value.as_millis()
                                ))?,
                                DurationType::Seconds => f.write(&format!(
                                    " = TimeSpan.FromSeconds({})",
                                    value.as_secs()
                                ))?,
                            }
                        }
                    }
                }

                f.write(";")?;
            }

            f.newline()?;

            // Write constructor
            if !native_struct.definition().all_fields_have_defaults() {
                documentation(f, |f| {
                    f.writeln("<summary>")?;
                    docstring_print(
                        f,
                        &format!(
                            "Initialize {{struct:{}}} to default values",
                            native_struct.name()
                        )
                        .into(),
                        lib,
                    )?;
                    f.write("</summary>")?;

                    for param in native_struct
                        .elements()
                        .filter(|el| !el.field_type.has_default())
                    {
                        f.writeln(&format!("<param name=\"{}\">", param.name.to_mixed_case()))?;
                        docstring_print(f, &param.doc.brief, lib)?;
                        f.write("</param>")?;
                    }

                    Ok(())
                })?;

                f.writeln(&format!(
                    "{} {}(",
                    constructor_visibility(native_struct.definition.visibility()),
                    struct_name
                ))?;
                f.write(
                    &native_struct
                        .elements()
                        .filter(|el| !el.field_type.has_default())
                        .map(|el| {
                            format!(
                                "{} {}",
                                el.field_type.to_all_types().as_dotnet_type(),
                                el.name.to_mixed_case()
                            )
                        })
                        .collect::<Vec<String>>()
                        .join(", "),
                )?;
                f.write(")")?;

                blocked(f, |f| {
                    for el in native_struct.elements() {
                        if !el.field_type.has_default() {
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
                    el.field_type.to_all_types().as_native_type(),
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
                        .field_type
                        .to_all_types()
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
                        .field_type
                        .to_all_types()
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
                        .field_type
                        .to_all_types()
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
