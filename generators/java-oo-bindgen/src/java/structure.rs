use super::*;
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

        Ok(())
    })
}
