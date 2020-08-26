use crate::*;
use heck::{CamelCase, MixedCase};
use oo_bindgen::native_function::*;
use oo_bindgen::native_struct::*;

pub(crate) fn generate(
    f: &mut impl Printer,
    native_struct: &StructHandle,
    lib: &Library,
) -> FormattingResult<()> {
    let struct_name = native_struct.name().to_camel_case();

    // Documentation
    if !native_struct.doc().is_empty() {
        documentation(f, |f| {
            f.newline()?;
            doc_print(f, &native_struct.doc(), lib)?;
            Ok(())
        })?;
    }

    // Structure definition
    f.writeln(&format!("public class {}", struct_name))?;
    blocked(f, |f| {
        // Write Java structure elements
        for el in native_struct.elements() {
            if !el.doc.is_empty() {
                documentation(f, |f| {
                    f.newline()?;
                    doc_print(f, &el.doc, lib)?;
                    Ok(())
                })?;
            }

            let java_type = JavaType(&el.element_type);
            f.writeln(&format!(
                "public {} {};",
                java_type.as_java_type(),
                el.name.to_mixed_case()
            ))?;
        }

        f.newline()?;

        // Write methods
        for method in &native_struct.methods {
            documentation(f, |f| {
                // Print top-level documentation
                f.newline()?;
                doc_print(f, &method.native_function.doc, lib)?;

                // Print each parameter value
                for param in method.native_function.parameters.iter().skip(1) {
                    f.writeln(&format!("@param {} ", param.name))?;
                    doc_print(f, &param.doc, lib)?;
                }

                // Print return value
                if let ReturnType::Type(_, doc) = &method.native_function.return_type {
                    f.writeln("@return ")?;
                    doc_print(f, doc, lib)?;
                }
                Ok(())
            })?;

            f.writeln(&format!(
                "public {} {}(",
                JavaReturnType(&method.native_function.return_type).as_java_type(),
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
                            JavaType(&param.param_type).as_java_type(),
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
                f.newline()?;
                doc_print(f, &method.native_function.doc, lib)?;

                // Print each parameter value
                for param in method.native_function.parameters.iter() {
                    f.writeln(&format!("@param {} ", param.name))?;
                    doc_print(f, &param.doc, lib)?;
                }

                // Print return value
                if let ReturnType::Type(_, doc) = &method.native_function.return_type {
                    f.writeln("@return ")?;
                    doc_print(f, doc, lib)?;
                }
                Ok(())
            })?;

            f.writeln(&format!(
                "public static {} {}(",
                JavaReturnType(&method.native_function.return_type).as_java_type(),
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
                            JavaType(&param.param_type).as_java_type(),
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

        f.newline()?;

        // Write native struct
        let field_order = native_struct
            .elements()
            .map(|el| format!("\"{}\"", el.name.to_mixed_case()))
            .collect::<Vec<_>>()
            .join(", ");
        f.writeln(&format!(
            "@com.sun.jna.Structure.FieldOrder({{ {} }})",
            field_order
        ))?;

        f.writeln("public static class Native extends com.sun.jna.Structure")?;
        blocked(f, |f| {
            // Write native elements
            for el in native_struct.elements() {
                let java_type = JavaType(&el.element_type);
                f.writeln(&format!(
                    "public {} {};",
                    java_type.as_native_type(),
                    el.name.to_mixed_case()
                ))?;
            }

            // ByValue type annotation for JNA
            f.writeln("public static class ByValue extends Native implements com.sun.jna.Structure.ByValue")?;
            blocked(f, |f| {
                f.writeln("public ByValue() { }")?;
                f.writeln(&format!(
                    "public ByValue({} self) {{ super(self); }}",
                    struct_name
                ))
            })?;

            f.newline()?;

            // ByReference type annotation for JNA
            f.writeln("public static class ByReference extends Native implements com.sun.jna.Structure.ByReference")?;
            blocked(f, |f| {
                f.writeln("public ByReference() { }")?;
                f.writeln(&format!(
                    "public ByReference({} self) {{ super(self); }}",
                    struct_name
                ))
            })?;

            f.newline()?;

            // Default constructor for JNA
            f.writeln("public Native() { }")?;

            f.newline()?;

            // Constructor from Java type
            f.writeln(&format!("Native({} self)", struct_name))?;
            blocked(f, |f| {
                for el in native_struct.elements() {
                    let el_name = el.name.to_mixed_case();

                    let java_type = JavaType(&el.element_type);
                    if let Some(conversion) = java_type.conversion() {
                        conversion.convert_to_native(
                            f,
                            &format!("self.{}", el_name),
                            &format!("this.{} = ", el_name),
                        )?;
                    } else {
                        f.writeln(&format!("this.{} = self.{};", el_name, el_name))?;
                    }
                }
                Ok(())
            })?;

            f.newline()?;

            // Convert from native to Java
            f.writeln(&format!(
                "static {} fromNative(Native nativeStruct)",
                struct_name
            ))?;
            blocked(f, |f| {
                f.writeln(&format!("{} result = new {}();", struct_name, struct_name))?;
                for el in native_struct.elements() {
                    let el_name = el.name.to_mixed_case();

                    let java_type = JavaType(&el.element_type);
                    if let Some(conversion) = java_type.conversion() {
                        conversion.convert_from_native(
                            f,
                            &format!("nativeStruct.{}", el_name),
                            &format!("result.{} = ", el_name),
                        )?;
                    } else {
                        f.writeln(&format!("result.{} = nativeStruct.{};", el_name, el_name))?;
                    }
                }
                f.writeln("return result;")
            })?;

            f.newline()?;

            // Finalizer
            f.writeln("@Override")?;
            f.writeln("public void finalize()")?;
            blocked(f, |f| {
                for el in native_struct.elements() {
                    let el_name = el.name.to_mixed_case();

                    let java_type = JavaType(&el.element_type);
                    if let Some(conversion) = java_type.conversion() {
                        conversion.convert_to_native_cleanup(f, &format!("this.{}", el_name))?;
                    }
                }
                Ok(())
            })
        })
    })
}
