use crate::*;
use heck::{CamelCase, MixedCase};
use oo_bindgen::class::*;

pub(crate) fn generate(
    f: &mut dyn Printer,
    class: &ClassHandle,
    lib: &Library,
) -> FormattingResult<()> {
    let classname = class.name().to_camel_case();

    print_license(f, &lib.license)?;
    print_imports(f)?;
    f.newline()?;

    namespaced(f, &lib.name, |f| {
        documentation(f, |f| {
            // Print top-level documentation
            xmldoc_print(f, &class.doc, lib)
        })?;

        let static_specifier = if class.is_static() { "static " } else { "" };
        f.writeln(&format!("public {}class {}", static_specifier, classname))?;
        if class.destructor.is_some() {
            f.write(": IDisposable")?;
        }

        blocked(f, |f| {
            if !class.is_static() {
                f.writeln("internal IntPtr self;")?;
                if class.destructor.is_some() {
                    f.writeln("private bool disposed = false;")?;
                }
                f.newline()?;

                f.writeln(&format!("internal {}(IntPtr self)", classname))?;
                blocked(f, |f| f.writeln("this.self = self;"))?;
                f.newline()?;
            }

            if let Some(constructor) = &class.constructor {
                generate_constructor(f, &classname, constructor, lib)?;
                f.newline()?;
            }

            if let Some(destructor) = &class.destructor {
                generate_destructor(f, &classname, destructor, lib)?;
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
        xmldoc_print(f, &constructor.doc, lib)?;
        f.newline()?;

        // Print each parameter value
        for param in &constructor.parameters {
            f.writeln(&format!("<param name=\"{}\">", param.name.to_mixed_case()))?;
            docstring_print(f, &param.doc, lib)?;
            f.write("</param>")?;
        }

        // Print return value
        if let ReturnType::Type(_, doc) = &constructor.return_type {
            f.writeln("<returns>")?;
            docstring_print(f, doc, lib)?;
            f.write("</returns>")?;
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
                    param.param_type.as_dotnet_type(),
                    param.name.to_mixed_case()
                )
            })
            .collect::<Vec<String>>()
            .join(", "),
    )?;
    f.write(")")?;

    blocked(f, |f| {
        call_native_function(f, &constructor, "this.self = ", None, true)
    })
}

fn generate_destructor(
    f: &mut dyn Printer,
    classname: &str,
    destructor: &NativeFunctionHandle,
    lib: &Library,
) -> FormattingResult<()> {
    // Public Dispose method
    documentation(f, |f| xmldoc_print(f, &destructor.doc, lib))?;

    f.writeln("public void Dispose()")?;
    blocked(f, |f| {
        f.writeln("Dispose(true);")?;
        f.writeln("GC.SuppressFinalize(this);")
    })?;

    f.newline()?;

    // Finalizer
    f.writeln(&format!("~{}()", classname))?;
    blocked(f, |f| f.writeln("Dispose(false);"))?;

    f.newline()?;

    // The IDisposable implementation
    f.writeln("protected virtual void Dispose(bool disposing)")?;
    blocked(f, |f| {
        f.writeln("if (this.disposed)")?;
        f.writeln("    return;")?;
        f.newline()?;
        f.writeln(&format!(
            "{}.{}(this.self);",
            NATIVE_FUNCTIONS_CLASSNAME, destructor.name
        ))?;
        f.newline()?;
        f.writeln("this.disposed = true;")
    })
}

fn generate_method(f: &mut dyn Printer, method: &Method, lib: &Library) -> FormattingResult<()> {
    documentation(f, |f| {
        // Print top-level documentation
        xmldoc_print(f, &method.native_function.doc, lib)?;
        f.newline()?;

        // Print each parameter value
        for param in method.native_function.parameters.iter().skip(1) {
            f.writeln(&format!("<param name=\"{}\">", param.name.to_mixed_case()))?;
            docstring_print(f, &param.doc, lib)?;
            f.write("</param>")?;
        }

        // Print return value
        if let ReturnType::Type(_, doc) = &method.native_function.return_type {
            f.writeln("<returns>")?;
            docstring_print(f, doc, lib)?;
            f.write("</returns>")?;
        }
        Ok(())
    })?;

    f.writeln(&format!(
        "public {} {}(",
        method.native_function.return_type.as_dotnet_type(),
        method.name.to_camel_case()
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
                    param.param_type.as_dotnet_type(),
                    param.name.to_mixed_case()
                )
            })
            .collect::<Vec<String>>()
            .join(", "),
    )?;
    f.write(")")?;

    blocked(f, |f| {
        call_native_function(
            f,
            &method.native_function,
            "return ",
            Some("this".to_string()),
            false,
        )
    })
}

fn generate_static_method(
    f: &mut dyn Printer,
    method: &Method,
    lib: &Library,
) -> FormattingResult<()> {
    documentation(f, |f| {
        // Print top-level documentation
        xmldoc_print(f, &method.native_function.doc, lib)?;
        f.newline()?;

        // Print each parameter value
        for param in &method.native_function.parameters {
            f.writeln(&format!("<param name=\"{}\">", param.name.to_mixed_case()))?;
            docstring_print(f, &param.doc, lib)?;
            f.write("</param>")?;
        }

        // Print return value
        if let ReturnType::Type(_, doc) = &method.native_function.return_type {
            f.writeln("<returns>")?;
            docstring_print(f, doc, lib)?;
            f.write("</returns>")?;
        }
        Ok(())
    })?;

    f.writeln(&format!(
        "public static {} {}(",
        method.native_function.return_type.as_dotnet_type(),
        method.name.to_camel_case()
    ))?;
    f.write(
        &method
            .native_function
            .parameters
            .iter()
            .map(|param| {
                format!(
                    "{} {}",
                    param.param_type.as_dotnet_type(),
                    param.name.to_mixed_case()
                )
            })
            .collect::<Vec<String>>()
            .join(", "),
    )?;
    f.write(")")?;

    blocked(f, |f| {
        call_native_function(f, &method.native_function, "return ", None, false)
    })
}

fn generate_async_method(
    f: &mut dyn Printer,
    method: &AsyncMethod,
    lib: &Library,
) -> FormattingResult<()> {
    let method_name = method.name.to_camel_case();
    let async_handler_name = format!("{}Handler", method_name);
    let return_type = method.return_type.as_dotnet_type();
    let one_time_callback_name = format!("I{}", method.one_time_callback_name.to_camel_case());
    let one_time_callback_param_name = method.one_time_callback_param_name.to_mixed_case();
    let callback_name = method.callback_name.to_camel_case();
    let callback_param_name = method.callback_param_name.to_mixed_case();

    // Write the task completion handler
    f.writeln(&format!(
        "internal class {} : {}",
        async_handler_name, one_time_callback_name
    ))?;
    blocked(f, |f| {
        f.writeln(&format!(
            "internal TaskCompletionSource<{}> tcs = new TaskCompletionSource<{}>();",
            return_type, return_type
        ))?;

        f.newline()?;

        f.writeln(&format!(
            "public void {}({} {})",
            callback_name, return_type, callback_param_name
        ))?;
        blocked(f, |f| {
            f.writeln(&format!("tcs.SetResult({});", callback_param_name))
        })
    })?;

    f.newline()?;

    // Documentation
    documentation(f, |f| {
        // Print top-level documentation
        xmldoc_print(f, &method.native_function.doc, lib)?;
        f.newline()?;

        // Print each parameter value
        for param in method
            .native_function
            .parameters
            .iter()
            .skip(1)
            .filter(|param| !matches!(param.param_type, Type::OneTimeCallback(_)))
        {
            f.writeln(&format!("<param name=\"{}\">", param.name.to_mixed_case()))?;
            docstring_print(f, &param.doc, lib)?;
            f.write("</param>")?;
        }

        // Print return value
        f.writeln("<returns>")?;
        docstring_print(f, &method.return_type_doc, lib)?;
        f.write("</returns>")?;
        Ok(())
    })?;

    f.writeln(&format!(
        "public Task<{}> {}(",
        return_type,
        method.name.to_camel_case()
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
                    param.param_type.as_dotnet_type(),
                    param.name.to_mixed_case()
                )
            })
            .collect::<Vec<String>>()
            .join(", "),
    )?;
    f.write(")")?;

    blocked(f, |f| {
        f.writeln(&format!(
            "var {} = new {}();",
            one_time_callback_param_name, async_handler_name
        ))?;
        call_native_function(
            f,
            &method.native_function,
            "return ",
            Some("this".to_string()),
            false,
        )?;
        f.writeln(&format!(
            "return {}.tcs.Task;",
            one_time_callback_param_name
        ))
    })
}
