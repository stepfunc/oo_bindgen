use crate::dotnet_type::*;
use heck::{CamelCase, MixedCase};
use oo_bindgen::class::DestructionMode;
use oo_bindgen::doc::*;
use oo_bindgen::formatting::*;

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
        Validated::Param(param_name) => {
            f.write(&format!("<c>{}</c>", param_name.to_mixed_case()))?
        }
        Validated::Class(class) => {
            f.write(&format!("<see cref=\"{}\" />", class.name.to_camel_case()))?;
        }
        Validated::ClassMethod(class, method) => f.write(&format!(
            "<see cref=\"{}.{}\" />",
            class.name().to_camel_case(),
            method.name.to_camel_case()
        ))?,
        Validated::ClassConstructor(class, constructor) => {
            let params = constructor
                .parameters
                .iter()
                .map(|param| param.arg_type.as_dotnet_type())
                .collect::<Vec<_>>()
                .join(", ");

            let class_name = class.name().to_camel_case();
            f.write(&format!(
                "<see cref=\"{}.{}({})\" />",
                class_name, class_name, params
            ))?;
        }
        Validated::ClassDestructor(class, _) => {
            let method_name = if let DestructionMode::Custom(name) = &class.destruction_mode {
                name.to_camel_case()
            } else {
                "Dispose".to_string()
            };

            f.write(&format!(
                "<see cref=\"{}.{}()\" />",
                class.name().to_camel_case(),
                method_name,
            ))?;
        }
        Validated::Struct(st) => {
            f.write(&format!("<see cref=\"{}\" />", st.name().to_camel_case()))?;
        }
        Validated::StructField(st, field_name) => {
            f.write(&format!(
                "<see cref=\"{}.{}\" />",
                st.name().to_camel_case(),
                field_name.to_camel_case()
            ))?;
        }
        Validated::Enum(handle) => {
            f.write(&format!("<see cref=\"{}\" />", handle.name.to_camel_case()))?;
        }
        Validated::EnumVariant(handle, variant) => {
            f.write(&format!(
                "<see cref=\"{}.{}\" />",
                handle.name.to_camel_case(),
                variant.to_camel_case()
            ))?;
        }
        Validated::Interface(interface) => {
            f.write(&format!(
                "<see cref=\"I{}\" />",
                interface.name.to_camel_case()
            ))?;
        }
        Validated::InterfaceMethod(interface, callback_name) => {
            f.write(&format!(
                "<see cref=\"I{}.{}\" />",
                interface.name.to_camel_case(),
                callback_name.to_camel_case()
            ))?;
        }
    }

    Ok(())
}
