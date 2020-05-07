use oo_bindgen::*;
use oo_bindgen::class::*;
use oo_bindgen::formatting::*;
use oo_bindgen::native_function::*;
use std::fmt::{Display};
use std::path::PathBuf;
use crate::formatting::*;

mod formatting;

pub struct DotnetBindgenConfig {
    pub output_dir: PathBuf,
    pub ffi_name: String,
    pub compiled_ffi_dir: PathBuf,
}

pub fn generate_dotnet_bindings(lib: &Library, config: &DotnetBindgenConfig) -> FormattingResult<()> {
    generate_csproj(lib, config)?;

    generate_native_func_class(lib, config)?;

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
    filename.push("NativeFunctions");
    filename.set_extension("cs");
    let mut f = FilePrinter::new(filename)?;

    print_license(&mut f, &lib.license)?;

    f.writeln("using System;")?;
    f.writeln("using System.Runtime.InteropServices;")?;
    f.newline()?;

    namespaced(&mut f, &lib.name, |f| {
        class(f, "NativeFunctions", |f| {
            for handle in lib.native_functions() {
                f.writeln(&format!("[DllImport(\"{}\")]", config.ffi_name))?;
                f.newline()?;
                f.write(&format!("static extern {} {}(", DotnetReturnType(&handle.return_type), handle.name))?;
                
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
        formatting::class(f, class.name(), |f| {
            f.writeln("private IntPtr self;")?;
            f.newline()?;

            f.writeln(&format!("internal {}(IntPtr self)", class.name()))?;
            f.writeln("{")?;
            f.writeln("    this.self = self;")?;
            f.writeln("}")?;
            f.newline()?;

            if let Some(constructor) = &class.constructor {
                f.writeln(&format!("public {}()", class.name()))?;
                f.writeln("{")?;
                f.writeln(&format!("    {}(NativeFunction.{}());", class.name(), constructor.name))?;
                f.writeln("}")?;
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
            Type::Uint8 => write!(f, "uint8_t"),
            Type::Sint8 => write!(f, "int8_t"),
            Type::Uint16 => write!(f, "uint16_t"),
            Type::Sint16 => write!(f, "int16_t"),
            Type::Uint32 => write!(f, "uint32_t"),
            Type::Sint32 => write!(f, "int32_t"),
            Type::Uint64 => write!(f, "uint64_t"),
            Type::Sint64 => write!(f, "int64_t"),
            Type::Float => write!(f, "float"),
            Type::Double => write!(f, "double"),
            Type::String => unimplemented!(),
            Type::Struct(handle) => write!(f, "{}", handle.name()),
            Type::StructRef(_) => write!(f, "IntPtr"),
            Type::ClassRef(_) => write!(f, "IntPtr"),
        }
    }
}
