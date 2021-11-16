use oo_bindgen::backend::*;
use oo_bindgen::model::*;

use crate::dotnet_type::*;

pub(crate) fn xmldoc_print(f: &mut dyn Printer, doc: &Doc<Validated>) -> FormattingResult<()> {
    f.writeln("<summary>")?;
    docstring_print(f, &doc.brief)?;
    f.write("</summary>")?;

    if !doc.details.is_empty() {
        f.writeln("<remarks>")?;
        for detail in &doc.details {
            match detail {
                DocParagraph::Details(docstring) => {
                    f.writeln("<para>")?;
                    docstring_print(f, docstring)?;
                    f.write("</para>")?;
                }
                DocParagraph::Warning(docstring) => {
                    f.writeln("<para><b>Warning:</b> ")?;
                    docstring_print(f, docstring)?;
                    f.write("</para>")?;
                }
            }
        }
        f.writeln("</remarks>")?;
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
            DocStringElement::Null => f.write("<c>null</c>")?,
            DocStringElement::Iterator => f.write("collection")?,
            DocStringElement::Reference(reference) => reference_print(f, reference)?,
        }
    }

    Ok(())
}

fn reference_print(f: &mut dyn Printer, reference: &Validated) -> FormattingResult<()> {
    match reference {
        Validated::Argument(param_name) => {
            f.write(&format!("<c>{}</c>", param_name.mixed_case()))?
        }
        Validated::Class(class) => {
            f.write(&format!("<see cref=\"{}\" />", class.name.camel_case()))?;
        }
        Validated::ClassMethod(class, method_name, _) => f.write(&format!(
            "<see cref=\"{}.{}\" />",
            class.name().camel_case(),
            method_name.camel_case()
        ))?,
        Validated::ClassConstructor(class, constructor) => {
            let params = constructor
                .function
                .arguments
                .iter()
                .map(|param| param.arg_type.as_dotnet_type())
                .collect::<Vec<_>>()
                .join(", ");

            let class_name = class.name().camel_case();
            f.write(&format!(
                "<see cref=\"{}.{}({})\" />",
                class_name, class_name, params
            ))?;
        }
        Validated::ClassDestructor(class, _) => {
            let method_name = if let DestructionMode::Custom(name) = &class.destruction_mode {
                name.camel_case()
            } else {
                "Dispose".to_string()
            };

            f.write(&format!(
                "<see cref=\"{}.{}()\" />",
                class.name().camel_case(),
                method_name,
            ))?;
        }
        Validated::Struct(st) => {
            f.write(&format!("<see cref=\"{}\" />", st.name().camel_case()))?;
        }
        Validated::StructField(st, field_name) => {
            f.write(&format!(
                "<see cref=\"{}.{}\" />",
                st.name().camel_case(),
                field_name.camel_case()
            ))?;
        }
        Validated::Enum(handle) => {
            f.write(&format!("<see cref=\"{}\" />", handle.name.camel_case()))?;
        }
        Validated::EnumVariant(handle, variant) => {
            f.write(&format!(
                "<see cref=\"{}.{}\" />",
                handle.name.camel_case(),
                variant.camel_case()
            ))?;
        }
        Validated::Interface(interface) => {
            f.write(&format!(
                "<see cref=\"I{}\" />",
                interface.name.camel_case()
            ))?;
        }
        Validated::InterfaceMethod(interface, callback_name) => {
            f.write(&format!(
                "<see cref=\"I{}.{}\" />",
                interface.name.camel_case(),
                callback_name.camel_case()
            ))?;
        }
    }

    Ok(())
}
