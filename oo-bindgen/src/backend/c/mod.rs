use std::fs;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use crate::backend::*;
use crate::model::*;

mod cpp;
mod ctype;
mod doc;
mod formatting;
mod header;

pub(crate) struct CBindgenConfig {
    pub(crate) output_dir: PathBuf,
    pub(crate) ffi_target_name: &'static str,
    pub(crate) ffi_name: &'static str,
    pub(crate) extra_files: Vec<PathBuf>,
    pub(crate) platform_locations: PlatformLocations,
    pub(crate) generate_doxygen: bool,
}

pub(crate) fn generate_c_package(lib: &Library, config: &CBindgenConfig) -> FormattingResult<()> {
    // Create header file
    let include_path = config.output_dir.join("include");
    let source_path = config.output_dir.join("src");

    header::generate_c_header(lib, &include_path)?;
    cpp::header::generate_header(lib, &include_path)?;
    cpp::implementation::generate_cpp_file(lib, &source_path)?;

    // Generate CMake config file
    generate_cmake_config(lib, config, &config.platform_locations)?;

    // for each platform location, copy the libraries
    for pl in config.platform_locations.iter() {
        let span = tracing::info_span!("libs", platform = pl.platform.target_triple);
        let _entered = span.enter();

        // Copy lib files (lib and DLL on Windows, .so on Linux)
        let lib_path = config
            .output_dir
            .join("lib")
            .join(pl.platform.target_triple);
        logged::create_dir_all(&lib_path)?;

        let lib_filename = pl.platform.static_lib_filename(&config.ffi_name);
        logged::copy(
            pl.location.join(&lib_filename),
            lib_path.join(&lib_filename),
        )?;

        let lib_filename = pl.platform.dyn_lib_filename(&config.ffi_name);
        logged::copy(
            pl.location.join(&lib_filename),
            lib_path.join(&lib_filename),
        )?;

        let bin_filename = pl.platform.bin_filename(&config.ffi_name);
        // Copy DLL on Windows
        logged::copy(
            pl.location.join(&bin_filename),
            lib_path.join(&bin_filename),
        )?;
    }

    // Copy extra files
    logged::copy(
        &lib.info.license_path,
        config
            .output_dir
            .join(lib.info.license_path.file_name().unwrap()),
    )?;
    for path in &config.extra_files {
        logged::copy(path, config.output_dir.join(path.file_name().unwrap()))?;
    }

    // Generate doxygen (if asked)
    if config.generate_doxygen {
        generate_doxygen(lib, config)?;
    }

    Ok(())
}

fn generate_doxygen(lib: &Library, config: &CBindgenConfig) -> FormattingResult<()> {
    // Copy doxygen awesome in target directory
    let doxygen_awesome = include_str!("../../../static/doxygen-awesome.css");
    fs::write(
        config.output_dir.join("doxygen-awesome.css"),
        doxygen_awesome,
    )?;

    // Write the logo file
    fs::write(config.output_dir.join("logo.png"), lib.info.logo_png)?;

    let include_path = "include";

    // Build C documentation
    fs::create_dir_all(config.output_dir.join("doc").join("c"))?;
    run_doxygen(
        &config.output_dir,
        &[
            &format!("PROJECT_NAME = {} (C API)", lib.settings.name),
            &format!("PROJECT_NUMBER = {}", lib.version),
            &format!("INPUT = include/{}.h", lib.settings.name),
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
    platform_locations: &PlatformLocations,
) -> FormattingResult<()> {
    fn write_set_libs(
        f: &mut dyn Printer,
        lib: &Library,
        config: &CBindgenConfig,
        pl: &PlatformLocation,
    ) -> FormattingResult<()> {
        indented(f, |f| {
            f.writeln(&format!(
                "set({}_IMPORTED_LOCATION {})",
                lib.settings.name.capital_snake_case(),
                pl.platform.bin_filename(&config.ffi_name)
            ))?;
            f.writeln(&format!(
                "set({}_STATIC_IMPORTED_LOCATION {})",
                lib.settings.name.capital_snake_case(),
                pl.platform.static_lib_filename(&config.ffi_name)
            ))?;
            f.writeln(&format!(
                "set({}_IMPORTED_IMPLIB {})",
                lib.settings.name.capital_snake_case(),
                pl.platform.dyn_lib_filename(&config.ffi_name)
            ))
        })
    }

    // Create file
    let cmake_path = config.output_dir.join("cmake");

    logged::create_dir_all(&cmake_path)?;

    let filename = cmake_path.join(format!("{}-config.cmake", lib.settings.name));
    let mut f = FilePrinter::new(filename)?;

    // Prefix used everywhere else
    f.writeln("set(prefix \"${CMAKE_CURRENT_LIST_DIR}/..\")")?;
    f.newline()?;

    // variable names
    let rust_target_var = format!("{}_RUST_TARGET", lib.settings.name.capital_snake_case());
    let imported_location_var = format!(
        "{}_IMPORTED_LOCATION",
        lib.settings.name.capital_snake_case()
    );
    let static_imported_location_var = format!(
        "{}_STATIC_IMPORTED_LOCATION",
        lib.settings.name.capital_snake_case()
    );
    let imported_implib_var = format!("{}_IMPORTED_IMPLIB", lib.settings.name.capital_snake_case());

    let (first, others) = platform_locations
        .locations
        .split_first()
        .expect("there must be at least one target");

    // first check that the target triple is defined
    f.writeln(&format!("if(NOT {})", rust_target_var))?;
    indented(&mut f, |f| {
        if others.is_empty() {
            f.writeln("# since there is only 1 target in this package we can assume this is what is wanted")?;
            f.writeln(&format!(
                "message(\"{} not set, default to the only library in this package: {}\")",
                rust_target_var, first.platform.target_triple
            ))?;
            f.writeln(&format!(
                "set({} \"{}\")",
                rust_target_var, first.platform.target_triple
            ))
        } else {
            f.writeln(&format!(
                "message(FATAL_ERROR \"{} not specified and there {} possible targets\")",
                rust_target_var,
                platform_locations.locations.len()
            ))
        }
    })?;
    f.writeln("endif()")?;

    f.newline()?;
    f.writeln(&format!(
        "message(\"{} is: ${{{}}}\")",
        rust_target_var, rust_target_var
    ))?;
    f.newline()?;

    // validate the target triple
    f.writeln(&format!(
        "if(${{{}}} STREQUAL \"{}\")",
        rust_target_var, first.platform.target_triple
    ))?;
    write_set_libs(&mut f, lib, config, first)?;
    for pl in others {
        f.writeln(&format!(
            "elseif(${{{}}} STREQUAL \"{}\")",
            rust_target_var, pl.platform.target_triple
        ))?;
        write_set_libs(&mut f, lib, config, pl)?;
    }
    f.writeln("else()")?;
    indented(&mut f, |f| {
        f.writeln(&format!(
            "message(FATAL_ERROR \"unknown target triple: ${{{}}}\")",
            rust_target_var
        ))
    })?;
    f.writeln("endif()")?;
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
            "IMPORTED_LOCATION \"${{prefix}}/lib/${{{}}}/${{{}}}\"",
            rust_target_var, imported_location_var
        ))?;
        f.writeln(&format!(
            "IMPORTED_IMPLIB \"${{prefix}}/lib/${{{}}}/${{{}}}\"",
            rust_target_var, imported_implib_var
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
            "IMPORTED_LOCATION \"${{prefix}}/lib/${{{}}}/${{{}}}\"",
            rust_target_var, static_imported_location_var
        ))?;
        f.writeln("INTERFACE_INCLUDE_DIRECTORIES \"${prefix}/include\"")?;
        f.writeln(&format!(
            "INTERFACE_LINK_LIBRARIES \"{}\"",
            get_link_dependencies(config).join(";")
        ))
    })?;
    f.writeln(")")?;

    f.newline()?;

    // C++ target
    f.writeln("get_property(languages GLOBAL PROPERTY ENABLED_LANGUAGES)")?;
    f.writeln("if(\"CXX\" IN_LIST languages)")?;
    indented(&mut f, |f| {
        f.writeln("set(CMAKE_CXX_STANDARD 11)")?;
        f.writeln(&format!(
            "add_library({}_cpp OBJECT EXCLUDE_FROM_ALL ${{prefix}}/src/{}.cpp)",
            lib.settings.name, lib.settings.name
        ))?;
        f.writeln(&format!(
            "target_compile_features({} INTERFACE cxx_std_14)",
            lib.settings.name
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

// from 39c1728e6cf10f8bc716fa27f318f2db562a05bc
fn get_link_dependencies(config: &CBindgenConfig) -> Vec<String> {
    let output = Command::new("cargo")
        .args(&[
            "rustc",
            "-p",
            config.ffi_target_name,
            "--",
            "--print",
            "native-static-libs",
        ])
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
