use crate::cpp::conversion::CoreCppType;
use crate::doc::{docstring_print_generic, doxygen_print_generic};
use oo_bindgen::doc::*;
use oo_bindgen::formatting::*;
use oo_bindgen::return_type::OptionalReturnType;
use oo_bindgen::types::Arg;

pub(crate) fn print_cpp_doc(f: &mut dyn Printer, doc: &Doc<Validated>) -> FormattingResult<()> {
    doxygen_print_generic(f, print_cpp_reference, doc)
}

pub(crate) fn print_cpp_argument_doc<T>(
    f: &mut dyn Printer,
    arg: &Arg<T, Validated>,
) -> FormattingResult<()>
where
    T: Clone,
{
    f.write(&format!("@param {} ", arg.name))?;
    print_cpp_doc_string(f, &arg.doc)
}

pub(crate) fn print_cpp_return_type_doc<T>(
    f: &mut dyn Printer,
    rt: &OptionalReturnType<T, Validated>,
) -> FormattingResult<()>
where
    T: Clone,
{
    if let Some(d) = &rt.get_doc() {
        f.newline()?;
        f.write("@return ")?;
        print_cpp_doc_string(f, d)?;
    }
    Ok(())
}

pub(crate) fn print_commented_cpp_doc(
    f: &mut dyn Printer,
    doc: &Doc<Validated>,
) -> FormattingResult<()> {
    doxygen(f, |f| print_cpp_doc(f, doc))
}

pub(crate) fn print_cpp_doc_string(
    f: &mut dyn Printer,
    docstring: &DocString<Validated>,
) -> FormattingResult<()> {
    docstring_print_generic(f, print_cpp_reference, docstring)
}

fn print_cpp_reference(f: &mut dyn Printer, reference: &Validated) -> FormattingResult<()> {
    match reference {
        Validated::Argument(param_name) => f.write(&format!("@p {}", param_name))?,
        Validated::Class(class) => {
            f.write(&format!("@ref {}", class.core_cpp_type()))?;
        }
        Validated::ClassMethod(class, method_name, _) => {
            f.write(&format!(
                "@ref {}::{}()",
                class.core_cpp_type(),
                method_name
            ))?;
        }
        Validated::ClassConstructor(class, _) => {
            f.write(&format!(
                "@ref {}::{}()",
                class.core_cpp_type(),
                class.core_cpp_type()
            ))?;
        }
        Validated::ClassDestructor(class, _) => {
            f.write(&format!(
                "@ref {}::~{}",
                class.core_cpp_type(),
                class.core_cpp_type()
            ))?;
        }
        Validated::Struct(st) => {
            f.write(&format!("@ref {}", st.core_cpp_type()))?;
        }
        Validated::StructField(st, field_name) => {
            f.write(&format!("@ref {}.{}", st.core_cpp_type(), field_name))?;
        }
        Validated::Enum(handle) => {
            f.write(&format!("@ref {}", handle.core_cpp_type()))?;
        }
        Validated::EnumVariant(handle, variant_name) => {
            let variant = format!("{}::{}", handle.core_cpp_type(), variant_name);
            f.write(&format!("@ref {}", variant))?;
        }
        Validated::Interface(interface) => {
            f.write(&format!("@ref {}", interface.core_cpp_type()))?;
        }
        Validated::InterfaceMethod(interface, callback_name) => {
            f.write(&format!(
                "@ref {}::{}()",
                interface.core_cpp_type(),
                callback_name
            ))?;
        }
    }

    Ok(())
}
