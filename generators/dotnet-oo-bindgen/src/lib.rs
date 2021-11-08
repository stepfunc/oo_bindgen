#![deny(
// dead_code,
arithmetic_overflow,
invalid_type_param_default,
//missing_fragment_specifier,
mutable_transmutes,
no_mangle_const_items,
overflowing_literals,
patterns_in_fns_without_body,
pub_use_of_private_extern_crate,
unknown_crate_types,
const_err,
order_dependent_trait_objects,
illegal_floating_point_literal_pattern,
improper_ctypes,
late_bound_lifetime_arguments,
non_camel_case_types,
non_shorthand_field_patterns,
non_snake_case,
non_upper_case_globals,
no_mangle_generic_items,
private_in_public,
stable_features,
type_alias_bounds,
tyvar_behind_raw_pointer,
unconditional_recursion,
unused_comparisons,
unreachable_pub,
anonymous_parameters,
missing_copy_implementations,
// missing_debug_implementations,
// missing_docs,
trivial_casts,
trivial_numeric_casts,
unused_import_braces,
unused_qualifications,
clippy::all
)]
#![forbid(
    unsafe_code,
    //intra_doc_link_resolution_failure, broken_intra_doc_links
    unaligned_references,
    while_true,
    bare_trait_objects
)]

use crate::doc::*;
use crate::dotnet_type::*;
use crate::formatting::*;
use heck::CamelCase;
use oo_bindgen::constants::*;
use oo_bindgen::doc::Validated;
use oo_bindgen::enum_type::*;
use oo_bindgen::error_type::ErrorType;
use oo_bindgen::formatting::*;
use oo_bindgen::function::*;
use oo_bindgen::platforms::*;
use oo_bindgen::structs::StructType;
use oo_bindgen::*;
use std::fs;
use std::path::PathBuf;

mod class;
mod doc;
mod dotnet_type;
mod formatting;
mod helpers;
mod interface;
mod structure;
mod wrappers;

pub const NATIVE_FUNCTIONS_CLASSNAME: &str = "NativeFunctions";

const SUPPORTED_PLATFORMS: &[Platform] = &[
    Platform::WinX64Msvc,
    Platform::LinuxX64Gnu,
    Platform::LinuxArm8Gnu,
];

pub struct DotnetBindgenConfig {
    pub output_dir: PathBuf,
    pub ffi_name: String,
    pub extra_files: Vec<PathBuf>,
    pub platforms: PlatformLocations,
}

pub fn generate_dotnet_bindings(
    lib: &Library,
    config: &DotnetBindgenConfig,
) -> FormattingResult<()> {
    fs::create_dir_all(&config.output_dir)?;

    generate_csproj(lib, config)?;
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

    Ok(())
}

fn generate_helpers(lib: &Library, config: &DotnetBindgenConfig) -> FormattingResult<()> {
    let mut filename = config.output_dir.clone();
    filename.push("Helpers");
    filename.set_extension("cs");
    let mut f = FilePrinter::new(filename)?;

    print_license(&mut f, &lib.info.license_description)?;
    f.writeln(include_str!("../copy/Helpers.cs"))
}

fn generate_csproj(lib: &Library, config: &DotnetBindgenConfig) -> FormattingResult<()> {
    // Open file
    let mut filename = config.output_dir.clone();
    filename.push(lib.settings.name.to_string());
    filename.set_extension("csproj");
    let mut f = FilePrinter::new(filename)?;

    f.writeln("<Project Sdk=\"Microsoft.NET.Sdk\">")?;
    f.writeln("  <PropertyGroup>")?;
    f.writeln("    <TargetFramework>netstandard2.0</TargetFramework>")?;
    f.writeln("    <GenerateDocumentationFile>true</GenerateDocumentationFile>")?;
    f.writeln("    <IncludeSymbols>true</IncludeSymbols>")?; // Include symbols
    f.writeln("    <SymbolPackageFormat>snupkg</SymbolPackageFormat>")?; // Use new file format
    f.writeln(&format!(
        "    <PackageId>{}</PackageId>",
        lib.settings.name.to_string()
    ))?;
    f.writeln(&format!(
        "    <PackageVersion>{}</PackageVersion>",
        lib.version.to_string()
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
    for p in config
        .platforms
        .iter()
        .filter(|x| SUPPORTED_PLATFORMS.iter().any(|y| *y == x.platform))
    {
        let filename = p.bin_filename(&config.ffi_name);
        let filepath = dunce::canonicalize(p.location.join(&filename))?;
        f.writeln(&format!("    <Content Include=\"{}\" Link=\"{}\" Pack=\"true\" PackagePath=\"runtimes/{}/native\" CopyToOutputDirectory=\"PreserveNewest\" />", filepath.to_string_lossy(), filename, dotnet_platform_string(p.platform)))?;
    }

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
        filename.push(st.name().to_camel_case());
        filename.set_extension("cs");
        let mut f = FilePrinter::new(filename)?;

        match st {
            StructType::FunctionArg(x) => structure::generate(&mut f, x, lib)?,
            StructType::FunctionReturn(x) => structure::generate(&mut f, x, lib)?,
            StructType::CallbackArg(x) => structure::generate(&mut f, x, lib)?,
            StructType::Universal(x) => structure::generate(&mut f, x, lib)?,
        }
    }

    Ok(())
}

fn generate_enums(lib: &Library, config: &DotnetBindgenConfig) -> FormattingResult<()> {
    for native_enum in lib.enums() {
        // Open file
        let mut filename = config.output_dir.clone();
        filename.push(native_enum.name.to_camel_case());
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
        filename.push(err.exception_name.to_string());
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

        f.writeln(&format!("public static class {}", set.name.to_camel_case()))?;
        blocked(f, |f| {
            for value in &set.values {
                documentation(f, |f| xmldoc_print(f, &value.doc))?;
                f.writeln(&format!(
                    "public const {} {} = {};",
                    get_type_as_string(&value.value),
                    value.name.to_camel_case(),
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

        f.writeln(&format!("public enum {}", native_enum.name.to_camel_case()))?;
        blocked(f, |f| {
            for variant in &native_enum.variants {
                documentation(f, |f| xmldoc_print(f, &variant.doc))?;
                f.writeln(&format!(
                    "{} =  {},",
                    variant.name.to_camel_case(),
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

        let error_name = err.inner.name.to_camel_case();
        let exception_name = err.exception_name.to_camel_case();

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
        filename.push(class.name().to_camel_case());
        filename.set_extension("cs");
        let mut f = FilePrinter::new(filename)?;

        class::generate(&mut f, class, lib)?;
    }

    for class in lib.static_classes() {
        // Open file
        let mut filename = config.output_dir.clone();
        filename.push(class.name.to_camel_case());
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
        filename.push(&format!("I{}", interface.name.to_camel_case()));
        filename.set_extension("cs");
        let mut f = FilePrinter::new(filename)?;

        interface::generate(&mut f, interface, lib)?;
    }

    Ok(())
}

fn generate_iterator_helpers(lib: &Library, config: &DotnetBindgenConfig) -> FormattingResult<()> {
    for iter in lib.iterators() {
        // Open file
        let mut filename = config.output_dir.clone();
        filename.push(&format!("{}Helpers", iter.name().to_camel_case()));
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
        filename.push(&format!("{}Helpers", coll.name().to_camel_case()));
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

fn dotnet_platform_string(platform: Platform) -> &'static str {
    // Names taken from https://docs.microsoft.com/en-us/dotnet/core/rid-catalog
    match platform {
        Platform::WinX64Msvc => "win-x64",
        Platform::LinuxX64Gnu => "linux-x64",
        Platform::LinuxArm8Gnu => "linux-arm64",
        _ => panic!("Unsupported platform"),
    }
}
