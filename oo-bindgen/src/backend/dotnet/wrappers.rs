use crate::backend::dotnet::*;

fn filter_has_error(
    x: &Handle<Function<Validated>>,
) -> Option<(Handle<Function<Validated>>, ErrorType<Validated>)> {
    x.error_type.get().map(|err| (x.clone(), err.clone()))
}

pub(crate) fn generate_native_functions_class(
    f: &mut dyn Printer,
    lib: &Library,
    config: &DotnetBindgenConfig,
) -> FormattingResult<()> {
    print_license(f, &lib.info.license_description)?;
    print_imports(f)?;
    f.newline()?;

    doxygen(f, |f| {
        // Doxygen main page
        f.writeln("@mainpage")?;
        f.newline()?;
        f.writeln(&lib.info.description)?;
        f.newline()?;
        f.writeln(&format!(
            "For complete documentation, see @ref {} namespace",
            lib.settings.name
        ))?;
        f.newline()?;
        f.writeln("@section license License")?;
        f.newline()?;
        for line in &lib.info.license_description {
            f.writeln(line)?;
        }

        Ok(())
    })?;
    f.newline()?;

    namespaced(f, &lib.settings.name, |f| {
        f.writeln(&format!("internal class {}", NATIVE_FUNCTIONS_CLASSNAME))?;
        blocked(f, |f| {
            f.writeln(&format!("const string VERSION = \"{}\";", lib.version))?;

            // Static constructor used to verify the version
            f.writeln(&format!("static {}()", NATIVE_FUNCTIONS_CLASSNAME))?;
            blocked(f, |f| {
                f.writeln("var loadedVersion = Helpers.RustString.FromNative(Version());")?;
                f.writeln("if (loadedVersion != VERSION)")?;
                blocked(f, |f| {
                    f.writeln(&format!("throw new Exception(\"{} module version mismatch. Expected \" + VERSION + \" but loaded \" + loadedVersion);", lib.settings.name))
                })
            })?;

            f.newline()?;

            for func in lib.functions() {
                f.newline()?;
                write_conversion_wrapper(f, func)?;
            }

            Ok(())
        })?;

        f.newline()?;
        f.writeln("internal class ExceptionWrappers")?;
        blocked(f, |f| {
            for (func, err) in lib.functions().filter_map(filter_has_error) {
                f.newline()?;
                write_exception_wrapper(f, &func, &err)?;
            }
            Ok(())
        })?;

        f.newline()?;

        f.writeln("internal class PInvoke")?;
        blocked(f, |f| {
            for func in lib.functions() {
                write_pinvoke_signature(f, func, &lib.settings.c_ffi_prefix, config)?;
            }
            Ok(())
        })
    })
}

fn write_exception_and_return_blocks(
    f: &mut dyn Printer,
    err: &ErrorType<Validated>,
    func: &Handle<Function<Validated>>,
    params: &str,
) -> FormattingResult<()> {
    match func.return_type.get() {
        Some(ret) => {
            f.writeln(&format!(
                "var _error_result = PInvoke.{}({}, out {} _return_value);",
                func.name.camel_case(),
                params,
                ret.value.get_native_type()
            ))?;
            f.writeln(&format!(
                "if(_error_result != {}.Ok)",
                err.inner.name.camel_case()
            ))?;
            blocked(f, |f| {
                f.writeln(&format!(
                    "throw new {}(_error_result);",
                    err.exception_name.camel_case()
                ))
            })?;
            f.writeln("return _return_value;")
        }
        None => {
            f.writeln(&format!(
                "var error = PInvoke.{}({});",
                func.name.camel_case(),
                params
            ))?;
            f.writeln(&format!("if(error != {}.Ok)", err.inner.name.camel_case()))?;
            blocked(f, |f| {
                f.writeln(&format!(
                    "throw new {}(error);",
                    err.exception_name.camel_case()
                ))
            })
        }
    }
}

fn write_conversion_wrapper(
    f: &mut dyn Printer,
    func: &Handle<Function<Validated>>,
) -> FormattingResult<()> {
    f.write(&format!(
        "internal static {} {}(",
        func.return_type.get_native_type(),
        func.name.camel_case()
    ))?;

    f.write(
        &func
            .arguments
            .iter()
            .map(|param| format!("{} {}", param.arg_type.get_native_type(), param.name))
            .collect::<Vec<String>>()
            .join(", "),
    )?;

    f.write(")")?;

    let params = func
        .arguments
        .iter()
        .map(|p| p.name.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    let target = if func.error_type.is_some() {
        "ExceptionWrappers"
    } else {
        "PInvoke"
    };

    blocked(f, |f| {
        f.newline()?;
        if func.return_type.is_some() {
            f.write("return ")?;
        }
        f.write(&format!(
            "{}.{}({});",
            target,
            func.name.camel_case(),
            params
        ))
    })
}

fn write_exception_wrapper(
    f: &mut dyn Printer,
    func: &Handle<Function<Validated>>,
    err: &ErrorType<Validated>,
) -> FormattingResult<()> {
    f.write(&format!(
        "internal static {} {}(",
        func.return_type.get_native_type(),
        func.name.camel_case(),
    ))?;

    f.write(
        &func
            .arguments
            .iter()
            .map(|param| format!("{} {}", param.arg_type.get_native_type(), param.name))
            .collect::<Vec<String>>()
            .join(", "),
    )?;

    f.write(")")?;

    let params = func
        .arguments
        .iter()
        .map(|p| p.name.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    blocked(f, |f| {
        write_exception_and_return_blocks(f, err, func, &params)
    })
}

fn write_pinvoke_signature(
    f: &mut dyn Printer,
    handle: &Handle<Function<Validated>>,
    prefix: &str,
    config: &DotnetBindgenConfig,
) -> FormattingResult<()> {
    f.writeln(&format!(
        "[DllImport(\"{}\", CallingConvention = CallingConvention.Cdecl, EntryPoint = \"{}_{}\")]",
        config.ffi_name, prefix, handle.name
    ))?;
    f.newline()?;

    if let Some(err) = handle.error_type.get() {
        f.write(&format!(
            "internal static extern {} {}(",
            err.inner.get_native_type(),
            handle.name.camel_case(),
        ))?;
    } else {
        f.write(&format!(
            "internal static extern {} {}(",
            handle.return_type.get_native_type(),
            handle.name.camel_case()
        ))?;
    }

    f.write(
        &handle
            .arguments
            .iter()
            .map(|param| format!("{} {}", param.arg_type.get_native_type(), param.name))
            .collect::<Vec<String>>()
            .join(", "),
    )?;

    if let SignatureType::ErrorWithReturn(_, ret, _) = handle.get_signature_type() {
        if !handle.arguments.is_empty() {
            f.write(", ")?;
        }
        f.write(&format!("out {} @out", ret.get_native_type()))?
    }

    f.write(");")
}
