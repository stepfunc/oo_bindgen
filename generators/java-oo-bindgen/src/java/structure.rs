use super::doc::*;
use super::*;
use heck::{CamelCase, MixedCase, ShoutySnakeCase};
use oo_bindgen::any_struct::*;
use oo_bindgen::struct_common::Visibility;
use oo_bindgen::types::DurationType;

fn constructor_visibility(struct_type: Visibility) -> &'static str {
    match struct_type {
        Visibility::Public => "public",
        Visibility::Private => "private",
    }
}

fn field_visibility(struct_type: Visibility) -> &'static str {
    match struct_type {
        Visibility::Public => "public",
        Visibility::Private => "private final",
    }
}

pub(crate) fn generate(
    f: &mut impl Printer,
    native_struct: &StructType,
    lib: &Library,
) -> FormattingResult<()> {
    let struct_name = native_struct.name().to_camel_case();

    let doc = match native_struct.visibility() {
        Visibility::Public => native_struct.doc().clone(),
        Visibility::Private => native_struct
            .doc()
            .clone()
            .warning("This class is an opaque handle and cannot be constructed by user code"),
    };

    // Documentation
    documentation(f, |f| javadoc_print(f, &doc, lib))?;

    // Structure definition
    f.writeln(&format!("public final class {}", struct_name))?;
    blocked(f, |f| {
        // Write Java structure elements
        for el in native_struct.fields() {
            documentation(f, |f| {
                javadoc_print(f, &el.doc, lib)?;

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
                    AnyStructFieldType::Enum(handle, default) => default.clone().map(|x| {
                        format!(
                            "{{@link {}#{}}}",
                            handle.name.to_camel_case(),
                            x.to_shouty_snake_case()
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
                    f.writeln(&format!("<p>Default value is {}</p>", default_value))?;
                }

                Ok(())
            })?;

            f.writeln(&format!(
                "{} {} {}",
                field_visibility(native_struct.visibility()),
                el.field_type.to_all_types().as_java_primitive(),
                el.name.to_mixed_case()
            ))?;
            match &el.field_type {
                AnyStructFieldType::Bool(default) => match default {
                    None => (),
                    Some(false) => f.write(" = false")?,
                    Some(true) => f.write(" = true")?,
                },
                AnyStructFieldType::Uint8(default) => {
                    if let Some(value) = default {
                        f.write(&format!(" = UByte.valueOf({})", value))?;
                    }
                }
                AnyStructFieldType::Sint8(default) => {
                    if let Some(value) = default {
                        f.write(&format!(" = (byte){}", value))?;
                    }
                }
                AnyStructFieldType::Uint16(default) => {
                    if let Some(value) = default {
                        f.write(&format!(" = UShort.valueOf({})", value))?;
                    }
                }
                AnyStructFieldType::Sint16(default) => {
                    if let Some(value) = default {
                        f.write(&format!(" = (short){}", value))?;
                    }
                }
                AnyStructFieldType::Uint32(default) => {
                    if let Some(value) = default {
                        f.write(&format!(" = UInteger.valueOf({}L)", value))?;
                    }
                }
                AnyStructFieldType::Sint32(default) => {
                    if let Some(value) = default {
                        f.write(&format!(" = {}", value))?;
                    }
                }
                AnyStructFieldType::Uint64(default) => {
                    if let Some(value) = default {
                        f.write(&format!(" = ULong.valueOf({}L)", value))?;
                    }
                }
                AnyStructFieldType::Sint64(default) => {
                    if let Some(value) = default {
                        f.write(&format!(" = {}L", value))?;
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
                AnyStructFieldType::Enum(handle, default) => {
                    if let Some(value) = default {
                        match handle.find_variant_by_name(value) {
                            Some(variant) => f.write(&format!(
                                " = {}.{}",
                                handle.name.to_camel_case(),
                                variant.name.to_shouty_snake_case()
                            ))?,
                            None => panic!("Variant {} not found in {}", value, handle.name),
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
                                " = java.time.Duration.ofMillis({})",
                                value.as_millis()
                            ))?,
                            DurationType::Seconds => f.write(&format!(
                                " = java.time.Duration.ofSeconds({})",
                                value.as_secs()
                            ))?,
                        }
                    }
                }
            }

            f.write(";")?;
        }

        f.newline()?;

        if !native_struct.all_fields_have_defaults() {
            documentation(f, |f| {
                f.newline()?;
                docstring_print(
                    f,
                    &format!(
                        "Initialize {{struct:{}}} to default values",
                        native_struct.name()
                    )
                    .into(),
                    lib,
                )?;
                f.newline()?;

                for param in native_struct
                    .fields()
                    .filter(|f| !f.field_type.has_default())
                {
                    f.writeln(&format!("@param {} ", param.name.to_mixed_case()))?;
                    docstring_print(f, &param.doc.brief, lib)?;
                }

                Ok(())
            })?;

            f.writeln(&format!(
                "{} {}(",
                constructor_visibility(native_struct.visibility()),
                struct_name,
            ))?;
            f.write(
                &native_struct
                    .fields()
                    .filter(|el| !el.field_type.has_default())
                    .map(|el| {
                        format!(
                            "{} {}",
                            el.field_type.to_all_types().as_java_primitive(),
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
                            el.name.to_mixed_case(),
                            el.name.to_mixed_case()
                        ))?;
                    }
                }
                Ok(())
            })?;

            f.newline()?;
        }

        Ok(())
    })
}
