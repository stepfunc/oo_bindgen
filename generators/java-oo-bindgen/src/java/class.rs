use super::doc::*;
use super::*;
use heck::{CamelCase, MixedCase};
use oo_bindgen::class::*;
use oo_bindgen::error_type::ExceptionType;

pub(crate) fn generate(
    f: &mut dyn Printer,
    class: &ClassHandle,
    lib: &Library,
) -> FormattingResult<()> {
    let classname = class.name().to_camel_case();

    // Documentation
    documentation(f, |f| javadoc_print(f, &class.doc, lib))?;

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
            generate_constructor(f, &classname, constructor, lib)?;
            f.newline()?;
        }

        if let Some(destructor) = &class.destructor {
            generate_destructor(f, destructor, &class.destruction_mode, lib)?;
            f.newline()?;
        }

        for method in &class.methods {
            generate_method(f, method, lib)?;
            f.newline()?;
        }

        for method in &class.async_methods {
            generate_async_method(f, method, lib)?;
            f.newline()?;
        }

        for method in &class.static_methods {
            generate_static_method(f, method, lib)?;
            f.newline()?;
        }

        Ok(())
    })
}

pub(crate) fn generate_static(
    f: &mut dyn Printer,
    class: &StaticClassHandle,
    lib: &Library,
) -> FormattingResult<()> {
    let classname = class.name.to_camel_case();

    // Documentation
    documentation(f, |f| javadoc_print(f, &class.doc, lib))?;

    // Class definition
    f.writeln(&format!("public final class {}", classname))?;

    blocked(f, |f| {
        // Private constructor to make it static
        f.writeln(&format!("private {}() {{ }}", classname))?;
        f.newline()?;

        for method in &class.static_methods {
            generate_static_method(f, method, lib)?;
            f.newline()?;
        }

        Ok(())
    })
}

fn generate_constructor(
    f: &mut dyn Printer,
    classname: &str,
    constructor: &NativeFunctionHandle,
    lib: &Library,
) -> FormattingResult<()> {
    documentation(f, |f| {
        // Print top-level documentation
        javadoc_print(f, &constructor.doc, lib)?;
        f.newline()?;

        // Print each parameter value
        for param in constructor.parameters.iter() {
            f.writeln(&format!("@param {} ", param.name.to_mixed_case()))?;
            docstring_print(f, &param.doc, lib)?;
        }

        // Print exception
        if let Some(error) = &constructor.error_type {
            f.writeln(&format!(
                "@throws {} {}",
                error.exception_name.to_camel_case(),
                error.inner.name.to_camel_case()
            ))?;
        }

        Ok(())
    })?;

    f.writeln(&format!("public {}(", classname))?;
    f.write(
        &constructor
            .parameters
            .iter()
            .map(|param| {
                format!(
                    "{} {}",
                    param.param_type.as_java_primitive(),
                    param.name.to_mixed_case()
                )
            })
            .collect::<Vec<String>>()
            .join(", "),
    )?;
    f.write(")")?;

    blocked(f, |f| {
        call_native_function(f, constructor, &format!("{} object = ", classname), false)?;
        f.writeln("this.self = object.self;")?;
        f.writeln("object.disposed.set(true);")
    })
}

fn generate_destructor(
    f: &mut dyn Printer,
    destructor: &NativeFunctionHandle,
    destruction_mode: &DestructionMode,
    lib: &Library,
) -> FormattingResult<()> {
    if destruction_mode.is_manual_destruction() {
        documentation(f, |f| {
            // Print top-level documentation
            javadoc_print(f, &destructor.doc, lib)?;
            f.newline()?;

            // Print each parameter value
            for param in destructor.parameters.iter().skip(1) {
                f.writeln(&format!("@param {} ", param.name.to_mixed_case()))?;
                docstring_print(f, &param.doc, lib)?;
            }

            Ok(())
        })?;
    }

    match destruction_mode {
        DestructionMode::Automatic => {
            f.writeln("private void close()")?;
        }
        DestructionMode::Custom(name) => {
            f.writeln(&format!("public void {}()", name.to_mixed_case()))?;
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
            NATIVE_FUNCTIONS_CLASSNAME, destructor.name
        ))
    })?;

    f.newline()?;

    // Finalizer method
    f.writeln("@Override")?;
    f.writeln("public void finalize()")?;
    blocked(f, |f| {
        if let DestructionMode::Custom(name) = destruction_mode {
            f.writeln(&format!("this.{}();", name.to_mixed_case()))
        } else {
            f.writeln("this.close();")
        }
    })
}

fn generate_method(f: &mut dyn Printer, method: &Method, lib: &Library) -> FormattingResult<()> {
    documentation(f, |f| {
        // Print top-level documentation
        javadoc_print(f, &method.native_function.doc, lib)?;
        f.newline()?;

        // Print each parameter value
        for param in method.native_function.parameters.iter().skip(1) {
            f.writeln(&format!("@param {} ", param.name.to_mixed_case()))?;
            docstring_print(f, &param.doc, lib)?;
        }

        // Print return value
        if let ReturnType::Type(_, doc) = &method.native_function.return_type {
            f.writeln("@return ")?;
            docstring_print(f, doc, lib)?;
        }

        // Print exception
        if let Some(error) = &method.native_function.error_type {
            f.writeln(&format!(
                "@throws {} {}",
                error.exception_name.to_camel_case(),
                error.inner.name.to_camel_case()
            ))?;
        }

        Ok(())
    })?;

    f.writeln(&format!(
        "public {} {}(",
        method.native_function.return_type.as_java_primitive(),
        method.name.to_mixed_case()
    ))?;
    f.write(
        &method
            .native_function
            .parameters
            .iter()
            .skip(1)
            .map(|param| {
                format!(
                    "{} {}",
                    param.param_type.as_java_primitive(),
                    param.name.to_mixed_case()
                )
            })
            .collect::<Vec<String>>()
            .join(", "),
    )?;
    f.write(")")?;

    if let Some(error) = &method.native_function.error_type {
        if error.exception_type == ExceptionType::CheckedException {
            f.write(&format!(" throws {}", error.exception_name.to_camel_case()))?;
        }
    }

    blocked(f, |f| {
        call_native_function(f, &method.native_function, "return ", true)
    })
}

fn generate_static_method(
    f: &mut dyn Printer,
    method: &Method,
    lib: &Library,
) -> FormattingResult<()> {
    documentation(f, |f| {
        // Print top-level documentation
        javadoc_print(f, &method.native_function.doc, lib)?;
        f.newline()?;

        // Print each parameter value
        for param in method.native_function.parameters.iter() {
            f.writeln(&format!("@param {} ", param.name.to_mixed_case()))?;
            docstring_print(f, &param.doc, lib)?;
        }

        // Print return value
        if let ReturnType::Type(_, doc) = &method.native_function.return_type {
            f.writeln("@return ")?;
            docstring_print(f, doc, lib)?;
        }

        // Print exception
        if let Some(error) = &method.native_function.error_type {
            f.writeln(&format!(
                "@throws {} {}",
                error.exception_name.to_camel_case(),
                error.inner.name.to_camel_case()
            ))?;
        }

        Ok(())
    })?;

    f.writeln(&format!(
        "public static {} {}(",
        method.native_function.return_type.as_java_primitive(),
        method.name.to_mixed_case()
    ))?;
    f.write(
        &method
            .native_function
            .parameters
            .iter()
            .map(|param| {
                format!(
                    "{} {}",
                    param.param_type.as_java_primitive(),
                    param.name.to_mixed_case()
                )
            })
            .collect::<Vec<String>>()
            .join(", "),
    )?;
    f.write(")")?;

    if let Some(error) = &method.native_function.error_type {
        if error.exception_type == ExceptionType::CheckedException {
            f.write(&format!(" throws {}", error.exception_name.to_camel_case()))?;
        }
    }

    blocked(f, |f| {
        call_native_function(f, &method.native_function, "return ", false)
    })
}

fn generate_async_method(
    f: &mut dyn Printer,
    method: &AsyncMethod,
    lib: &Library,
) -> FormattingResult<()> {
    let return_type = method.return_type.as_java_object();
    let one_time_callback_name = method.one_time_callback_name.to_camel_case();
    let one_time_callback_param_name = method.one_time_callback_param_name.to_mixed_case();
    let callback_param_name = method.callback_param_name.to_mixed_case();

    let future_type = format!("java.util.concurrent.CompletableFuture<{}>", return_type);

    // Documentation
    documentation(f, |f| {
        // Print top-level documentation
        javadoc_print(f, &method.native_function.doc, lib)?;
        f.newline()?;

        // Print each parameter value
        for param in method
            .native_function
            .parameters
            .iter()
            .skip(1)
            .filter(|param| !matches!(param.param_type, Type::Interface(_)))
        {
            f.writeln(&format!("@param {} ", param.name.to_mixed_case()))?;
            docstring_print(f, &param.doc, lib)?;
        }

        // Print return value
        f.writeln("@return ")?;
        docstring_print(f, &method.return_type_doc, lib)?;

        // Print exception
        if let Some(error) = &method.native_function.error_type {
            f.writeln(&format!(
                "@throws {} {}",
                error.exception_name.to_camel_case(),
                error.inner.name.to_camel_case()
            ))?;
        }

        Ok(())
    })?;

    f.writeln(&format!(
        "public java.util.concurrent.CompletionStage<{}> {}(",
        return_type,
        method.name.to_mixed_case()
    ))?;
    f.write(
        &method
            .native_function
            .parameters
            .iter()
            .skip(1)
            .filter(|param| !matches!(param.param_type, Type::Interface(_)))
            .map(|param| {
                format!(
                    "{} {}",
                    param.param_type.as_java_primitive(),
                    param.name.to_mixed_case()
                )
            })
            .collect::<Vec<String>>()
            .join(", "),
    )?;
    f.write(")")?;

    if let Some(error) = &method.native_function.error_type {
        if error.exception_type == ExceptionType::CheckedException {
            f.write(&format!(" throws {}", error.exception_name.to_camel_case()))?;
        }
    }

    blocked(f, |f| {
        f.writeln(&format!("{} future = new {}();", future_type, future_type))?;

        f.writeln(&format!(
            "{} {} = {} -> {{",
            one_time_callback_name, one_time_callback_param_name, callback_param_name
        ))?;
        indented(f, |f| {
            f.writeln(&format!("future.complete({});", callback_param_name))
        })?;
        f.writeln("};")?;

        call_native_function(f, &method.native_function, "return ", true)?;
        f.writeln("return future;")
    })
}
