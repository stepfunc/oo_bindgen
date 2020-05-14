use oo_bindgen::*;
use oo_bindgen::formatting::*;
use oo_bindgen::native_enum::*;
use oo_bindgen::native_function::*;
use oo_bindgen::platforms::*;
use std::fs;
use std::path::PathBuf;
use crate::conversion::*;
use crate::formatting::*;

mod class;
mod conversion;
mod formatting;
mod interface;
mod structure;

const NATIVE_FUNCTIONS_CLASSNAME: &'static str = "NativeFunctions";

pub struct DotnetBindgenConfig {
    pub output_dir: PathBuf,
    pub ffi_name: String,
    pub platforms: PlatformLocations,
}

pub fn generate_dotnet_bindings(lib: &Library, config: &DotnetBindgenConfig) -> FormattingResult<()> {
    fs::create_dir_all(&config.output_dir)?;
    generate_csproj(lib, config)?;

    generate_native_func_class(lib, config)?;

    generate_structs(lib, config)?;
    generate_enums(lib, config)?;
    generate_classes(lib, config)?;
    generate_interfaces(lib, config)?;

    Ok(())
}

fn generate_csproj(lib: &Library, config: &DotnetBindgenConfig) -> FormattingResult<()> {
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

    for p in config.platforms.iter() {
        let filename = p.bin_filename(&config.ffi_name);
        let filepath = dunce::canonicalize(p.location.join(&filename))?;
        f.writeln(&format!("    <Content Include=\"{}\" Link=\"{}\" Pack=\"true\" PackagePath=\"runtimes/{}/native\" CopyToOutputDirectory=\"PreserveNewest\" />", filepath.to_string_lossy(), filename, p.platform.to_string()))?;
    }

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
                f.write(&format!("internal static extern {} {}(", DotnetReturnType(&handle.return_type).as_native_type(), handle.name))?;
                
                f.write(
                    &handle.parameters.iter()
                        .map(|param| format!("{} {}", DotnetType(&param.param_type).as_native_type(), param.name))
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

        structure::generate(&mut f, native_struct, lib)?;
    }

    Ok(())
}

fn generate_enums(lib: &Library, config: &DotnetBindgenConfig) -> FormattingResult<()> {
    for native_enum in lib.native_enums() {
        // Open file
        let mut filename = config.output_dir.clone();
        filename.push(&native_enum.name);
        filename.set_extension("cs");
        let mut f = FilePrinter::new(filename)?;
    
        generate_enum(&mut f, native_enum, lib)?;
    }

    Ok(())
}

fn generate_enum(f: &mut impl Printer, native_enum: &NativeEnumHandle, lib: &Library) -> FormattingResult<()> {
    print_license(f, &lib.license)?;

    f.writeln("using System;")?;
    f.writeln("using System.Runtime.InteropServices;")?;
    f.newline()?;

    namespaced(f, &lib.name, |f| {
        f.writeln(&format!("public enum {}", native_enum.name))?;
        blocked(f, |f| {
            for variant in &native_enum.variants {
                f.writeln(&format!("{} =  {},", variant.name, variant.value))?;
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

        class::generate(&mut f, class, lib)?;
    }

    Ok(())
}

fn generate_interfaces(lib: &Library, config: &DotnetBindgenConfig) -> FormattingResult<()> {
    for interface in lib.interfaces() {
        // Open file
        let mut filename = config.output_dir.clone();
        filename.push(&interface.name);
        filename.set_extension("cs");
        let mut f = FilePrinter::new(filename)?;

        interface::generate(&mut f, interface, lib)?;
    }

    Ok(())
}

fn print_license(f: &mut dyn Printer, license: &Vec<String>) -> FormattingResult<()> {
    commented(f, |f| {
        for line in license.iter() {
            f.writeln(line)?;
        }
        Ok(())
    })
}
