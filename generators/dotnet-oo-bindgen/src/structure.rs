use crate::*;
use heck::{CamelCase, MixedCase};
use oo_bindgen::any_struct::*;
use oo_bindgen::struct_common::*;
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
    native_struct: &StructType,
    lib: &Library,
) -> FormattingResult<()> {
    let struct_name = native_struct.name().to_camel_case();
    let struct_native_name = format!("{}Native", struct_name);

    print_license(f, &lib.info.license_description)?;
    print_imports(f)?;
    f.newline()?;

    let doc = match native_struct.visibility() {
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
            for el in native_struct.fields() {
                documentation(f, |f| {
                    // Print top-level documentation
                    xmldoc_print(f, &el.doc, lib)?;

                    let default_value = match &el.field_type {
                        AnyStructFieldType::Bool(default) => default.map(|x| x.to_string()),
                        AnyStructFieldType::Uint8(default) => default.map(|x| x.to_string()),
                        AnyStructFieldType::Sint8(default) => default.map(|x| x.to_string()),
                        AnyStructFieldType::Uint16(default) => default.map(|x| x.to_string()),
                        AnyStructFieldType::Sint16(default) => default.map(|x| x.to_string()),
                        AnyStructFieldType::Uint32(default) => default.map(|x| x.to_string()),
                        AnyStructFieldType::Sint32(default) => default.map(|x| x.to_string()),
                        AnyStructFieldType::Uint64(default) => default.map(|x| x.to_string()),
                        AnyStructFieldType::Sint64(default) => default.map(|x| x.to_string()),
                        AnyStructFieldType::Float(default) => default.map(|x| x.to_string()),
                        AnyStructFieldType::Double(default) => default.map(|x| x.to_string()),
                        AnyStructFieldType::String(default) => {
                            default.clone().map(|x| format!("\"{}\"", x))
                        }
                        AnyStructFieldType::Struct(_) => None,
                        AnyStructFieldType::StructRef(_) => None,
                        AnyStructFieldType::Enum(field) => field.clone().default_variant.map(|x| {
                            format!(
                                "<see cref=\"{}.{}\" />",
                                field.handle.name.to_camel_case(),
                                x.to_camel_case()
                            )
                        }),
                        AnyStructFieldType::ClassRef(_) => None,
                        AnyStructFieldType::Interface(_) => None,
                        AnyStructFieldType::Iterator(_) => None,
                        AnyStructFieldType::Collection(_) => None,
                        AnyStructFieldType::Duration(_, default) => {
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
                    field_visibility(native_struct.visibility()),
                    el.field_type.to_any_type().as_dotnet_type(),
                    el.name.to_camel_case()
                ))?;
                match &el.field_type {
                    AnyStructFieldType::Bool(default) => match default {
                        None => (),
                        Some(false) => f.write(" = false")?,
                        Some(true) => f.write(" = true")?,
                    },
                    AnyStructFieldType::Uint8(default) => {
                        if let Some(value) = default {
                            f.write(&format!(" = (byte){}", value))?;
                        }
                    }
                    AnyStructFieldType::Sint8(default) => {
                        if let Some(value) = default {
                            f.write(&format!(" = (sbyte){}", value))?;
                        }
                    }
                    AnyStructFieldType::Uint16(default) => {
                        if let Some(value) = default {
                            f.write(&format!(" = (ushort){}", value))?;
                        }
                    }
                    AnyStructFieldType::Sint16(default) => {
                        if let Some(value) = default {
                            f.write(&format!(" = (short){}", value))?;
                        }
                    }
                    AnyStructFieldType::Uint32(default) => {
                        if let Some(value) = default {
                            f.write(&format!(" = (uint){}", value))?;
                        }
                    }
                    AnyStructFieldType::Sint32(default) => {
                        if let Some(value) = default {
                            f.write(&format!(" = (int){}", value))?;
                        }
                    }
                    AnyStructFieldType::Uint64(default) => {
                        if let Some(value) = default {
                            f.write(&format!(" = (ulong){}", value))?;
                        }
                    }
                    AnyStructFieldType::Sint64(default) => {
                        if let Some(value) = default {
                            f.write(&format!(" = (long){}", value))?;
                        }
                    }
                    AnyStructFieldType::Float(default) => {
                        if let Some(value) = default {
                            f.write(&format!(" = {}f", value))?;
                        }
                    }
                    AnyStructFieldType::Double(default) => {
                        if let Some(value) = default {
                            f.write(&format!(" = {}", value))?;
                        }
                    }
                    AnyStructFieldType::String(default) => {
                        if let Some(value) = default {
                            f.write(&format!(" = \"{}\"", &value))?;
                        }
                    }
                    AnyStructFieldType::Struct(handle) => {
                        if handle.all_fields_have_defaults() {
                            f.write(&format!(" = new {}()", handle.name().to_camel_case()))?;
                        }
                    }
                    AnyStructFieldType::StructRef(_) => (),
                    AnyStructFieldType::Enum(field) => {
                        if let Some(value) = &field.default_variant {
                            match field.handle.find_variant_by_name(value) {
                                Some(variant) => f.write(&format!(
                                    " = {}.{}",
                                    field.handle.name.to_camel_case(),
                                    variant.name.to_camel_case()
                                ))?,
                                None => {
                                    panic!("Variant {} not found in {}", value, field.handle.name)
                                }
                            }
                        }
                    }
                    AnyStructFieldType::ClassRef(_) => (),
                    AnyStructFieldType::Interface(_) => (),
                    AnyStructFieldType::Iterator(_) => (),
                    AnyStructFieldType::Collection(_) => (),
                    AnyStructFieldType::Duration(mapping, default) => {
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
            if !native_struct.all_fields_have_defaults() {
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
                        .fields()
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
                    constructor_visibility(native_struct.visibility()),
                    struct_name
                ))?;
                f.write(
                    &native_struct
                        .fields()
                        .filter(|el| !el.field_type.has_default())
                        .map(|el| {
                            format!(
                                "{} {}",
                                el.field_type.to_any_type().as_dotnet_type(),
                                el.name.to_mixed_case()
                            )
                        })
                        .collect::<Vec<String>>()
                        .join(", "),
                )?;
                f.write(")")?;

                blocked(f, |f| {
                    for el in native_struct.fields() {
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

            Ok(())
        })?;

        f.newline()?;

        // Write native struct
        f.writeln("[StructLayout(LayoutKind.Sequential)]")?;
        f.writeln(&format!("internal struct {}", struct_native_name))?;
        blocked(f, |f| {
            // Write native elements
            for el in native_struct.fields() {
                f.writeln(&format!(
                    "{} {};",
                    el.field_type.to_any_type().as_native_type(),
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
                for el in native_struct.fields() {
                    let el_name = el.name.to_camel_case();

                    let conversion = el
                        .field_type
                        .to_any_type()
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
                for el in native_struct.fields() {
                    let el_name = el.name.to_camel_case();

                    let conversion = el
                        .field_type
                        .to_any_type()
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
                for el in native_struct.fields() {
                    let el_name = el.name.to_camel_case();

                    if let Some(cleanup) = el
                        .field_type
                        .to_any_type()
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
