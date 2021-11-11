use crate::helpers::call_native_function;
use crate::*;
use oo_bindgen::class::*;

pub(crate) fn generate(
    f: &mut dyn Printer,
    class: &Handle<Class<Validated>>,
    lib: &Library,
) -> FormattingResult<()> {
    let classname = class.name().camel_case();

    print_license(f, &lib.info.license_description)?;
    print_imports(f)?;
    f.newline()?;

    namespaced(f, &lib.settings.name, |f| {
        documentation(f, |f| {
            // Print top-level documentation
            xmldoc_print(f, &class.doc)
        })?;

        f.writeln(&format!("public sealed class {}", classname))?;
        if matches!(class.destruction_mode, DestructionMode::Dispose) {
            f.write(": IDisposable")?;
        }

        blocked(f, |f| {
            f.writeln("internal readonly IntPtr self;")?;
            if class.destructor.is_some() {
                f.writeln("private bool disposed = false;")?;
            }
            f.newline()?;

            f.writeln(&format!("internal {}(IntPtr self)", classname))?;
            blocked(f, |f| f.writeln("this.self = self;"))?;
            f.newline()?;

            f.writeln(&format!(
                "internal static {} FromNative(IntPtr self)",
                classname
            ))?;
            blocked(f, |f| {
                f.writeln(&format!("{} result = null;", classname))?;
                f.writeln("if (self != IntPtr.Zero)")?;
                blocked(f, |f| {
                    f.writeln(&format!("result = new {}(self);", classname))
                })?;
                f.writeln("return result;")
            })?;
            f.newline()?;

            if let Some(constructor) = &class.constructor {
                generate_constructor(f, &classname, constructor)?;
                f.newline()?;
            }

            if let Some(destructor) = &class.destructor {
                generate_destructor(f, &classname, destructor, &class.destruction_mode)?;
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
    })
}

pub(crate) fn generate_static(
    f: &mut dyn Printer,
    class: &Handle<StaticClass<Validated>>,
    lib: &Library,
) -> FormattingResult<()> {
    let classname = class.name.camel_case();

    print_license(f, &lib.info.license_description)?;
    print_imports(f)?;
    f.newline()?;

    namespaced(f, &lib.settings.name, |f| {
        documentation(f, |f| {
            // Print top-level documentation
            xmldoc_print(f, &class.doc)
        })?;

        f.writeln(&format!("public static class {}", classname))?;

        blocked(f, |f| {
            for method in &class.static_methods {
                generate_static_method(f, method)?;
                f.newline()?;
            }

            Ok(())
        })
    })
}

fn generate_constructor(
    f: &mut dyn Printer,
    classname: &str,
    constructor: &ClassConstructor<Validated>,
) -> FormattingResult<()> {
    documentation(f, |f| {
        // Print top-level documentation
        xmldoc_print(f, &constructor.function.doc)?;
        f.newline()?;

        // Print each parameter value
        for param in &constructor.function.parameters {
            f.writeln(&format!("<param name=\"{}\">", param.name.mixed_case()))?;
            docstring_print(f, &param.doc)?;
            f.write("</param>")?;
        }

        // Print return value
        if let FunctionReturnType::Type(_, doc) = &constructor.function.return_type {
            f.writeln("<returns>")?;
            docstring_print(f, doc)?;
            f.write("</returns>")?;
        }

        // Print exception
        if let Some(error) = &constructor.function.error_type {
            f.writeln(&format!(
                "<exception cref=\"{}\" />",
                error.exception_name.camel_case()
            ))?;
        }

        Ok(())
    })?;

    f.writeln(&format!("public {}(", classname))?;
    f.write(
        &constructor
            .function
            .parameters
            .iter()
            .map(|param| {
                format!(
                    "{} {}",
                    param.arg_type.as_dotnet_type(),
                    param.name.mixed_case()
                )
            })
            .collect::<Vec<String>>()
            .join(", "),
    )?;
    f.write(")")?;

    blocked(f, |f| {
        call_native_function(f, &constructor.function, "this.self = ", None, true)
    })
}

fn generate_destructor(
    f: &mut dyn Printer,
    classname: &str,
    destructor: &ClassDestructor<Validated>,
    destruction_mode: &DestructionMode,
) -> FormattingResult<()> {
    if destruction_mode.is_manual_destruction() {
        // Public Dispose method
        documentation(f, |f| xmldoc_print(f, &destructor.function.doc))?;

        let method_name = if let DestructionMode::Custom(name) = destruction_mode {
            name.camel_case()
        } else {
            "Dispose".to_string()
        };

        f.writeln(&format!("public void {}()", method_name))?;
        blocked(f, |f| {
            f.writeln("Dispose(true);")?;
            f.writeln("GC.SuppressFinalize(this);")
        })?;

        f.newline()?;
    }

    // Finalizer
    documentation(f, |f| {
        f.writeln("<summary>")?;
        f.write("Finalizer")?;
        f.write("</summary>")
    })?;
    f.writeln(&format!("~{}()", classname))?;
    blocked(f, |f| f.writeln("Dispose(false);"))?;

    f.newline()?;

    // The IDisposable implementation
    f.writeln("private void Dispose(bool disposing)")?;
    blocked(f, |f| {
        f.writeln("if (this.disposed)")?;
        f.writeln("    return;")?;
        f.newline()?;
        f.writeln(&format!(
            "{}.{}(this.self);",
            NATIVE_FUNCTIONS_CLASSNAME, destructor.function.name
        ))?;
        f.newline()?;
        f.writeln("this.disposed = true;")
    })
}

fn generate_method(f: &mut dyn Printer, method: &Method<Validated>) -> FormattingResult<()> {
    documentation(f, |f| {
        // Print top-level documentation
        xmldoc_print(f, &method.native_function.doc)?;
        f.newline()?;

        // Print each parameter value
        for param in method.native_function.parameters.iter().skip(1) {
            f.writeln(&format!("<param name=\"{}\">", param.name.mixed_case()))?;
            docstring_print(f, &param.doc)?;
            f.write("</param>")?;
        }

        // Print return value
        if let FunctionReturnType::Type(_, doc) = &method.native_function.return_type {
            f.writeln("<returns>")?;
            docstring_print(f, doc)?;
            f.write("</returns>")?;
        }

        // Print exception
        if let Some(error) = &method.native_function.error_type {
            f.writeln(&format!(
                "<exception cref=\"{}\" />",
                error.exception_name.camel_case()
            ))?;
        }

        Ok(())
    })?;

    f.writeln(&format!(
        "public {} {}(",
        method.native_function.return_type.as_dotnet_type(),
        method.name.camel_case()
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
                    param.arg_type.as_dotnet_type(),
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
            &method.native_function,
            "return ",
            Some("this".to_string()),
            false,
        )
    })
}

fn generate_static_method(
    f: &mut dyn Printer,
    method: &StaticMethod<Validated>,
) -> FormattingResult<()> {
    documentation(f, |f| {
        // Print top-level documentation
        xmldoc_print(f, &method.native_function.doc)?;
        f.newline()?;

        // Print each parameter value
        for param in &method.native_function.parameters {
            f.writeln(&format!("<param name=\"{}\">", param.name.mixed_case()))?;
            docstring_print(f, &param.doc)?;
            f.write("</param>")?;
        }

        // Print return value
        if let FunctionReturnType::Type(_, doc) = &method.native_function.return_type {
            f.writeln("<returns>")?;
            docstring_print(f, doc)?;
            f.write("</returns>")?;
        }

        // Print exception
        if let Some(error) = &method.native_function.error_type {
            f.writeln(&format!(
                "<exception cref=\"{}\" />",
                error.exception_name.camel_case()
            ))?;
        }

        Ok(())
    })?;

    f.writeln(&format!(
        "public static {} {}(",
        method.native_function.return_type.as_dotnet_type(),
        method.name.camel_case()
    ))?;
    f.write(
        &method
            .native_function
            .parameters
            .iter()
            .map(|param| {
                format!(
                    "{} {}",
                    param.arg_type.as_dotnet_type(),
                    param.name.mixed_case()
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
    method: &FutureMethod<Validated>,
) -> FormattingResult<()> {
    let callback_success_type = method.future.value_type.as_dotnet_type();

    // Documentation
    documentation(f, |f| {
        // Print top-level documentation
        xmldoc_print(f, &method.native_function.doc)?;
        f.newline()?;

        // Print each parameter value
        for param in method
            .native_function
            .parameters
            .iter()
            .skip(1)
            .filter(|param| !matches!(param.arg_type, FunctionArgument::Interface(_)))
        {
            f.writeln(&format!("<param name=\"{}\">", param.name.mixed_case()))?;
            docstring_print(f, &param.doc)?;
            f.write("</param>")?;
        }

        // Print return value
        f.writeln("<returns>")?;
        docstring_print(f, &method.future.value_type_doc)?;
        f.write("</returns>")?;

        // Print exception
        if let Some(error) = &method.native_function.error_type {
            f.writeln(&format!(
                "<exception cref=\"{}\" />",
                error.exception_name.camel_case()
            ))?;
        }

        Ok(())
    })?;

    f.writeln(&format!(
        "public Task<{}> {}(",
        callback_success_type,
        method.name.camel_case()
    ))?;
    f.write(
        &method
            .native_function
            .parameters
            .iter()
            .skip(1)
            .filter(|param| !matches!(param.arg_type, FunctionArgument::Interface(_)))
            .map(|param| {
                format!(
                    "{} {}",
                    param.arg_type.as_dotnet_type(),
                    param.name.mixed_case()
                )
            })
            .collect::<Vec<String>>()
            .join(", "),
    )?;
    f.write(")")?;

    let tcs_var_name = "_oo_bindgen_tcs";
    let result_name = "_oo_bindgen_result";

    blocked(f, |f| {
        f.writeln(&format!(
            "var {} = new TaskCompletionSource<{}>();",
            tcs_var_name, callback_success_type
        ))?;
        f.writeln(&format!(
            "Action<{}> callback = ({}) => Task.Run(() => {}.SetResult({}));",
            callback_success_type, result_name, tcs_var_name, result_name
        ))?;
        call_native_function(
            f,
            &method.native_function,
            "return ",
            Some("this".to_string()),
            false,
        )?;
        f.writeln(&format!("return {}.Task;", tcs_var_name))
    })
}
