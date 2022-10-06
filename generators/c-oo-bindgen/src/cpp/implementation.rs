use std::path::Path;

use oo_bindgen::backend::*;
use oo_bindgen::model::*;

use crate::cpp::conversion::*;
use crate::cpp::formatting::{const_ref, mut_ref, namespace, std_move, unique_ptr, FriendClass};
use crate::ctype::CType;

pub(crate) fn generate_cpp_file(lib: &Library, path: &Path) -> FormattingResult<()> {
    // Open the file
    logged::create_dir_all(&path)?;
    let filename = path.join(format!("{}.cpp", lib.settings.name));
    let mut f = FilePrinter::new(filename)?;

    f.writeln(&format!("#include \"{}.h\"", lib.settings.name))?;
    f.writeln(&format!("#include \"{}.hpp\"", lib.settings.name))?;
    f.newline()?;

    namespace(&mut f, &lib.settings.c_ffi_prefix, |f| {
        write_friend_classes(lib, f)
    })?;

    f.newline()?;

    // conversions
    namespace(&mut f, "convert", |f| {
        for line in include_str!("./snippet/convert_time.cpp").lines() {
            f.writeln(line)?;
        }
        f.newline()?;

        // emit the conversions in statement order as some conversions reference other conversions
        for statement in lib.statements() {
            write_conversions(f, statement)?;
        }

        Ok(())
    })?;

    namespace(&mut f, &lib.settings.c_ffi_prefix, |f| {
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
        write_collection_class_definition(f, col)?;
        write_collection_class_friend(f, col)?;
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
    col: &Handle<Collection<Validated>>,
) -> FormattingResult<()> {
    //let c_type = col.collection_type.to_c_type(&lib.c_ffi_prefix);
    let cpp_type = col.collection_class.core_cpp_type();
    let constructor = format!("fn::{}", col.create_func.name);

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
            f.writeln(&format!("fn::{}(*this, x);", col.add_func.name))
        })
    })?;
    f.newline()?;

    // write the destructor
    f.writeln(&format!("{}::~{}()", cpp_type, cpp_type))?;
    blocked(f, |f| {
        f.writeln(&format!("fn::{}(*this);", col.delete_func.name))
    })?;
    f.newline()
}

fn write_collection_class_definition(
    f: &mut dyn Printer,
    col: &Handle<Collection<Validated>>,
) -> FormattingResult<()> {
    let cpp_type = col.collection_class.core_cpp_type();
    let c_type = col.collection_class.to_c_type();
    f.writeln(&format!("class {}", cpp_type))?;
    f.writeln("{")?;
    indented(f, |f| {
        f.writeln(&format!(
            "friend class {};",
            col.collection_class.friend_class()
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
    f: &mut dyn Printer,
    col: &Handle<Collection<Validated>>,
) -> FormattingResult<()> {
    let c_type = col.collection_class.to_c_type();
    f.writeln(&format!("class {}", col.collection_class.friend_class()))?;
    f.writeln("{")?;
    f.writeln("public:")?;
    indented(f, |f| {
        f.writeln(&format!(
            "static {}* get({}& value)",
            c_type,
            col.collection_class.core_cpp_type()
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
        write_iterator_friend_class(f, it)?;
    }

    for class in lib.classes() {
        print_friend_class(f, class)?;
    }

    Ok(())
}

fn write_function_wrappers(lib: &Library, f: &mut dyn Printer) -> FormattingResult<()> {
    f.writeln("// C++ wrappers around native functions that do argument conversion and map errors to exceptions")?;
    f.writeln("// We don't convert the return type here as there are nuances that require it to be converted at the call site")?;
    namespace(f, "fn", |f| {
        for func in lib.functions() {
            write_function_wrapper(f, func)?;
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
        write_iterator_methods(f, it)?;
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
    f: &mut dyn Printer,
    it: &Handle<AbstractIterator<Validated>>,
) -> FormattingResult<()> {
    let c_class_type = it.iter_class.to_c_type();
    let cpp_class_type = it.iter_class.core_cpp_type();
    let c_next = it.next_function.to_c_type();
    let cpp_value_type = it.item_type.core_cpp_type();
    let c_value_type = it.item_type.to_c_type();

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

        match it.item_type {
            IteratorItemType::Primitive(_) => f.writeln(&format!(
                "return *reinterpret_cast<{}>(this->current);",
                c_value_type
            )),
            IteratorItemType::Struct(_) => f.writeln(&format!(
                "return ::convert::to_cpp(*reinterpret_cast<{}*>(this->current));",
                c_value_type
            )),
        }
    })?;

    f.newline()?;

    Ok(())
}

fn write_static_class_method(
    f: &mut dyn Printer,
    class: &Handle<StaticClass<Validated>>,
    method: &StaticMethod<Validated>,
) -> FormattingResult<()> {
    fn get_invocation_args(args: &[Arg<FunctionArgument, Validated>]) -> String {
        args.iter()
            .map(transform_arg)
            .collect::<Vec<String>>()
            .join(", ")
    }

    let args: String = method
        .native_function
        .arguments
        .iter()
        .map(|arg| format!("{} {}", arg.arg_type.get_cpp_function_arg_type(), arg.name))
        .collect::<Vec<String>>()
        .join(", ");

    f.writeln(&format!(
        "{} {}::{}({})",
        method
            .native_function
            .return_type
            .get_cpp_function_return_type(),
        class.core_cpp_type(),
        method.name,
        args
    ))?;
    blocked(f, |f| {
        let invocation = format!(
            "fn::{}({})",
            method.native_function.name,
            get_invocation_args(&method.native_function.arguments)
        );

        match &method.native_function.return_type.get_value() {
            None => f.writeln(&format!("{};", invocation)),
            Some(t) => {
                let transformed_return_value = if t.transform_in_wrapper() {
                    f.writeln("// return type already transformed in the wrapper")?;
                    invocation
                } else {
                    // perform the transform here
                    f.writeln("// transform the return value")?;
                    t.to_cpp_return_value(invocation)
                };
                f.writeln(&format!("return {};", transformed_return_value))
            }
        }
    })
}

fn write_struct_constructors<T>(
    f: &mut dyn Printer,
    st: &Handle<Struct<T, Validated>>,
) -> FormattingResult<()>
where
    T: StructFieldType + CppFunctionArgType + TypeInfo,
{
    if !st.has_full_initializer() {
        let constructor = Handle::new(Initializer::full(
            InitializerType::Normal,
            brief("full constructor"),
        ));
        write_struct_constructor(f, st, &constructor)?;
    }

    for constructor in &st.initializers {
        write_struct_constructor(f, st, constructor)?;
    }
    Ok(())
}

fn get_default_value(default: &ValidatedDefaultValue) -> String {
    match default {
        ValidatedDefaultValue::Bool(x) => x.to_string(),
        ValidatedDefaultValue::Number(x) => match x {
            NumberValue::U8(x) => x.to_string(),
            NumberValue::S8(x) => x.to_string(),
            NumberValue::U16(x) => x.to_string(),
            NumberValue::S16(x) => x.to_string(),
            NumberValue::U32(x) => x.to_string(),
            NumberValue::S32(x) => x.to_string(),
            NumberValue::U64(x) => x.to_string(),
            NumberValue::S64(x) => x.to_string(),
            NumberValue::Float(x) => format!("{}f", x),
            NumberValue::Double(x) => x.to_string(),
        },
        ValidatedDefaultValue::Duration(_, x) => {
            format!("std::chrono::milliseconds({})", x.as_millis())
        }
        ValidatedDefaultValue::Enum(x, variant) => {
            format!("{}::{}", x.core_cpp_type(), variant)
        }
        ValidatedDefaultValue::String(x) => {
            format!("\"{}\"", x)
        }
        ValidatedDefaultValue::DefaultStruct(st, ct, c_name) => match ct {
            InitializerType::Normal => format!("{}()", st.name().camel_case()),
            InitializerType::Static => format!("{}()", c_name.camel_case()),
        },
    }
}

fn write_struct_constructor<T>(
    f: &mut dyn Printer,
    st: &Handle<Struct<T, Validated>>,
    con: &Handle<Initializer<Validated>>,
) -> FormattingResult<()>
where
    T: StructFieldType + CppFunctionArgType + TypeInfo,
{
    let struct_name = st.core_cpp_type();
    let args = st
        .initializer_args(con.clone())
        .map(|f| format!("{} {}", f.field_type.get_cpp_function_arg_type(), f.name))
        .collect::<Vec<String>>()
        .join(", ");

    match con.initializer_type {
        InitializerType::Normal => {
            f.writeln(&format!("{}::{}({}) : ", struct_name, struct_name, args))?;
            indented(f, |f| {
                for (field, last) in st.fields.iter().with_last() {
                    let value = match con.values.iter().find(|x| x.name == field.name) {
                        None => {
                            let argument = if field.field_type.is_move_type() {
                                std_move(field.name.clone())
                            } else {
                                field.name.to_string()
                            };
                            format!("{}({})", field.name, argument)
                        }
                        Some(default) => {
                            format!("{}({})", field.name, get_default_value(&default.value))
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
        InitializerType::Static => {
            f.writeln(&format!(
                "{} {}::{}({})",
                struct_name, struct_name, con.name, args
            ))?;
            blocked(f, |f| {
                f.writeln(&format!("return {}(", struct_name))?;
                indented(f, |f| {
                    for (field, last) in st.fields.iter().with_last() {
                        let value = match con.values.iter().find(|x| x.name == field.name) {
                            None => field.name.to_string(),
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

fn write_class_implementation(
    f: &mut dyn Printer,
    handle: &Handle<Class<Validated>>,
) -> FormattingResult<()> {
    let cpp_name = handle.core_cpp_type();

    // write constructor
    for constructor in &handle.constructor {
        f.writeln(&format!(
            "{}::{}({}) : self(fn::{}({}))",
            cpp_name,
            cpp_name,
            cpp_function_args(&constructor.function.arguments),
            constructor.function.name,
            cpp_function_arg_invocation(&constructor.function.arguments)
        ))?;
        f.writeln("{}")?;
        f.newline()?;
    }

    // write the destructor
    for destructor in &handle.destructor {
        f.writeln(&format!("{}::~{}()", cpp_name, cpp_name))?;
        blocked(f, |f| {
            f.writeln("if(self)")?;
            blocked(f, |f| {
                f.writeln(&format!("fn::{}(*this);", destructor.function.name))
            })?;
            Ok(())
        })?;
    }

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
    for method in &handle.future_methods {
        write_class_future_method_impl(f, handle, method)?;
    }

    f.newline()
}

fn write_class_static_method_impl(
    f: &mut dyn Printer,
    handle: &Handle<Class<Validated>>,
    method: &StaticMethod<Validated>,
) -> FormattingResult<()> {
    let cpp_name = handle.core_cpp_type();
    let return_type = method
        .native_function
        .return_type
        .get_cpp_function_return_type();

    let native_function_name = method.native_function.name.clone();
    let invocation = format!(
        "fn::{}({})",
        native_function_name,
        cpp_function_arg_invocation(&method.native_function.arguments)
    );

    f.writeln(&format!(
        "{} {}::{}({})",
        return_type,
        cpp_name,
        method.name,
        cpp_function_args(&method.native_function.arguments)
    ))?;
    blocked(f, |f| {
        match &method.native_function.return_type.get_value() {
            None => f.writeln(&format!("{};", invocation)),
            Some(t) => {
                let transformed_return_value = if t.transform_in_wrapper() {
                    f.writeln("// return type already transformed in the wrapper")?;
                    invocation
                } else {
                    // perform the transform here
                    f.writeln("// transform the return value")?;
                    t.to_cpp_return_value(invocation)
                };
                f.writeln(&format!("return {};", transformed_return_value))
            }
        }
    })?;
    f.newline()
}

fn write_class_method_impl(
    f: &mut dyn Printer,
    handle: &Handle<Class<Validated>>,
    method: &Method<Validated>,
) -> FormattingResult<()> {
    write_class_method_impl_generic(f, handle, &method.name, &method.native_function)
}

fn write_class_future_method_impl(
    f: &mut dyn Printer,
    handle: &Handle<Class<Validated>>,
    method: &FutureMethod<Validated>,
) -> FormattingResult<()> {
    write_class_method_impl_generic(f, handle, &method.name, &method.native_function)
}

fn write_class_method_impl_generic(
    f: &mut dyn Printer,
    handle: &Handle<Class<Validated>>,
    cpp_method_name: &str,
    native_function: &Handle<Function<Validated>>,
) -> FormattingResult<()> {
    let cpp_name = handle.core_cpp_type();
    let args = &native_function.arguments[1..];
    let native_function_name = native_function.name.clone();
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
        cpp_method_name,
        cpp_function_args(args)
    ))?;
    blocked(f, |f| {
        write_move_self_guard(f)?;
        f.newline()?;

        match &native_function.return_type.get_value() {
            None => f.writeln(&format!("{};", invocation)),
            Some(t) => {
                let transformed_return_value = if t.transform_in_wrapper() {
                    f.writeln("// return type already transformed in the wrapper")?;
                    invocation
                } else {
                    // perform the transform here
                    f.writeln("// transform the return value")?;
                    t.to_cpp_return_value(invocation)
                };
                f.writeln(&format!("return {};", transformed_return_value))
            }
        }
    })?;
    f.newline()
}

fn print_friend_class(
    f: &mut dyn Printer,
    handle: &Handle<Class<Validated>>,
) -> FormattingResult<()> {
    let iterator = include_str!("./snippet/class_friend_class.hpp");
    let c_type = handle.declaration.to_c_type();
    let cpp_type = handle.core_cpp_type();
    for line in iterator.lines() {
        let substituted = line
            .replace("<c_type>", &c_type)
            .replace("<cpp_type>", &cpp_type);
        f.writeln(&substituted)?;
    }
    f.newline()
}

fn cpp_function_args(args: &[Arg<FunctionArgument, Validated>]) -> String {
    args.iter()
        .map(|arg| format!("{} {}", arg.arg_type.get_cpp_function_arg_type(), arg.name))
        .collect::<Vec<String>>()
        .join(", ")
}

fn transform_arg(arg: &Arg<FunctionArgument, Validated>) -> String {
    if arg.arg_type.is_move_type() {
        std_move(arg.name.clone())
    } else {
        arg.name.to_string()
    }
}

fn cpp_function_arg_invocation(args: &[Arg<FunctionArgument, Validated>]) -> String {
    args.iter()
        .map(transform_arg)
        .collect::<Vec<String>>()
        .join(", ")
}

const RETURN_VALUE: &str = "_oo_bindgen_return_value";

fn write_function_wrapper(
    f: &mut dyn Printer,
    func: &Handle<Function<Validated>>,
) -> FormattingResult<()> {
    fn write_error_check(f: &mut dyn Printer, err: &ErrorType<Validated>) -> FormattingResult<()> {
        let c_success_variant = &format!(
            "{}_{}_{}",
            err.inner.settings.c_ffi_prefix.capital_snake_case(),
            err.inner.name.capital_snake_case(),
            err.inner
                .variants
                .first()
                .unwrap()
                .name
                .capital_snake_case(),
        );
        f.writeln(&format!("if(_error != {})", c_success_variant))?;
        blocked(f, |f| {
            f.writeln(&format!(
                "throw {}({});",
                err.exception_name.camel_case(),
                err.inner.to_cpp("_error".to_string())
            ))
        })
    }

    fn write_shadowed_conversions(
        f: &mut dyn Printer,
        func: &Handle<Function<Validated>>,
    ) -> FormattingResult<()> {
        for arg in &func.arguments {
            if arg.arg_type.requires_shadow_parameter() {
                f.writeln(&format!(
                    "auto _{} = {};",
                    arg.name,
                    arg.arg_type
                        .to_native_function_argument(arg.name.to_string())
                ))?;
            }
        }
        Ok(())
    }

    fn write_args(
        f: &mut dyn Printer,
        func: &Handle<Function<Validated>>,
        has_out_param: bool,
    ) -> FormattingResult<()> {
        for (arg, last) in func.arguments.iter().with_last() {
            let conversion = match arg.arg_type.shadow_parameter_mapping() {
                None => arg
                    .arg_type
                    .to_native_function_argument(arg.name.to_string()),
                Some(transform) => transform(format!("_{}", arg.name)),
            };
            if last && !has_out_param {
                f.writeln(&conversion)?;
            } else {
                f.writeln(&format!("{},", conversion))?;
            }
        }
        if has_out_param {
            f.writeln(&format!("&{}", RETURN_VALUE))?;
        }
        Ok(())
    }

    let return_type = match &func.return_type.get_value() {
        None => func.return_type.get_cpp_function_return_type(),
        Some(t) => {
            if t.transform_in_wrapper() {
                func.return_type.get_cpp_function_return_type()
            } else {
                t.to_c_type()
            }
        }
    };

    f.writeln(&format!(
        "{} {}({})",
        return_type,
        func.name,
        cpp_function_args(&func.arguments)
    ))?;

    blocked(f, |f| {
        let c_func_name = func.to_c_type();
        write_shadowed_conversions(f, func)?;
        match func.error_type.get() {
            None => match &func.return_type.get_value() {
                None => {
                    f.writeln(&format!("{}(", c_func_name))?;
                    indented(f, |f| write_args(f, func, false))?;
                    f.writeln(");")
                }
                Some(t) => {
                    f.writeln(&format!("const auto {} = {}(", RETURN_VALUE, c_func_name))?;
                    indented(f, |f| write_args(f, func, false))?;
                    f.writeln(");")?;
                    let return_value = if t.transform_in_wrapper() {
                        t.to_cpp_return_value(RETURN_VALUE.to_string())
                    } else {
                        RETURN_VALUE.to_string()
                    };
                    f.writeln(&format!("return {};", return_value))
                }
            },
            Some(err) => match &func.return_type.get_value() {
                None => {
                    f.writeln(&format!("const auto _error = {}(", c_func_name))?;
                    indented(f, |f| write_args(f, func, false))?;
                    f.writeln(");")?;
                    write_error_check(f, err)
                }
                Some(t) => {
                    f.writeln(&format!("{} {};", t.to_c_type(), RETURN_VALUE))?;
                    f.writeln(&format!("const auto _error =  {}(", c_func_name))?;
                    indented(f, |f| write_args(f, func, true))?;
                    f.writeln(");")?;
                    write_error_check(f, err)?;
                    let return_value = if t.transform_in_wrapper() {
                        t.to_cpp_return_value(RETURN_VALUE.to_string())
                    } else {
                        RETURN_VALUE.to_string()
                    };
                    f.writeln(&format!("return {};", return_value))
                }
            },
        }
    })?;
    f.newline()
}

fn write_conversions(
    f: &mut dyn Printer,
    statement: &Statement<Validated>,
) -> FormattingResult<()> {
    match statement {
        Statement::StructDefinition(x) => match x {
            StructType::FunctionArg(x) => write_cpp_to_native_struct_conversion(f, x),
            StructType::FunctionReturn(x) => write_native_to_cpp_struct_conversion(f, x),
            StructType::CallbackArg(x) => write_native_to_cpp_struct_conversion(f, x),
            StructType::Universal(x) => {
                write_cpp_to_native_struct_conversion(f, x)?;
                write_native_to_cpp_struct_conversion(f, x)
            }
        },
        Statement::EnumDefinition(x) => {
            write_enum_to_native_conversion(f, x)?;
            write_enum_to_cpp_conversion(f, x)
        }
        Statement::InterfaceDefinition(x) => {
            write_cpp_interface_to_native_conversion(f, x.untyped())
        }
        Statement::ClassDefinition(x) => write_class_construct_helper(f, x),
        Statement::IteratorDeclaration(x) => {
            write_iterator_construct_helper(f, x)?;
            write_iterator_to_native_helper(f, x)
        }
        _ => Ok(()),
    }
}

fn write_iterator_construct_helper(
    f: &mut dyn Printer,
    handle: &Handle<AbstractIterator<Validated>>,
) -> FormattingResult<()> {
    let cpp_type = handle.core_cpp_type();
    let signature = format!(
        "::{}::{} construct({}* self)",
        handle.settings.c_ffi_prefix,
        cpp_type,
        handle.iter_class.to_c_type()
    );
    f.writeln(&signature)?;
    blocked(f, |f| {
        f.writeln(&format!(
            "return ::{}::{}::init(self);",
            handle.settings.c_ffi_prefix,
            handle.friend_class()
        ))
    })?;
    f.newline()
}

fn write_iterator_to_native_helper(
    f: &mut dyn Printer,
    handle: &Handle<AbstractIterator<Validated>>,
) -> FormattingResult<()> {
    let cpp_type = handle.core_cpp_type();
    let signature = format!(
        "{}* to_native(const ::{}::{}& value)",
        handle.iter_class.to_c_type(),
        handle.settings.c_ffi_prefix,
        cpp_type
    );
    f.writeln(&signature)?;
    blocked(f, |f| {
        f.writeln(&format!(
            "return ::{}::{}::get(value);",
            handle.settings.c_ffi_prefix,
            handle.friend_class()
        ))
    })?;
    f.newline()
}

fn write_class_construct_helper(
    f: &mut dyn Printer,
    handle: &Handle<Class<Validated>>,
) -> FormattingResult<()> {
    let cpp_type = handle.core_cpp_type();
    let signature = format!(
        "::{}::{} to_cpp({}* self)",
        handle.settings.c_ffi_prefix,
        cpp_type,
        handle.declaration.to_c_type()
    );
    f.writeln(&signature)?;
    blocked(f, |f| {
        f.writeln(&format!(
            "return ::{}::{}::init(self);",
            handle.settings.c_ffi_prefix,
            handle.friend_class()
        ))
    })?;
    f.newline()
}

fn write_cpp_struct_friend_class<T>(
    f: &mut dyn Printer,
    handle: &Handle<Struct<T, Validated>>,
) -> FormattingResult<()>
where
    T: StructFieldType + CppFunctionArgType + TypeInfo,
{
    let args = handle
        .fields
        .iter()
        .map(|x| format!("{} {}", x.field_type.get_cpp_function_arg_type(), x.name))
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
                        format!("std::move({})", field.name)
                    } else {
                        field.name.to_string()
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
                    field.name,
                    const_ref(cpp_type.clone())
                ))?;
                blocked(f, |f| f.writeln(&format!("return value.{};", field.name)))?;
            }
        }
        Ok(())
    })?;
    f.writeln("};")?;
    f.newline()
}

fn write_iterator_friend_class(
    f: &mut dyn Printer,
    handle: &Handle<AbstractIterator<Validated>>,
) -> FormattingResult<()> {
    let c_type = handle.iter_class.to_c_type();

    f.writeln(&format!("class {}", handle.friend_class()))?;
    f.writeln("{")?;
    f.writeln("public:")?;
    indented(f, |f| {
        f.newline()?;
        f.writeln(&format!(
            "static {} init({}* value)",
            handle.core_cpp_type(),
            handle.iter_class.to_c_type()
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
    handle: &Handle<Struct<T, Validated>>,
) -> FormattingResult<()>
where
    T: StructFieldType + ToNativeStructField,
{
    let namespaced_type = format!(
        "::{}::{}",
        handle.declaration.inner.settings.c_ffi_prefix,
        handle.core_cpp_type()
    );
    let value_type = if handle.fields.iter().any(|f| f.field_type.requires_move()) {
        mut_ref(namespaced_type)
    } else {
        const_ref(namespaced_type)
    };

    let c_type = handle.to_c_type();
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
                        format!("value.{}", field.name)
                    }
                    Visibility::Private => {
                        format!(
                            "::{}::{}::get_{}(value)",
                            handle.settings().c_ffi_prefix,
                            handle.friend_class(),
                            field.name
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
    handle: &Handle<Struct<T, Validated>>,
) -> FormattingResult<()>
where
    T: StructFieldType + ToCppStructField,
{
    let c_type = handle.to_c_type();

    let cpp_type = format!(
        "::{}::{}",
        handle.settings().c_ffi_prefix,
        handle.core_cpp_type()
    );
    f.writeln(&format!("{} to_cpp({} value)", cpp_type, const_ref(c_type)))?;
    blocked(f, |f| {
        f.writeln(&format!(
            "return ::{}::{}::init(",
            handle.settings().c_ffi_prefix,
            handle.friend_class()
        ))?;
        indented(f, |f| {
            for (field, last) in handle.fields.iter().with_last() {
                let native_value = format!("value.{}", field.name);
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

fn write_enum_to_string_impl(
    f: &mut dyn Printer,
    handle: &Handle<Enum<Validated>>,
) -> FormattingResult<()> {
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
                handle.name.camel_case()
            ))
        })?;
        f.writeln("}")
    })?;
    f.writeln("}")?;
    f.newline()
}

fn write_enum_to_native_conversion(
    f: &mut dyn Printer,
    handle: &Handle<Enum<Validated>>,
) -> FormattingResult<()> {
    let cpp_type = format!(
        "::{}::{}",
        handle.settings.c_ffi_prefix,
        handle.core_cpp_type()
    );
    f.writeln(&format!(
        "{} to_native({} value)",
        handle.to_c_type(),
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
                    handle.settings.c_ffi_prefix,
                    handle.name.camel_case(),
                    v.name,
                    handle.settings.c_ffi_prefix.capital_snake_case(),
                    handle.name.capital_snake_case(),
                    v.name.capital_snake_case(),
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
    f: &mut dyn Printer,
    handle: &Handle<Enum<Validated>>,
) -> FormattingResult<()> {
    let cpp_type = format!(
        "::{}::{}",
        handle.settings.c_ffi_prefix,
        handle.core_cpp_type()
    );
    f.writeln(&format!(
        "{} to_cpp({} value)",
        cpp_type,
        handle.to_c_type()
    ))?;
    f.writeln("{")?;
    indented(f, |f| {
        f.writeln("switch(value)")?;
        f.writeln("{")?;
        indented(f, |f| {
            for v in &handle.variants {
                f.writeln(&format!(
                    "case {}_{}_{}: return ::{}::{}::{};",
                    handle.settings.c_ffi_prefix.capital_snake_case(),
                    handle.name.capital_snake_case(),
                    v.name.capital_snake_case(),
                    handle.settings.c_ffi_prefix,
                    handle.name.camel_case(),
                    v.name
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
    interface: &Handle<Interface<Validated>>,
    cb: &CallbackFunction<Validated>,
) -> FormattingResult<()> {
    let cpp_type = format!(
        "::{}::{}",
        interface.settings.c_ffi_prefix,
        interface.core_cpp_type()
    );

    fn write_invocation_lines(
        f: &mut dyn Printer,
        cb: &CallbackFunction<Validated>,
    ) -> FormattingResult<()> {
        for (arg, last) in cb.arguments.iter().with_last() {
            let conversion = if arg.arg_type.requires_shadow_parameter() {
                format!("_{}", arg.name)
            } else {
                arg.arg_type.to_cpp_callback_argument(arg.name.to_string())
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
        .map(|x| format!("{} {}", x.arg_type.to_c_type(), x.name))
        .chain(std::iter::once("void* ctx".to_string()))
        .collect::<Vec<String>>()
        .join(", ");

    f.writeln(&format!(
        "[]({}) -> {} {{",
        args,
        cb.return_type.to_c_type()
    ))?;
    indented(f, |f| {
        for arg in &cb.arguments {
            if arg.arg_type.requires_shadow_parameter() {
                f.writeln(&format!(
                    "auto _{} = {};",
                    arg.name,
                    arg.arg_type.to_cpp_callback_argument(arg.name.to_string())
                ))?;
            }
        }

        let function = format!("reinterpret_cast<{}*>(ctx)->{}", cpp_type, cb.name);
        match &cb.return_type.get_value() {
            None => {
                f.writeln(&format!("{}(", function))?;
                indented(f, |f| write_invocation_lines(f, cb))?;
                f.writeln(");")
            }
            Some(t) => {
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
    handle: &Handle<Interface<Validated>>,
) -> FormattingResult<()> {
    let c_type = handle.to_c_type();
    let cpp_type = format!(
        "::{}::{}",
        handle.settings.c_ffi_prefix,
        handle.core_cpp_type()
    );
    let argument_type = match handle.mode {
        InterfaceCategory::Synchronous => mut_ref(cpp_type.clone()),
        InterfaceCategory::Asynchronous => unique_ptr(cpp_type.clone()),
        InterfaceCategory::Future => unique_ptr(cpp_type.clone()),
    };
    f.writeln(&format!("{} to_native({} value)", c_type, argument_type,))?;
    blocked(f, |f| {
        f.writeln(&format!("return {} {{", handle.to_c_type()))?;
        indented(f, |f| {
            for cb in &handle.callbacks {
                write_callback_function(f, handle, cb)?;
            }
            match handle.mode {
                InterfaceCategory::Synchronous => {
                    f.writeln("[](void*){}, // nothing to free")?;
                }
                InterfaceCategory::Asynchronous | InterfaceCategory::Future => {
                    f.writeln(&format!(
                        "[](void* ctx) {{ delete reinterpret_cast<{}*>(ctx); }},",
                        cpp_type
                    ))?;
                }
            }
            match handle.mode {
                InterfaceCategory::Synchronous => {
                    f.writeln("&value // the pointer will outlive the callbacks")?;
                }
                InterfaceCategory::Asynchronous | InterfaceCategory::Future => {
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
