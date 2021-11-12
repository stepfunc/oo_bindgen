use crate::ctype::CType;
use oo_bindgen::doc::*;
use oo_bindgen::formatting::*;

pub(crate) type ReferencePrinter =
    fn(f: &mut dyn Printer, reference: &Validated) -> FormattingResult<()>;

pub(crate) fn doxygen_print(f: &mut dyn Printer, doc: &Doc<Validated>) -> FormattingResult<()> {
    doxygen_print_generic(f, print_c_reference, doc)
}

pub(crate) fn docstring_print(
    f: &mut dyn Printer,
    docstring: &DocString<Validated>,
) -> FormattingResult<()> {
    docstring_print_generic(f, print_c_reference, docstring)
}

pub(crate) fn doxygen_print_generic(
    f: &mut dyn Printer,
    print_reference: ReferencePrinter,
    doc: &Doc<Validated>,
) -> FormattingResult<()> {
    f.writeln("@brief ")?;
    docstring_print_generic(f, print_reference, &doc.brief)?;

    for detail in &doc.details {
        f.newline()?;

        match detail {
            DocParagraph::Details(docstring) => {
                f.newline()?;
                docstring_print_generic(f, print_reference, docstring)?;
            }
            DocParagraph::Warning(docstring) => {
                f.writeln("@warning ")?;
                docstring_print_generic(f, print_reference, docstring)?;
            }
        }
    }

    Ok(())
}

pub(crate) fn docstring_print_generic(
    f: &mut dyn Printer,
    print_reference: ReferencePrinter,
    docstring: &DocString<Validated>,
) -> FormattingResult<()> {
    for el in docstring.elements() {
        match el {
            DocStringElement::Text(text) => f.write(text)?,
            DocStringElement::Null => f.write("@p NULL")?,
            DocStringElement::Iterator => f.write("iterator")?,
            DocStringElement::Reference(reference) => print_reference(f, reference)?,
        }
    }

    Ok(())
}

fn print_c_reference(f: &mut dyn Printer, reference: &Validated) -> FormattingResult<()> {
    match reference {
        Validated::Argument(param_name) => f.write(&format!("@p {}", param_name))?,
        Validated::Class(class) => {
            f.write(&format!("@ref {}", class.to_c_type()))?;
        }
        Validated::ClassMethod(class, method_name, _) => {
            f.write(&format!(
                "@ref {}_{}",
                class.settings.c_ffi_prefix, method_name
            ))?;
        }
        Validated::ClassConstructor(class, constructor) => {
            f.write(&format!(
                "@ref {}_{}",
                class.settings.c_ffi_prefix, constructor.function.name
            ))?;
        }
        Validated::ClassDestructor(class, destructor) => {
            f.write(&format!(
                "@ref {}_{}",
                class.settings.c_ffi_prefix, destructor.function.name
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
                handle.settings.c_ffi_prefix.capital_snake_case(),
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
