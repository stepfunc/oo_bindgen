use std::fmt::Formatter;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use crate::backend::*;
use crate::model::*;

use conversion::*;
use doc::*;
use formatting::*;

mod class;
mod conversion;
mod doc;
mod formatting;
mod helpers;
mod interface;
mod structure;
mod wrappers;

pub(crate) const NATIVE_FUNCTIONS_CLASSNAME: &str = "NativeFunctions";

/// Map from Rust platform to a .NET platform string
///
/// Packages not in this map will cause an error
fn dotnet_platform_string(platform: &Platform) -> Option<&'static str> {
    // Names taken from https://docs.microsoft.com/en-us/dotnet/core/rid-catalog
    match *platform {
        // Windows targets
        platform::X86_64_PC_WINDOWS_MSVC => Some("win-x64"),
        platform::I686_PC_WINDOWS_MSVC => Some("win-x86"),
        // OSX targets
        platform::X86_64_APPLE_DARWIN => Some("osx-x64"),
        platform::AARCH64_APPLE_DARWIN => Some("osx-arm64"),
        // Linux GLIBC targets
        platform::X86_64_UNKNOWN_LINUX_GNU => Some("linux-x64"),
        platform::AARCH64_UNKNOWN_LINUX_GNU => Some("linux-arm64"),
        platform::ARM_UNKNOWN_LINUX_GNUEABIHF => Some("linux-arm"),
        // Linux MUSL targets
        platform::X86_64_UNKNOWN_LINUX_MUSL => Some("linux-musl-x64"),
        platform::AARCH64_UNKNOWN_LINUX_MUSL => Some("linux-musl-arm64"),
        platform::ARM_UNKNOWN_LINUX_MUSLEABIHF => Some("linux-musl-arm"),
        // other targets just use the target triple
        _ => None,
    }
}

/// Target framework - affects runtime compatible and allowed language features
///
/// Default C# versions for different targets specified here:
///
/// <https://learn.microsoft.com/en-us/dotnet/csharp/language-reference/configure-language-version>
///
#[derive(Debug, Copy, Clone, clap::ValueEnum)]
pub(crate) enum TargetFramework {
    /// .NET Standard 2.0 - Compatible with .NET Framework 4.6.1 -> 4.8
    /// Defaults to C# 7.3
    NetStandard2_0,
    /// .NET Standard 2.1 - NOT compatible with any .NET Framework
    /// Defaults to C# 8.0
    NetStandard2_1,
}

impl std::fmt::Display for TargetFramework {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TargetFramework {
    pub(crate) fn get_target_framework_str(&self) -> &'static str {
        match self {
            TargetFramework::NetStandard2_0 => "netstandard2.0",
            TargetFramework::NetStandard2_1 => "netstandard2.1",
        }
    }

    pub(crate) fn supports_default_interface_methods(&self) -> bool {
        match self {
            TargetFramework::NetStandard2_0 => false,
            TargetFramework::NetStandard2_1 => true,
        }
    }
}

pub(crate) struct DotnetBindgenConfig {
    pub(crate) output_dir: PathBuf,
    pub(crate) ffi_name: &'static str,
    pub(crate) extra_files: Vec<PathBuf>,
    pub(crate) platforms: PlatformLocations,
    pub(crate) generate_doxygen: bool,
    pub(crate) target_framework: TargetFramework,
}

pub(crate) fn generate_dotnet_bindings(
    lib: &Library,
    config: &DotnetBindgenConfig,
) -> FormattingResult<()> {
    logged::create_dir_all(&config.output_dir)?;

    generate_csproj(lib, config)?;
    generate_targets_scripts(lib, config)?;
    generate_native_functions(lib, config)?;
    generate_constants(lib, config)?;
    generate_structs(lib, config)?;
    generate_enums(lib, config)?;
    generate_exceptions(lib, config)?;
    generate_classes(lib, config)?;
    generate_interfaces(lib, config)?;
    generate_collection_helpers(lib, config)?;
    generate_iterator_helpers(lib, config)?;

    // generate the helper classes
    generate_helpers(lib, config)?;

    if config.generate_doxygen {
        generate_doxygen(lib, config)?;
    }

    Ok(())
}

fn generate_helpers(lib: &Library, config: &DotnetBindgenConfig) -> FormattingResult<()> {
    let mut filename = config.output_dir.clone();
    filename.push("Helpers");
    filename.set_extension("cs");
    let mut f = FilePrinter::new(filename)?;

    print_license(&mut f, &lib.info.license_description)?;
    f.writeln(include_str!("../../../static/dotnet/Helpers.cs"))
}

fn generate_csproj(lib: &Library, config: &DotnetBindgenConfig) -> FormattingResult<()> {
    // Open file
    let mut filename = config.output_dir.clone();
    filename.push(lib.settings.name.to_string());
    filename.set_extension("csproj");
    let mut f = FilePrinter::new(filename)?;

    f.writeln("<Project Sdk=\"Microsoft.NET.Sdk\">")?;
    f.writeln("  <PropertyGroup>")?;
    f.writeln(&format!(
        "    <TargetFramework>{}</TargetFramework>",
        config.target_framework.get_target_framework_str()
    ))?;
    f.writeln("    <GenerateDocumentationFile>true</GenerateDocumentationFile>")?;
    f.writeln("    <IncludeSymbols>true</IncludeSymbols>")?; // Include symbols
    f.writeln("    <SymbolPackageFormat>snupkg</SymbolPackageFormat>")?; // Use new file format
    f.writeln(&format!("    <PackageId>{}</PackageId>", lib.settings.name))?;
    f.writeln(&format!(
        "    <PackageVersion>{}</PackageVersion>",
        lib.version
    ))?;
    f.writeln(&format!(
        "    <Description>{}</Description>",
        lib.info.description
    ))?;
    f.writeln(&format!(
        "    <PackageProjectUrl>{}</PackageProjectUrl>",
        lib.info.project_url
    ))?;
    f.writeln(&format!(
        "    <RepositoryUrl>https://github.com/{}.git</RepositoryUrl>",
        lib.info.repository
    ))?;
    f.writeln("    <RepositoryType>git</RepositoryType>")?;
    f.writeln(&format!(
        "    <PackageLicenseFile>{}</PackageLicenseFile>",
        lib.info.license_path.file_name().unwrap().to_string_lossy()
    ))?;
    f.writeln("  </PropertyGroup>")?;
    f.newline()?;
    f.writeln("  <ItemGroup>")?;

    // Include each compiled FFI lib
    for p in config.platforms.iter() {
        let ps = dotnet_platform_string(&p.platform)
            .unwrap_or_else(|| panic!("No RID mapped for Rust target: {}", p.platform));
        let filename = p.platform.bin_filename(&config.ffi_name);
        let filepath = dunce::canonicalize(p.location.join(&filename))?;
        f.writeln(&format!("    <Content Include=\"{}\" Link=\"{}\" Pack=\"true\" PackagePath=\"runtimes/{}/native\" CopyToOutputDirectory=\"PreserveNewest\" />", filepath.to_string_lossy(), filename, ps))?;
    }

    // Include the target files to force the copying of DLLs of NuGet packages on .NET Framework
    // See https://github.com/stepfunc/dnp3/issues/147
    f.writeln(&format!("    <Content Include=\"build/net45/{}.targets\" Pack=\"true\" PackagePath=\"build/net45/\" />", lib.settings.name))?;
    f.writeln(&format!("    <Content Include=\"buildTransitive/net45/{}.targets\" Pack=\"true\" PackagePath=\"buildTransitive/net45/\" />", lib.settings.name))?;

    f.writeln("  </ItemGroup>")?;

    // Dependencies and files to include
    f.writeln("  <ItemGroup>")?;
    f.writeln(
        "    <PackageReference Include=\"System.Collections.Immutable\" Version=\"1.7.1\" />",
    )?;
    f.writeln(&format!(
        "    <None Include=\"{}\" Pack=\"true\" PackagePath=\"\" />",
        dunce::canonicalize(&lib.info.license_path)?.to_string_lossy()
    ))?;
    for path in &config.extra_files {
        f.writeln(&format!(
            "    <None Include=\"{}\" Pack=\"true\" PackagePath=\"\" />",
            dunce::canonicalize(path)?.to_string_lossy()
        ))?;
    }
    f.writeln("  </ItemGroup>")?;

    f.writeln("</Project>")
}

fn generate_targets_scripts(lib: &Library, config: &DotnetBindgenConfig) -> FormattingResult<()> {
    // The target file is used to automatically copy the DLL to the build directory when using
    // .NET Framework (Windows only). In .NET Core or .NET 5/6, the DLLs are automatically
    // loaded from the appropriate runtime/*/native directory.
    // This solution is based on gRPC library.
    // We only support x64 Platform and .NET Framework 4.5 or higher.
    // See https://github.com/stepfunc/dnp3/issues/147

    // Main target file
    {
        let mut filename = config.output_dir.clone();
        filename.push("build");
        filename.push("net45");

        fs::create_dir_all(&filename)?;

        filename.push(lib.settings.name.to_string());
        filename.set_extension("targets");
        let mut f = FilePrinter::new(filename)?;

        f.writeln("<?xml version=\"1.0\" encoding=\"utf-8\"?>")?;
        f.writeln("<Project ToolsVersion=\"4.0\" xmlns=\"http://schemas.microsoft.com/developer/msbuild/2003\">")?;
        f.writeln("  <ItemGroup>")?;

        for p in config.platforms.iter() {
            if let Some(ps) = dotnet_platform_string(&p.platform) {
                if p.platform.target_os == OS::Windows && p.platform.target_arch == Arch::X86_64 {
                    f.writeln(&format!("    <Content Condition=\"'$(Platform)' == 'x64'\" Include=\"$(MSBuildThisFileDirectory)../../runtimes/{}/native/{}\" Link=\"{}\" CopyToOutputDirectory=\"Always\" Visible=\"false\" NuGetPackageId=\"{}\" />", ps, p.platform.bin_filename(&config.ffi_name), p.platform.bin_filename(&config.ffi_name), lib.settings.name))?;
                } else if p.platform.target_os == OS::Windows && p.platform.target_arch == Arch::X86
                {
                    f.writeln(&format!("    <Content Condition=\"'$(Platform)' == 'x86'\" Include=\"$(MSBuildThisFileDirectory)../../runtimes/{}/native/{}\" Link=\"{}\" CopyToOutputDirectory=\"Always\" Visible=\"false\" NuGetPackageId=\"{}\" />", ps, p.platform.bin_filename(&config.ffi_name), p.platform.bin_filename(&config.ffi_name), lib.settings.name))?;
                }
            }
        }

        f.writeln("  </ItemGroup>")?;
        f.writeln("</Project>")?;
    }

    // Transistive target file (simply points to the main one)
    {
        let mut filename = config.output_dir.clone();
        filename.push("buildTransitive");
        filename.push("net45");

        fs::create_dir_all(&filename)?;

        filename.push(lib.settings.name.to_string());
        filename.set_extension("targets");
        let mut f = FilePrinter::new(filename)?;

        f.writeln("<?xml version=\"1.0\" encoding=\"utf-8\"?>")?;
        f.writeln("<Project ToolsVersion=\"4.0\" xmlns=\"http://schemas.microsoft.com/developer/msbuild/2003\">")?;
        f.writeln(&format!(
            "  <Import Project=\"$(MSBuildThisFileDirectory)../../build/net45/{}.targets\" />",
            lib.settings.name
        ))?;
        f.writeln("</Project>")?;
    }

    Ok(())
}

fn generate_native_functions(lib: &Library, config: &DotnetBindgenConfig) -> FormattingResult<()> {
    let mut filename = config.output_dir.clone();
    filename.push(NATIVE_FUNCTIONS_CLASSNAME);
    filename.set_extension("cs");
    let mut f = FilePrinter::new(filename)?;

    wrappers::generate_native_functions_class(&mut f, lib, config)
}

fn generate_constants(lib: &Library, config: &DotnetBindgenConfig) -> FormattingResult<()> {
    for constants in lib.constants() {
        // Open file
        let mut filename = config.output_dir.clone();
        filename.push(constants.name.to_string());
        filename.set_extension("cs");
        let mut f = FilePrinter::new(filename)?;

        generate_constant_set(&mut f, constants, lib)?;
    }

    Ok(())
}

fn generate_structs(lib: &Library, config: &DotnetBindgenConfig) -> FormattingResult<()> {
    for st in lib.structs() {
        // Open file
        let mut filename = config.output_dir.clone();
        filename.push(st.name().camel_case());
        filename.set_extension("cs");
        let mut f = FilePrinter::new(filename)?;

        structure::generate(&mut f, lib, st)?;
    }

    Ok(())
}

fn generate_enums(lib: &Library, config: &DotnetBindgenConfig) -> FormattingResult<()> {
    for native_enum in lib.enums() {
        // Open file
        let mut filename = config.output_dir.clone();
        filename.push(native_enum.name.camel_case());
        filename.set_extension("cs");
        let mut f = FilePrinter::new(filename)?;

        generate_enum(&mut f, native_enum, lib)?;
    }

    Ok(())
}

fn generate_exceptions(lib: &Library, config: &DotnetBindgenConfig) -> FormattingResult<()> {
    for err in lib.error_types() {
        // Open file
        let mut filename = config.output_dir.clone();
        filename.push(err.exception_name.camel_case());
        filename.set_extension("cs");
        let mut f = FilePrinter::new(filename)?;

        generate_exception(&mut f, err, lib)?;
    }

    Ok(())
}

fn generate_constant_set(
    f: &mut impl Printer,
    set: &Handle<ConstantSet<Validated>>,
    lib: &Library,
) -> FormattingResult<()> {
    fn get_type_as_string(value: &ConstantValue) -> &'static str {
        match value {
            ConstantValue::U8(_, _) => "byte",
        }
    }

    fn get_value_as_string(value: &ConstantValue) -> String {
        match value {
            ConstantValue::U8(x, Representation::Hex) => format!("0x{:02X?}", x),
        }
    }

    print_license(f, &lib.info.license_description)?;
    print_imports(f)?;
    f.newline()?;

    namespaced(f, &lib.settings.name, |f| {
        documentation(f, |f| {
            // Print top-level documentation
            xmldoc_print(f, &set.doc)
        })?;

        f.writeln(&format!("public static class {}", set.name.camel_case()))?;
        blocked(f, |f| {
            for value in &set.values {
                documentation(f, |f| xmldoc_print(f, &value.doc))?;
                f.writeln(&format!(
                    "public const {} {} = {};",
                    get_type_as_string(&value.value),
                    value.name.camel_case(),
                    get_value_as_string(&value.value),
                ))?;
            }
            Ok(())
        })
    })
}

fn generate_enum(
    f: &mut impl Printer,
    native_enum: &Handle<Enum<Validated>>,
    lib: &Library,
) -> FormattingResult<()> {
    print_license(f, &lib.info.license_description)?;
    print_imports(f)?;
    f.newline()?;

    namespaced(f, &lib.settings.name, |f| {
        documentation(f, |f| {
            // Print top-level documentation
            xmldoc_print(f, &native_enum.doc)
        })?;

        f.writeln(&format!("public enum {}", native_enum.name.camel_case()))?;
        blocked(f, |f| {
            for variant in &native_enum.variants {
                documentation(f, |f| xmldoc_print(f, &variant.doc))?;
                f.writeln(&format!(
                    "{} =  {},",
                    variant.name.camel_case(),
                    variant.value
                ))?;
            }
            Ok(())
        })
    })
}

fn generate_exception(
    f: &mut impl Printer,
    err: &ErrorType<Validated>,
    lib: &Library,
) -> FormattingResult<()> {
    print_license(f, &lib.info.license_description)?;
    print_imports(f)?;
    f.newline()?;

    namespaced(f, &lib.settings.name, |f| {
        documentation(f, |f| {
            // Print top-level documentation
            xmldoc_print(f, &err.inner.doc)
        })?;

        let error_name = err.inner.name.camel_case();
        let exception_name = err.exception_name.camel_case();

        f.writeln(&format!("public class {}: Exception", exception_name))?;
        blocked(f, |f| {
            documentation(f, |f| {
                f.writeln("<summary>")?;
                f.write("Error detail")?;
                f.write("</summary>")
            })?;
            f.writeln(&format!("public readonly {} error;", error_name))?;
            f.newline()?;
            f.writeln(&format!(
                "internal {}({} error) : base(error.ToString())",
                exception_name, error_name
            ))?;
            blocked(f, |f| f.writeln("this.error = error;"))
        })
    })
}

fn generate_classes(lib: &Library, config: &DotnetBindgenConfig) -> FormattingResult<()> {
    for class in lib.classes() {
        // Open file
        let mut filename = config.output_dir.clone();
        filename.push(class.name().camel_case());
        filename.set_extension("cs");
        let mut f = FilePrinter::new(filename)?;

        class::generate(&mut f, class, lib)?;
    }

    for class in lib.static_classes() {
        // Open file
        let mut filename = config.output_dir.clone();
        filename.push(class.name.camel_case());
        filename.set_extension("cs");
        let mut f = FilePrinter::new(filename)?;

        class::generate_static(&mut f, class, lib)?;
    }

    Ok(())
}

fn generate_interfaces(lib: &Library, config: &DotnetBindgenConfig) -> FormattingResult<()> {
    for interface in lib.interfaces() {
        // Open file
        let mut filename = config.output_dir.clone();
        filename.push(&format!("I{}", interface.name().camel_case()));
        filename.set_extension("cs");
        let mut f = FilePrinter::new(filename)?;

        interface::generate(&mut f, interface, lib, config.target_framework)?;
    }

    Ok(())
}

fn generate_iterator_helpers(lib: &Library, config: &DotnetBindgenConfig) -> FormattingResult<()> {
    for iter in lib.iterators() {
        // Open file
        let mut filename = config.output_dir.clone();
        filename.push(&format!("{}Helpers", iter.name().camel_case()));
        filename.set_extension("cs");
        let mut f = FilePrinter::new(filename)?;

        helpers::generate_iterator_helpers(&mut f, iter, lib)?;
    }

    Ok(())
}

fn generate_collection_helpers(
    lib: &Library,
    config: &DotnetBindgenConfig,
) -> FormattingResult<()> {
    for coll in lib.collections() {
        // Open file
        let mut filename = config.output_dir.clone();
        filename.push(&format!("{}Helpers", coll.name().camel_case()));
        filename.set_extension("cs");
        let mut f = FilePrinter::new(filename)?;

        helpers::generate_collection_helpers(&mut f, coll, lib)?;
    }

    Ok(())
}

fn print_license(f: &mut dyn Printer, license: &[String]) -> FormattingResult<()> {
    commented(f, |f| {
        for line in license.iter() {
            f.writeln(line)?;
        }
        Ok(())
    })
}

fn print_imports(f: &mut dyn Printer) -> FormattingResult<()> {
    f.writeln("using System;")?;
    f.writeln("using System.Runtime.InteropServices;")?;
    f.writeln("using System.Threading.Tasks;")?;
    f.writeln("using System.Collections.Immutable;")
}

fn generate_doxygen(lib: &Library, config: &DotnetBindgenConfig) -> FormattingResult<()> {
    // Copy doxygen awesome in target directory
    let doxygen_awesome = include_str!("../../../static/doxygen-awesome.css");
    fs::write(
        config.output_dir.join("doxygen-awesome.css"),
        doxygen_awesome,
    )?;

    // Write the logo file
    fs::write(config.output_dir.join("logo.png"), lib.info.logo_png)?;

    run_doxygen(
        &config.output_dir,
        &[
            &format!("PROJECT_NAME = {} (.NET API)", lib.settings.name),
            &format!("PROJECT_NUMBER = {}", lib.version),
            "INPUT = ./",
            "HTML_OUTPUT = doc",
            // Output customization
            "GENERATE_LATEX = NO",       // No LaTeX
            "HIDE_UNDOC_CLASSES = YES",  // I guess this will help the output
            "ALWAYS_DETAILED_SEC = YES", // Always print detailed section
            "AUTOLINK_SUPPORT = NO",     // Only link when we explicitly want to
            // Styling
            "HTML_EXTRA_STYLESHEET = doxygen-awesome.css",
            "GENERATE_TREEVIEW = YES",
            "PROJECT_LOGO = logo.png",
            "HTML_COLORSTYLE_HUE = 209", // See https://jothepro.github.io/doxygen-awesome-css/index.html#autotoc_md14
            "HTML_COLORSTYLE_SAT = 255",
            "HTML_COLORSTYLE_GAMMA = 113",
        ],
    )?;

    Ok(())
}

fn run_doxygen(cwd: &Path, config_lines: &[&str]) -> FormattingResult<()> {
    let mut command = Command::new("doxygen")
        .current_dir(cwd)
        .arg("-")
        .stdin(std::process::Stdio::piped())
        .spawn()?;

    {
        let stdin = command.stdin.as_mut().unwrap();

        for line in config_lines {
            stdin.write_all(&format!("{}\n", line).into_bytes())?;
        }
    }

    command.wait()?;

    Ok(())
}
