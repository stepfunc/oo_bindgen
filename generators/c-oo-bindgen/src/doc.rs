use crate::ctype::CType;
use heck::{ShoutySnakeCase, SnakeCase};
use oo_bindgen::doc::*;
use oo_bindgen::formatting::*;
use oo_bindgen::Library;

pub(crate) fn doxygen_print(f: &mut dyn Printer, doc: &Doc, lib: &Library) -> FormattingResult<()> {
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
    docstring: &DocString,
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
    reference: &DocReference,
    lib: &Library,
) -> FormattingResult<()> {
    match reference {
        DocReference::Param(param_name) => {
            f.write(&format!("@p {}", param_name.to_snake_case()))?
        }
        DocReference::Class(class_name) => {
            let handle = lib.find_class_declaration(class_name).unwrap();
            f.write(&format!("@ref {}", handle.to_c_type()))?;
        }
        DocReference::ClassMethod(class_name, method_name) => {
            let func_name = &lib
                .find_class(class_name)
                .unwrap()
                .find_method(method_name)
                .unwrap()
                .name;
            f.write(&format!("@ref {}_{}", lib.settings.c_ffi_prefix, func_name))?;
        }
        DocReference::ClassConstructor(class_name) => {
            let handle = lib.find_class(class_name).unwrap();
            f.write(&format!(
                "@ref {}_{}",
                lib.settings.c_ffi_prefix,
                handle.constructor.as_ref().unwrap().name
            ))?;
        }
        DocReference::ClassDestructor(class_name) => {
            let handle = lib.find_class(class_name).unwrap();
            f.write(&format!(
                "@ref {}_{}",
                lib.settings.c_ffi_prefix,
                handle.destructor.as_ref().unwrap().name
            ))?;
        }
        DocReference::Struct(struct_name) => {
            let struct_name = lib.find_struct(struct_name).unwrap().declaration();
            f.write(&format!("@ref {}", struct_name.to_c_type()))?;
        }
        DocReference::StructElement(struct_name, element_name) => {
            let handle = lib.find_struct(struct_name).unwrap();
            f.write(&format!(
                "@ref {}.{}",
                handle.to_c_type(),
                element_name.to_snake_case()
            ))?;
        }
        DocReference::Enum(enum_name) => {
            let enum_name = lib.find_enum(enum_name).unwrap();
            f.write(&format!("@ref {}", enum_name.to_c_type()))?;
        }
        DocReference::EnumVariant(enum_name, variant_name) => {
            let handle = lib.find_enum(enum_name).unwrap();
            f.write(&format!(
                "@ref {}_{}_{}",
                lib.settings.c_ffi_prefix.to_shouty_snake_case(),
                handle.name.to_shouty_snake_case(),
                variant_name.to_shouty_snake_case()
            ))?;
        }
        DocReference::Interface(interface_name) => {
            let handle = lib.find_interface(interface_name).unwrap();
            f.write(&format!("@ref {}", handle.to_c_type()))?;
        }
        DocReference::InterfaceMethod(interface_name, callback_name) => {
            let handle = &lib.find_interface(interface_name).unwrap();
            f.write(&format!(
                "@ref {}.{}",
                handle.to_c_type(),
                callback_name.to_snake_case()
            ))?;
        }
    }

    Ok(())
}
