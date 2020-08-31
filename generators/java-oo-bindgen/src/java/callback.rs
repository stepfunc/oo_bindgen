use super::*;
use heck::{CamelCase, MixedCase};
use oo_bindgen::callback::*;
use oo_bindgen::native_function::*;

pub(crate) fn generate(
    f: &mut dyn Printer,
    callback: &OneTimeCallbackHandle,
    lib: &Library,
) -> FormattingResult<()> {
    let callback_name = callback.name.to_camel_case();

    f.writeln(&format!("public interface {}", callback_name))?;
    blocked(f, |f| {
        // Write each required method
        for func in callback.callbacks() {
            // Documentation
            documentation(f, |f| {
                // Print top-level documentation
                f.newline()?;
                doc_print(f, &func.doc, lib)?;

                // Print each parameter value
                for param in &func.parameters {
                    if let CallbackParameter::Parameter(param) = param {
                        f.writeln(&format!("@param {} ", param.name))?;
                        doc_print(f, &param.doc, lib)?;
                    }
                }

                // Print return value
                if let ReturnType::Type(_, doc) = &func.return_type {
                    f.writeln("@return ")?;
                    doc_print(f, doc, lib)?;
                }

                Ok(())
            })?;

            // Callback signature
            f.writeln(&format!(
                "{} {}(",
                JavaReturnType(&func.return_type).as_java_type(),
                func.name.to_mixed_case()
            ))?;
            f.write(
                &func
                    .parameters
                    .iter()
                    .filter_map(|param| match param {
                        CallbackParameter::Parameter(param) => Some(format!(
                            "{} {}",
                            JavaType(&param.param_type).as_java_type(),
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
