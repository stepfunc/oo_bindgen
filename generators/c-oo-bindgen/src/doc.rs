use crate::ctype::CType;
use oo_bindgen::doc::*;
use oo_bindgen::formatting::*;
use oo_bindgen::Library;

pub(crate) fn doxygen_print(
    f: &mut dyn Printer,
    doc: &Doc<Validated>,
    lib: &Library,
) -> FormattingResult<()> {
    f.writeln("@brief ")?;
    docstring_print(f, &doc.brief, lib)?;

    for detail in &doc.details {
        f.newline()?;

        match detail {
            DocParagraph::Details(docstring) => {
                f.newline()?;
                docstring_print(f, docstring, lib)?;
            }
            DocParagraph::Warning(docstring) => {
                f.writeln("@warning ")?;
                docstring_print(f, docstring, lib)?;
            }
        }
    }

    Ok(())
}

pub(crate) fn docstring_print(
    f: &mut dyn Printer,
    docstring: &DocString<Validated>,
    lib: &Library,
) -> FormattingResult<()> {
    for el in docstring.elements() {
        match el {
            DocStringElement::Text(text) => f.write(text)?,
            DocStringElement::Null => f.write("@p NULL")?,
            DocStringElement::Iterator => f.write("iterator")?,
            DocStringElement::Reference(reference) => reference_print(f, reference, lib)?,
        }
    }

    Ok(())
}

fn reference_print(
    f: &mut dyn Printer,
    reference: &Validated,
    lib: &Library,
) -> FormattingResult<()> {
    match reference {
        Validated::Argument(param_name) => f.write(&format!("@p {}", param_name))?,
        Validated::Class(class) => {
            f.write(&format!("@ref {}", class.to_c_type()))?;
        }
        Validated::ClassMethod(_, method_name, _) => {
            f.write(&format!(
                "@ref {}_{}",
                lib.settings.c_ffi_prefix, method_name
            ))?;
        }
        Validated::ClassConstructor(_, constructor) => {
            f.write(&format!(
                "@ref {}_{}",
                lib.settings.c_ffi_prefix, constructor.function.name
            ))?;
        }
        Validated::ClassDestructor(_, destructor) => {
            f.write(&format!(
                "@ref {}_{}",
                lib.settings.c_ffi_prefix, destructor.function.name
            ))?;
        }
        Validated::Struct(st) => {
            f.write(&format!("@ref {}", st.to_c_type()))?;
        }
        Validated::StructField(st, field_name) => {
            f.write(&format!("@ref {}.{}", st.to_c_type(), field_name))?;
        }
        Validated::Enum(handle) => {
            f.write(&format!("@ref {}", handle.to_c_type()))?;
        }
        Validated::EnumVariant(handle, variant_name) => {
            f.write(&format!(
                "@ref {}_{}_{}",
                lib.settings.c_ffi_prefix.capital_snake_case(),
                handle.name.capital_snake_case(),
                variant_name.capital_snake_case()
            ))?;
        }
        Validated::Interface(interface) => {
            f.write(&format!("@ref {}", interface.to_c_type()))?;
        }
        Validated::InterfaceMethod(interface, callback_name) => {
            f.write(&format!("@ref {}.{}", interface.to_c_type(), callback_name))?;
        }
    }

    Ok(())
}
