use oo_bindgen::backend::*;
use oo_bindgen::model::*;

use crate::cpp::conversion::{CoreCppType, CppFunctionArgType};
use crate::doc::{docstring_print_generic, doxygen_print_generic};

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

fn highlight(expr: String) -> String {
    format!("<em>{}</em>", expr)
}

fn print_cpp_reference(f: &mut dyn Printer, reference: &Validated) -> FormattingResult<()> {
    match reference {
        Validated::Argument(param_name) => f.write(&format!("@p {}", param_name))?,
        Validated::Class(class) => {
            f.write(&format!("@ref {}", class.core_cpp_type()))?;
        }
        Validated::ClassMethod(class, method_name, _) => {
            // explicit links to class methods are broken when they take parameters, but implicit links always work
            // since we don't allow overloading in the model, we don't need to reference parameters
            f.write(&format!("{}::{}()", class.core_cpp_type(), method_name))?;
        }
        Validated::ClassConstructor(class, constructor) => {
            let cpp_type = class.core_cpp_type();
            let args = constructor
                .function
                .arguments
                .iter()
                .map(|x| x.arg_type.get_cpp_function_arg_type())
                .collect::<Vec<String>>()
                .join(",");

            f.write(&format!("@ref {}::{}({})", cpp_type, cpp_type, args))?;
        }
        Validated::ClassDestructor(class, _) => {
            let cpp_type = class.core_cpp_type();

            /*
               Explicit links to destructors are just plain broken in doxygen v1.9.2
               It generates correct links however without an explicit @ref or #!
            */
            f.write(&format!("{}::~{}()", cpp_type, cpp_type))?;
        }
        Validated::Struct(st) => {
            // explicit links to structs don't always work :(
            f.write(&st.core_cpp_type())?;
        }
        Validated::StructField(st, field_name) => {
            f.write(&format!("@ref {}.{}", st.core_cpp_type(), field_name))?;
        }
        Validated::Enum(handle) => {
            /*
                Links to enum classes (explicit and implicit) are hopelessly broken in doxygen v1.9.2

                Just use italic text for now
            */
            f.write(&highlight(handle.core_cpp_type()))?;
        }
        Validated::EnumVariant(handle, variant_name) => {
            /*
                Links to enum variants (explicit and implicit) are hopelessly broken in doxygen v1.9.2

                Just use italic text for now
            */
            f.write(&highlight(format!(
                "{}::{}",
                handle.core_cpp_type(),
                variant_name
            )))?;
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
