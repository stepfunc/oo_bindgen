mod formatting;
mod names;
mod types;

use heck::{CamelCase, ShoutySnakeCase, SnakeCase};
use oo_bindgen::callback::{
    CallbackFunction, CallbackParameter, InterfaceElement, InterfaceHandle,
};
use oo_bindgen::constants::{ConstantSetHandle, ConstantValue, Representation};
use oo_bindgen::error_type::ErrorType;
use oo_bindgen::formatting::{indented, FilePrinter, FormattingResult, Printer};
use oo_bindgen::iterator::IteratorHandle;
use oo_bindgen::native_enum::NativeEnumHandle;
use oo_bindgen::native_struct::{
    NativeStructDeclaration, NativeStructElement, NativeStructHandle, NativeStructType,
    StructElementType,
};
use oo_bindgen::{Library, Statement};
use std::path::Path;

use crate::CFormatting;
use formatting::namespace;
use names::*;
use oo_bindgen::class::{
    AsyncMethod, ClassDeclarationHandle, ClassHandle, Method, StaticClassHandle,
};
use oo_bindgen::native_function::{NativeFunctionHandle, Parameter, ReturnType, Type};
use oo_bindgen::types::{BasicType, DurationType};
use types::*;

const FRIEND_CLASS_NAME: &str = "InternalFriendClass";

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

fn print_enum(f: &mut dyn Printer, e: &NativeEnumHandle) -> FormattingResult<()> {
    f.writeln(&format!("enum class {} {{", e.cpp_name()))?;
    indented(f, |f| {
        for v in &e.variants {
            f.writeln(&format!("{} = {},", v.cpp_name(), v.value))?;
        }
        Ok(())
    })?;
    f.writeln("};")?;
    f.newline()?;
    f.writeln(&format!("const char* to_string({} value);", e.cpp_name()))?;
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
                v.cpp_name(),
                get_value(v.value)
            ))?;
        }
        Ok(())
    })?;
    f.writeln("}")?;
    f.newline()
}

fn print_exception(f: &mut dyn Printer, e: &ErrorType) -> FormattingResult<()> {
    f.writeln(&format!(
        "class {} : public std::logic_error {{",
        e.cpp_name()
    ))?;
    f.writeln("public:")?;
    indented(f, |f| {
        f.writeln("// underlying error enum")?;
        f.writeln(&format!("{} error;", e.inner.cpp_name()))?;
        f.writeln(&format!("{}({} error);", e.cpp_name(), e.inner.cpp_name()))?;
        Ok(())
    })?;
    f.writeln("};")?;
    f.newline()
}

fn print_native_struct_decl(
    f: &mut dyn Printer,
    s: &NativeStructDeclaration,
) -> FormattingResult<()> {
    f.writeln(&format!("struct {};", s.cpp_name()))?;
    f.newline()
}

fn get_struct_default_constructor_args(handle: &NativeStructHandle) -> String {
    handle
        .elements
        .iter()
        .flat_map(|x| {
            if x.element_type.has_default() {
                None
            } else {
                Some(format!(
                    "{} {}",
                    x.element_type.to_type().get_cpp_struct_constructor_type(),
                    x.name
                ))
            }
        })
        .collect::<Vec<String>>()
        .join(", ")
}

fn get_struct_full_constructor_args(handle: &NativeStructHandle) -> String {
    handle
        .elements
        .iter()
        .map(|x| {
            format!(
                "{} {}",
                x.element_type.to_type().get_cpp_struct_constructor_type(),
                x.name
            )
        })
        .collect::<Vec<String>>()
        .join(", ")
}

fn print_native_struct_header(
    f: &mut dyn Printer,
    handle: &NativeStructHandle,
    _lib: &Library,
) -> FormattingResult<()> {
    f.writeln(&format!("struct {} {{", handle.cpp_name()))?;
    f.writeln(&format!("    friend class {};", FRIEND_CLASS_NAME))?;
    if let NativeStructType::Opaque = handle.struct_type {
        f.writeln("private:")?;
    }
    indented(f, |f| {
        // delete the default constructor unless all fields have default values in which case it'll
        // be written below
        if !handle.all_fields_have_defaults() {
            f.writeln(&format!("{}() = delete;", handle.cpp_name()))?;
        }

        // constructor that applies the default values
        f.writeln(&format!(
            "{}({});",
            handle.cpp_name(),
            get_struct_default_constructor_args(handle)
        ))?;
        f.newline()?;

        // write a full constructor unless the one above already takes all the values
        if !handle.no_fields_have_defaults() {
            f.writeln(&format!(
                "{}({});",
                handle.cpp_name(),
                get_struct_full_constructor_args(handle)
            ))?;
            f.newline()?;
        }

        for field in &handle.elements {
            f.writeln(&format!(
                "{} {};",
                field.element_type.to_type().get_cpp_struct_member_type(),
                field.cpp_name()
            ))?;
        }

        Ok(())
    })?;

    f.writeln("};")?;
    f.newline()
}

fn print_interface(f: &mut dyn Printer, handle: &InterfaceHandle) -> FormattingResult<()> {
    f.writeln(&format!("class {} {{", handle.cpp_name()))?;
    f.writeln("public:")?;
    indented(f, |f| {
        f.writeln(&format!("virtual ~{}() = default;", handle.cpp_name()))?;
        f.newline()?;
        for elem in &handle.elements {
            if let InterfaceElement::CallbackFunction(func) = elem {
                let args: String = func
                    .parameters
                    .iter()
                    .flat_map(|p| match p {
                        CallbackParameter::Arg(_) => None,
                        CallbackParameter::Parameter(p) => Some(format!(
                            "{} {}",
                            p.param_type.get_cpp_func_argument_type(),
                            p.cpp_name()
                        )),
                    })
                    .collect::<Vec<String>>()
                    .join(", ");

                f.writeln(&format!(
                    "virtual {} {}({}) = 0;",
                    func.return_type.get_cpp_return_type(),
                    func.cpp_name(),
                    args
                ))?;
            }
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

fn print_class_decl(f: &mut dyn Printer, handle: &ClassDeclarationHandle) -> FormattingResult<()> {
    f.writeln(&format!("class {};", handle.cpp_name()))?;
    f.newline()
}

fn cpp_arguments<'a, T>(iter: T) -> String
where
    T: Iterator<Item = &'a Parameter>,
{
    iter.map(|p| {
        format!(
            "{} {}",
            p.param_type.get_cpp_func_argument_type(),
            p.cpp_name()
        )
    })
    .collect::<Vec<String>>()
    .join(", ")
}

fn print_method(f: &mut dyn Printer, method: &Method) -> FormattingResult<()> {
    let args = cpp_arguments(method.native_function.parameters.iter().skip(1));

    f.writeln(&format!(
        "{} {}({});",
        method.native_function.return_type.get_cpp_return_type(),
        method.cpp_name(),
        args
    ))
}

fn print_static_method(f: &mut dyn Printer, method: &Method) -> FormattingResult<()> {
    let args = cpp_arguments(method.native_function.parameters.iter());

    f.writeln(&format!(
        "static {} {}({});",
        method.native_function.return_type.get_cpp_return_type(),
        method.cpp_name(),
        args
    ))
}

fn print_async_method(f: &mut dyn Printer, method: &AsyncMethod) -> FormattingResult<()> {
    let args: String = cpp_arguments(method.native_function.parameters.iter().skip(1));

    f.writeln(&format!(
        "{} {}({});",
        method.native_function.return_type.get_cpp_return_type(),
        method.cpp_name(),
        args
    ))
}

fn print_class(f: &mut dyn Printer, handle: &ClassHandle) -> FormattingResult<()> {
    f.writeln(&format!("class {} {{", handle.cpp_name()))?;
    indented(f, |f| {
        f.writeln(&format!("friend class {};", FRIEND_CLASS_NAME))?;
        f.writeln("// pointer to the underlying C type")?;
        f.writeln("void* self;")?;
        f.writeln("// constructor only accessible internally")?;
        f.writeln(&format!(
            "{}(void* self): self(self) {{}}",
            handle.cpp_name()
        ))?;
        print_deleted_copy_and_assignment(f, &handle.cpp_name())
    })?;
    f.newline()?;
    f.writeln("public:")?;
    indented(f, |f| {
        if let Some(x) = &handle.constructor {
            let args = cpp_arguments(x.parameters.iter());
            f.writeln(&format!("{}({});", handle.cpp_name(), args))?;
        };
        if handle.destructor.is_some() {
            f.writeln(&format!("~{}();", handle.cpp_name()))?;
        };
        for method in &handle.methods {
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

fn print_static_class(f: &mut dyn Printer, handle: &StaticClassHandle) -> FormattingResult<()> {
    f.writeln(&format!("class {} {{", handle.cpp_name()))?;
    indented(f, |f| {
        f.writeln(&format!("{}() = delete;", handle.cpp_name()))
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

fn print_iterator_definition(f: &mut dyn Printer) -> FormattingResult<()> {
    let iterator = include_str!("./snippet/iterator.hpp");
    for line in iterator.lines() {
        f.writeln(line)?;
    }
    f.newline()
}

fn print_header_namespace_contents(lib: &Library, f: &mut dyn Printer) -> FormattingResult<()> {
    print_version(lib, f)?;

    print_iterator_definition(f)?;

    f.writeln("// forward declare the friend class which can access C++ class internals")?;
    f.writeln(&format!("class {};", FRIEND_CLASS_NAME))?;
    f.newline()?;

    for statement in lib.into_iter() {
        match &statement {
            Statement::Constants(x) => print_constants(f, x)?,
            Statement::EnumDefinition(x) => print_enum(f, x)?,
            Statement::ErrorType(x) => print_exception(f, x)?,
            Statement::NativeStructDeclaration(x) => print_native_struct_decl(f, x)?,
            Statement::NativeStructDefinition(x) => print_native_struct_header(f, x, lib)?,
            Statement::InterfaceDefinition(x) => print_interface(f, x)?,
            Statement::ClassDeclaration(x) => print_class_decl(f, x)?,
            Statement::ClassDefinition(x) => print_class(f, x)?,
            Statement::StaticClassDefinition(x) => print_static_class(f, x)?,
            Statement::IteratorDeclaration(_) => {
                // custom iterator type is only in CPP
            }
            Statement::StructDefinition(_) => {
                // ignoring these for now
            }
            Statement::CollectionDeclaration(_) => {
                // only used for transforms ATM
            }
            Statement::NativeFunctionDeclaration(_) => {
                // not used in C++
            }
        }
    }

    Ok(())
}

fn convert_native_struct_elem_to_cpp(elem: &NativeStructElement) -> String {
    let base_name = format!("x.{}", elem.name);
    convert_to_cpp(&elem.element_type.to_type(), base_name)
}

fn convert_native_struct_ptr_elem_to_cpp(elem: &NativeStructElement) -> String {
    let base_name = format!("x->{}", elem.name);
    convert_to_cpp(&elem.element_type.to_type(), base_name)
}

fn convert_native_struct_elem_from_cpp(elem: &NativeStructElement) -> String {
    let base_name = format!("x.{}", elem.name);
    convert_to_c(&elem.element_type.to_type(), base_name)
}

fn convert_native_struct_ptr_elem_from_cpp(elem: &NativeStructElement) -> String {
    let base_name = format!("x->{}", elem.name);
    convert_to_c(&elem.element_type.to_type(), base_name)
}

fn print_friend_class_decl(lib: &Library, f: &mut dyn Printer) -> FormattingResult<()> {
    f.writeln(&format!("class {} {{", FRIEND_CLASS_NAME))?;
    indented(f, |f| {
        f.writeln("public:")?;

        for handle in lib.native_structs() {
            f.writeln(&format!(
                "static {} to_cpp(const {}& x);",
                handle.declaration().cpp_name(),
                handle.to_c_type(&lib.c_ffi_prefix)
            ))?;
            f.writeln(&format!(
                "static std::unique_ptr<{}> to_cpp_ref(const {}* x);",
                handle.declaration().cpp_name(),
                handle.to_c_type(&lib.c_ffi_prefix)
            ))?;
            f.writeln(&format!(
                "static {} from_cpp(const {}& x);",
                handle.to_c_type(&lib.c_ffi_prefix),
                handle.declaration().cpp_name()
            ))?;
            f.writeln(&format!(
                "static std::unique_ptr<{}> from_cpp_ref(const {}* x);",
                handle.to_c_type(&lib.c_ffi_prefix),
                handle.declaration().cpp_name()
            ))?;
            f.newline()?;
        }

        Ok(())
    })?;
    f.writeln("};")?;
    f.newline()
}

fn print_friend_class_impl(lib: &Library, f: &mut dyn Printer) -> FormattingResult<()> {
    for handle in lib.native_structs() {
        print_struct_conversion_impl(lib, f, handle)?;
    }

    f.newline()
}

fn print_struct_conversion_impl(
    lib: &Library,
    f: &mut dyn Printer,
    handle: &NativeStructHandle,
) -> FormattingResult<()> {
    f.writeln(&format!(
        "{} {}::to_cpp(const {}& x)",
        handle.declaration().cpp_name(),
        FRIEND_CLASS_NAME,
        handle.to_c_type(&lib.c_ffi_prefix)
    ))?;
    f.writeln("{")?;
    indented(f, |f| {
        f.writeln(&format!("return {}(", handle.declaration.cpp_name()))?;
        indented(f, |f| {
            for elem in handle.elements.iter() {
                let conversion = convert_native_struct_elem_to_cpp(elem);
                f.writeln(&conversion)?;
            }
            Ok(())
        })?;
        f.writeln(");")
    })?;
    f.writeln("}")?;

    f.newline()?;

    f.writeln(&format!(
        "std::unique_ptr<{}> {}::to_cpp_ref(const {}* x)",
        handle.declaration().cpp_name(),
        FRIEND_CLASS_NAME,
        handle.to_c_type(&lib.c_ffi_prefix)
    ))?;
    f.writeln("{")?;
    indented(f, |f| {
        f.writeln("if (x) {")?;
        indented(f, |f| {
            f.writeln(&format!(
                "return std::make_unique<{}>(",
                handle.declaration.cpp_name()
            ))?;
            indented(f, |f| {
                for elem in handle.elements.iter() {
                    let conversion = convert_native_struct_ptr_elem_to_cpp(elem);
                    f.writeln(&conversion)?;
                }
                Ok(())
            })?;
            f.writeln(");")
        })?;
        f.writeln("}")?;
        f.writeln("else {")?;
        indented(f, |f| f.writeln("return nullptr;"))?;
        f.writeln("}")
    })?;
    f.writeln("}")?;

    f.newline()?;

    f.writeln(&format!(
        "{} {}::from_cpp(const {}& x)",
        handle.to_c_type(&lib.c_ffi_prefix),
        FRIEND_CLASS_NAME,
        handle.declaration().cpp_name(),
    ))?;
    f.writeln("{")?;
    indented(f, |f| {
        f.writeln("return {")?;
        indented(f, |f| {
            for elem in handle.elements.iter() {
                let conversion = convert_native_struct_elem_from_cpp(elem);
                f.writeln(&conversion)?;
            }
            Ok(())
        })?;
        f.writeln("};")
    })?;
    f.writeln("}")?;

    f.newline()?;

    f.writeln(&format!(
        "std::unique_ptr<{}> {}::from_cpp_ref(const {}* x)",
        handle.to_c_type(&lib.c_ffi_prefix),
        FRIEND_CLASS_NAME,
        handle.declaration().cpp_name(),
    ))?;
    f.writeln("{")?;
    indented(f, |f| {
        f.writeln("if (x) {")?;
        indented(f, |f| {
            f.writeln(&format!(
                "return std::unique_ptr<{}>(new {} {{",
                handle.to_c_type(&lib.c_ffi_prefix),
                handle.to_c_type(&lib.c_ffi_prefix)
            ))?;
            indented(f, |f| {
                for elem in handle.elements.iter() {
                    let conversion = convert_native_struct_ptr_elem_from_cpp(elem);
                    f.writeln(&conversion)?;
                }
                Ok(())
            })?;
            f.writeln("});")
        })?;
        f.writeln("}")?;
        f.writeln("else {")?;
        indented(f, |f| f.writeln("return nullptr;"))?;
        f.writeln("}")
    })?;
    f.writeln("}")?;

    f.newline()
}

fn print_enum_conversions(
    lib: &Library,
    f: &mut dyn Printer,
    handle: &NativeEnumHandle,
) -> FormattingResult<()> {
    f.writeln(&format!(
        "{} to_cpp({}_{}_t value)",
        handle.cpp_name(),
        lib.c_ffi_prefix,
        handle.name.to_snake_case()
    ))?;
    f.writeln("{")?;
    indented(f, |f| {
        f.writeln("switch(value)")?;
        f.writeln("{")?;
        indented(f, |f| {
            for v in &handle.variants {
                f.writeln(&format!(
                    "case {}_{}_{}: return {}::{};",
                    lib.c_ffi_prefix.to_shouty_snake_case(),
                    handle.name.to_shouty_snake_case(),
                    v.name.to_shouty_snake_case(),
                    handle.name.to_camel_case(),
                    v.name.to_snake_case()
                ))?;
            }
            f.writeln("default: throw std::invalid_argument(\"bad enum conversion\");")?;
            Ok(())
        })?;
        f.writeln("}")
    })?;
    f.writeln("}")?;

    f.newline()?;

    f.writeln(&format!(
        "{}_{}_t from_cpp({} value)",
        lib.c_ffi_prefix,
        handle.name.to_snake_case(),
        handle.cpp_name(),
    ))?;
    f.writeln("{")?;
    indented(f, |f| {
        f.writeln("switch(value)")?;
        f.writeln("{")?;
        indented(f, |f| {
            for v in &handle.variants {
                f.writeln(&format!(
                    "case {}::{}: return {}_{}_{};",
                    handle.name.to_camel_case(),
                    v.name.to_snake_case(),
                    lib.c_ffi_prefix.to_shouty_snake_case(),
                    handle.name.to_shouty_snake_case(),
                    v.name.to_shouty_snake_case(),
                ))?;
            }
            f.writeln("default: throw std::invalid_argument(\"bad enum conversion\");")?;
            Ok(())
        })?;
        f.writeln("}")
    })?;
    f.writeln("}")?;
    f.newline()
}

fn convert_basic_type_to_cpp(typ: &BasicType, expr: String) -> String {
    match typ {
        BasicType::Bool => expr,
        BasicType::Uint8 => expr,
        BasicType::Sint8 => expr,
        BasicType::Uint16 => expr,
        BasicType::Sint16 => expr,
        BasicType::Uint32 => expr,
        BasicType::Sint32 => expr,
        BasicType::Uint64 => expr,
        BasicType::Sint64 => expr,
        BasicType::Float => expr,
        BasicType::Double => expr,
        BasicType::Duration(t) => match t {
            DurationType::Milliseconds => {
                format!("convert::from_sec_u64({})", expr)
            }
            DurationType::Seconds => {
                format!("convert::from_msec_u64({})", expr)
            }
        },
        BasicType::Enum(_) => format!("convert::to_cpp({})", expr),
    }
}

fn convert_to_cpp(typ: &Type, expr: String) -> String {
    match typ {
        Type::Basic(x) => convert_basic_type_to_cpp(x, expr),
        Type::String => format!("std::string({})", expr),
        Type::Struct(_) => format!("{}::to_cpp({})", FRIEND_CLASS_NAME, expr),
        Type::StructRef(_) => format!("{}::to_cpp_ref({})", FRIEND_CLASS_NAME, expr),
        Type::ClassRef(_) => format!("{}::to_cpp({})", FRIEND_CLASS_NAME, expr),
        Type::Interface(_) => "nullptr".to_string(), // Conversion from C to C++ is not allowed
        Type::Iterator(_) => format!("convert::to_vec({})", expr),
        Type::Collection(_) => "nullptr".to_string(), // Conversion from C to C++ is not allowed
    }
}

fn convert_basic_type_to_c(t: &BasicType, expr: String) -> String {
    match t {
        BasicType::Bool => expr,
        BasicType::Uint8 => expr,
        BasicType::Sint8 => expr,
        BasicType::Uint16 => expr,
        BasicType::Sint16 => expr,
        BasicType::Uint32 => expr,
        BasicType::Sint32 => expr,
        BasicType::Uint64 => expr,
        BasicType::Sint64 => expr,
        BasicType::Float => expr,
        BasicType::Double => expr,
        BasicType::Duration(t) => match t {
            DurationType::Milliseconds => {
                format!("convert::to_sec_u64({})", expr)
            }
            DurationType::Seconds => {
                format!("convert::to_msec_u64({})", expr)
            }
        },
        BasicType::Enum(_) => format!("convert::from_cpp({})", expr),
    }
}

fn convert_to_c(typ: &Type, expr: String) -> String {
    match typ {
        Type::Basic(t) => convert_basic_type_to_c(t, expr),
        Type::String => format!("{}.c_str()", expr),
        Type::Struct(_) => format!("{}::from_cpp({})", FRIEND_CLASS_NAME, expr),
        Type::StructRef(_) => format!("{}::from_cpp_ref({})", FRIEND_CLASS_NAME, expr),
        Type::ClassRef(_) => unimplemented!(),
        Type::Interface(_) => format!("convert::from_cpp({})", expr),
        Type::Iterator(_) => "nullptr".to_string(), // Conversion not supported
        Type::Collection(_) => unimplemented!(),
    }
}

fn print_interface_conversions(
    lib: &Library,
    f: &mut dyn Printer,
    handle: &InterfaceHandle,
) -> FormattingResult<()> {
    fn get_invocation(handle: &InterfaceHandle, func: &CallbackFunction) -> String {
        let args = func
            .parameters
            .iter()
            .flat_map(|p| match p {
                CallbackParameter::Parameter(p) => {
                    Some(convert_to_cpp(&p.param_type, p.cpp_name()))
                }
                CallbackParameter::Arg(_) => None,
            })
            .collect::<Vec<String>>()
            .join(", ");

        format!(
            "reinterpret_cast<{}*>({})->{}({})",
            handle.cpp_name(),
            func.arg_name.to_snake_case(),
            func.cpp_name(),
            args
        )
    }

    f.writeln(&format!(
        "{}_{}_t from_cpp(std::unique_ptr<{}> value)",
        lib.c_ffi_prefix,
        handle.name.to_snake_case(),
        handle.cpp_name(),
    ))?;
    f.writeln("{")?;
    indented(f, |f| {
        // Note: Designated initializers (i.e. C-style struct initialization) were standardized in C99,
        // but only to C++ in C++20. Therefore, we cannot use it here as we target a lower version of C++.
        f.writeln("return {")?;
        indented(f, |f| {
            for x in &handle.elements {
                match x {
                    InterfaceElement::Arg(_) => {
                        f.writeln("value.release(),")?;
                    }
                    InterfaceElement::DestroyFunction(_) => {
                        f.writeln(&format!(
                            "[](void* {}) {{ delete reinterpret_cast<{}*>({}); }},",
                            handle.arg_name,
                            handle.cpp_name(),
                            handle.arg_name
                        ))?;
                    }
                    InterfaceElement::CallbackFunction(func) => {
                        f.writeln(&format!(
                            "[]({}) -> {} {{",
                            crate::chelpers::callback_parameters_with_var_names(lib, func),
                            func.return_type.to_c_type(&lib.c_ffi_prefix)
                        ))?;
                        indented(f, |f| {
                            match &func.return_type {
                                ReturnType::Type(t, _) => {
                                    let value = get_invocation(handle, func);

                                    f.writeln(&format!("return {};", convert_to_c(t, value)))?;
                                }
                                ReturnType::Void => {
                                    f.writeln(&format!("{};", get_invocation(handle, func)))?;
                                }
                            }
                            Ok(())
                        })?;
                        f.writeln("},")?;
                    }
                }
            }
            Ok(())
        })?;
        f.writeln("};")
    })?;
    f.writeln("}")?;
    f.newline()
}

fn print_iterator_conversions(
    lib: &Library,
    f: &mut dyn Printer,
    handle: &IteratorHandle,
) -> FormattingResult<()> {
    f.writeln(&format!(
        "std::vector<{}> to_vec({}* x)",
        handle.item_type.cpp_name(),
        handle.iter_type.to_c_type(&lib.c_ffi_prefix)
    ))?;
    f.writeln("{")?;
    indented(f, |f| {
        let function_name = format!("{}_{}", lib.c_ffi_prefix, handle.native_func.name);
        f.writeln(&format!(
            "auto result = std::vector<{}>();",
            handle.item_type.cpp_name()
        ))?;
        f.writeln(&format!("auto it = {}(x);", function_name))?;
        f.writeln("while (it != nullptr) {")?;
        indented(f, |f| {
            f.writeln(&format!(
                "result.push_back({});",
                convert_to_cpp(&Type::Struct(handle.item_type.clone()), "*it".to_string())
            ))?;
            f.writeln(&format!("it = {}(x);", function_name))
        })?;
        f.writeln("}")?;
        f.newline()?;
        f.writeln("return result;")
    })?;
    f.writeln("}")?;
    f.newline()
}

fn print_enum_to_string_impl(
    f: &mut dyn Printer,
    handle: &NativeEnumHandle,
) -> FormattingResult<()> {
    f.writeln(&format!(
        "const char* to_string({} value)",
        handle.cpp_name()
    ))?;
    f.writeln("{")?;
    indented(f, |f| {
        f.writeln("switch(value)")?;
        f.writeln("{")?;
        indented(f, |f| {
            for v in &handle.variants {
                f.writeln(&format!(
                    "case {}::{}: return \"{}\";",
                    handle.cpp_name(),
                    v.cpp_name(),
                    v.name
                ))?;
            }
            f.writeln(&format!(
                "default: throw std::invalid_argument(\"Undefined value for enum '{}'\");",
                handle.name
            ))
        })?;
        f.writeln("}")
    })?;
    f.writeln("}")?;
    f.newline()
}

fn get_initializer_value(e: &NativeStructElement) -> String {
    match &e.element_type {
        StructElementType::Bool(v) => v.map(|x| format!("{}", x)).unwrap_or_else(|| e.cpp_name()),
        StructElementType::Uint8(v) => v.map(|x| format!("{}", x)).unwrap_or_else(|| e.cpp_name()),
        StructElementType::Sint8(v) => v.map(|x| format!("{}", x)).unwrap_or_else(|| e.cpp_name()),
        StructElementType::Uint16(v) => v.map(|x| format!("{}", x)).unwrap_or_else(|| e.cpp_name()),
        StructElementType::Sint16(v) => v.map(|x| format!("{}", x)).unwrap_or_else(|| e.cpp_name()),
        StructElementType::Uint32(v) => v.map(|x| format!("{}", x)).unwrap_or_else(|| e.cpp_name()),
        StructElementType::Sint32(v) => v.map(|x| format!("{}", x)).unwrap_or_else(|| e.cpp_name()),
        StructElementType::Uint64(v) => v.map(|x| format!("{}", x)).unwrap_or_else(|| e.cpp_name()),
        StructElementType::Sint64(v) => v.map(|x| format!("{}", x)).unwrap_or_else(|| e.cpp_name()),
        StructElementType::Float(v) => v.map(|x| format!("{}", x)).unwrap_or_else(|| e.cpp_name()),
        StructElementType::Double(v) => v.map(|x| format!("{}", x)).unwrap_or_else(|| e.cpp_name()),
        StructElementType::String(v) => v
            .as_ref()
            .map(|x| format!("\"{}\"", x))
            .unwrap_or(format!("std::move({})", e.cpp_name())),
        StructElementType::Struct(x) => {
            if x.all_fields_have_defaults() {
                format!("{}()", x.cpp_name())
            } else {
                e.cpp_name()
            }
        }
        StructElementType::StructRef(_) => unimplemented!(),
        StructElementType::Enum(x, v) => v
            .as_ref()
            .map(|v| format!("{}::{}", x.cpp_name(), v.to_snake_case()))
            .unwrap_or_else(|| e.cpp_name()),
        StructElementType::ClassRef(_) => unimplemented!(),
        StructElementType::Interface(_) => format!("std::move({})", e.cpp_name()),
        StructElementType::Iterator(_) => e.cpp_name(),
        StructElementType::Collection(_) => e.cpp_name(),
        StructElementType::Duration(_, v) => v
            .map(|v| format!("std::chrono::milliseconds({})", v.as_millis()))
            .unwrap_or_else(|| e.cpp_name()),
    }
}

fn print_struct_constructor_impl(
    f: &mut dyn Printer,
    handle: &NativeStructHandle,
) -> FormattingResult<()> {
    let name = handle.cpp_name();
    f.writeln(&format!(
        "{}::{}({}) :",
        name,
        name,
        get_struct_default_constructor_args(handle)
    ))?;
    indented(f, |f| {
        let last = handle.elements.len() - 1;
        for (i, e) in handle.elements.iter().enumerate() {
            if i == last {
                f.writeln(&format!("{}({})", e.cpp_name(), get_initializer_value(e)))?;
            } else {
                f.writeln(&format!("{}({}),", e.cpp_name(), get_initializer_value(e)))?;
            }
        }
        Ok(())
    })?;
    f.writeln("{}")?;
    f.newline()
}

fn print_exception_wrappers(lib: &Library, f: &mut dyn Printer) -> FormattingResult<()> {
    if !lib.native_functions().any(|f| f.error_type.is_some()) {
        return Ok(());
    }

    fn print_check_exception(f: &mut dyn Printer, err: &ErrorType) -> FormattingResult<()> {
        f.writeln("if(error) {")?;
        indented(f, |f| {
            f.writeln(&format!(
                "throw {}(convert::to_cpp(error));",
                err.exception_name.to_camel_case()
            ))
        })?;
        f.writeln("}")
    }

    fn print_with_returned_value(
        lib: &Library,
        f: &mut dyn Printer,
        func: &NativeFunctionHandle,
        err: &ErrorType,
    ) -> FormattingResult<()> {
        let args = func
            .parameters
            .iter()
            .map(|p| p.name.clone())
            .collect::<Vec<String>>()
            .join(", ");

        f.writeln(&format!(
            "{} returned_value;",
            func.return_type.to_c_type(&lib.c_ffi_prefix)
        ))?;
        f.writeln(&format!(
            "const auto error = {}_{}({}, &returned_value);",
            lib.c_ffi_prefix,
            func.name.to_snake_case(),
            args
        ))?;
        print_check_exception(f, err)?;
        f.writeln("return returned_value;")
    }

    fn print_without_returned_value(
        lib: &Library,
        f: &mut dyn Printer,
        func: &NativeFunctionHandle,
        err: &ErrorType,
    ) -> FormattingResult<()> {
        let args = func
            .parameters
            .iter()
            .map(|p| p.name.clone())
            .collect::<Vec<String>>()
            .join(", ");

        f.writeln(&format!(
            "const auto error = {}_{}({});",
            lib.c_ffi_prefix,
            func.name.to_snake_case(),
            args
        ))?;
        print_check_exception(f, err)
    }

    // write native function wrappers
    namespace(f, "ex_wrap", |f| {
        for func in lib.native_functions() {
            if let Some(err) = &func.error_type {
                let args = func
                    .parameters
                    .iter()
                    .map(|p| {
                        format!(
                            "{} {}",
                            p.param_type.to_c_type(&lib.c_ffi_prefix),
                            p.name.to_snake_case()
                        )
                    })
                    .collect::<Vec<String>>()
                    .join(", ");

                f.writeln(&format!(
                    "{} {}({})",
                    &func.return_type.to_c_type(&lib.c_ffi_prefix),
                    func.name,
                    args
                ))?;
                f.writeln("{")?;
                indented(f, |f| {
                    match func.return_type {
                        ReturnType::Void => {
                            print_without_returned_value(lib, f, func, err)?;
                        }
                        ReturnType::Type(_, _) => {
                            print_with_returned_value(lib, f, func, err)?;
                        }
                    }
                    Ok(())
                })?;
                f.writeln("}")?;
                f.newline()?;
            }
        }
        Ok(())
    })?;
    f.newline()
}

fn print_impl_namespace_contents(lib: &Library, f: &mut dyn Printer) -> FormattingResult<()> {
    let time_conversions = include_str!("./snippet/convert_time.cpp");

    // conversions
    namespace(f, "convert", |f| {
        for line in time_conversions.lines() {
            f.writeln(line)?;
        }
        f.newline()?;

        for e in lib.native_enums() {
            print_enum_conversions(lib, f, e)?;
        }

        for interface in lib.interfaces() {
            print_interface_conversions(lib, f, interface)?;
        }

        for handle in lib.iterators() {
            print_iterator_conversions(lib, f, handle)?;
        }

        Ok(())
    })?;

    print_exception_wrappers(lib, f)?;

    // enum to string helpers
    for e in lib.native_enums() {
        print_enum_to_string_impl(f, e)?;
    }

    print_friend_class_decl(lib, f)?;
    print_friend_class_impl(lib, f)?;

    // struct constructors
    for handle in lib.native_structs() {
        print_struct_constructor_impl(f, handle)?;
    }

    Ok(())
}

pub(crate) fn generate_cpp_header(lib: &Library, path: &Path) -> FormattingResult<()> {
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

pub(crate) fn generate_cpp_impl(lib: &Library, path: &Path) -> FormattingResult<()> {
    // Open the file
    std::fs::create_dir_all(&path)?;
    let filename = path.join(format!("{}.cpp", lib.name));
    let mut f = FilePrinter::new(filename)?;

    // include guard
    f.writeln(&format!("#include \"{}.hpp\"", lib.name))?;
    f.writeln(&format!("#include \"{}.h\"", lib.name))?;
    f.newline()?;

    namespace(&mut f, &lib.c_ffi_prefix, |f| {
        print_impl_namespace_contents(lib, f)
    })?;

    Ok(())
}
