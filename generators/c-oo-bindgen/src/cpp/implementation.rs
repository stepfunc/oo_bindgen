use crate::cpp::conversion::*;
use crate::cpp::formatting::{const_ref, mut_ref, namespace, unique_ptr, FriendClass};
use crate::ctype::CType;
use heck::{CamelCase, ShoutySnakeCase, SnakeCase};
use oo_bindgen::class::ClassHandle;
use oo_bindgen::enum_type::EnumHandle;
use oo_bindgen::error_type::ErrorType;
use oo_bindgen::formatting::{blocked, indented, FilePrinter, FormattingResult, Printer};
use oo_bindgen::function::{FunctionHandle, FunctionReturnType};
use oo_bindgen::interface::{CallbackFunction, CallbackReturnType, InterfaceHandle, InterfaceType};
use oo_bindgen::iterator::IteratorHandle;
use oo_bindgen::structs::{Struct, StructFieldType, Visibility};
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

    namespace(&mut f, &lib.c_ffi_prefix, |f| write_friend_classes(lib, f))?;

    f.newline()?;

    // conversions
    namespace(&mut f, "convert", |f| {
        for line in include_str!("./snippet/convert_time.cpp").lines() {
            f.writeln(line)?;
        }
        f.newline()?;

        // emit the conversions in statement order as some conversions reference other conversions
        for statement in lib.statements() {
            write_conversions(lib, f, statement)?;
        }

        Ok(())
    })?;

    namespace(&mut f, &lib.c_ffi_prefix, |f| {
        write_function_wrappers(lib, f)?;

        // finally, we can implement the public API
        write_api_implementation(lib, f)
    })?;

    Ok(())
}

fn write_friend_classes(lib: &Library, f: &mut dyn Printer) -> FormattingResult<()> {
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

    for it in lib.iterators() {
        write_iterator_friend_class(lib, f, it)?;
    }

    for class in lib.classes() {
        print_friend_class(f, lib, class)?;
    }

    Ok(())
}

fn write_function_wrappers(lib: &Library, f: &mut dyn Printer) -> FormattingResult<()> {
    f.writeln("// C++ wrappers around native functions that do argument conversion and map errors to exceptions")?;
    f.writeln("// We don't convert the return type here as there are nuances that require it to be converted at the call site")?;
    namespace(f, "fn", |f| {
        for func in lib.functions() {
            write_function_wrapper(lib, f, func)?;
        }
        Ok(())
    })
}

fn write_api_implementation(lib: &Library, f: &mut dyn Printer) -> FormattingResult<()> {
    for e in lib.enums() {
        write_enum_to_string_impl(f, e)?;
    }

    Ok(())
}

fn print_friend_class(
    f: &mut dyn Printer,
    lib: &Library,
    handle: &ClassHandle,
) -> FormattingResult<()> {
    let iterator = include_str!("./snippet/class_friend_class.hpp");
    let c_type = format!("{}_{}_t", lib.c_ffi_prefix, handle.name().to_snake_case());
    let cpp_type = handle.core_type();
    for line in iterator.lines() {
        let substituted = line
            .replace("<c_type>", &c_type)
            .replace("<cpp_type>", &cpp_type);
        f.writeln(&substituted)?;
    }
    f.newline()
}

fn write_function_wrapper(
    lib: &Library,
    f: &mut dyn Printer,
    func: &FunctionHandle,
) -> FormattingResult<()> {
    fn write_error_check(
        lib: &Library,
        f: &mut dyn Printer,
        err: &ErrorType,
    ) -> FormattingResult<()> {
        let c_success_variant = &format!(
            "{}_{}_{}",
            lib.c_ffi_prefix.to_shouty_snake_case(),
            err.inner.name.to_shouty_snake_case(),
            err.inner
                .variants
                .first()
                .unwrap()
                .name
                .to_shouty_snake_case(),
        );
        f.writeln(&format!("if(_error != {})", c_success_variant))?;
        blocked(f, |f| {
            f.writeln(&format!(
                "throw {}({});",
                err.exception_name.to_camel_case(),
                err.inner.to_cpp("_error".to_string())
            ))
        })
    }

    fn write_shadowed_conversions(
        f: &mut dyn Printer,
        func: &FunctionHandle,
    ) -> FormattingResult<()> {
        for arg in &func.parameters {
            if arg.arg_type.requires_shadow_parameter() {
                let arg_name = arg.name.to_snake_case();
                f.writeln(&format!(
                    "auto _{} = {};",
                    &arg_name,
                    arg.arg_type.to_native_function_argument(arg_name.clone())
                ))?;
            }
        }
        Ok(())
    }

    fn write_args(
        f: &mut dyn Printer,
        func: &FunctionHandle,
        has_out_param: bool,
    ) -> FormattingResult<()> {
        for (arg, last) in func.parameters.iter().with_last() {
            let conversion = match arg.arg_type.shadow_parameter_mapping() {
                None => arg
                    .arg_type
                    .to_native_function_argument(arg.name.to_snake_case()),
                Some(transform) => transform(format!("_{}", arg.name.to_snake_case())),
            };
            if last && !has_out_param {
                f.writeln(&conversion)?;
            } else {
                f.writeln(&format!("{},", conversion))?;
            }
        }
        if has_out_param {
            f.writeln("_return_value")?;
        }
        Ok(())
    }

    let args = func
        .parameters
        .iter()
        .map(|arg| {
            format!(
                "{} {}",
                arg.arg_type.get_cpp_function_arg_type(),
                arg.name.to_snake_case()
            )
        })
        .collect::<Vec<String>>()
        .join(", ");

    f.writeln(&format!(
        "{} {}({})",
        func.return_type.to_c_type(&lib.c_ffi_prefix),
        func.name.to_snake_case(),
        args
    ))?;
    blocked(f, |f| {
        let c_func_name = format!("{}_{}", lib.c_ffi_prefix, func.name);
        write_shadowed_conversions(f, func)?;
        match &func.error_type {
            None => match &func.return_type {
                FunctionReturnType::Void => {
                    f.writeln(&format!("{}(", c_func_name))?;
                    indented(f, |f| write_args(f, func, false))?;
                    f.writeln(");")
                }
                FunctionReturnType::Type(_, _) => {
                    f.writeln(&format!("return {}(", c_func_name))?;
                    indented(f, |f| write_args(f, func, false))?;
                    f.writeln(");")
                }
            },
            Some(err) => match &func.return_type {
                FunctionReturnType::Void => {
                    f.writeln(&format!("const auto _error = {}(", c_func_name))?;
                    indented(f, |f| write_args(f, func, false))?;
                    f.writeln(");")?;
                    write_error_check(lib, f, err)
                }
                FunctionReturnType::Type(t, _) => {
                    f.writeln(&format!(
                        "{}* _return_value = nullptr;",
                        t.to_c_type(&lib.c_ffi_prefix)
                    ))?;
                    f.writeln(&format!("const auto _error =  {}(", c_func_name))?;
                    indented(f, |f| write_args(f, func, true))?;
                    f.writeln(");")?;
                    write_error_check(lib, f, err)?;
                    f.writeln("return *_return_value;")
                }
            },
        }
    })?;
    f.newline()
}

fn write_conversions(
    lib: &Library,
    f: &mut dyn Printer,
    statement: &Statement,
) -> FormattingResult<()> {
    match statement {
        Statement::StructDefinition(x) => match x {
            StructType::FunctionArg(x) => write_cpp_to_native_struct_conversion(f, lib, x),
            StructType::FunctionReturn(x) => {
                write_cpp_to_native_struct_conversion(f, lib, x)?;
                write_native_to_cpp_struct_conversion(f, lib, x)
            }
            StructType::CallbackArg(x) => {
                write_cpp_to_native_struct_conversion(f, lib, x)?;
                write_native_to_cpp_struct_conversion(f, lib, x)
            }
            StructType::Universal(x) => {
                write_cpp_to_native_struct_conversion(f, lib, x)?;
                write_native_to_cpp_struct_conversion(f, lib, x)
            }
        },
        Statement::EnumDefinition(x) => write_enum_conversions(lib, f, x),
        Statement::InterfaceDefinition(x) => {
            // write synchronous and asynchronous conversions
            write_cpp_interface_to_native_conversion(f, lib, x, InterfaceType::Asynchronous)?;
            write_cpp_interface_to_native_conversion(f, lib, x, InterfaceType::Synchronous)
        }
        Statement::ClassDefinition(x) => write_class_construct_helper(lib, f, x),
        Statement::IteratorDeclaration(x) => {
            write_iterator_construct_helper(lib, f, x)?;
            write_iterator_to_native_helper(lib, f, x)
        }
        _ => Ok(()),
    }
}

fn write_iterator_construct_helper(
    lib: &Library,
    f: &mut dyn Printer,
    handle: &IteratorHandle,
) -> FormattingResult<()> {
    let cpp_type = handle.core_type();
    let signature = format!(
        "::{}::{} construct({}* self)",
        lib.c_ffi_prefix,
        cpp_type,
        handle.iter_type.to_c_type(&lib.c_ffi_prefix)
    );
    f.writeln(&signature)?;
    blocked(f, |f| {
        f.writeln(&format!(
            "return ::{}::{}::init(self);",
            lib.c_ffi_prefix,
            handle.friend_class()
        ))
    })?;
    f.newline()
}

fn write_iterator_to_native_helper(
    lib: &Library,
    f: &mut dyn Printer,
    handle: &IteratorHandle,
) -> FormattingResult<()> {
    let cpp_type = handle.core_type();
    let signature = format!(
        "{}* to_native(const ::{}::{}& value)",
        handle.iter_type.to_c_type(&lib.c_ffi_prefix),
        lib.c_ffi_prefix,
        cpp_type
    );
    f.writeln(&signature)?;
    blocked(f, |f| {
        f.writeln(&format!(
            "return ::{}::{}::get(value);",
            lib.c_ffi_prefix,
            handle.friend_class()
        ))
    })?;
    f.newline()
}

fn write_class_construct_helper(
    lib: &Library,
    f: &mut dyn Printer,
    handle: &ClassHandle,
) -> FormattingResult<()> {
    let cpp_type = handle.core_type();
    let signature = format!(
        "::{}::{} construct({}* self)",
        lib.c_ffi_prefix,
        cpp_type,
        handle.declaration.to_c_type(&lib.c_ffi_prefix)
    );
    f.writeln(&signature)?;
    blocked(f, |f| {
        f.writeln(&format!(
            "return ::{}::{}::init(self);",
            lib.c_ffi_prefix,
            handle.friend_class()
        ))
    })?;
    f.newline()
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

    f.writeln(&format!("class {}", handle.friend_class()))?;
    f.writeln("{")?;
    f.writeln("public:")?;
    indented(f, |f| {
        let cpp_type = handle.core_type();
        f.writeln(&format!("static {} init({})", cpp_type, args))?;
        blocked(f, |f| {
            f.writeln(&format!("return {}(", cpp_type))?;
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
            f.writeln(");")
        })?;

        if handle.visibility == Visibility::Private {
            for field in handle.fields() {
                f.newline()?;
                f.writeln(&format!(
                    "static {} get_{}({} value)",
                    field.field_type.get_cpp_function_arg_type(),
                    field.name.to_snake_case(),
                    const_ref(cpp_type.clone())
                ))?;
                blocked(f, |f| {
                    f.writeln(&format!("return value.{};", field.name.to_snake_case()))
                })?;
            }
        }
        Ok(())
    })?;
    f.writeln("};")?;
    f.newline()
}

fn write_iterator_friend_class(
    lib: &Library,
    f: &mut dyn Printer,
    handle: &IteratorHandle,
) -> FormattingResult<()> {
    let c_type = handle.iter_type.to_c_type(&lib.c_ffi_prefix);

    f.writeln(&format!("class {}", handle.friend_class()))?;
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
        })?;
        f.newline()?;
        f.writeln(&format!(
            "static {}* get(const {}& value)",
            c_type,
            handle.core_type()
        ))?;
        blocked(f, |f| {
            f.writeln(&format!(
                "return reinterpret_cast<{}*>(value.iter);",
                c_type
            ))
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
    let namespaced_type = format!("::{}::{}", lib.c_ffi_prefix, handle.core_type());
    let value_type = if handle.fields.iter().any(|f| f.field_type.requires_move()) {
        mut_ref(namespaced_type)
    } else {
        const_ref(namespaced_type)
    };

    let c_type = handle.to_c_type(&lib.c_ffi_prefix);
    f.writeln(&format!(
        "static {} to_native({} value)",
        c_type, value_type
    ))?;
    blocked(f, |f| {
        f.writeln(&format!("return {} {{", c_type))?;
        indented(f, |f| {
            for field in &handle.fields {
                let cpp_value = match handle.visibility {
                    Visibility::Public => {
                        format!("value.{}", field.name.to_snake_case())
                    }
                    Visibility::Private => {
                        format!(
                            "::{}::{}::get_{}(value)",
                            lib.c_ffi_prefix,
                            handle.friend_class(),
                            field.name.to_snake_case()
                        )
                    }
                };
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

    let cpp_type = format!("::{}::{}", lib.c_ffi_prefix, handle.core_type());
    f.writeln(&format!("{} to_cpp({} value)", cpp_type, const_ref(c_type)))?;
    blocked(f, |f| {
        f.writeln(&format!(
            "return ::{}::{}::init(",
            lib.c_ffi_prefix,
            handle.friend_class()
        ))?;
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

fn write_enum_to_string_impl(f: &mut dyn Printer, handle: &EnumHandle) -> FormattingResult<()> {
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
    let cpp_type = format!("::{}::{}", lib.c_ffi_prefix, handle.core_type());
    f.writeln(&format!(
        "{} from_native({}_{}_t value)",
        cpp_type,
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
                    "case {}_{}_{}: return ::{}::{}::{};",
                    lib.c_ffi_prefix.to_shouty_snake_case(),
                    handle.name.to_shouty_snake_case(),
                    v.name.to_shouty_snake_case(),
                    lib.c_ffi_prefix,
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
        "{}_{}_t to_native({} value)",
        lib.c_ffi_prefix,
        handle.name.to_snake_case(),
        cpp_type
    ))?;
    f.writeln("{")?;
    indented(f, |f| {
        f.writeln("switch(value)")?;
        f.writeln("{")?;
        indented(f, |f| {
            for v in &handle.variants {
                f.writeln(&format!(
                    "case ::{}::{}::{}: return {}_{}_{};",
                    lib.c_ffi_prefix,
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
    let cpp_type = format!("::{}::{}", lib.c_ffi_prefix, interface.core_type());

    fn write_invocation_lines(f: &mut dyn Printer, cb: &CallbackFunction) -> FormattingResult<()> {
        for (arg, last) in cb.arguments.iter().with_last() {
            let conversion = if arg.arg_type.requires_shadow_parameter() {
                format!("_{}", arg.name.to_snake_case())
            } else {
                arg.arg_type
                    .to_cpp_callback_argument(arg.name.to_snake_case())
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
            if arg.arg_type.requires_shadow_parameter() {
                f.writeln(&format!(
                    "auto _{} = {};",
                    arg.name.to_snake_case(),
                    arg.arg_type
                        .to_cpp_callback_argument(arg.name.to_snake_case())
                ))?;
            }
        }

        let function = format!(
            "reinterpret_cast<{}*>(ctx)->{}",
            cpp_type,
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
    interface_type: InterfaceType,
) -> FormattingResult<()> {
    let c_type = handle.to_c_type(&lib.c_ffi_prefix);
    let cpp_type = format!("::{}::{}", lib.c_ffi_prefix, handle.core_type());
    let argument_type = match interface_type {
        InterfaceType::Synchronous => mut_ref(cpp_type.clone()),
        InterfaceType::Asynchronous => unique_ptr(cpp_type.clone()),
    };
    f.writeln(&format!("{} to_native({} value)", c_type, argument_type,))?;
    blocked(f, |f| {
        f.writeln(&format!(
            "return {} {{",
            handle.to_c_type(&lib.c_ffi_prefix)
        ))?;
        indented(f, |f| {
            for cb in &handle.callbacks {
                write_callback_function(f, lib, handle, cb)?;
            }
            match interface_type {
                InterfaceType::Synchronous => {
                    f.writeln("[](void*){}, // nothing to free")?;
                }
                InterfaceType::Asynchronous => {
                    f.writeln(&format!(
                        "[](void* ctx) {{ delete reinterpret_cast<{}*>(ctx); }},",
                        cpp_type
                    ))?;
                }
            }
            match interface_type {
                InterfaceType::Synchronous => {
                    f.writeln("&value // the pointer will outlive the callbacks")?;
                }
                InterfaceType::Asynchronous => {
                    f.writeln("value.release()")?;
                }
            }
            Ok(())
        })?;
        f.writeln("};")?;
        Ok(())
    })?;
    f.newline()?;
    Ok(())
}