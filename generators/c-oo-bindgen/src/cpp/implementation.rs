use crate::cpp::conversion::*;
use crate::cpp::formatting::{const_ref, friend_class, mut_ref, namespace, unique_ptr};
use crate::ctype::CType;
use heck::{CamelCase, ShoutySnakeCase, SnakeCase};
use oo_bindgen::enum_type::EnumHandle;
use oo_bindgen::formatting::{blocked, indented, FilePrinter, FormattingResult, Printer};
use oo_bindgen::interface::{CallbackFunction, CallbackReturnType, InterfaceHandle};
use oo_bindgen::iterator::IteratorHandle;
use oo_bindgen::structs::{Struct, StructFieldType};
use oo_bindgen::util::WithLastIndication;
use oo_bindgen::{Handle, Library, Statement, StructType};
use std::path::Path;

pub(crate) fn generate_cpp_file(lib: &Library, path: &Path) -> FormattingResult<()> {
    // Open the file
    std::fs::create_dir_all(&path)?;
    let filename = path.join(format!("{}.cpp", lib.name));
    let mut f = FilePrinter::new(filename)?;

    f.writeln(&format!("#include \"{}.h\"", lib.name))?;
    f.writeln(&format!("#include \"{}.hpp\"", lib.name))?;
    f.newline()?;

    namespace(&mut f, &lib.c_ffi_prefix, |f| {
        write_impl_namespace_contents(lib, f)
    })?;

    Ok(())
}

fn write_impl_namespace_contents(lib: &Library, f: &mut dyn Printer) -> FormattingResult<()> {
    let time_conversions = include_str!("./snippet/convert_time.cpp");

    // write the friend class implementations
    for it in lib.iterators() {
        write_iterator_friend_class(lib, f, it)?;
    }

    for st in lib.structs() {
        match st {
            StructType::FunctionArg(x) => {
                write_cpp_struct_friend_class(f, x)?;
            }
            StructType::FunctionReturn(x) => {
                write_cpp_struct_friend_class(f, x)?;
            }
            StructType::CallbackArg(x) => {
                write_cpp_struct_friend_class(f, x)?;
            }
            StructType::Universal(x) => {
                write_cpp_struct_friend_class(f, x)?;
            }
        }
    }

    // conversions
    namespace(f, "convert", |f| {
        for line in time_conversions.lines() {
            f.writeln(line)?;
        }
        f.newline()?;

        // emit the conversions in statement order as some conversions reference other conversions
        for statement in lib.statements() {
            write_conversion_function(lib, f, statement)?;
        }

        Ok(())
    })?;

    Ok(())
}

fn write_conversion_function(
    lib: &Library,
    f: &mut dyn Printer,
    statement: &Statement,
) -> FormattingResult<()> {
    match statement {
        Statement::StructDefinition(x) => match x {
            StructType::FunctionArg(x) => write_cpp_to_native_struct_conversion(f, lib, x),
            StructType::FunctionReturn(x) => write_native_to_cpp_struct_conversion(f, lib, x),
            StructType::CallbackArg(x) => write_native_to_cpp_struct_conversion(f, lib, x),
            StructType::Universal(x) => {
                write_cpp_to_native_struct_conversion(f, lib, x)?;
                write_native_to_cpp_struct_conversion(f, lib, x)
            }
        },
        Statement::EnumDefinition(x) => {
            write_enum_conversions(lib, f, x)?;
            print_enum_to_string_impl(f, x)
        }
        Statement::InterfaceDefinition(x) => write_cpp_interface_to_native_conversion(f, lib, x),
        _ => Ok(()),
    }
}

fn write_cpp_struct_friend_class<T>(
    f: &mut dyn Printer,
    handle: &Handle<Struct<T>>,
) -> FormattingResult<()>
where
    T: StructFieldType + CppFunctionArgType + IsConstructByMove,
{
    let args = handle
        .fields
        .iter()
        .map(|x| {
            format!(
                "{} {}",
                x.field_type.get_cpp_function_arg_type(),
                x.name.to_snake_case()
            )
        })
        .collect::<Vec<String>>()
        .join(", ");

    f.writeln(&format!("class {}", friend_class(handle.core_type())))?;
    f.writeln("{")?;
    f.writeln("public:")?;
    indented(f, |f| {
        f.writeln(&format!("static {} init({})", handle.core_type(), args))?;
        blocked(f, |f| {
            f.writeln(&format!("return {}(", handle.core_type()))?;
            indented(f, |f| {
                for (field, last) in handle.fields().with_last() {
                    let value = if field.field_type.is_construct_by_move() {
                        format!("std::move({})", field.name.to_snake_case())
                    } else {
                        field.name.to_snake_case()
                    };

                    if last {
                        f.writeln(&value)?;
                    } else {
                        f.writeln(&format!("{},", &value))?;
                    }
                }
                Ok(())
            })?;
            f.writeln(");")?;
            Ok(())
        })
    })?;
    f.writeln("};")?;
    f.newline()
}

fn write_iterator_friend_class(
    lib: &Library,
    f: &mut dyn Printer,
    handle: &IteratorHandle,
) -> FormattingResult<()> {
    f.writeln(&format!("class {}", friend_class(handle.core_type())))?;
    f.writeln("{")?;
    f.writeln("public:")?;
    indented(f, |f| {
        f.newline()?;
        f.writeln(&format!(
            "static {} init({}* value)",
            handle.core_type(),
            handle.iter_type.to_c_type(&lib.c_ffi_prefix)
        ))?;
        blocked(f, |f| {
            f.writeln(&format!("return {}(value);", handle.core_type()))
        })
    })?;
    f.writeln("};")?;
    f.newline()
}

fn write_cpp_to_native_struct_conversion<T>(
    f: &mut dyn Printer,
    lib: &Library,
    handle: &Handle<Struct<T>>,
) -> FormattingResult<()>
where
    T: StructFieldType + ToNativeStructField,
{
    let value_type = if handle.fields.iter().any(|f| f.field_type.requires_move()) {
        mut_ref(handle.core_type())
    } else {
        const_ref(handle.core_type())
    };

    let c_type = handle.to_c_type(&lib.c_ffi_prefix);
    f.writeln(&format!("{} to_native({} value)", c_type, value_type))?;
    blocked(f, |f| {
        f.writeln(&format!("return {} {{", c_type))?;
        indented(f, |f| {
            for field in &handle.fields {
                let cpp_value = format!("value.{}", field.name.to_snake_case());
                let conversion = field.field_type.to_native_struct_field(cpp_value);
                f.writeln(&format!("{},", conversion))?;
            }
            Ok(())
        })?;
        f.writeln("};")?;
        Ok(())
    })?;
    f.newline()
}

fn write_native_to_cpp_struct_conversion<T>(
    f: &mut dyn Printer,
    lib: &Library,
    handle: &Handle<Struct<T>>,
) -> FormattingResult<()>
where
    T: StructFieldType + ToCppStructField,
{
    let c_type = handle.to_c_type(&lib.c_ffi_prefix);
    let cpp_type = handle.core_type();
    f.writeln(&format!("{} to_cpp({} value)", cpp_type, const_ref(c_type)))?;
    blocked(f, |f| {
        f.writeln(&format!("return {}::init(", friend_class(cpp_type)))?;
        indented(f, |f| {
            for (field, last) in handle.fields.iter().with_last() {
                let native_value = format!("value.{}", field.name.to_snake_case());
                let conversion = field.field_type.to_cpp_struct_field(native_value);

                if last {
                    f.writeln(&conversion)?;
                } else {
                    f.writeln(&format!("{},", conversion))?;
                }
            }
            Ok(())
        })?;
        f.writeln(");")?;
        Ok(())
    })?;
    f.newline()
}

fn print_enum_to_string_impl(f: &mut dyn Printer, handle: &EnumHandle) -> FormattingResult<()> {
    f.writeln(&format!(
        "const char* to_string({} value)",
        handle.core_type()
    ))?;
    f.writeln("{")?;
    indented(f, |f| {
        f.writeln("switch(value)")?;
        f.writeln("{")?;
        indented(f, |f| {
            for v in &handle.variants {
                f.writeln(&format!(
                    "case {}::{}: return \"{}\";",
                    handle.core_type(),
                    v.core_type(),
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

fn write_enum_conversions(
    lib: &Library,
    f: &mut dyn Printer,
    handle: &EnumHandle,
) -> FormattingResult<()> {
    f.writeln(&format!(
        "{} enum_from_native({}_{}_t value)",
        handle.core_type(),
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
        "{}_{}_t enum_to_native({} value)",
        lib.c_ffi_prefix,
        handle.name.to_snake_case(),
        handle.core_type(),
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

fn write_callback_function(
    f: &mut dyn Printer,
    lib: &Library,
    interface: &InterfaceHandle,
    cb: &CallbackFunction,
) -> FormattingResult<()> {
    fn write_invocation_lines(f: &mut dyn Printer, cb: &CallbackFunction) -> FormattingResult<()> {
        for (arg, last) in cb.arguments.iter().with_last() {
            let conversion = if arg.arg_type.is_pass_by_mut_ref() {
                format!("_{}", arg.name.to_snake_case())
            } else {
                arg.arg_type
                    .to_native_callback_argument(arg.name.to_snake_case())
            };
            if last {
                f.writeln(&conversion)?;
            } else {
                f.writeln(&format!("{},", conversion))?;
            }
        }
        Ok(())
    }

    let args = cb
        .arguments
        .iter()
        .map(|x| {
            format!(
                "{} {}",
                x.arg_type.to_c_type(&lib.c_ffi_prefix),
                x.name.to_snake_case()
            )
        })
        .chain(std::iter::once("void* ctx".to_string()))
        .collect::<Vec<String>>()
        .join(", ");

    f.writeln(&format!(
        "[]({}) -> {} {{",
        args,
        cb.return_type.to_c_type(&lib.c_ffi_prefix)
    ))?;
    indented(f, |f| {
        for arg in &cb.arguments {
            if arg.arg_type.is_pass_by_mut_ref() {
                f.writeln(&format!(
                    "auto _{} = {};",
                    arg.name.to_snake_case(),
                    arg.arg_type
                        .to_native_callback_argument(arg.name.to_snake_case())
                ))?;
            }
        }

        let function = format!(
            "reinterpret_cast<{}*>(ctx)->{}",
            interface.core_type(),
            cb.name.to_snake_case()
        );
        match &cb.return_type {
            CallbackReturnType::Void => {
                f.writeln(&format!("{}(", function))?;
                indented(f, |f| write_invocation_lines(f, cb))?;
                f.writeln(");")
            }
            CallbackReturnType::Type(t, _) => {
                f.writeln(&format!("auto _cpp_return = {}(", function))?;
                indented(f, |f| write_invocation_lines(f, cb))?;
                f.writeln(");")?;
                f.writeln(&format!(
                    "return {};",
                    t.to_native_callback_return_value("_cpp_return".to_string())
                ))
            }
        }
    })?;
    f.writeln("},")
}

fn write_cpp_interface_to_native_conversion(
    f: &mut dyn Printer,
    lib: &Library,
    handle: &InterfaceHandle,
) -> FormattingResult<()> {
    let c_type = handle.to_c_type(&lib.c_ffi_prefix);
    f.writeln(&format!(
        "{} to_native({} value)",
        c_type,
        unique_ptr(handle.core_type())
    ))?;
    blocked(f, |f| {
        f.writeln(&format!(
            "return {} {{",
            handle.to_c_type(&lib.c_ffi_prefix)
        ))?;
        indented(f, |f| {
            for cb in &handle.callbacks {
                write_callback_function(f, lib, handle, cb)?;
            }
            f.writeln(&format!(
                "[](void* ctx) {{ delete reinterpret_cast<{}*>(ctx); }},",
                handle.core_type()
            ))?;
            f.writeln("value.release()")?;
            Ok(())
        })?;
        f.writeln("};")?;
        Ok(())
    })?;
    f.newline()?;
    Ok(())
}
