use oo_bindgen::backend::*;
use oo_bindgen::model::*;

use crate::conversion::DotnetType;
use crate::formatting::*;
use crate::{print_imports, print_license, DotnetBindgenConfig, NATIVE_FUNCTIONS_CLASSNAME};

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

    namespaced(f, &lib.settings.name, |f| {
        f.writeln(&format!("internal class {}", NATIVE_FUNCTIONS_CLASSNAME))?;
        blocked(f, |f| {
            for func in lib.functions() {
                f.newline()?;
                write_conversion_wrapper(f, func, &lib.settings.c_ffi_prefix)?;
            }
            Ok(())
        })?;

        f.newline()?;
        f.writeln("internal class ExceptionWrappers")?;
        blocked(f, |f| {
            for (func, err) in lib.functions().filter_map(filter_has_error) {
                f.newline()?;
                write_exception_wrapper(f, &func, &err, &lib.settings.c_ffi_prefix)?;
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
    prefix: &str,
) -> FormattingResult<()> {
    match func.return_type.get() {
        Some(ret) => {
            f.writeln(&format!("{} _return_value;", ret.value.as_native_type()))?;
            f.writeln(&format!(
                "var _error_result = PInvoke.{}_{}({}, out _return_value);",
                prefix, func.name, params
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
                "var error = PInvoke.{}_{}({});",
                prefix, func.name, params
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
    prefix: &str,
) -> FormattingResult<()> {
    f.write(&format!(
        "internal static {} {}(",
        func.return_type.as_native_type(),
        func.name
    ))?;

    f.write(
        &func
            .arguments
            .iter()
            .map(|param| format!("{} {}", param.arg_type.as_native_type(), param.name))
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
        f.write(&format!("{}.{}_{}({});", target, prefix, func.name, params))
    })
}

fn write_exception_wrapper(
    f: &mut dyn Printer,
    func: &Handle<Function<Validated>>,
    err: &ErrorType<Validated>,
    prefix: &str,
) -> FormattingResult<()> {
    f.write(&format!(
        "internal static {} {}_{}(",
        func.return_type.as_native_type(),
        prefix,
        func.name,
    ))?;

    f.write(
        &func
            .arguments
            .iter()
            .map(|param| format!("{} {}", param.arg_type.as_native_type(), param.name))
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
        write_exception_and_return_blocks(f, err, func, &params, prefix)
    })
}

fn write_pinvoke_signature(
    f: &mut dyn Printer,
    handle: &Handle<Function<Validated>>,
    prefix: &str,
    config: &DotnetBindgenConfig,
) -> FormattingResult<()> {
    f.writeln(&format!(
        "[DllImport(\"{}\", CallingConvention = CallingConvention.Cdecl)]",
        config.ffi_name
    ))?;
    f.newline()?;

    if let Some(err) = handle.error_type.get() {
        f.write(&format!(
            "internal static extern {} {}_{}(",
            err.inner.as_native_type(),
            prefix,
            handle.name,
        ))?;
    } else {
        f.write(&format!(
            "internal static extern {} {}_{}(",
            handle.return_type.as_native_type(),
            prefix,
            handle.name
        ))?;
    }

    f.write(
        &handle
            .arguments
            .iter()
            .map(|param| format!("{} {}", param.arg_type.as_native_type(), param.name))
            .collect::<Vec<String>>()
            .join(", "),
    )?;

    if let SignatureType::ErrorWithReturn(_, ret, _) = handle.get_signature_type() {
        if !handle.arguments.is_empty() {
            f.write(", ")?;
        }
        f.write(&format!("out {} @out", ret.as_native_type()))?
    }

    f.write(");")
}
