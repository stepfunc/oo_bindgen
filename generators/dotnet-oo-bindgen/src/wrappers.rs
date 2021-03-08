use oo_bindgen::formatting::{FormattingResult, Printer};
use oo_bindgen::Library;

use crate::conversion::DotnetType;
use crate::formatting::*;
use crate::{print_imports, print_license, DotnetBindgenConfig, NATIVE_FUNCTIONS_CLASSNAME};
use oo_bindgen::native_function::{NativeFunctionHandle, NativeFunctionType};

pub(crate) fn generate_native_functions_class(
    f: &mut dyn Printer,
    lib: &Library,
    config: &DotnetBindgenConfig,
) -> FormattingResult<()> {
    print_license(f, &lib.license)?;
    print_imports(f)?;
    f.newline()?;

    namespaced(f, &lib.name, |f| {
        f.writeln(&format!("internal class {}", NATIVE_FUNCTIONS_CLASSNAME))?;
        blocked(f, |f| {
            for func in lib.native_functions() {
                f.newline()?;
                write_function_wrapper(f, func)?;
            }

            f.newline()?;
            write_inner_class(f, lib, config)
        })
    })
}

fn write_function_wrapper(
    f: &mut dyn Printer,
    func: &NativeFunctionHandle,
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
            .map(|param| format!("{} {}", param.param_type.as_native_type(), param.name))
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

    blocked(f, |f| match func.get_type() {
        NativeFunctionType::NoErrorNoReturn => {
            f.writeln(&format!("Inner.{}({});", func.name, params))
        }
        NativeFunctionType::NoErrorWithReturn(_, _) => {
            f.writeln(&format!("return Inner.{}({});", func.name, params))
        }
        NativeFunctionType::ErrorNoReturn(err) => {
            f.writeln(&format!("var error = Inner.{}({});", func.name, params))?;
            f.writeln(&format!("if(error != {}.Ok)", err.inner.name))?;
            blocked(f, |f| {
                f.writeln(&format!("throw new {}(error);", err.exception_name))
            })
        }
        NativeFunctionType::ErrorWithReturn(err, ret, _) => {
            f.writeln(&format!("{} _return_value;", ret.as_native_type()))?;
            f.writeln(&format!(
                "var _error_result = Inner.{}({}, out _return_value);",
                func.name, params
            ))?;
            f.writeln(&format!("if(_error_result != {}.Ok)", err.inner.name))?;
            blocked(f, |f| {
                f.writeln(&format!("throw new {}(_error_result);", err.exception_name))
            })?;
            f.writeln("return _return_value;")
        }
    })
}

fn write_inner_class(
    f: &mut dyn Printer,
    lib: &Library,
    config: &DotnetBindgenConfig,
) -> FormattingResult<()> {
    f.writeln("private class Inner")?;
    blocked(f, |f| {
        for handle in lib.native_functions() {
            f.writeln(&format!(
                "[DllImport(\"{}\", CallingConvention = CallingConvention.Cdecl)]",
                config.ffi_name
            ))?;
            f.newline()?;

            if let Some(err) = &handle.error_type {
                f.write(&format!(
                    "internal static extern {} {}(",
                    err.to_enum_type().as_native_type(),
                    handle.name
                ))?;
            } else {
                f.write(&format!(
                    "internal static extern {} {}(",
                    handle.return_type.as_native_type(),
                    handle.name
                ))?;
            }

            f.write(
                &handle
                    .parameters
                    .iter()
                    .map(|param| format!("{} {}", param.param_type.as_native_type(), param.name))
                    .collect::<Vec<String>>()
                    .join(", "),
            )?;

            if let NativeFunctionType::ErrorWithReturn(_, ret, _) = handle.get_type() {
                if !handle.parameters.is_empty() {
                    f.write(", ")?;
                }
                f.write(&format!("out {} @out", ret.as_native_type()))?
            }

            f.write(");")?;
            f.newline()?;
        }

        Ok(())
    })
}
