use super::doc::*;
use super::*;
use oo_bindgen::class::*;
use oo_bindgen::doc::Validated;
use oo_bindgen::error_type::ExceptionType;

pub(crate) fn generate(
    f: &mut dyn Printer,
    class: &Handle<Class<Validated>>,
) -> FormattingResult<()> {
    let classname = class.name().camel_case();

    // Documentation
    documentation(f, |f| javadoc_print(f, &class.doc))?;

    // Class definition
    f.writeln(&format!("public final class {}", classname))?;
    if matches!(class.destruction_mode, DestructionMode::Dispose) {
        f.write(" implements AutoCloseable")?;
    }

    blocked(f, |f| {
        f.writeln("final private long self;")?;
        if class.destructor.is_some() {
            f.writeln("private java.util.concurrent.atomic.AtomicBoolean disposed = new java.util.concurrent.atomic.AtomicBoolean(false);")?;
        }

        f.newline()?;

        f.writeln(&format!("private {}(long self)", classname))?;
        blocked(f, |f| f.writeln("this.self = self;"))?;

        f.newline()?;

        if let Some(constructor) = &class.constructor {
            generate_constructor(f, &classname, constructor)?;
            f.newline()?;
        }

        if let Some(destructor) = &class.destructor {
            generate_destructor(f, destructor, &class.destruction_mode)?;
            f.newline()?;
        }

        for method in &class.methods {
            generate_method(f, method)?;
            f.newline()?;
        }

        for method in &class.future_methods {
            generate_async_method(f, method)?;
            f.newline()?;
        }

        for method in &class.static_methods {
            generate_static_method(f, method)?;
            f.newline()?;
        }

        Ok(())
    })
}

pub(crate) fn generate_static(
    f: &mut dyn Printer,
    class: &Handle<StaticClass<Validated>>,
) -> FormattingResult<()> {
    let classname = class.name.camel_case();

    // Documentation
    documentation(f, |f| javadoc_print(f, &class.doc))?;

    // Class definition
    f.writeln(&format!("public final class {}", classname))?;

    blocked(f, |f| {
        // Private constructor to make it static
        f.writeln(&format!("private {}() {{ }}", classname))?;
        f.newline()?;

        for method in &class.static_methods {
            generate_static_method(f, method)?;
            f.newline()?;
        }

        Ok(())
    })
}

fn generate_constructor(
    f: &mut dyn Printer,
    classname: &str,
    constructor: &ClassConstructor<Validated>,
) -> FormattingResult<()> {
    documentation(f, |f| {
        // Print top-level documentation
        javadoc_print(f, &constructor.function.doc)?;
        f.newline()?;

        // Print each parameter value
        for param in constructor.function.arguments.iter() {
            f.writeln(&format!("@param {} ", param.name.mixed_case()))?;
            docstring_print(f, &param.doc)?;
        }

        // Print exception
        if let Some(error) = &constructor.function.error_type.get() {
            f.writeln(&format!(
                "@throws {} {}",
                error.exception_name.camel_case(),
                error.inner.name.camel_case()
            ))?;
        }

        Ok(())
    })?;

    f.writeln(&format!("public {}(", classname))?;
    f.write(
        &constructor
            .function
            .arguments
            .iter()
            .map(|param| {
                format!(
                    "{} {}",
                    param.arg_type.as_java_primitive(),
                    param.name.mixed_case()
                )
            })
            .collect::<Vec<String>>()
            .join(", "),
    )?;
    f.write(")")?;

    blocked(f, |f| {
        call_native_function(
            f,
            &constructor.function,
            &format!("{} object = ", classname),
            false,
        )?;
        f.writeln("this.self = object.self;")?;
        f.writeln("object.disposed.set(true);")
    })
}

fn generate_destructor(
    f: &mut dyn Printer,
    destructor: &ClassDestructor<Validated>,
    destruction_mode: &DestructionMode,
) -> FormattingResult<()> {
    if destruction_mode.is_manual_destruction() {
        documentation(f, |f| {
            // Print top-level documentation
            javadoc_print(f, &destructor.function.doc)?;
            f.newline()?;

            // Print each parameter value
            for param in destructor.function.arguments.iter().skip(1) {
                f.writeln(&format!("@param {} ", param.name.mixed_case()))?;
                docstring_print(f, &param.doc)?;
            }

            Ok(())
        })?;
    }

    match destruction_mode {
        DestructionMode::Automatic => {
            f.writeln("private void close()")?;
        }
        DestructionMode::Custom(name) => {
            f.writeln(&format!("public void {}()", name.mixed_case()))?;
        }
        DestructionMode::Dispose => {
            // AutoCloseable implementation
            f.writeln("@Override")?;
            f.writeln("public void close()")?;
        }
    }

    blocked(f, |f| {
        f.writeln("if (this.disposed.getAndSet(true))")?;
        f.writeln("    return;")?;

        f.newline()?;

        f.writeln(&format!(
            "{}.{}(this);",
            NATIVE_FUNCTIONS_CLASSNAME, destructor.function.name
        ))
    })?;

    f.newline()?;

    // Finalizer method
    f.writeln("@Override")?;
    f.writeln("public void finalize()")?;
    blocked(f, |f| {
        if let DestructionMode::Custom(name) = destruction_mode {
            f.writeln(&format!("this.{}();", name.mixed_case()))
        } else {
            f.writeln("this.close();")
        }
    })
}

fn generate_method(f: &mut dyn Printer, method: &Method<Validated>) -> FormattingResult<()> {
    documentation(f, |f| {
        // Print top-level documentation
        javadoc_print(f, &method.native_function.doc)?;
        f.newline()?;

        // Print each parameter value
        for param in method.native_function.arguments.iter().skip(1) {
            f.writeln(&format!("@param {} ", param.name.mixed_case()))?;
            docstring_print(f, &param.doc)?;
        }

        // Print return value
        if let Some(doc) = &method.native_function.return_type.get_doc() {
            f.writeln("@return ")?;
            docstring_print(f, doc)?;
        }

        // Print exception
        if let Some(error) = &method.native_function.error_type.get() {
            f.writeln(&format!(
                "@throws {} {}",
                error.exception_name.camel_case(),
                error.inner.name.camel_case()
            ))?;
        }

        Ok(())
    })?;

    f.writeln(&format!(
        "public {} {}(",
        method.native_function.return_type.as_java_primitive(),
        method.name.mixed_case()
    ))?;
    f.write(
        &method
            .native_function
            .arguments
            .iter()
            .skip(1)
            .map(|param| {
                format!(
                    "{} {}",
                    param.arg_type.as_java_primitive(),
                    param.name.mixed_case()
                )
            })
            .collect::<Vec<String>>()
            .join(", "),
    )?;
    f.write(")")?;

    if let Some(error) = method.native_function.error_type.get() {
        if error.exception_type == ExceptionType::CheckedException {
            f.write(&format!(" throws {}", error.exception_name.camel_case()))?;
        }
    }

    blocked(f, |f| {
        call_native_function(f, &method.native_function, "return ", true)
    })
}

fn generate_static_method(
    f: &mut dyn Printer,
    method: &StaticMethod<Validated>,
) -> FormattingResult<()> {
    documentation(f, |f| {
        // Print top-level documentation
        javadoc_print(f, &method.native_function.doc)?;
        f.newline()?;

        // Print each parameter value
        for param in method.native_function.arguments.iter() {
            f.writeln(&format!("@param {} ", param.name.mixed_case()))?;
            docstring_print(f, &param.doc)?;
        }

        // Print return value
        if let Some(doc) = &method.native_function.return_type.get_doc() {
            f.writeln("@return ")?;
            docstring_print(f, doc)?;
        }

        // Print exception
        if let Some(error) = &method.native_function.error_type.get() {
            f.writeln(&format!(
                "@throws {} {}",
                error.exception_name.camel_case(),
                error.inner.name.camel_case()
            ))?;
        }

        Ok(())
    })?;

    f.writeln(&format!(
        "public static {} {}(",
        method.native_function.return_type.as_java_primitive(),
        method.name.mixed_case()
    ))?;
    f.write(
        &method
            .native_function
            .arguments
            .iter()
            .map(|param| {
                format!(
                    "{} {}",
                    param.arg_type.as_java_primitive(),
                    param.name.mixed_case()
                )
            })
            .collect::<Vec<String>>()
            .join(", "),
    )?;
    f.write(")")?;

    if let Some(error) = method.native_function.error_type.get() {
        if error.exception_type == ExceptionType::CheckedException {
            f.write(&format!(" throws {}", error.exception_name.camel_case()))?;
        }
    }

    blocked(f, |f| {
        call_native_function(f, &method.native_function, "return ", false)
    })
}

fn generate_async_method(
    f: &mut dyn Printer,
    method: &FutureMethod<Validated>,
) -> FormattingResult<()> {
    let return_type = method.future.value_type.as_java_object();
    let settings = method.future.interface.settings.clone();
    let one_time_callback_name = method.future.interface.name.camel_case();
    let one_time_callback_param_name = settings.future.success_single_parameter_name.mixed_case();
    let callback_param_name = settings
        .future
        .async_method_callback_parameter_name
        .mixed_case();

    let future_type = format!("java.util.concurrent.CompletableFuture<{}>", return_type);

    // Documentation
    documentation(f, |f| {
        // Print top-level documentation
        javadoc_print(f, &method.native_function.doc)?;
        f.newline()?;

        // Print each parameter value
        for param in method
            .native_function
            .arguments
            .iter()
            .skip(1)
            .filter(|param| !matches!(param.arg_type, FunctionArgument::Interface(_)))
        {
            f.writeln(&format!("@param {} ", param.name.mixed_case()))?;
            docstring_print(f, &param.doc)?;
        }

        // Print return value
        f.writeln("@return ")?;
        docstring_print(f, &method.future.value_type_doc)?;

        // Print exception
        if let Some(error) = &method.native_function.error_type.get() {
            f.writeln(&format!(
                "@throws {} {}",
                error.exception_name.camel_case(),
                error.inner.name.camel_case()
            ))?;
        }

        Ok(())
    })?;

    f.writeln(&format!(
        "public java.util.concurrent.CompletionStage<{}> {}(",
        return_type,
        method.name.mixed_case()
    ))?;
    f.write(
        &method
            .native_function
            .arguments
            .iter()
            .skip(1)
            .filter(|param| !matches!(param.arg_type, FunctionArgument::Interface(_)))
            .map(|param| {
                format!(
                    "{} {}",
                    param.arg_type.as_java_primitive(),
                    param.name.mixed_case()
                )
            })
            .collect::<Vec<String>>()
            .join(", "),
    )?;
    f.write(")")?;

    if let Some(error) = method.native_function.error_type.get() {
        if error.exception_type == ExceptionType::CheckedException {
            f.write(&format!(" throws {}", error.exception_name.camel_case()))?;
        }
    }

    blocked(f, |f| {
        f.writeln(&format!("{} future = new {}();", future_type, future_type))?;

        f.writeln(&format!(
            "{} {} = {} -> {{",
            one_time_callback_name, callback_param_name, one_time_callback_param_name
        ))?;
        indented(f, |f| {
            f.writeln(&format!(
                "future.complete({});",
                one_time_callback_param_name
            ))
        })?;
        f.writeln("};")?;

        call_native_function(f, &method.native_function, "return ", true)?;
        f.writeln("return future;")
    })
}
