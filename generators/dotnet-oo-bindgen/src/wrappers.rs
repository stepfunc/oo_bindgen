use oo_bindgen::formatting::{FormattingResult, Printer};
use oo_bindgen::function::{FunctionHandle, SignatureType};
use oo_bindgen::Library;

use crate::conversion::DotnetType;
use crate::formatting::*;
use crate::{print_imports, print_license, DotnetBindgenConfig, NATIVE_FUNCTIONS_CLASSNAME};

use heck::SnakeCase;

pub(crate) fn generate_native_functions_class(
    f: &mut dyn Printer,
    lib: &Library,
    config: &DotnetBindgenConfig,
) -> FormattingResult<()> {
    print_license(f, &lib.info.license_description)?;
    print_imports(f)?;
    f.newline()?;

    namespaced(f, &lib.name, |f| {
        f.writeln(&format!("internal class {}", NATIVE_FUNCTIONS_CLASSNAME))?;
        blocked(f, |f| {
            for func in lib.functions() {
                f.newline()?;
                write_conversion_wrapper(f, func, &lib.c_ffi_prefix)?;
            }
            Ok(())
        })?;

        f.newline()?;
        f.writeln("internal class ExceptionWrappers")?;
        blocked(f, |f| {
            for func in lib.functions().filter(|x| x.error_type.is_some()) {
                f.newline()?;
                write_exception_wrapper(f, func, &lib.c_ffi_prefix)?;
            }
            Ok(())
        })?;

        f.newline()?;

        f.writeln("internal class PInvoke")?;
        blocked(f, |f| {
            for func in lib.functions() {
                write_pinvoke_signature(f, func, &lib.c_ffi_prefix, config)?;
            }
            Ok(())
        })
    })
}

fn write_exception_and_return_block(
    f: &mut dyn Printer,
    func: &FunctionHandle,
    params: &str,
    prefix: &str,
) -> FormattingResult<()> {
    match func.get_signature_type() {
        SignatureType::NoErrorNoReturn => {
            unreachable!()
        }
        SignatureType::NoErrorWithReturn(_, _) => {
            unreachable!()
        }
        SignatureType::ErrorNoReturn(err) => {
            f.writeln(&format!(
                "var error = PInvoke.{}_{}({});",
                prefix, func.name, params
            ))?;
            f.writeln(&format!("if(error != {}.Ok)", err.inner.name))?;
            blocked(f, |f| {
                f.writeln(&format!("throw new {}(error);", err.exception_name))
            })
        }
        SignatureType::ErrorWithReturn(err, ret, _) => {
            f.writeln(&format!("{} _return_value;", ret.as_native_type()))?;
            f.writeln(&format!(
                "var _error_result = PInvoke.{}_{}({}, out _return_value);",
                prefix, func.name, params
            ))?;
            f.writeln(&format!("if(_error_result != {}.Ok)", err.inner.name))?;
            blocked(f, |f| {
                f.writeln(&format!("throw new {}(_error_result);", err.exception_name))
            })?;
            f.writeln("return _return_value;")
        }
    }
}

fn write_conversion_wrapper(
    f: &mut dyn Printer,
    func: &FunctionHandle,
    prefix: &str,
) -> FormattingResult<()> {
    f.write(&format!(
        "internal static {} {}(",
        func.return_type.as_native_type(),
        func.name
    ))?;

    f.write(
        &func
            .parameters
            .iter()
            .map(|param| {
                format!(
                    "{} {}",
                    param.arg_type.to_any_type().as_native_type(),
                    param.name
                )
            })
            .collect::<Vec<String>>()
            .join(", "),
    )?;

    f.write(")")?;

    let params = func
        .parameters
        .iter()
        .map(|p| p.name.clone())
        .collect::<Vec<String>>()
        .join(", ");

    let target = if func.error_type.is_some() {
        "ExceptionWrappers"
    } else {
        "PInvoke"
    };

    blocked(f, |f| {
        f.newline()?;
        if !func.return_type.is_void() {
            f.write("return ")?;
        }
        f.write(&format!("{}.{}_{}({});", target, prefix, func.name, params))
    })
}

fn write_exception_wrapper(
    f: &mut dyn Printer,
    func: &FunctionHandle,
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
            .parameters
            .iter()
            .map(|param| {
                format!(
                    "{} {}",
                    param.arg_type.to_any_type().as_native_type(),
                    param.name
                )
            })
            .collect::<Vec<String>>()
            .join(", "),
    )?;

    f.write(")")?;

    let params = func
        .parameters
        .iter()
        .map(|p| p.name.clone())
        .collect::<Vec<String>>()
        .join(", ");

    blocked(f, |f| {
        write_exception_and_return_block(f, func, &params, prefix)
    })
}

fn write_pinvoke_signature(
    f: &mut dyn Printer,
    handle: &FunctionHandle,
    prefix: &str,
    config: &DotnetBindgenConfig,
) -> FormattingResult<()> {
    f.writeln(&format!(
        "[DllImport(\"{}\", CallingConvention = CallingConvention.Cdecl)]",
        config.ffi_name
    ))?;
    f.newline()?;

    if let Some(err) = &handle.error_type {
        f.write(&format!(
            "internal static extern {} {}_{}(",
            err.to_enum_type().as_native_type(),
            prefix,
            handle.name,
        ))?;
    } else {
        f.write(&format!(
            "internal static extern {} {}_{}(",
            handle.return_type.as_native_type(),
            prefix,
            handle.name.to_snake_case(),
        ))?;
    }

    f.write(
        &handle
            .parameters
            .iter()
            .map(|param| {
                format!(
                    "{} {}",
                    param.arg_type.to_any_type().as_native_type(),
                    param.name
                )
            })
            .collect::<Vec<String>>()
            .join(", "),
    )?;

    if let SignatureType::ErrorWithReturn(_, ret, _) = handle.get_signature_type() {
        if !handle.parameters.is_empty() {
            f.write(", ")?;
        }
        f.write(&format!("out {} @out", ret.as_native_type()))?
    }

    f.write(");")
}
