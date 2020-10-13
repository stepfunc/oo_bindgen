use super::*;
use super::doc::*;
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
    documentation(f, |f| javadoc_print(f, &native_struct.doc(), lib))?;

    // Structure definition
    f.writeln(&format!("public class {}", struct_name))?;
    blocked(f, |f| {
        // Write Java structure elements
        for el in native_struct.elements() {
            documentation(f, |f| javadoc_print(f, &el.doc, lib))?;

            f.writeln(&format!(
                "public {} {};",
                el.element_type.as_java_primitive(),
                el.name.to_mixed_case()
            ))?;
        }

        f.newline()?;

        // Write methods
        for method in &native_struct.methods {
            documentation(f, |f| {
                // Print top-level documentation
                javadoc_print(f, &method.native_function.doc, lib)?;
                f.newline()?;

                // Print each parameter value
                for param in method.native_function.parameters.iter().skip(1) {
                    f.writeln(&format!("@param {} ", param.name))?;
                    docstring_print(f, &param.doc, lib)?;
                }

                // Print return value
                if let ReturnType::Type(_, doc) = &method.native_function.return_type {
                    f.writeln("@return ")?;
                    docstring_print(f, doc, lib)?;
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

            blocked(f, |f| {
                call_native_function(
                    f,
                    &method.native_function,
                    "return ",
                    true,
                )
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
                    f.writeln(&format!("@param {} ", param.name))?;
                    docstring_print(f, &param.doc, lib)?;
                }

                // Print return value
                if let ReturnType::Type(_, doc) = &method.native_function.return_type {
                    f.writeln("@return ")?;
                    docstring_print(f, doc, lib)?;
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

            blocked(f, |f| {
                call_native_function(f, &method.native_function, "return ", false)
            })?;
        }

        Ok(())
    })
}
