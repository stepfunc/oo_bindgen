use super::doc::*;
use super::*;
use heck::{CamelCase, MixedCase};
use oo_bindgen::class::*;

pub(crate) fn generate(
    f: &mut dyn Printer,
    class: &ClassHandle,
    lib: &Library,
) -> FormattingResult<()> {
    let classname = class.name().to_camel_case();

    // Documentation
    documentation(f, |f| javadoc_print(f, &class.doc, lib))?;

    // Class definition
    f.writeln(&format!("public class {}", classname))?;
    if class.destructor.is_some() {
        f.write(" implements AutoCloseable")?;
    }

    blocked(f, |f| {
        if !class.is_static() {
            f.writeln("private long self;")?;
            if class.destructor.is_some() {
                f.writeln("private boolean disposed = false;")?;
            }
            f.newline()?;
        }

        if let Some(constructor) = &class.constructor {
            generate_constructor(f, &classname, constructor, lib)?;
            f.newline()?;
        }

        if let Some(destructor) = &class.destructor {
            generate_destructor(f, destructor, lib)?;
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
            f.writeln(&format!("@param {} ", param.name))?;
            docstring_print(f, &param.doc, lib)?;
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
        call_native_function(f, &constructor, &format!("{} object = ", classname), false)?;
        f.writeln("this.self = object.self;")
    })
}

fn generate_destructor(
    f: &mut dyn Printer,
    destructor: &NativeFunctionHandle,
    lib: &Library,
) -> FormattingResult<()> {
    documentation(f, |f| {
        // Print top-level documentation
        javadoc_print(f, &destructor.doc, lib)?;
        f.newline()?;

        // Print each parameter value
        for param in destructor.parameters.iter().skip(1) {
            f.writeln(&format!("@param {} ", param.name))?;
            docstring_print(f, &param.doc, lib)?;
        }

        Ok(())
    })?;

    // AutoCloseable implementation
    f.writeln("@Override")?;
    f.writeln("public void close()")?;
    blocked(f, |f| {
        f.writeln("if (this.disposed)")?;
        f.writeln("    return;")?;
        f.newline()?;
        f.writeln(&format!(
            "{}.{}(this);",
            NATIVE_FUNCTIONS_CLASSNAME, destructor.name
        ))?;
        f.newline()?;
        f.writeln("this.disposed = true;")
    })
}

fn generate_method(f: &mut dyn Printer, method: &Method, lib: &Library) -> FormattingResult<()> {
    documentation(f, |f| {
        // Print top-level documentation
        javadoc_print(f, &method.native_function.doc, lib)?;
        f.newline()?;

        // Print each parameter value
        for param in method.native_function.parameters.iter().skip(1) {
            f.writeln(&format!("@param {} ", param.name))?;
            docstring_print(f, &param.doc, lib)?;
        }

        // Print return value
        if let ReturnType::Type(_, doc) = &method.native_function.return_type {
            f.writeln("@return ")?;
            docstring_print(f, doc, lib)?;
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
            f.writeln(&format!("@param {} ", param.name))?;
            docstring_print(f, &param.doc, lib)?;
        }

        // Print return value
        if let ReturnType::Type(_, doc) = &method.native_function.return_type {
            f.writeln("@return ")?;
            docstring_print(f, doc, lib)?;
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
            .filter(|param| !matches!(param.param_type, Type::OneTimeCallback(_)))
        {
            f.writeln(&format!("@param {} ", param.name))?;
            docstring_print(f, &param.doc, lib)?;
        }

        // Print return value
        f.writeln("@return ")?;
        docstring_print(f, &method.return_type_doc, lib)
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
            .filter(|param| !matches!(param.param_type, Type::OneTimeCallback(_)))
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
