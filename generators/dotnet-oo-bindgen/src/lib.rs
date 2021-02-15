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
    safe_packed_borrows,
    while_true,
    bare_trait_objects
)]

use crate::conversion::*;
use crate::doc::*;
use crate::formatting::*;
use heck::CamelCase;
use oo_bindgen::formatting::*;
use oo_bindgen::native_enum::*;
use oo_bindgen::native_function::*;
use oo_bindgen::platforms::*;
use oo_bindgen::*;
use std::fs;
use std::path::PathBuf;

mod callback;
mod class;
mod conversion;
mod doc;
mod formatting;
mod interface;
mod structure;

const NATIVE_FUNCTIONS_CLASSNAME: &str = "NativeFunctions";

pub struct DotnetBindgenConfig {
    pub output_dir: PathBuf,
    pub ffi_name: String,
    pub platforms: PlatformLocations,
}

pub fn generate_dotnet_bindings(
    lib: &Library,
    config: &DotnetBindgenConfig,
) -> FormattingResult<()> {
    fs::create_dir_all(&config.output_dir)?;
    generate_csproj(lib, config)?;

    generate_native_func_class(lib, config)?;

    generate_structs(lib, config)?;
    generate_enums(lib, config)?;
    generate_classes(lib, config)?;
    generate_interfaces(lib, config)?;
    generate_one_time_callbacks(lib, config)?;

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
    f.writeln(&format!(
        "    <Version>{}</Version>",
        lib.version.to_string()
    ))?;
    f.writeln("  </PropertyGroup>")?;
    f.newline()?;
    f.writeln("  <ItemGroup>")?;

    for p in config
        .platforms
        .iter()
        .filter(|x| x.platform != Platform::LinuxMusl)
    {
        let filename = p.bin_filename(&config.ffi_name);
        let filepath = dunce::canonicalize(p.location.join(&filename))?;
        f.writeln(&format!("    <Content Include=\"{}\" Link=\"{}\" Pack=\"true\" PackagePath=\"runtimes/{}/native\" CopyToOutputDirectory=\"PreserveNewest\" />", filepath.to_string_lossy(), filename, p.platform.to_string()))?;
    }

    f.writeln("  </ItemGroup>")?;

    f.writeln("  <ItemGroup>")?;
    f.writeln(
        "    <PackageReference Include=\"System.Collections.Immutable\" Version=\"1.7.1\" />",
    )?;
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
    print_imports(&mut f)?;
    f.newline()?;

    namespaced(&mut f, &lib.name, |f| {
        f.writeln(&format!("internal class {}", NATIVE_FUNCTIONS_CLASSNAME))?;
        blocked(f, |f| {
            for handle in lib.native_functions() {
                f.writeln(&format!(
                    "[DllImport(\"{}\", CallingConvention = CallingConvention.Cdecl)]",
                    config.ffi_name
                ))?;
                f.newline()?;
                f.write(&format!(
                    "internal static extern {} {}(",
                    handle.return_type.as_native_type(),
                    handle.name
                ))?;

                f.write(
                    &handle
                        .parameters
                        .iter()
                        .map(|param| {
                            format!("{} {}", param.param_type.as_native_type(), param.name)
                        })
                        .collect::<Vec<String>>()
                        .join(", "),
                )?;
                f.write(");")?;
                f.newline()?;
            }

            Ok(())
        })
    })
}

fn generate_structs(lib: &Library, config: &DotnetBindgenConfig) -> FormattingResult<()> {
    for native_struct in lib.structs() {
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

fn generate_enum(
    f: &mut impl Printer,
    native_enum: &NativeEnumHandle,
    lib: &Library,
) -> FormattingResult<()> {
    print_license(f, &lib.license)?;
    print_imports(f)?;
    f.newline()?;

    namespaced(f, &lib.name, |f| {
        documentation(f, |f| {
            // Print top-level documentation
            xmldoc_print(f, &native_enum.doc, lib)
        })?;

        f.writeln(&format!("public enum {}", native_enum.name.to_camel_case()))?;
        blocked(f, |f| {
            for variant in &native_enum.variants {
                documentation(f, |f| xmldoc_print(f, &variant.doc, lib))?;
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

fn generate_one_time_callbacks(
    lib: &Library,
    config: &DotnetBindgenConfig,
) -> FormattingResult<()> {
    for cb in lib.one_time_callbacks() {
        // Open file
        let mut filename = config.output_dir.clone();
        filename.push(&cb.name);
        filename.set_extension("cs");
        let mut f = FilePrinter::new(filename)?;

        callback::generate(&mut f, cb, lib)?;
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
