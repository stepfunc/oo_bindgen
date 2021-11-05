use super::conversion::*;
use heck::{CamelCase, MixedCase, ShoutySnakeCase};
use oo_bindgen::class::DestructionMode;
use oo_bindgen::doc::*;
use oo_bindgen::formatting::*;

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
        Validated::Param(param_name) => {
            f.write(&format!("{{@code {}}}", param_name.to_mixed_case()))?
        }
        Validated::Class(class) => {
            f.write(&format!("{{@link {}}}", class.name.to_camel_case()))?;
        }
        Validated::ClassMethod(class, method) => {
            f.write(&format!(
                "{{@link {}#{}}}",
                class.name().to_camel_case(),
                method.name.to_mixed_case()
            ))?;
        }
        Validated::ClassConstructor(class, constructor) => {
            let params = constructor
                .parameters
                .iter()
                .map(|param| param.arg_type.as_java_primitive())
                .collect::<Vec<_>>()
                .join(", ");

            let class_name = class.name().to_camel_case();
            f.write(&format!(
                "{{@link {}#{}({})}}",
                class_name, class_name, params
            ))?;
        }
        Validated::ClassDestructor(class, _) => {
            let method_name = if let DestructionMode::Custom(name) = &class.destruction_mode {
                name.to_mixed_case()
            } else {
                "close".to_string()
            };

            f.write(&format!(
                "{{@link {}#{}}}",
                class.name().to_camel_case(),
                method_name
            ))?;
        }
        Validated::Struct(st) => {
            f.write(&format!("{{@link {}}}", st.name().to_camel_case()))?;
        }
        Validated::StructField(st, field_name) => {
            f.write(&format!(
                "{{@link {}#{}}}",
                st.name().to_camel_case(),
                field_name.to_mixed_case()
            ))?;
        }
        Validated::Enum(handle) => {
            f.write(&format!("{{@link {}}}", handle.name.to_camel_case()))?;
        }
        Validated::EnumVariant(handle, variant_name) => {
            f.write(&format!(
                "{{@link {}#{}}}",
                handle.name.to_camel_case(),
                variant_name.to_shouty_snake_case()
            ))?;
        }
        Validated::Interface(interface) => {
            f.write(&format!("{{@link {}}}", interface.name.to_camel_case()))?;
        }
        Validated::InterfaceMethod(interface, callback_name) => {
            f.write(&format!(
                "{{@link {}#{}}}",
                interface.name.to_camel_case(),
                callback_name.to_mixed_case()
            ))?;
        }
    }

    Ok(())
}
