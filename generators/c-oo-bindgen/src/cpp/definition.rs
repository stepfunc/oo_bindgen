use crate::cpp::conversion::*;
use crate::cpp::formatting::{friend_class, namespace};
use heck::{CamelCase, SnakeCase};
use oo_bindgen::class::{
    AsyncMethod, ClassDeclarationHandle, ClassHandle, Method, StaticClassHandle,
};
use oo_bindgen::constants::{ConstantSetHandle, ConstantValue, Representation};
use oo_bindgen::enum_type::EnumHandle;
use oo_bindgen::error_type::ErrorType;
use oo_bindgen::formatting::{indented, FilePrinter, FormattingResult, Printer};
use oo_bindgen::function::FunctionArgument;
use oo_bindgen::interface::InterfaceHandle;
use oo_bindgen::iterator::IteratorHandle;
use oo_bindgen::structs::{
    Constructor, ConstructorType, Struct, StructDeclaration, StructFieldType, Visibility,
};
use oo_bindgen::types::Arg;
use oo_bindgen::{Handle, Library, Statement, StructType};
use std::path::Path;

pub(crate) fn generate_header(lib: &Library, path: &Path) -> FormattingResult<()> {
    // Open the file
    std::fs::create_dir_all(&path)?;
    let filename = path.join(format!("{}.hpp", lib.name));
    let mut f = FilePrinter::new(filename)?;

    // include guard
    f.writeln("#pragma once")?;
    f.newline()?;
    f.writeln("#include <cstdint>")?;
    f.writeln("#include <stdexcept>")?;
    f.writeln("#include <chrono>")?;
    f.writeln("#include <memory>")?;
    f.writeln("#include <vector>")?;
    f.newline()?;

    namespace(&mut f, &lib.c_ffi_prefix, |f| {
        print_header_namespace_contents(lib, f)
    })?;

    Ok(())
}

fn print_header_namespace_contents(lib: &Library, f: &mut dyn Printer) -> FormattingResult<()> {
    print_version(lib, f)?;
    f.newline()?;

    for statement in lib.statements() {
        match &statement {
            Statement::Constants(x) => print_constants(f, x)?,
            Statement::EnumDefinition(x) => print_enum(f, x)?,
            Statement::ErrorType(x) => print_exception(f, x)?,
            Statement::StructDeclaration(x) => print_struct_decl(f, x)?,
            Statement::StructDefinition(x) => match x {
                StructType::FunctionArg(x) => print_struct_definition(f, x)?,
                StructType::FunctionReturn(x) => print_struct_definition(f, x)?,
                StructType::CallbackArg(x) => print_struct_definition(f, x)?,
                StructType::Universal(x) => print_struct_definition(f, x)?,
            },
            Statement::InterfaceDefinition(x) => print_interface(f, x)?,
            Statement::ClassDeclaration(x) => print_class_decl(f, x)?,
            Statement::ClassDefinition(x) => print_class_definition(f, x)?,
            Statement::StaticClassDefinition(x) => print_static_class(f, x)?,
            Statement::IteratorDeclaration(x) => print_iterator_definition(f, x)?,
            Statement::CollectionDeclaration(_) => {
                // collections are just vectors in C++
            }
            Statement::FunctionDefinition(_) => {
                // not used in C++
            }
        }
    }

    Ok(())
}

fn print_iterator_definition(f: &mut dyn Printer, iter: &IteratorHandle) -> FormattingResult<()> {
    let iterator = include_str!("./snippet/iterator.hpp");
    for line in iterator.lines() {
        let substituted = line
            .replace("<name>", &iter.core_type())
            .replace("<snake_name>", &iter.core_type().to_snake_case())
            .replace("<iter_type>", &iter.item_type.core_type());
        f.writeln(&substituted)?;
    }
    f.newline()
}

fn print_class_decl(f: &mut dyn Printer, handle: &ClassDeclarationHandle) -> FormattingResult<()> {
    f.writeln(&format!("class {};", handle.core_type()))?;
    f.newline()
}

fn print_version(lib: &Library, f: &mut dyn Printer) -> FormattingResult<()> {
    let name = lib.c_ffi_prefix.to_snake_case();

    // Version number
    f.writeln(&format!(
        "constexpr uint64_t {}_version_major = {};",
        name, lib.version.major
    ))?;
    f.writeln(&format!(
        "constexpr uint64_t {}_version_minor = {};",
        name, lib.version.minor
    ))?;
    f.writeln(&format!(
        "constexpr uint64_t {}_version_patch = {};",
        name, lib.version.patch
    ))?;
    f.writeln(&format!(
        "constexpr char const* {}_version_string = \"{}\";",
        name,
        lib.version.to_string()
    ))?;
    f.newline()
}

fn print_constants(f: &mut dyn Printer, c: &ConstantSetHandle) -> FormattingResult<()> {
    fn get_value(v: ConstantValue) -> String {
        match v {
            ConstantValue::U8(v, Representation::Hex) => format!("0x{:02X}", v),
        }
    }

    fn get_type(v: ConstantValue) -> &'static str {
        match v {
            ConstantValue::U8(_, _) => "uint8_t",
        }
    }

    f.writeln(&format!("namespace {} {{", c.name.to_snake_case()))?;
    indented(f, |f| {
        for v in &c.values {
            f.writeln(&format!(
                "constexpr {} {} = {};",
                get_type(v.value),
                v.core_type(),
                get_value(v.value)
            ))?;
        }
        Ok(())
    })?;
    f.writeln("}")?;
    f.newline()
}

fn print_enum(f: &mut dyn Printer, e: &EnumHandle) -> FormattingResult<()> {
    f.writeln(&format!("enum class {} {{", e.core_type()))?;
    indented(f, |f| {
        for v in &e.variants {
            f.writeln(&format!("{} = {},", v.core_type(), v.value))?;
        }
        Ok(())
    })?;
    f.writeln("};")?;
    f.newline()?;
    f.writeln(&format!("const char* to_string({} value);", e.core_type()))?;
    f.newline()
}

fn print_exception(f: &mut dyn Printer, e: &ErrorType) -> FormattingResult<()> {
    f.writeln(&format!(
        "class {} : public std::logic_error {{",
        e.core_type()
    ))?;
    f.writeln("public:")?;
    indented(f, |f| {
        f.writeln("// underlying error enum")?;
        f.writeln(&format!("{} error;", e.inner.core_type()))?;
        f.writeln(&format!(
            "{}({} error);",
            e.core_type(),
            e.inner.core_type()
        ))?;
        Ok(())
    })?;
    f.writeln("};")?;
    f.newline()
}

fn print_struct_decl(f: &mut dyn Printer, s: &StructDeclaration) -> FormattingResult<()> {
    f.writeln(&format!("struct {};", s.core_type()))?;
    f.newline()
}

fn print_struct_definition<T>(
    f: &mut dyn Printer,
    handle: &Handle<Struct<T>>,
) -> FormattingResult<()>
where
    T: StructFieldType + CppStructType + CppFunctionArgType,
{
    f.writeln(&format!("struct {} {{", handle.core_type()))?;

    indented(f, |f| {
        f.writeln(&format!(
            "friend class {};",
            friend_class(handle.core_type())
        ))?;
        f.newline()
    })?;

    if let Visibility::Private = handle.visibility {
        f.writeln("private:")?;
        f.newline()?
    }

    if !handle.has_full_constructor() {
        if handle.visibility == Visibility::Public {
            f.writeln("private:")?;
        }
        indented(f, |f| {
            // write a default constructor
            let constructor = Handle::new(Constructor::full(
                "".to_string(),
                ConstructorType::Normal,
                "Fully initialize".into(),
            ));
            print_constructor_definition(f, handle, &constructor)
        })?;
        if handle.visibility == Visibility::Public {
            f.writeln("public:")?;
        }
    }

    indented(f, |f| {
        // delete the default constructor unless the struct has one
        if !handle.has_default_constructor() {
            f.writeln(&format!("{}() = delete;", handle.core_type()))?;
        }

        // write the constructors
        for c in &handle.constructors {
            f.newline()?;
            print_constructor_definition(f, handle, c)?;
        }

        f.newline()?;
        for field in &handle.fields {
            f.writeln(&format!(
                "{} {};",
                field.field_type.struct_member_type(),
                field.core_type()
            ))?;
        }

        Ok(())
    })?;

    f.writeln("};")?;
    f.newline()
}

fn print_interface(f: &mut dyn Printer, handle: &InterfaceHandle) -> FormattingResult<()> {
    f.writeln(&format!("class {} {{", handle.core_type()))?;
    f.writeln("public:")?;
    indented(f, |f| {
        f.writeln(&format!("virtual ~{}() = default;", handle.core_type()))?;
        f.newline()?;
        for cb in &handle.callbacks {
            let args: String = cb
                .arguments
                .iter()
                .map(|arg| {
                    format!(
                        "{} {}",
                        arg.arg_type.get_cpp_callback_arg_type(),
                        arg.core_type()
                    )
                })
                .collect::<Vec<String>>()
                .join(", ");

            f.writeln(&format!(
                "virtual {} {}({}) = 0;",
                cb.return_type.get_cpp_callback_return_type(),
                cb.core_type(),
                args
            ))?;
        }
        Ok(())
    })?;
    f.writeln("};")?;
    f.newline()
}
fn print_constructor_definition<T>(
    f: &mut dyn Printer,
    handle: &Handle<Struct<T>>,
    constructor: &Handle<Constructor>,
) -> FormattingResult<()>
where
    T: StructFieldType + CppFunctionArgType,
{
    let args = handle
        .constructor_args(constructor.clone())
        .map(|x| {
            format!(
                "{} {}",
                x.field_type.get_cpp_function_arg_type(),
                x.name.to_snake_case()
            )
        })
        .collect::<Vec<String>>()
        .join(", ");

    match constructor.constructor_type {
        ConstructorType::Normal => f.writeln(&format!("{}({});", handle.core_type(), args))?,
        ConstructorType::Static => f.writeln(&format!(
            "static {} {}({});",
            handle.core_type(),
            constructor.name.to_camel_case(),
            args
        ))?,
    }

    f.newline()
}

fn print_class_definition(f: &mut dyn Printer, handle: &ClassHandle) -> FormattingResult<()> {
    f.writeln(&format!("class {} {{", handle.core_type()))?;
    indented(f, |f| {
        f.writeln(&format!(
            "friend class {};",
            friend_class(handle.core_type())
        ))?;
        f.writeln("// pointer to the underlying C type")?;
        f.writeln("void* self;")?;
        f.writeln("// constructor only accessible internally")?;
        f.writeln(&format!(
            "{}(void* self): self(self) {{}}",
            handle.core_type()
        ))?;
        print_deleted_copy_and_assignment(f, &handle.core_type())
    })?;
    f.newline()?;
    f.writeln("public:")?;
    indented(f, |f| {
        if let Some(x) = &handle.constructor {
            let args = cpp_arguments(x.parameters.iter());
            f.writeln(&format!("{}({});", handle.core_type(), args))?;
        };
        if handle.destructor.is_some() {
            f.writeln(&format!("~{}();", handle.core_type()))?;
        };

        for method in &handle.methods {
            f.newline()?;
            print_method(f, method)?;
        }

        for method in &handle.static_methods {
            print_static_method(f, method)?;
        }

        for method in &handle.async_methods {
            print_async_method(f, method)?;
        }

        Ok(())
    })?;
    f.writeln("};")?;
    f.newline()
}

fn print_method(f: &mut dyn Printer, method: &Method) -> FormattingResult<()> {
    let args = cpp_arguments(method.native_function.parameters.iter().skip(1));

    f.writeln(&format!(
        "{} {}({});",
        method
            .native_function
            .return_type
            .get_cpp_function_return_type(),
        method.name.to_snake_case(),
        args
    ))
}

fn print_static_method(f: &mut dyn Printer, method: &Method) -> FormattingResult<()> {
    let args = cpp_arguments(method.native_function.parameters.iter());

    f.writeln(&format!(
        "static {} {}({});",
        method
            .native_function
            .return_type
            .get_cpp_function_return_type(),
        method.name.to_snake_case(),
        args
    ))
}

fn print_async_method(f: &mut dyn Printer, method: &AsyncMethod) -> FormattingResult<()> {
    let args: String = cpp_arguments(method.native_function.parameters.iter().skip(1));

    f.writeln(&format!(
        "{} {}({});",
        method
            .native_function
            .return_type
            .get_cpp_function_return_type(),
        method.name.to_snake_case(),
        args
    ))
}

fn print_static_class(f: &mut dyn Printer, handle: &StaticClassHandle) -> FormattingResult<()> {
    f.writeln(&format!("class {} {{", handle.core_type()))?;
    indented(f, |f| {
        f.writeln(&format!("{}() = delete;", handle.core_type()))
    })?;
    f.writeln("public:")?;
    indented(f, |f| {
        for method in &handle.static_methods {
            print_static_method(f, method)?;
        }
        Ok(())
    })?;
    f.writeln("};")?;
    f.newline()
}

fn print_deleted_copy_and_assignment(f: &mut dyn Printer, name: &str) -> FormattingResult<()> {
    f.writeln("// non-copyable")?;
    f.writeln(&format!("{}(const {}&) = delete;", name, name))?;
    f.writeln(&format!("{}& operator=(const {}&) = delete;", name, name))
}

fn cpp_arguments<'a, T>(iter: T) -> String
where
    T: Iterator<Item = &'a Arg<FunctionArgument>>,
{
    iter.map(|p| {
        format!(
            "{} {}",
            p.arg_type.get_cpp_function_arg_type(),
            p.core_type(),
        )
    })
    .collect::<Vec<String>>()
    .join(", ")
}
