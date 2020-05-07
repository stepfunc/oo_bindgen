use oo_bindgen::*;
use oo_bindgen::class::*;
use oo_bindgen::formatting::*;
use oo_bindgen::native_function::*;
use oo_bindgen::native_struct::*;
use std::fmt::{Display};
use std::path::PathBuf;
use crate::formatting::*;

mod formatting;

const NATIVE_FUNCTIONS_CLASSNAME: &'static str = "NativeFunctions";

pub struct DotnetBindgenConfig {
    pub output_dir: PathBuf,
    pub ffi_name: String,
    pub compiled_ffi_dir: PathBuf,
}

pub fn generate_dotnet_bindings(lib: &Library, config: &DotnetBindgenConfig) -> FormattingResult<()> {
    generate_csproj(lib, config)?;

    generate_native_func_class(lib, config)?;

    generate_structs(lib, config)?;
    generate_classes(lib, config)?;

    Ok(())
}

fn generate_csproj(lib: &Library, config: &DotnetBindgenConfig) -> FormattingResult<()> {
    let binary_filename = &format!("{}.dll", config.ffi_name);
    let binary_path = config.compiled_ffi_dir.join(binary_filename);

    // Open file
    let mut filename = config.output_dir.clone();
    filename.push(lib.name.clone());
    filename.set_extension("csproj");
    let mut f = FilePrinter::new(filename)?;

    f.writeln("<Project Sdk=\"Microsoft.NET.Sdk\">")?;
    f.writeln("  <PropertyGroup>")?;
    f.writeln("    <TargetFramework>netstandard2.0</TargetFramework>")?;
    f.writeln("  </PropertyGroup>")?;
    f.newline()?;
    f.writeln("  <ItemGroup>")?;
    f.writeln(&format!("    <Content Include=\"{}\" Link=\"{}\" Pack=\"true\" PackagePath=\"runtimes/win-x64/native\" CopyToOutputDirectory=\"PreserveNewest\" />", binary_path.canonicalize()?.to_string_lossy(), binary_filename))?;
    f.writeln("  </ItemGroup>")?;
    f.writeln("</Project>")
}

fn generate_native_func_class(lib: &Library, config: &DotnetBindgenConfig) -> FormattingResult<()> {
    // Open file
    let mut filename = config.output_dir.clone();
    filename.push(NATIVE_FUNCTIONS_CLASSNAME);
    filename.set_extension("cs");
    let mut f = FilePrinter::new(filename)?;

    print_license(&mut f, &lib.license)?;

    f.writeln("using System;")?;
    f.writeln("using System.Runtime.InteropServices;")?;
    f.newline()?;

    namespaced(&mut f, &lib.name, |f| {
        f.writeln(&format!("internal class {}", NATIVE_FUNCTIONS_CLASSNAME))?;
        blocked(f, |f| {
            for handle in lib.native_functions() {
                f.writeln(&format!("[DllImport(\"{}\")]", config.ffi_name))?;
                f.newline()?;
                f.write(&format!("internal static extern {} {}(", DotnetReturnType(&handle.return_type), handle.name))?;
                
                f.write(
                    &handle.parameters.iter()
                        .map(|param| format!("{} {}", DotnetType(&param.param_type), param.name))
                        .collect::<Vec<String>>()
                        .join(", ")
                )?;
                f.write(");")?;
                f.newline()?;
            }
        
            Ok(())
        })
    })
}

fn generate_structs(lib: &Library, config: &DotnetBindgenConfig) -> FormattingResult<()> {
    for native_struct in lib.native_structs() {
        // Open file
        let mut filename = config.output_dir.clone();
        filename.push(native_struct.name());
        filename.set_extension("cs");
        let mut f = FilePrinter::new(filename)?;
    
        generate_struct(&mut f, native_struct, lib)?;
    }

    Ok(())
}

fn generate_struct(f: &mut impl Printer, native_struct: &NativeStructHandle, lib: &Library) -> FormattingResult<()> {
    print_license(f, &lib.license)?;

    f.writeln("using System;")?;
    f.writeln("using System.Runtime.InteropServices;")?;
    f.newline()?;

    namespaced(f, &lib.name, |f| {
        f.writeln("[StructLayout(LayoutKind.Sequential)]")?;
        f.writeln(&format!("public struct {}", native_struct.name()))?;
        blocked(f, |f| {
            for el in &native_struct.elements {
                f.writeln(&format!("public {} {};", DotnetType(&el.element_type), el.name))?;
            }
            Ok(())
        })
    })
}

fn generate_classes(lib: &Library, config: &DotnetBindgenConfig) -> FormattingResult<()> {
    for class in lib.classes() {
        // Open file
        let mut filename = config.output_dir.clone();
        filename.push(class.name());
        filename.set_extension("cs");
        let mut f = FilePrinter::new(filename)?;
    
        generate_class(&mut f, class, lib)?;
    }

    Ok(())
}

fn generate_class(f: &mut impl Printer, class: &ClassHandle, lib: &Library) -> FormattingResult<()> {
    print_license(f, &lib.license)?;

    f.writeln("using System;")?;
    f.writeln("using System.Runtime.InteropServices;")?;
    f.newline()?;

    namespaced(f, &lib.name, |f| {
        f.writeln(&format!("public class {}", class.name()))?;
        blocked(f, |f| {
            f.writeln("private IntPtr self;")?;
            f.newline()?;

            f.writeln(&format!("internal {}(IntPtr self)", class.name()))?;
            blocked(f, |f| {
                f.writeln("this.self = self;")
            })?;
            f.newline()?;

            if let Some(constructor) = &class.constructor {
                f.writeln(&format!("public {}(", class.name()))?;
                f.write(
                    &constructor.parameters.iter()
                        .map(|param| format!("{} {}", DotnetType(&param.param_type), param.name))
                        .collect::<Vec<String>>()
                        .join(", ")
                )?;
                f.write(")")?;

                blocked(f, |f| {
                    f.writeln(&format!("this.self = {}.{}(", NATIVE_FUNCTIONS_CLASSNAME, constructor.name))?;

                    f.write(
                        &constructor.parameters.iter()
                            .map(|param| format!("{}", param.name))
                            .collect::<Vec<String>>()
                            .join(", ")
                    )?;
                    f.write(");")
                })?;
            }

            Ok(())
        })
    })
}

fn print_license(f: &mut dyn Printer, license: &Vec<String>) -> FormattingResult<()> {
    commented(f, |f| {
        for line in license.iter() {
            f.writeln(line)?;
        }
        Ok(())
    })
}

struct DotnetReturnType<'a>(&'a ReturnType);

impl <'a> Display for DotnetReturnType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.0 {
            ReturnType::Void => write!(f, "void"),
            ReturnType::Type(return_type) => write!(f, "{}", DotnetType(&return_type)),
        }
    }
}

struct DotnetType<'a>(&'a Type);

impl<'a> Display for DotnetType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.0 {
            Type::Bool => write!(f, "bool"),
            Type::Uint8 => write!(f, "byte"),
            Type::Sint8 => write!(f, "sbyte"),
            Type::Uint16 => write!(f, "ushort"),
            Type::Sint16 => write!(f, "short"),
            Type::Uint32 => write!(f, "uint"),
            Type::Sint32 => write!(f, "int"),
            Type::Uint64 => write!(f, "ulong"),
            Type::Sint64 => write!(f, "long"),
            Type::Float => write!(f, "float"),
            Type::Double => write!(f, "double"),
            Type::String => unimplemented!(),
            Type::Struct(handle) => write!(f, "{}", handle.name()),
            Type::StructRef(handle) => write!(f, "{}", handle.name),
            Type::ClassRef(_) => write!(f, "IntPtr"),
        }
    }
}
