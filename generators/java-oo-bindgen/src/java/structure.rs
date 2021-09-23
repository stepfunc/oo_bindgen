use super::doc::*;
use super::*;
use heck::{CamelCase, MixedCase, ShoutySnakeCase};
use oo_bindgen::error_type::ExceptionType;
use oo_bindgen::native_struct::*;

fn constructor_visibility(struct_type: NativeStructType) -> &'static str {
    match struct_type {
        NativeStructType::Public => "public",
        NativeStructType::Opaque => "private",
    }
}

fn field_visibility(struct_type: NativeStructType) -> &'static str {
    match struct_type {
        NativeStructType::Public => "public",
        NativeStructType::Opaque => "private final",
    }
}

pub(crate) fn generate(
    f: &mut impl Printer,
    native_struct: &StructHandle,
    lib: &Library,
) -> FormattingResult<()> {
    let struct_name = native_struct.name().to_camel_case();

    let doc = match native_struct.definition.struct_type {
        NativeStructType::Public => native_struct.doc().clone(),
        NativeStructType::Opaque => native_struct
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
        for el in native_struct.elements() {
            documentation(f, |f| {
                javadoc_print(f, &el.doc, lib)?;

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
                            "{{@link {}#{}}}",
                            handle.name.to_camel_case(),
                            x.to_shouty_snake_case()
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
                    f.writeln(&format!("<p>Default value is {}</p>", default_value))?;
                }

                Ok(())
            })?;

            f.writeln(&format!(
                "{} {} {}",
                field_visibility(native_struct.definition.struct_type),
                el.element_type.to_type().as_java_primitive(),
                el.name.to_mixed_case()
            ))?;
            match &el.element_type {
                StructElementType::Bool(default) => match default {
                    None => (),
                    Some(false) => f.write(" = false")?,
                    Some(true) => f.write(" = true")?,
                },
                StructElementType::Uint8(default) => {
                    if let Some(value) = default {
                        f.write(&format!(" = UByte.valueOf({})", value))?;
                    }
                }
                StructElementType::Sint8(default) => {
                    if let Some(value) = default {
                        f.write(&format!(" = (byte){}", value))?;
                    }
                }
                StructElementType::Uint16(default) => {
                    if let Some(value) = default {
                        f.write(&format!(" = UShort.valueOf({})", value))?;
                    }
                }
                StructElementType::Sint16(default) => {
                    if let Some(value) = default {
                        f.write(&format!(" = (short){}", value))?;
                    }
                }
                StructElementType::Uint32(default) => {
                    if let Some(value) = default {
                        f.write(&format!(" = UInteger.valueOf({}L)", value))?;
                    }
                }
                StructElementType::Sint32(default) => {
                    if let Some(value) = default {
                        f.write(&format!(" = {}", value))?;
                    }
                }
                StructElementType::Uint64(default) => {
                    if let Some(value) = default {
                        f.write(&format!(" = ULong.valueOf({}L)", value))?;
                    }
                }
                StructElementType::Sint64(default) => {
                    if let Some(value) = default {
                        f.write(&format!(" = {}L", value))?;
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
                                variant.name.to_shouty_snake_case()
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
                                " = java.time.Duration.ofMillis({})",
                                value.as_millis()
                            ))?,
                            DurationMapping::Seconds => f.write(&format!(
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

        if !native_struct.definition().is_default_constructed() {
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
                    .elements()
                    .filter(|el| !el.element_type.has_default())
                {
                    f.writeln(&format!("@param {} ", param.name.to_mixed_case()))?;
                    docstring_print(f, &param.doc.brief, lib)?;
                }

                Ok(())
            })?;

            f.writeln(&format!(
                "{} {}(",
                constructor_visibility(native_struct.definition.struct_type),
                struct_name,
            ))?;
            f.write(
                &native_struct
                    .elements()
                    .filter(|el| !el.element_type.has_default())
                    .map(|el| {
                        format!(
                            "{} {}",
                            el.element_type.to_type().as_java_primitive(),
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
                            el.name.to_mixed_case(),
                            el.name.to_mixed_case()
                        ))?;
                    }
                }
                Ok(())
            })?;

            f.newline()?;
        }

        // Write methods
        for method in &native_struct.methods {
            documentation(f, |f| {
                // Print top-level documentation
                javadoc_print(f, &method.native_function.doc, lib)?;
                f.newline()?;

                // Print each parameter value
                for param in method.native_function.parameters.iter().skip(1) {
                    f.writeln(&format!("@param {} ", param.name.to_mixed_case()))?;
                    docstring_print(f, &param.doc, lib)?;
                }

                // Print return value
                if let ReturnType::Type(_, doc) = &method.native_function.return_type {
                    f.writeln("@return ")?;
                    docstring_print(f, doc, lib)?;
                }

                // Print exception
                if let Some(error) = &method.native_function.error_type {
                    f.writeln(&format!(
                        "@throws {} {}",
                        error.exception_name.to_camel_case(),
                        error.inner.name.to_camel_case()
                    ))?;
                }

                Ok(())
            })?;

            f.writeln(&format!(
                "public {} {}(",
                method.native_function.return_type.as_java_primitive(),
                method.name.to_mixed_case()
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
                            param.param_type.as_java_primitive(),
                            param.name.to_mixed_case()
                        )
                    })
                    .collect::<Vec<String>>()
                    .join(", "),
            )?;
            f.write(")")?;

            if let Some(error) = &method.native_function.error_type {
                if error.exception_type == ExceptionType::CheckedException {
                    f.write(&format!(" throws {}", error.exception_name.to_camel_case()))?;
                }
            }

            blocked(f, |f| {
                call_native_function(f, &method.native_function, "return ", true)
            })?;
        }

        f.newline()?;

        // Write static methods
        for method in &native_struct.static_methods {
            documentation(f, |f| {
                // Print top-level documentation
                javadoc_print(f, &method.native_function.doc, lib)?;
                f.newline()?;

                // Print each parameter value
                for param in method.native_function.parameters.iter() {
                    f.writeln(&format!("@param {} ", param.name.to_mixed_case()))?;
                    docstring_print(f, &param.doc, lib)?;
                }

                // Print return value
                if let ReturnType::Type(_, doc) = &method.native_function.return_type {
                    f.writeln("@return ")?;
                    docstring_print(f, doc, lib)?;
                }

                // Print exception
                if let Some(error) = &method.native_function.error_type {
                    f.writeln(&format!(
                        "@throws {} {}",
                        error.exception_name.to_camel_case(),
                        error.inner.name.to_camel_case()
                    ))?;
                }

                Ok(())
            })?;

            f.writeln(&format!(
                "public static {} {}(",
                method.native_function.return_type.as_java_primitive(),
                method.name.to_mixed_case()
            ))?;
            f.write(
                &method
                    .native_function
                    .parameters
                    .iter()
                    .map(|param| {
                        format!(
                            "{} {}",
                            param.param_type.as_java_primitive(),
                            param.name.to_mixed_case()
                        )
                    })
                    .collect::<Vec<String>>()
                    .join(", "),
            )?;
            f.write(")")?;

            if let Some(error) = &method.native_function.error_type {
                if error.exception_type == ExceptionType::CheckedException {
                    f.write(&format!(" throws {}", error.exception_name.to_camel_case()))?;
                }
            }

            blocked(f, |f| {
                call_native_function(f, &method.native_function, "return ", false)
            })?;
        }

        Ok(())
    })
}
