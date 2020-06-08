use oo_bindgen::*;
use oo_bindgen::class::*;
use oo_bindgen::formatting::*;
use heck::{CamelCase, MixedCase};
use crate::*;

pub(crate) fn generate(f: &mut dyn Printer, class: &ClassHandle, lib: &Library) -> FormattingResult<()> {
    let classname = class.name().to_camel_case();

    print_license(f, &lib.license)?;

    f.writeln("using System;")?;
    f.writeln("using System.Runtime.InteropServices;")?;
    f.newline()?;

    namespaced(f, &lib.name, |f| {
        let static_specifier = if class.is_static() { "static " } else { "" };
        f.writeln(&format!("public {}class {}", static_specifier, classname))?;
        if class.destructor.is_some() {
            f.write(": IDisposable")?;
        }

        blocked(f, |f| {
            if !class.is_static() {
                f.writeln("private IntPtr self;")?;
                if class.destructor.is_some() {
                    f.writeln("private bool disposed = false;")?;
                }
                f.newline()?;

                f.writeln(&format!("internal {}(IntPtr self)", classname))?;
                blocked(f, |f| {
                    f.writeln("this.self = self;")
                })?;
                f.newline()?;
            }

            if let Some(constructor) = &class.constructor {
                generate_constructor(f, &classname, constructor)?;
                f.newline()?;
            }

            if let Some(destructor) = &class.destructor {
                generate_destructor(f, &classname, destructor)?;
                f.newline()?;
            }

            for method in &class.methods {
                generate_method(f, method)?;
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

fn generate_constructor(f: &mut dyn Printer, classname: &str, constructor: &NativeFunctionHandle) -> FormattingResult<()> {
    f.writeln(&format!("public {}(", classname))?;
    f.write(
        &constructor.parameters.iter()
            .map(|param| format!("{} {}", DotnetType(&param.param_type).as_dotnet_type(), param.name.to_mixed_case()))
            .collect::<Vec<String>>()
            .join(", ")
    )?;
    f.write(")")?;

    blocked(f, |f| {
        call_native_function(f, &constructor, "this.self = ", None, true)
    })
}

fn generate_destructor(f: &mut dyn Printer, classname: &str, destructor: &NativeFunctionHandle) -> FormattingResult<()> {
    // Public Dispose method
    f.writeln("public void Dispose()")?;
    blocked(f, |f| {
        f.writeln("Dispose(true);")?;
        f.writeln("GC.SuppressFinalize(this);")
    })?;

    f.newline()?;

    // Finalizer
    f.writeln(&format!("~{}()", classname))?;
    blocked(f, |f| {
        f.writeln("Dispose(false);")
    })?;

    f.newline()?;

    // The IDisposable implementation
    f.writeln("protected virtual void Dispose(bool disposing)")?;
    blocked(f, |f| {
        f.writeln("if (this.disposed)")?;
        f.writeln("    return;")?;
        f.newline()?;
        f.writeln(&format!("{}.{}(this.self);", NATIVE_FUNCTIONS_CLASSNAME, destructor.name))?;
        f.newline()?;
        f.writeln("this.disposed = true;")
    })
}

fn generate_method(f: &mut dyn Printer, method: &Method) -> FormattingResult<()> {
    f.writeln(&format!("public {} {}(", DotnetReturnType(&method.native_function.return_type).as_dotnet_type(), method.name.to_camel_case()))?;
    f.write(
        &method.native_function.parameters.iter().skip(1)
            .map(|param| format!("{} {}", DotnetType(&param.param_type).as_dotnet_type(), param.name.to_mixed_case()))
            .collect::<Vec<String>>()
            .join(", ")
    )?;
    f.write(")")?;

    blocked(f, |f| {
        call_native_function(f, &method.native_function, "return ", Some("this".to_string()), false)
    })
}

fn generate_static_method(f: &mut dyn Printer, method: &Method) -> FormattingResult<()> {
    f.writeln(&format!("public static {} {}(", DotnetReturnType(&method.native_function.return_type).as_dotnet_type(), method.name.to_camel_case()))?;
    f.write(
        &method.native_function.parameters.iter()
            .map(|param| format!("{} {}", DotnetType(&param.param_type).as_dotnet_type(), param.name.to_mixed_case()))
            .collect::<Vec<String>>()
            .join(", ")
    )?;
    f.write(")")?;

    blocked(f, |f| {
        call_native_function(f, &method.native_function, "return ", None, false)
    })
}
