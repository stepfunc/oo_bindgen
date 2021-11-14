use crate::cpp::conversion::CoreCppType;
use crate::doc::{docstring_print_generic, doxygen_print_generic};
use oo_bindgen::class::{Method, StaticMethod};
use oo_bindgen::doc::*;
use oo_bindgen::formatting::*;
use oo_bindgen::function::{ClassConstructor, Function, FutureMethod};
use oo_bindgen::return_type::OptionalReturnType;
use oo_bindgen::types::Arg;

pub(crate) fn print_cpp_doc(f: &mut dyn Printer, doc: &Doc<Validated>) -> FormattingResult<()> {
    doxygen_print_generic(f, print_cpp_reference, doc)
}

pub(crate) fn print_cpp_method_docs(
    f: &mut dyn Printer,
    method: &Method<Validated>,
) -> FormattingResult<()> {
    print_cpp_function_docs(f, &method.native_function, true, true)
}

pub(crate) fn print_cpp_static_method_docs(
    f: &mut dyn Printer,
    method: &StaticMethod<Validated>,
) -> FormattingResult<()> {
    print_cpp_function_docs(f, &method.native_function, false, true)
}

pub(crate) fn print_cpp_future_method_docs(
    f: &mut dyn Printer,
    method: &FutureMethod<Validated>,
) -> FormattingResult<()> {
    print_cpp_function_docs(f, &method.native_function, true, true)
}

pub(crate) fn print_cpp_constructor_docs(
    f: &mut dyn Printer,
    constructor: &ClassConstructor<Validated>,
) -> FormattingResult<()> {
    print_cpp_function_docs(f, &constructor.function, false, false)
}

fn print_cpp_function_docs(
    f: &mut dyn Printer,
    function: &Function<Validated>,
    is_instance_method: bool,
    print_return: bool,
) -> FormattingResult<()> {
    doxygen(f, |f| {
        print_cpp_doc(f, &function.doc)?;
        f.newline()?;
        if is_instance_method {
            for arg in function.arguments.iter().skip(1) {
                f.newline()?;
                print_cpp_argument_doc(f, arg)?;
            }
        } else {
            for arg in function.arguments.iter() {
                f.newline()?;
                print_cpp_argument_doc(f, arg)?;
            }
        }
        if print_return {
            print_cpp_return_type_doc(f, &function.return_type)?;
        }
        if let Some(err) = &function.error_type.get() {
            f.writeln(&format!("@throws {}", err.exception_name.camel_case()))?;
        }
        Ok(())
    })
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
