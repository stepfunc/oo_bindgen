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
    // intra_doc_link_resolution_failure, broken_intra_doc_links
    unaligned_references,
    while_true,
    bare_trait_objects
)]

use std::fs;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use oo_bindgen::backend::*;
use oo_bindgen::model::*;

mod cpp;
mod ctype;
mod doc;
mod formatting;
mod header;

pub struct CBindgenConfig {
    pub output_dir: PathBuf,
    pub ffi_target_name: String,
    pub ffi_name: String,
    pub is_release: bool,
    pub extra_files: Vec<PathBuf>,
    pub platform_location: PlatformLocation,
    pub generate_doxygen: bool,
}

pub fn generate_c_package(lib: &Library, config: &CBindgenConfig) -> FormattingResult<()> {
    let output_dir = config
        .output_dir
        .join(config.platform_location.platform.target_triple);

    // Create header file
    let include_path = output_dir.join("include");
    let source_path = output_dir.join("src");

    crate::header::generate_c_header(lib, &include_path)?;
    crate::cpp::header::generate_header(lib, &include_path)?;
    crate::cpp::implementation::generate_cpp_file(lib, &source_path)?;

    // Generate CMake config file
    generate_cmake_config(lib, config, &config.platform_location)?;

    // Copy lib files (lib and DLL on Windows, .so on Linux)
    let lib_path = output_dir
        .join("lib")
        .join(config.platform_location.platform.target_triple);
    fs::create_dir_all(&lib_path)?;

    let lib_filename = config
        .platform_location
        .platform
        .static_lib_filename(&config.ffi_name);
    fs::copy(
        config.platform_location.location.join(&lib_filename),
        lib_path.join(&lib_filename),
    )?;

    let lib_filename = config
        .platform_location
        .platform
        .dyn_lib_filename(&config.ffi_name);
    fs::copy(
        config.platform_location.location.join(&lib_filename),
        lib_path.join(&lib_filename),
    )?;

    // Copy DLL on Windows
    let bin_filename = config
        .platform_location
        .platform
        .bin_filename(&config.ffi_name);
    fs::copy(
        config.platform_location.location.join(&bin_filename),
        lib_path.join(&bin_filename),
    )?;

    // Copy extra files
    fs::copy(
        &lib.info.license_path,
        output_dir.join(lib.info.license_path.file_name().unwrap()),
    )?;
    for path in &config.extra_files {
        fs::copy(path, output_dir.join(path.file_name().unwrap()))?;
    }

    // Generate doxygen (if asked)
    if config.generate_doxygen {
        generate_doxygen(lib, config)?;
    }

    Ok(())
}

fn generate_doxygen(lib: &Library, config: &CBindgenConfig) -> FormattingResult<()> {
    // Copy doxygen awesome in target directory
    let doxygen_awesome = include_str!("../../doxygen-awesome.css");
    fs::write(
        config.output_dir.join("doxygen-awesome.css"),
        doxygen_awesome,
    )?;

    // Write the logo file
    fs::write(config.output_dir.join("logo.png"), lib.info.logo_png)?;

    let include_path = format!(
        "{}/include",
        config.platform_location.platform.target_triple
    );

    // Build C documentation
    fs::create_dir_all(config.output_dir.join("doc").join("c"))?;
    run_doxygen(
        &config.output_dir,
        &[
            &format!("PROJECT_NAME = {} (C API)", lib.settings.name),
            &format!("PROJECT_NUMBER = {}", lib.version),
            &format!("INPUT = {}/{}.h", include_path, lib.settings.name),
            "HTML_OUTPUT = doc/c",
            // Output customization
            "GENERATE_LATEX = NO",                          // No LaTeX
            "EXTRACT_STATIC = YES",                         // We want all functions
            "TYPEDEF_HIDES_STRUCT = YES",                   // To avoid a large typedef table
            "AUTOLINK_SUPPORT = NO",                        // Only link when we explicitly want to
            "OPTIMIZE_OUTPUT_FOR_C = YES",                  // I guess this will help the output
            "ALWAYS_DETAILED_SEC = YES",                    // Always print detailed section
            &format!("STRIP_FROM_PATH = {}", include_path), // Remove include path
            // Styling
            "HTML_EXTRA_STYLESHEET = doxygen-awesome.css",
            "GENERATE_TREEVIEW = YES",
            "PROJECT_LOGO = logo.png",
            "HTML_COLORSTYLE_HUE = 209", // See https://jothepro.github.io/doxygen-awesome-css/index.html#autotoc_md14
            "HTML_COLORSTYLE_SAT = 255",
            "HTML_COLORSTYLE_GAMMA = 113",
        ],
    )?;

    // Build C++ documentation
    fs::create_dir_all(config.output_dir.join("doc").join("cpp"))?;
    run_doxygen(
        &config.output_dir,
        &[
            &format!("PROJECT_NAME = {} (C++ API)", lib.settings.name),
            &format!("PROJECT_NUMBER = {}", lib.version),
            &format!("INPUT = {}/{}.hpp", include_path, lib.settings.name),
            "HTML_OUTPUT = doc/cpp",
            // Output customization
            "GENERATE_LATEX = NO",                          // No LaTeX
            "EXTRACT_STATIC = YES",                         // We want all functions
            "ALWAYS_DETAILED_SEC = YES",                    // Always print detailed section
            &format!("STRIP_FROM_PATH = {}", include_path), // Remove include path
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

fn generate_cmake_config(
    lib: &Library,
    config: &CBindgenConfig,
    platform_location: &PlatformLocation,
) -> FormattingResult<()> {
    // Create file
    let cmake_path = config
        .output_dir
        .join(platform_location.platform.target_triple)
        .join("cmake");
    fs::create_dir_all(&cmake_path)?;
    let filename = cmake_path.join(format!("{}-config.cmake", lib.settings.name));
    let mut f = FilePrinter::new(filename)?;

    let link_deps = get_link_dependencies(config);

    // Prefix used everywhere else
    f.writeln("set(prefix \"${CMAKE_CURRENT_LIST_DIR}/..\")")?;
    f.newline()?;

    // Write dynamic library version
    f.writeln(&format!(
        "add_library({} SHARED IMPORTED GLOBAL)",
        lib.settings.name
    ))?;
    f.writeln(&format!(
        "set_target_properties({} PROPERTIES",
        lib.settings.name
    ))?;
    indented(&mut f, |f| {
        f.writeln(&format!(
            "IMPORTED_LOCATION \"${{prefix}}/lib/{}/{}\"",
            platform_location.platform.target_triple,
            platform_location.platform.bin_filename(&config.ffi_name)
        ))?;
        f.writeln(&format!(
            "IMPORTED_IMPLIB \"${{prefix}}/lib/{}/{}\"",
            platform_location.platform.target_triple,
            platform_location
                .platform
                .dyn_lib_filename(&config.ffi_name)
        ))?;
        f.writeln("INTERFACE_INCLUDE_DIRECTORIES \"${prefix}/include\"")
    })?;
    f.writeln(")")?;

    f.newline()?;

    // Write static library
    f.writeln(&format!(
        "add_library({}_static STATIC IMPORTED GLOBAL)",
        lib.settings.name
    ))?;
    f.writeln(&format!(
        "set_target_properties({}_static PROPERTIES",
        lib.settings.name
    ))?;
    indented(&mut f, |f| {
        f.writeln(&format!(
            "IMPORTED_LOCATION \"${{prefix}}/lib/{}/{}\"",
            platform_location.platform.target_triple,
            platform_location
                .platform
                .static_lib_filename(&config.ffi_name)
        ))?;
        f.writeln("INTERFACE_INCLUDE_DIRECTORIES \"${prefix}/include\"")?;
        f.writeln(&format!(
            "INTERFACE_LINK_LIBRARIES \"{}\"",
            link_deps.join(";")
        ))
    })?;
    f.writeln(")")?;

    f.newline()?;

    // C++ target
    f.writeln("get_property(languages GLOBAL PROPERTY ENABLED_LANGUAGES)")?;
    f.writeln("if(\"CXX\" IN_LIST languages)")?;
    indented(&mut f, |f| {
        f.writeln(&format!(
            "add_library({}_cpp OBJECT EXCLUDE_FROM_ALL ${{prefix}}/src/{}.cpp)",
            lib.settings.name, lib.settings.name
        ))?;
        f.writeln(&format!(
            "target_link_libraries({}_cpp {})",
            lib.settings.name, lib.settings.name
        ))?;

        Ok(())
    })?;
    f.writeln("endif()")?;

    Ok(())
}

fn get_link_dependencies(config: &CBindgenConfig) -> Vec<String> {
    let mut args = Vec::from(["rustc", "-p", &config.ffi_target_name]);

    if config.is_release {
        args.push("--release");
    }

    args.extend(&["--", "--print", "native-static-libs"]);

    let output = Command::new("cargo")
        .args(&args)
        .output()
        .expect("failed to run cargo");

    assert!(
        output.status.success(),
        "failed to get the link dependencies"
    );

    // It prints to stderr for some reason
    let result = String::from_utf8_lossy(&output.stderr);

    // Find where the libs are written
    const PATTERN: &str = "native-static-libs: ";
    let pattern_idx = result
        .find(PATTERN)
        .expect("failed to parse link dependencies");
    let deps = &result[pattern_idx + PATTERN.len()..result.len()];
    let endline = deps.find('\n').expect("failed to parse link dependencies");
    let deps = &deps[0..endline];

    // Extract the libs
    let mut result = deps
        .split_whitespace()
        .map(|x| x.to_owned())
        .collect::<Vec<_>>();

    // Remove duplicates
    result.dedup();

    result
}
