use crate::conversion::*;
use heck::{CamelCase, MixedCase};
use oo_bindgen::doc::*;
use oo_bindgen::formatting::*;
use oo_bindgen::Library;

pub(crate) fn xmldoc_print(f: &mut dyn Printer, doc: &Doc, lib: &Library) -> FormattingResult<()> {
    f.writeln("<summary>")?;
    docstring_print(f, &doc.brief, lib)?;
    f.write("</summary>")?;

    if !doc.details.is_empty() {
        f.writeln("<remarks>")?;
        for detail in &doc.details {
            match detail {
                DocParagraph::Details(docstring) => {
                    f.writeln("<para>")?;
                    docstring_print(f, docstring, lib)?;
                    f.write("</para>")?;
                }
                DocParagraph::Warning(docstring) => {
                    f.writeln("<para><b>Warning:</b> ")?;
                    docstring_print(f, docstring, lib)?;
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
    docstring: &DocString,
    lib: &Library,
) -> FormattingResult<()> {
    for el in docstring.elements() {
        match el {
            DocStringElement::Text(text) => f.write(text)?,
            DocStringElement::Null => f.write("<c>null</c>")?,
            DocStringElement::Iterator => f.write("collection")?,
            DocStringElement::Reference(reference) => reference_print(f, reference, lib)?,
        }
    }

    Ok(())
}

fn reference_print(
    f: &mut dyn Printer,
    reference: &DocReference,
    lib: &Library,
) -> FormattingResult<()> {
    match reference {
        DocReference::Param(param_name) => {
            f.write(&format!("<c>{}</c>", param_name.to_mixed_case()))?
        }
        DocReference::Class(class_name) => {
            f.write(&format!("<see cref=\"{}\" />", class_name.to_camel_case()))?;
        }
        DocReference::ClassMethod(class_name, method_name) => {
            f.write(&format!(
                "<see cref=\"{}.{}\" />",
                class_name.to_camel_case(),
                method_name.to_camel_case()
            ))?;
        }
        DocReference::ClassConstructor(class_name) => {
            let func = lib
                .find_class(class_name)
                .unwrap()
                .constructor
                .as_ref()
                .unwrap();
            let params = func
                .parameters
                .iter()
                .map(|param| param.param_type.as_dotnet_type())
                .collect::<Vec<_>>()
                .join(", ");
            f.write(&format!(
                "<see cref=\"{}.{}({})\" />",
                class_name, class_name, params
            ))?;
        }
        DocReference::ClassDestructor(class_name) => {
            f.write(&format!(
                "<see cref=\"{}.Dispose()\" />",
                class_name.to_camel_case()
            ))?;
        }
        DocReference::Struct(struct_name) => {
            f.write(&format!("<see cref=\"{}\" />", struct_name.to_camel_case()))?;
        }
        DocReference::StructMethod(struct_name, method_name) => {
            f.write(&format!(
                "<see cref=\"{}.{}\" />",
                struct_name.to_camel_case(),
                method_name.to_camel_case()
            ))?;
        }
        DocReference::StructElement(struct_name, element_name) => {
            f.write(&format!(
                "<see cref=\"{}.{}\" />",
                struct_name.to_camel_case(),
                element_name.to_camel_case()
            ))?;
        }
        DocReference::Enum(enum_name) => {
            f.write(&format!("<see cref=\"{}\" />", enum_name.to_camel_case()))?;
        }
        DocReference::EnumVariant(enum_name, variant_name) => {
            f.write(&format!(
                "<see cref=\"{}.{}\" />",
                enum_name.to_camel_case(),
                variant_name.to_camel_case()
            ))?;
        }
        DocReference::Interface(interface_name) => {
            f.write(&format!(
                "<see cref=\"I{}\" />",
                interface_name.to_camel_case()
            ))?;
        }
        DocReference::InterfaceMethod(interface_name, callback_name) => {
            f.write(&format!(
                "<see cref=\"I{}.{}\" />",
                interface_name.to_camel_case(),
                callback_name.to_camel_case()
            ))?;
        }
    }

    Ok(())
}
