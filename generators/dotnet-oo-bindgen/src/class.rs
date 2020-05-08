use oo_bindgen::*;
use oo_bindgen::class::*;
use oo_bindgen::formatting::*;
use crate::*;

pub fn generate_class(f: &mut dyn Printer, class: &ClassHandle, lib: &Library) -> FormattingResult<()> {
    print_license(f, &lib.license)?;

    f.writeln("using System;")?;
    f.writeln("using System.Runtime.InteropServices;")?;
    f.newline()?;

    namespaced(f, &lib.name, |f| {
        f.writeln(&format!("public class {}", class.name()))?;
        if class.destructor.is_some() {
            f.write(": IDisposable")?;
        }
        f.newline()?;

        blocked(f, |f| {
            f.writeln("private IntPtr self;")?;
            if class.destructor.is_some() {
                f.writeln("private bool disposed = false;")?;
            }
            f.newline()?;

            f.writeln(&format!("internal {}(IntPtr self)", class.name()))?;
            blocked(f, |f| {
                f.writeln("this.self = self;")
            })?;
            f.newline()?;

            if let Some(constructor) = &class.constructor {
                generate_constructor(f, class.name(), constructor)?;
                f.newline()?;
            }

            if let Some(destructor) = &class.destructor {
                generate_destructor(f, class.name(), destructor)?;
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
            .map(|param| format!("{} {}", DotnetType(&param.param_type).dotnet_parameter(), param.name))
            .collect::<Vec<String>>()
            .join(", ")
    )?;
    f.write(")")?;

    blocked(f, |f| {
        f.writeln(&format!("this.self = {}.{}(", NATIVE_FUNCTIONS_CLASSNAME, constructor.name))?;

        f.write(
            &constructor.parameters.iter()
                .map(|param| DotnetParameter(param).arg())
                .collect::<Vec<String>>()
                .join(", ")
        )?;
        f.write(");")
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

struct DotnetParameter<'a>(&'a Parameter);

impl<'a> DotnetParameter<'a> {
    fn arg(&self) -> String {
        match &self.0.param_type {
            Type::Bool => self.0.name.to_string(),
            Type::Uint8 => self.0.name.to_string(),
            Type::Sint8 => self.0.name.to_string(),
            Type::Uint16 => self.0.name.to_string(),
            Type::Sint16 => self.0.name.to_string(),
            Type::Uint32 => self.0.name.to_string(),
            Type::Sint32 => self.0.name.to_string(),
            Type::Uint64 => self.0.name.to_string(),
            Type::Sint64 => self.0.name.to_string(),
            Type::Float => unimplemented!(),
            Type::Double => unimplemented!(),
            Type::String => unimplemented!(),
            Type::Struct(_) => self.0.name.to_string(),
            Type::StructRef(_) => format!("ref {}", self.0.name.to_string()),
            Type::Enum(_) => self.0.name.to_string(),
            Type::ClassRef(_) => format!("{}.self", self.0.name.to_string()),
        }
    }
}
