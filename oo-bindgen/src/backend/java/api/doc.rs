use crate::backend::*;
use crate::model::*;

use super::conversion::*;

pub(crate) fn javadoc_print(f: &mut dyn Printer, doc: &Doc<Validated>) -> FormattingResult<()> {
    f.newline()?;
    docstring_print(f, &doc.brief)?;

    for detail in &doc.details {
        f.newline()?;

        match detail {
            DocParagraph::Details(docstring) => {
                f.writeln("<p>")?;
                docstring_print(f, docstring)?;
                f.write("</p>")?;
            }
            DocParagraph::Warning(docstring) => {
                f.writeln("<p><b>Warning:</b> ")?;
                docstring_print(f, docstring)?;
                f.write("</p>")?;
            }
        }
    }

    Ok(())
}

pub(crate) fn docstring_print(
    f: &mut dyn Printer,
    docstring: &DocString<Validated>,
) -> FormattingResult<()> {
    for el in docstring.elements() {
        match el {
            DocStringElement::Text(text) => f.write(text)?,
            DocStringElement::Null => f.write("{@code null}")?,
            DocStringElement::Iterator => f.write("collection")?,
            DocStringElement::Reference(reference) => reference_print(f, reference)?,
        }
    }

    Ok(())
}

fn reference_print(f: &mut dyn Printer, reference: &Validated) -> FormattingResult<()> {
    match reference {
        Validated::Argument(param_name) => {
            f.write(&format!("{{@code {}}}", param_name.mixed_case()))?
        }
        Validated::Class(class) => {
            f.write(&format!("{{@link {}}}", class.name.camel_case()))?;
        }
        Validated::ClassMethod(class, method_name, _) => {
            f.write(&format!(
                "{{@link {}#{}}}",
                class.name().camel_case(),
                method_name.mixed_case()
            ))?;
        }
        Validated::ClassConstructor(class, constructor) => {
            let params = constructor
                .function
                .arguments
                .iter()
                .map(|param| param.arg_type.as_java_primitive())
                .collect::<Vec<_>>()
                .join(", ");

            let class_name = class.name().camel_case();
            f.write(&format!(
                "{{@link {}#{}({})}}",
                class_name, class_name, params
            ))?;
        }
        Validated::ClassDestructor(class, _) => {
            let method_name = if let DestructionMode::Custom(name) = &class.destruction_mode {
                name.mixed_case()
            } else {
                "close".to_string()
            };

            f.write(&format!(
                "{{@link {}#{}}}",
                class.name().camel_case(),
                method_name
            ))?;
        }
        Validated::Struct(st) => {
            f.write(&format!("{{@link {}}}", st.name().camel_case()))?;
        }
        Validated::StructField(st, field_name) => {
            f.write(&format!(
                "{{@link {}#{}}}",
                st.name().camel_case(),
                field_name.mixed_case()
            ))?;
        }
        Validated::Enum(handle) => {
            f.write(&format!("{{@link {}}}", handle.name.camel_case()))?;
        }
        Validated::EnumVariant(handle, variant_name) => {
            f.write(&format!(
                "{{@link {}#{}}}",
                handle.name.camel_case(),
                variant_name.capital_snake_case()
            ))?;
        }
        Validated::Interface(interface) => {
            f.write(&format!("{{@link {}}}", interface.name.camel_case()))?;
        }
        Validated::InterfaceMethod(interface, callback_name) => {
            f.write(&format!(
                "{{@link {}#{}}}",
                interface.name.camel_case(),
                callback_name.mixed_case()
            ))?;
        }
    }

    Ok(())
}
