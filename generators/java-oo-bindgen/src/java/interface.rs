use super::doc::*;
use super::*;
use heck::{CamelCase, MixedCase};
use oo_bindgen::callback::*;

pub(crate) fn generate(
    f: &mut dyn Printer,
    interface: &InterfaceHandle,
    lib: &Library,
) -> FormattingResult<()> {
    let interface_name = interface.name.to_camel_case();

    documentation(f, |f| {
        // Print top-level documentation
        javadoc_print(f, &interface.doc, lib)
    })?;

    if interface.is_functional() {
        f.writeln("@FunctionalInterface")?;
    }
    f.writeln(&format!("public interface {}", interface_name))?;
    blocked(f, |f| {
        // Write each required method
        for func in interface
            .callbacks()
            .filter(|func| func.name != interface.destroy_name)
        {
            // Documentation
            documentation(f, |f| {
                // Print top-level documentation
                javadoc_print(f, &func.doc, lib)?;
                f.newline()?;

                // Print each parameter value
                for param in &func.parameters {
                    if let CallbackParameter::Parameter(param) = param {
                        f.writeln(&format!("@param {} ", param.name.to_mixed_case()))?;
                        docstring_print(f, &param.doc, lib)?;
                    }
                }

                // Print return value
                if let ReturnType::Type(_, doc) = &func.return_type {
                    f.writeln("@return ")?;
                    docstring_print(f, doc, lib)?;
                }

                Ok(())
            })?;

            // Callback signature
            f.writeln(&format!(
                "{} {}(",
                func.return_type.as_java_primitive(),
                func.name.to_mixed_case()
            ))?;
            f.write(
                &func
                    .parameters
                    .iter()
                    .filter_map(|param| match param {
                        CallbackParameter::Parameter(param) => Some(format!(
                            "{} {}",
                            param.arg_type.as_java_primitive(),
                            param.name.to_mixed_case()
                        )),
                        _ => None,
                    })
                    .collect::<Vec<String>>()
                    .join(", "),
            )?;
            f.write(");")?;
        }

        Ok(())
    })
}
