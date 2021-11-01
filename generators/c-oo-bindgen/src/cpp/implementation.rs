use crate::cpp::conversion::*;
use crate::cpp::formatting::{const_ref, mut_ref, namespace, std_move, unique_ptr, FriendClass};
use crate::ctype::CType;
use heck::{CamelCase, ShoutySnakeCase, SnakeCase};
use oo_bindgen::class::{AsyncMethod, ClassHandle, Method, StaticClassHandle};
use oo_bindgen::collection::CollectionHandle;
use oo_bindgen::enum_type::EnumHandle;
use oo_bindgen::error_type::ErrorType;
use oo_bindgen::formatting::{blocked, indented, FilePrinter, FormattingResult, Printer};
use oo_bindgen::function::{FunctionArgument, FunctionHandle, FunctionReturnType};
use oo_bindgen::interface::{CallbackFunction, CallbackReturnType, InterfaceHandle, InterfaceType};
use oo_bindgen::iterator::IteratorHandle;
use oo_bindgen::structs::{
    Constructor, ConstructorType, Number, Struct, StructFieldType, ValidatedConstructorDefault,
    Visibility,
};
use oo_bindgen::types::Arg;
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
        // definitions of the collection classes and friends
        write_collection_class_definitions(lib, f)?;

        // C++ wrappers around native functions that get used in class methods
        write_function_wrappers(lib, f)?;

        // collection class constructors destructors that call the functions wrappers
        write_collection_class_implementations(lib, f)?;

        // finally, we can implement the public API
        write_api_implementation(lib, f)
    })?;

    Ok(())
}

fn write_collection_class_definitions(lib: &Library, f: &mut dyn Printer) -> FormattingResult<()> {
    for col in lib.collections() {
        write_collection_class_definition(lib, f, col)?;
        write_collection_class_friend(lib, f, col)?;
    }
    Ok(())
}

fn write_collection_class_implementations(
    lib: &Library,
    f: &mut dyn Printer,
) -> FormattingResult<()> {
    for col in lib.collections() {
        write_collection_class_implementation(f, col)?;
    }
    Ok(())
}

fn write_collection_class_implementation(
    f: &mut dyn Printer,
    col: &CollectionHandle,
) -> FormattingResult<()> {
    //let c_type = col.collection_type.to_c_type(&lib.c_ffi_prefix);
    let cpp_type = col.collection_type.core_cpp_type();
    let constructor = format!("fn::{}", col.create_func.name.to_snake_case());

    let construct_self = if col.has_reserve {
        format!("{}(static_cast<uint32_t>(values.size()))", constructor)
    } else {
        format!("{}()", constructor)
    };

    // write the constructor
    f.writeln(&format!(
        "{}::{}({} values) : self({})",
        cpp_type,
        cpp_type,
        const_ref(col.core_cpp_type()),
        construct_self
    ))?;
    blocked(f, |f| {
        f.writeln("for(const auto& x : values)")?;
        blocked(f, |f| {
            f.writeln(&format!(
                "fn::{}(*this, x);",
                col.add_func.name.to_snake_case()
            ))
        })
    })?;
    f.newline()?;

    // write the destructor
    f.writeln(&format!("{}::~{}()", cpp_type, cpp_type))?;
    blocked(f, |f| {
        f.writeln(&format!(
            "fn::{}(*this);",
            col.delete_func.name.to_snake_case()
        ))
    })?;
    f.newline()
}

fn write_collection_class_definition(
    lib: &Library,
    f: &mut dyn Printer,
    col: &CollectionHandle,
) -> FormattingResult<()> {
    let cpp_type = col.collection_type.core_cpp_type();
    let c_type = col.collection_type.to_c_type(&lib.c_ffi_prefix);
    f.writeln(&format!("class {}", cpp_type))?;
    f.writeln("{")?;
    indented(f, |f| {
        f.writeln(&format!(
            "friend class {};",
            col.collection_type.friend_class()
        ))?;
        f.writeln(&format!("{}* self;", c_type))
    })?;
    f.writeln("public:")?;
    indented(f, |f| {
        f.writeln(&format!(
            "{}({} values);",
            cpp_type,
            const_ref(col.core_cpp_type())
        ))?;
        f.writeln(&format!("~{}();", cpp_type))
    })?;
    f.writeln("};")?;
    f.newline()
}

fn write_collection_class_friend(
    lib: &Library,
    f: &mut dyn Printer,
    col: &CollectionHandle,
) -> FormattingResult<()> {
    let c_type = col.collection_type.to_c_type(&lib.c_ffi_prefix);
    f.writeln(&format!("class {}", col.collection_type.friend_class()))?;
    f.writeln("{")?;
    f.writeln("public:")?;
    indented(f, |f| {
        f.writeln(&format!(
            "static {}* get({}& value)",
            c_type,
            col.collection_type.core_cpp_type()
        ))?;
        blocked(f, |f| f.writeln("return value.self;"))?;
        Ok(())
    })?;
    f.writeln("};")?;
    f.newline()
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
    })?;
    f.newline()
}

fn write_api_implementation(lib: &Library, f: &mut dyn Printer) -> FormattingResult<()> {
    for e in lib.enums() {
        write_enum_to_string_impl(f, e)?;
    }

    for it in lib.iterators() {
        write_iterator_methods(lib, f, it)?;
    }

    for st in lib.structs() {
        match st {
            StructType::FunctionArg(x) => write_struct_constructors(f, x)?,
            StructType::FunctionReturn(x) => write_struct_constructors(f, x)?,
            StructType::CallbackArg(x) => write_struct_constructors(f, x)?,
            StructType::Universal(x) => write_struct_constructors(f, x)?,
        }
    }

    for c in lib.classes() {
        write_class_implementation(f, c)?;
    }

    for c in lib.static_classes() {
        for m in &c.static_methods {
            write_static_class_method(f, c, m)?;
            f.newline()?;
        }
    }

    Ok(())
}

fn write_iterator_methods(
    lib: &Library,
    f: &mut dyn Printer,
    it: &IteratorHandle,
) -> FormattingResult<()> {
    let c_class_type = it.iter_type.to_c_type(&lib.c_ffi_prefix);
    let cpp_class_type = it.iter_type.core_cpp_type();
    let c_next = format!("{}_{}", lib.c_ffi_prefix, it.function.name.to_snake_case());
    let cpp_value_type = it.item_type.core_cpp_type();
    let c_value_type = it.item_type.to_c_type(&lib.c_ffi_prefix);

    f.writeln(&format!("bool {}::next()", cpp_class_type))?;
    blocked(f, |f| {
        f.writeln("if(!this->iter)")?;
        blocked(f, |f| f.writeln("return false;"))?;

        f.newline()?;

        f.writeln(&format!(
            "this->current = {}(reinterpret_cast<{}*>(this->iter));",
            c_next, c_class_type
        ))?;
        f.writeln("return this->current;")?;

        Ok(())
    })?;

    f.newline()?;

    f.writeln(&format!("{} {}::get()", cpp_value_type, cpp_class_type))?;
    blocked(f, |f| {
        f.writeln("if(!this->current)")?;
        blocked(f, |f| {
            f.writeln("throw std::logic_error(\"end of iterator\");")
        })?;

        f.newline()?;

        f.writeln(&format!(
            "return ::convert::to_cpp(*reinterpret_cast<{}*>(this->current));",
            c_value_type
        ))?;

        Ok(())
    })?;

    f.newline()?;

    Ok(())
}

fn write_static_class_method(
    f: &mut dyn Printer,
    class: &StaticClassHandle,
    method: &Method,
) -> FormattingResult<()> {
    fn get_invocation_args(args: &[Arg<FunctionArgument>]) -> String {
        args.iter()
            .map(|x| x.name.to_snake_case())
            .collect::<Vec<String>>()
            .join(", ")
    }

    let args: String = method
        .native_function
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
        "{} {}::{}({})",
        method
            .native_function
            .return_type
            .get_cpp_function_return_type(),
        class.core_cpp_type(),
        method.name.to_snake_case(),
        args
    ))?;
    blocked(f, |f| {
        let invocation = format!(
            "fn::{}({})",
            method.native_function.name.to_snake_case(),
            get_invocation_args(&method.native_function.parameters)
        );

        match &method.native_function.return_type {
            FunctionReturnType::Void => f.writeln(&format!("{};", invocation)),
            FunctionReturnType::Type(t, _) => {
                f.writeln(&format!("return {};", t.to_cpp_return_value(invocation)))
            }
        }
    })
}

fn write_struct_constructors<T>(f: &mut dyn Printer, st: &Handle<Struct<T>>) -> FormattingResult<()>
where
    T: StructFieldType + CppFunctionArgType + TypeInfo,
{
    if !st.has_full_constructor() {
        let constructor = Handle::new(Constructor::full(
            "".to_string(),
            ConstructorType::Normal,
            "".into(),
        ));
        write_struct_constructor(f, st, &constructor)?;
    }

    for constructor in &st.constructors {
        write_struct_constructor(f, st, constructor)?;
    }
    Ok(())
}

fn get_default_value(default: &ValidatedConstructorDefault) -> String {
    match default {
        ValidatedConstructorDefault::Bool(x) => x.to_string(),
        ValidatedConstructorDefault::Numeric(x) => match x {
            Number::U8(x) => x.to_string(),
            Number::S8(x) => x.to_string(),
            Number::U16(x) => x.to_string(),
            Number::S16(x) => x.to_string(),
            Number::U32(x) => x.to_string(),
            Number::S32(x) => x.to_string(),
            Number::U64(x) => x.to_string(),
            Number::S64(x) => x.to_string(),
            Number::Float(x) => format!("{}f", x),
            Number::Double(x) => x.to_string(),
        },
        ValidatedConstructorDefault::Duration(_, x) => {
            format!("std::chrono::milliseconds({})", x.as_millis())
        }
        ValidatedConstructorDefault::Enum(x, variant) => {
            format!("{}::{}", x.core_cpp_type(), variant.to_snake_case())
        }
        ValidatedConstructorDefault::String(x) => {
            format!("\"{}\"", x)
        }
        ValidatedConstructorDefault::DefaultStruct(st, ct, c_name) => match ct {
            ConstructorType::Normal => format!("{}()", st.name().to_camel_case()),
            ConstructorType::Static => format!("{}()", c_name.to_camel_case()),
        },
    }
}

fn write_struct_constructor<T>(
    f: &mut dyn Printer,
    st: &Handle<Struct<T>>,
    con: &Handle<Constructor>,
) -> FormattingResult<()>
where
    T: StructFieldType + CppFunctionArgType + TypeInfo,
{
    let struct_name = st.core_cpp_type();
    let args = st
        .constructor_args(con.clone())
        .map(|f| {
            format!(
                "{} {}",
                f.field_type.get_cpp_function_arg_type(),
                f.name.to_snake_case()
            )
        })
        .collect::<Vec<String>>()
        .join(", ");

    match con.constructor_type {
        ConstructorType::Normal => {
            f.writeln(&format!("{}::{}({}) : ", struct_name, struct_name, args))?;
            indented(f, |f| {
                for (field, last) in st.fields.iter().with_last() {
                    let value = match con.values.iter().find(|x| x.name == field.name) {
                        None => {
                            let argument = if field.field_type.is_move_type() {
                                std_move(field.name.to_snake_case())
                            } else {
                                field.name.to_snake_case()
                            };
                            format!("{}({})", field.name.to_snake_case(), argument)
                        }
                        Some(default) => {
                            format!(
                                "{}({})",
                                field.name.to_snake_case(),
                                get_default_value(&default.value)
                            )
                        }
                    };

                    if last {
                        f.writeln(&value)?;
                    } else {
                        f.writeln(&format!("{},", value))?;
                    }
                }
                Ok(())
            })?;
            f.writeln("{}")?;
        }
        ConstructorType::Static => {
            f.writeln(&format!(
                "{} {}::{}({})",
                struct_name,
                struct_name,
                con.name.to_snake_case(),
                args
            ))?;
            blocked(f, |f| {
                f.writeln(&format!("return {}(", struct_name))?;
                indented(f, |f| {
                    for (field, last) in st.fields.iter().with_last() {
                        let value = match con.values.iter().find(|x| x.name == field.name) {
                            None => field.name.to_snake_case(),
                            Some(iv) => get_default_value(&iv.value),
                        };
                        if last {
                            f.writeln(&value)?;
                        } else {
                            f.writeln(&format!("{},", value))?;
                        }
                    }
                    Ok(())
                })?;
                f.writeln(");")
            })?;
        }
    }

    f.newline()
}

fn write_move_self_guard(f: &mut dyn Printer) -> FormattingResult<()> {
    f.writeln("if(!self)")?;
    blocked(f, |f| {
        f.writeln("throw std::logic_error(\"class method invoked after move operation\");")
    })
}

fn write_class_implementation(f: &mut dyn Printer, handle: &ClassHandle) -> FormattingResult<()> {
    let cpp_name = handle.core_cpp_type();

    // write constructor
    for constructor in &handle.constructor {
        f.writeln(&format!(
            "{}::{}({}) : self(fn::{}({}))",
            cpp_name,
            cpp_name,
            cpp_function_args(&constructor.parameters),
            constructor.name.to_snake_case(),
            cpp_function_arg_invocation(&constructor.parameters)
        ))?;
        f.writeln("{}")?;
        f.newline()?;
    }

    // write the destructor
    f.writeln(&format!("{}::~{}()", cpp_name, cpp_name))?;
    blocked(f, |f| {
        if let Some(destructor) = &handle.destructor {
            f.writeln("if(self)")?;
            blocked(f, |f| {
                f.writeln(&format!("fn::{}(*this);", destructor.name.to_snake_case()))
            })?;
        }
        Ok(())
    })?;

    f.newline()?;

    // write the static methods
    for method in &handle.static_methods {
        write_class_static_method_impl(f, handle, method)?;
    }

    // write the methods
    for method in &handle.methods {
        write_class_method_impl(f, handle, method)?;
    }

    // write the async methods
    for method in &handle.async_methods {
        write_class_async_method_impl(f, handle, method)?;
    }

    f.newline()
}

fn write_class_static_method_impl(
    f: &mut dyn Printer,
    handle: &ClassHandle,
    method: &Method,
) -> FormattingResult<()> {
    let cpp_name = handle.core_cpp_type();
    let return_type = method
        .native_function
        .return_type
        .get_cpp_function_return_type();

    let native_function_name = method.native_function.name.to_snake_case();
    let invocation = format!(
        "fn::{}({})",
        native_function_name,
        cpp_function_arg_invocation(&method.native_function.parameters)
    );

    f.writeln(&format!(
        "{} {}::{}({})",
        return_type,
        cpp_name,
        method.name.to_snake_case(),
        cpp_function_args(&method.native_function.parameters)
    ))?;
    blocked(f, |f| match &method.native_function.return_type {
        FunctionReturnType::Void => f.writeln(&format!("{};", invocation)),
        FunctionReturnType::Type(t, _) => {
            f.writeln(&format!("return {};", t.to_cpp_return_value(invocation)))
        }
    })?;
    f.newline()
}

fn write_class_method_impl(
    f: &mut dyn Printer,
    handle: &ClassHandle,
    method: &Method,
) -> FormattingResult<()> {
    write_class_method_impl_generic(f, handle, &method.name, &method.native_function)
}

fn write_class_async_method_impl(
    f: &mut dyn Printer,
    handle: &ClassHandle,
    method: &AsyncMethod,
) -> FormattingResult<()> {
    write_class_method_impl_generic(f, handle, &method.name, &method.native_function)
}

fn write_class_method_impl_generic(
    f: &mut dyn Printer,
    handle: &ClassHandle,
    cpp_method_name: &str,
    native_function: &FunctionHandle,
) -> FormattingResult<()> {
    let cpp_name = handle.core_cpp_type();
    let args = &native_function.parameters[1..];
    let native_function_name = native_function.name.to_snake_case();
    let invocation = if args.is_empty() {
        format!("fn::{}(*this)", native_function_name)
    } else {
        format!(
            "fn::{}(*this, {})",
            native_function_name,
            cpp_function_arg_invocation(args)
        )
    };

    f.writeln(&format!(
        "{} {}::{}({})",
        native_function.return_type.get_cpp_function_return_type(),
        cpp_name,
        cpp_method_name.to_snake_case(),
        cpp_function_args(args)
    ))?;
    blocked(f, |f| {
        write_move_self_guard(f)?;
        f.newline()?;

        match &native_function.return_type {
            FunctionReturnType::Void => f.writeln(&format!("{};", invocation)),
            FunctionReturnType::Type(t, _) => {
                f.writeln(&format!("return {};", t.to_cpp_return_value(invocation)))
            }
        }
    })?;
    f.newline()
}

fn print_friend_class(
    f: &mut dyn Printer,
    lib: &Library,
    handle: &ClassHandle,
) -> FormattingResult<()> {
    let iterator = include_str!("./snippet/class_friend_class.hpp");
    let c_type = format!("{}_{}_t", lib.c_ffi_prefix, handle.name().to_snake_case());
    let cpp_type = handle.core_cpp_type();
    for line in iterator.lines() {
        let substituted = line
            .replace("<c_type>", &c_type)
            .replace("<cpp_type>", &cpp_type);
        f.writeln(&substituted)?;
    }
    f.newline()
}

fn cpp_function_args(args: &[Arg<FunctionArgument>]) -> String {
    args.iter()
        .map(|arg| {
            format!(
                "{} {}",
                arg.arg_type.get_cpp_function_arg_type(),
                arg.name.to_snake_case()
            )
        })
        .collect::<Vec<String>>()
        .join(", ")
}

fn get_arg_transform(arg: &FunctionArgument) -> Option<fn(String) -> String> {
    match arg {
        FunctionArgument::Basic(_) => None,
        FunctionArgument::String(_) => None,
        FunctionArgument::Collection(_) => None,
        FunctionArgument::Struct(_) => None,
        FunctionArgument::StructRef(_) => None,
        FunctionArgument::ClassRef(_) => None,
        FunctionArgument::Interface(x) => match x.interface_type {
            InterfaceType::Synchronous => None,
            InterfaceType::Asynchronous => Some(|x| format!("std::move({})", x)),
        },
    }
}

fn cpp_function_arg_invocation(args: &[Arg<FunctionArgument>]) -> String {
    args.iter()
        .map(|x| {
            let arg_name = x.name.to_snake_case();
            match get_arg_transform(&x.arg_type) {
                None => arg_name,
                Some(transform) => transform(arg_name),
            }
        })
        .collect::<Vec<String>>()
        .join(", ")
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
            f.writeln("&_return_value")?;
        }
        Ok(())
    }

    f.writeln(&format!(
        "{} {}({})",
        func.return_type.to_c_type(&lib.c_ffi_prefix),
        func.name.to_snake_case(),
        cpp_function_args(&func.parameters)
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
                        "{} _return_value;",
                        t.to_c_type(&lib.c_ffi_prefix)
                    ))?;
                    f.writeln(&format!("const auto _error =  {}(", c_func_name))?;
                    indented(f, |f| write_args(f, func, true))?;
                    f.writeln(");")?;
                    write_error_check(lib, f, err)?;
                    f.writeln("return _return_value;")
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
            StructType::FunctionReturn(x) => write_native_to_cpp_struct_conversion(f, lib, x),
            StructType::CallbackArg(x) => write_native_to_cpp_struct_conversion(f, lib, x),
            StructType::Universal(x) => {
                write_cpp_to_native_struct_conversion(f, lib, x)?;
                write_native_to_cpp_struct_conversion(f, lib, x)
            }
        },
        Statement::EnumDefinition(x) => {
            write_enum_to_native_conversion(lib, f, x)?;
            write_enum_to_cpp_conversion(lib, f, x)
        }
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
    let cpp_type = handle.core_cpp_type();
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
    let cpp_type = handle.core_cpp_type();
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
    let cpp_type = handle.core_cpp_type();
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
    T: StructFieldType + CppFunctionArgType + TypeInfo,
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
        let cpp_type = handle.core_cpp_type();
        f.writeln(&format!("static {} init({})", cpp_type, args))?;
        blocked(f, |f| {
            f.writeln(&format!("return {}(", cpp_type))?;
            indented(f, |f| {
                for (field, last) in handle.fields().with_last() {
                    let value = if field.field_type.is_move_type() {
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
            handle.core_cpp_type(),
            handle.iter_type.to_c_type(&lib.c_ffi_prefix)
        ))?;
        blocked(f, |f| {
            f.writeln(&format!("return {}(value);", handle.core_cpp_type()))
        })?;
        f.newline()?;
        f.writeln(&format!(
            "static {}* get(const {}& value)",
            c_type,
            handle.core_cpp_type()
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
    let namespaced_type = format!("::{}::{}", lib.c_ffi_prefix, handle.core_cpp_type());
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

    let cpp_type = format!("::{}::{}", lib.c_ffi_prefix, handle.core_cpp_type());
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
        handle.core_cpp_type()
    ))?;
    f.writeln("{")?;
    indented(f, |f| {
        f.writeln("switch(value)")?;
        f.writeln("{")?;
        indented(f, |f| {
            for v in &handle.variants {
                f.writeln(&format!(
                    "case {}::{}: return \"{}\";",
                    handle.core_cpp_type(),
                    v.core_cpp_type(),
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

fn write_enum_to_native_conversion(
    lib: &Library,
    f: &mut dyn Printer,
    handle: &EnumHandle,
) -> FormattingResult<()> {
    let cpp_type = format!("::{}::{}", lib.c_ffi_prefix, handle.core_cpp_type());
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

fn write_enum_to_cpp_conversion(
    lib: &Library,
    f: &mut dyn Printer,
    handle: &EnumHandle,
) -> FormattingResult<()> {
    let cpp_type = format!("::{}::{}", lib.c_ffi_prefix, handle.core_cpp_type());
    f.writeln(&format!(
        "{} to_cpp({}_{}_t value)",
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

    f.newline()
}

fn write_callback_function(
    f: &mut dyn Printer,
    lib: &Library,
    interface: &InterfaceHandle,
    cb: &CallbackFunction,
) -> FormattingResult<()> {
    let cpp_type = format!("::{}::{}", lib.c_ffi_prefix, interface.core_cpp_type());

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
    let cpp_type = format!("::{}::{}", lib.c_ffi_prefix, handle.core_cpp_type());
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
