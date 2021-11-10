use super::doc::*;
use super::*;
use oo_bindgen::doc::Validated;
use oo_bindgen::interface::*;

pub(crate) fn generate(
    f: &mut dyn Printer,
    interface: &Handle<Interface<Validated>>,
) -> FormattingResult<()> {
    let interface_name = interface.name.camel_case();

    documentation(f, |f| {
        // Print top-level documentation
        javadoc_print(f, &interface.doc)
    })?;

    if interface.is_functional() {
        f.writeln("@FunctionalInterface")?;
    }
    f.writeln(&format!("public interface {}", interface_name))?;
    blocked(f, |f| {
        // Write each required method
        for func in interface.callbacks.iter() {
            // Documentation
            documentation(f, |f| {
                // Print top-level documentation
                javadoc_print(f, &func.doc)?;
                f.newline()?;

                // Print each argument value
                for arg in &func.arguments {
                    f.writeln(&format!("@param {} ", arg.name.mixed_case()))?;
                    docstring_print(f, &arg.doc)?;
                }

                // Print return value
                if let CallbackReturnType::Type(_, doc) = &func.return_type {
                    f.writeln("@return ")?;
                    docstring_print(f, doc)?;
                }

                Ok(())
            })?;

            // Callback signature
            f.writeln(&format!(
                "{} {}(",
                func.return_type.as_java_primitive(),
                func.name.mixed_case()
            ))?;
            f.write(
                &func
                    .arguments
                    .iter()
                    .map(|arg| {
                        format!(
                            "{} {}",
                            arg.arg_type.as_java_primitive(),
                            arg.name.mixed_case()
                        )
                    })
                    .collect::<Vec<String>>()
                    .join(", "),
            )?;
            f.write(");")?;
        }

        Ok(())
    })
}
