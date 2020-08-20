#![deny(
// dead_code,
arithmetic_overflow,
invalid_type_param_default,
missing_fragment_specifier,
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
    intra_doc_link_resolution_failure,
    safe_packed_borrows,
    while_true,
    bare_trait_objects
)]

use crate::conversion::*;
use crate::formatting::*;
use heck::{CamelCase, KebabCase};
use oo_bindgen::formatting::*;
use oo_bindgen::platforms::*;
use oo_bindgen::*;
use std::fs;
use std::path::PathBuf;

mod callback;
mod class;
mod conversion;
mod enumeration;
mod formatting;
mod interface;
mod structure;

const NATIVE_FUNCTIONS_CLASSNAME: &str = "NativeFunctions";

pub struct JavaBindgenConfig {
    pub output_dir: PathBuf,
    pub ffi_name: String,
    pub group_id: String,
    pub platforms: PlatformLocations,
}

impl JavaBindgenConfig {
    fn source_dir(&self, lib: &Library) -> PathBuf {
        let mut result = self.output_dir.clone();
        result.extend(&["src", "main", "java"]);
        for dir in self.group_id.split('.') {
            result.push(dir);
        }
        result.push(&lib.name.to_kebab_case());
        result
    }

    fn resource_dir(&self) -> PathBuf {
        let mut result = self.output_dir.clone();
        result.extend(&["src", "main", "resources"]);
        result
    }
}

pub fn generate_java_bindings(
    lib: &Library,
    config: &JavaBindgenConfig,
) -> FormattingResult<()> {
    fs::create_dir_all(&config.output_dir)?;

    // Create the pom.xml
    generate_pom(lib, config)?;

    // Copy the compiled libraries to the resource folder
    for platform in config.platforms.iter() {
        let platform_directory = match platform.platform {
            Platform::Win64 => "win32-x86-64",
            Platform::Win32 => "win32-x86",
            Platform::Linux => "linux-x86-64",
        };
        let mut target_dir = config.resource_dir();
        target_dir.push(platform_directory);
        fs::create_dir_all(&target_dir)?;

        let mut source_file = platform.location.clone();
        source_file.push(platform.bin_filename(&config.ffi_name));

        let mut target_file = target_dir.clone();
        target_file.push(platform.bin_filename(&config.ffi_name));

        fs::copy(source_file, target_file)?;
    }

    // Create the source directory
    fs::create_dir_all(&config.source_dir(lib))?;

    // Create all the direct mappings
    generate_native_func_class(lib, config)?;

    // Generate the user-facing stuff
    generate_structs(lib, config)?;
    generate_enums(lib, config)?;
    generate_classes(lib, config)?;
    generate_interfaces(lib, config)?;
    generate_callbacks(lib, config)?;

    Ok(())
}

fn generate_pom(lib: &Library, config: &JavaBindgenConfig) -> FormattingResult<()> {
    // Open file
    let mut filename = config.output_dir.clone();
    filename.push("pom");
    filename.set_extension("xml");
    let mut f = FilePrinter::new(filename)?;

    f.writeln("<?xml version=\"1.0\" encoding=\"UTF-8\"?>")?;
    f.writeln("<project xmlns=\"http://maven.apache.org/POM/4.0.0\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:schemaLocation=\"http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd\">")?;
    indented(&mut f, |f| {
        f.writeln("<modelVersion>4.0.0</modelVersion>")?;
        f.newline()?;
        f.writeln(&format!("<groupId>{}</groupId>", config.group_id))?;
        f.writeln(&format!("<artifactId>{}</artifactId>", lib.name.to_kebab_case()))?;
        f.writeln(&format!("<version>{}</version>", lib.version.to_string()))?;
        f.newline()?;
        f.writeln("<properties>")?;
        f.writeln("    <project.java.version>1.8</project.java.version>")?;
        f.writeln("    <project.build.sourceEncoding>UTF-8</project.build.sourceEncoding>")?;
        f.writeln("    <maven.compiler.target>1.8</maven.compiler.target>")?;
	    f.writeln("    <maven.compiler.source>1.8</maven.compiler.source>")?;
        f.writeln("</properties>")?;
        f.newline()?;
        f.writeln("<dependencies>")?;
        f.writeln("    <dependency>")?;
        f.writeln("        <groupId>net.java.dev.jna</groupId>")?;
        f.writeln("        <artifactId>jna</artifactId>")?;
        f.writeln("        <version>5.5.0</version>")?;
        f.writeln("    </dependency>")?;
        f.writeln("</dependencies>")?;

        Ok(())
    })?;
    f.writeln("</project>")
}

fn generate_native_func_class(lib: &Library, config: &JavaBindgenConfig) -> FormattingResult<()> {
    let mut f = create_file(NATIVE_FUNCTIONS_CLASSNAME, config, lib)?;

    f.writeln(&format!("class {}", NATIVE_FUNCTIONS_CLASSNAME))?;
    blocked(&mut f, |f| {
        // Load the library
        f.writeln("static")?;
        blocked(f, |f| {
            f.writeln("System.setProperty(\"jna.encoding\", \"UTF-8\");")?;
            f.writeln(&format!("com.sun.jna.Native.register(\"{}\");", config.ffi_name))
        })?;

        // Write each native functions
        for handle in lib.native_functions() {
            f.newline()?;
            f.write(&format!(
                "static native {} {}(",
                JavaReturnType(&handle.return_type).as_native_type(),
                handle.name
            ))?;

            f.write(
                &handle
                    .parameters
                    .iter()
                    .map(|param| {
                        format!(
                            "{} {}",
                            JavaType(&param.param_type).as_native_type(),
                            param.name
                        )
                    })
                    .collect::<Vec<String>>()
                    .join(", "),
            )?;
            f.write(");")?;
            f.newline()?;
        }

        Ok(())
    })
}

fn generate_structs(lib: &Library, config: &JavaBindgenConfig) -> FormattingResult<()> {
    for native_struct in lib.structs() {
        let mut f = create_file(&native_struct.name().to_camel_case(), config, lib)?;
        structure::generate(&mut f, native_struct, lib)?;
    }

    Ok(())
}

fn generate_enums(lib: &Library, config: &JavaBindgenConfig) -> FormattingResult<()> {
    for native_enum in lib.native_enums() {
        let mut f = create_file(&native_enum.name.to_camel_case(), config, lib)?;
        enumeration::generate(&mut f, native_enum, lib)?;
    }

    Ok(())
}

fn generate_classes(lib: &Library, config: &JavaBindgenConfig) -> FormattingResult<()> {
    for class in lib.classes() {
        let mut f = create_file(&class.name().to_camel_case(), config, lib)?;
        class::generate(&mut f, class, lib)?;
    }

    Ok(())
}

fn generate_interfaces(lib: &Library, config: &JavaBindgenConfig) -> FormattingResult<()> {
    for interface in lib.interfaces() {
        let mut f = create_file(&interface.name.to_camel_case(), config, lib)?;
        interface::generate(&mut f, interface, lib)?;
    }

    Ok(())
}

fn generate_callbacks(lib: &Library, config: &JavaBindgenConfig) -> FormattingResult<()> {
    for callback in lib.one_time_callbacks() {
        let mut f = create_file(&callback.name.to_camel_case(), config, lib)?;
        callback::generate(&mut f, callback, lib)?;
    }

    Ok(())
}

fn create_file(name: &str, config: &JavaBindgenConfig, lib: &Library) -> FormattingResult<FilePrinter> {
    // Open file
    let mut filename = config.source_dir(lib);
    filename.push(name);
    filename.set_extension("java");
    let mut f = FilePrinter::new(filename)?;

    print_license(&mut f, &lib.license)?;
    print_package(&mut f, config, lib)?;
    f.newline()?;

    Ok(f)
}

fn print_license(f: &mut dyn Printer, license: &[String]) -> FormattingResult<()> {
    commented(f, |f| {
        for line in license.iter() {
            f.writeln(line)?;
        }
        Ok(())
    })
}

fn print_package(f: &mut dyn Printer, config: &JavaBindgenConfig, lib: &Library) -> FormattingResult<()> {
    f.writeln(&format!("package {}.{};", config.group_id, lib.name.to_kebab_case()))
}
