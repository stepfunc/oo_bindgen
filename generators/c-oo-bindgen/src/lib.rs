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

use crate::doc::*;
use crate::formatting::*;
use oo_bindgen::class::*;
use oo_bindgen::constants::{ConstantSet, ConstantValue, Representation};
use oo_bindgen::doc::*;
use oo_bindgen::enum_type::*;
use oo_bindgen::formatting::*;
use oo_bindgen::function::*;
use oo_bindgen::interface::*;
use oo_bindgen::platforms::*;
use oo_bindgen::structs::*;
use oo_bindgen::types::{BasicType, TypeExtractor};
use oo_bindgen::*;

use crate::ctype::CType;
use oo_bindgen::util::WithLastIndication;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

mod chelpers;
mod cpp;
mod ctype;
mod doc;
mod formatting;

pub struct CBindgenConfig {
    pub output_dir: PathBuf,
    pub ffi_target_name: String,
    pub ffi_name: String,
    pub is_release: bool,
    pub extra_files: Vec<PathBuf>,
    pub platform_location: PlatformLocation,
}

pub fn generate_c_package(lib: &Library, config: &CBindgenConfig) -> FormattingResult<()> {
    let output_dir = config
        .output_dir
        .join(config.platform_location.platform.as_string());

    // Create header file
    let include_path = output_dir.join("include");
    let source_path = output_dir.join("src");

    generate_c_header(lib, &include_path)?;
    crate::cpp::definition::generate_header(lib, &include_path)?;
    crate::cpp::implementation::generate_cpp_file(lib, &source_path)?;

    // Generate CMake config file
    generate_cmake_config(lib, config, &config.platform_location)?;

    // Copy lib files (lib and DLL on Windows, .so on Linux)
    let lib_path = output_dir
        .join("lib")
        .join(config.platform_location.platform.as_string());
    fs::create_dir_all(&lib_path)?;

    let lib_filename = config
        .platform_location
        .static_lib_filename(&config.ffi_name);
    fs::copy(
        config.platform_location.location.join(&lib_filename),
        lib_path.join(&lib_filename),
    )?;

    let lib_filename = config.platform_location.dyn_lib_filename(&config.ffi_name);
    fs::copy(
        config.platform_location.location.join(&lib_filename),
        lib_path.join(&lib_filename),
    )?;

    // Copy DLL on Windows
    let bin_filename = config.platform_location.bin_filename(&config.ffi_name);
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

    Ok(())
}

pub fn generate_doxygen(lib: &Library, config: &CBindgenConfig) -> FormattingResult<()> {
    // Build documentation
    let mut command = Command::new("doxygen")
        .current_dir(&config.output_dir)
        .arg("-")
        .stdin(std::process::Stdio::piped())
        .spawn()
        .expect("failed to spawn doxygen");

    {
        let stdin = command.stdin.as_mut().unwrap();
        stdin
            .write_all(&format!("PROJECT_NAME = {}\n", lib.settings.name).into_bytes())
            .unwrap();
        stdin
            .write_all(&format!("PROJECT_NUMBER = {}\n", lib.version.to_string()).into_bytes())
            .unwrap();
        stdin.write_all(b"HTML_OUTPUT = doc\n").unwrap();
        stdin.write_all(b"GENERATE_LATEX = NO\n").unwrap();
        stdin.write_all(b"EXTRACT_STATIC = YES\n").unwrap();
        stdin
            .write_all(
                &format!(
                    "INPUT = {}/include\n",
                    config.platform_location.platform.as_string()
                )
                .into_bytes(),
            )
            .unwrap();
    }

    command.wait()?;

    Ok(())
}

fn generate_c_header(lib: &Library, path: &Path) -> FormattingResult<()> {
    let uppercase_name = lib.settings.c_ffi_prefix.to_uppercase();

    // Open file
    fs::create_dir_all(&path)?;
    let filename = path.join(format!("{}.h", lib.settings.name));
    let mut f = FilePrinter::new(filename)?;

    // Print license
    commented(&mut f, |f| {
        for line in lib.info.license_description.iter() {
            f.writeln(line)?;
        }
        Ok(())
    })?;

    // Header guard
    f.writeln("#pragma once")?;
    f.newline()?;

    // C++ guard
    cpp_guard(&mut f, |f| {
        f.newline()?;

        // Version number
        f.writeln(&format!(
            "#define {}_VERSION_MAJOR {}",
            uppercase_name, lib.version.major
        ))?;
        f.writeln(&format!(
            "#define {}_VERSION_MINOR {}",
            uppercase_name, lib.version.minor
        ))?;
        f.writeln(&format!(
            "#define {}_VERSION_PATCH {}",
            uppercase_name, lib.version.patch
        ))?;
        f.writeln(&format!(
            "#define {}_VERSION_STRING \"{}\"",
            uppercase_name,
            lib.version.to_string()
        ))?;
        f.newline()?;

        // Standard includes needed
        f.writeln("#include <stdbool.h>")?;
        f.writeln("#include <stdint.h>")?;
        f.newline()?;

        // Doxygen needs this
        f.writeln("/// @file")?;
        f.newline()?;

        // Iterate through each statement and print them
        for statement in lib.statements() {
            match statement {
                Statement::Constants(handle) => write_constants_definition(f, handle, lib)?,
                Statement::StructDeclaration(handle) => {
                    let c_type = handle.to_c_type();
                    f.writeln(&format!("typedef struct {} {};", c_type, c_type))?;
                }
                Statement::StructDefinition(st) => match st {
                    StructType::FunctionArg(x) => write_struct_definition(f, x, lib)?,
                    StructType::FunctionReturn(x) => write_struct_definition(f, x, lib)?,
                    StructType::CallbackArg(x) => write_struct_definition(f, x, lib)?,
                    StructType::Universal(x) => write_struct_definition(f, x, lib)?,
                },
                Statement::EnumDefinition(handle) => write_enum_definition(f, handle, lib)?,
                Statement::ClassDeclaration(handle) => write_class_declaration(f, handle)?,
                Statement::FunctionDefinition(handle) => write_function(f, handle, lib)?,
                Statement::InterfaceDefinition(handle) => write_interface(f, handle, lib)?,
                _ => (),
            }
            f.newline()?;
        }

        Ok(())
    })
}

fn write_constants_definition(
    f: &mut dyn Printer,
    handle: &Handle<ConstantSet<Validated>>,
    lib: &Library,
) -> FormattingResult<()> {
    fn get_constant_value(value: ConstantValue) -> String {
        match value {
            ConstantValue::U8(value, Representation::Hex) => format!("0x{:02X?}", value),
        }
    }

    for item in &handle.values {
        doxygen(f, |f| doxygen_print(f, &item.doc, lib))?;
        f.writeln(&format!(
            "#define {}_{}_{} {}",
            lib.settings.c_ffi_prefix.capital_snake_case(),
            handle.name.capital_snake_case(),
            item.name.capital_snake_case(),
            get_constant_value(item.value)
        ))?;
    }
    Ok(())
}

fn write_struct_definition<T>(
    f: &mut dyn Printer,
    handle: &Handle<Struct<T, Validated>>,
    lib: &Library,
) -> FormattingResult<()>
where
    T: StructFieldType + TypeExtractor + CType,
{
    let doc = match handle.visibility {
        Visibility::Public => handle.doc.clone(),
        Visibility::Private => handle
            .doc
            .clone()
            .warning("This struct should never be initialized or modified by user code"),
    };

    doxygen(f, |f| doxygen_print(f, &doc, lib))?;

    // Write the struct definition
    f.writeln(&format!("typedef struct {}", handle.to_c_type()))?;
    f.writeln("{")?;
    indented(f, |f| {
        for element in &handle.fields {
            doxygen(f, |f| {
                doxygen_print(f, &element.doc, lib)?;

                if let Some(t) = &element.field_type.get_duration_type() {
                    f.writeln(&format!("@note The unit is {}", t.unit()))?;
                }

                Ok(())
            })?;
            f.writeln(&format!(
                "{} {};",
                element.field_type.to_c_type(),
                element.name,
            ))?;
        }
        Ok(())
    })?;
    f.writeln(&format!("}} {};", handle.to_c_type()))?;

    // user should never try to initialize opaque structs, so don't suggest this is OK
    if handle.visibility != Visibility::Private {
        f.newline()?;
        for c in &handle.initializers {
            write_struct_initializer(f, lib, c, handle)?;
            f.newline()?;
        }
    }

    Ok(())
}

fn get_default_value(default: &ValidatedDefaultValue) -> String {
    match default {
        ValidatedDefaultValue::Bool(x) => x.to_string(),
        ValidatedDefaultValue::Numeric(x) => match x {
            Number::U8(x) => x.to_string(),
            Number::S8(x) => x.to_string(),
            Number::U16(x) => x.to_string(),
            Number::S16(x) => x.to_string(),
            Number::U32(x) => x.to_string(),
            Number::S32(x) => x.to_string(),
            Number::U64(x) => x.to_string(),
            Number::S64(x) => x.to_string(),
            Number::Float(x) => format!("{}f", x),
            Number::Double(x) => x.to_string(),
        },
        ValidatedDefaultValue::Duration(t, x) => t.get_value_string(*x),
        ValidatedDefaultValue::Enum(x, variant) => {
            format!(
                "{}_{}_{}",
                x.settings.c_ffi_prefix.capital_snake_case(),
                x.name.capital_snake_case(),
                variant.capital_snake_case()
            )
        }
        ValidatedDefaultValue::String(x) => format!("\"{}\"", x),
        ValidatedDefaultValue::DefaultStruct(handle, _, name) => {
            format!(
                "{}_{}_{}()",
                &handle.settings().c_ffi_prefix,
                handle.name(),
                name
            )
        }
    }
}

fn write_struct_initializer<T>(
    f: &mut dyn Printer,
    lib: &Library,
    initializer: &Initializer<Validated>,
    handle: &Handle<Struct<T, Validated>>,
) -> FormattingResult<()>
where
    T: StructFieldType + CType + TypeExtractor,
{
    doxygen(f, |f| {
        f.writeln("@brief ")?;
        docstring_print(f, &initializer.doc.brief, lib)?;

        if !initializer.values.is_empty() {
            f.newline()?;
            f.writeln("@note")?;
            for value in initializer.values.iter() {
                f.writeln(&format!("{} is initialized to {}", value.name, value.value))?;
            }
            f.newline()?;
        }

        f.writeln("@returns ")?;
        docstring_print(f, &text(&format!("New instance of {}", handle.name())), lib)?;

        Ok(())
    })?;

    let params = handle
        .fields()
        .filter(|f| !initializer.values.iter().any(|cf| cf.name == f.name))
        .map(|el| format!("{} {}", el.field_type.to_c_type(), el.name))
        .collect::<Vec<String>>()
        .join(", ");

    f.writeln(&format!(
        "static {} {}_{}_{}({})",
        handle.to_c_type(),
        handle.declaration.inner.settings.c_ffi_prefix,
        handle.name(),
        initializer.name,
        params
    ))?;

    blocked(f, |f| {
        f.writeln(&format!("{} _return_value = {{", handle.to_c_type()))?;
        indented(f, |f| {
            for (field, last) in handle.fields.iter().with_last() {
                let value: String = match initializer.values.iter().find(|x| x.name == field.name) {
                    Some(x) => get_default_value(&x.value),
                    None => field.name.to_string(),
                };
                let value = if last { value } else { format!("{},", value) };
                f.writeln(&value)?;
            }
            Ok(())
        })?;

        f.writeln("};")?;
        f.writeln("return _return_value;")
    })?;

    Ok(())
}

fn write_enum_definition(
    f: &mut dyn Printer,
    handle: &Handle<Enum<Validated>>,
    lib: &Library,
) -> FormattingResult<()> {
    doxygen(f, |f| doxygen_print(f, &handle.doc, lib))?;

    f.writeln(&format!("typedef enum {}", handle.to_c_type()))?;
    f.writeln("{")?;
    indented(f, |f| {
        for variant in &handle.variants {
            doxygen(f, |f| doxygen_print(f, &variant.doc, lib))?;
            f.writeln(&format!(
                "{}_{}_{} = {},",
                lib.settings.c_ffi_prefix.capital_snake_case(),
                handle.name.capital_snake_case(),
                variant.name.capital_snake_case(),
                variant.value
            ))?;
        }
        Ok(())
    })?;
    f.writeln(&format!("}} {};", handle.to_c_type()))?;

    f.newline()?;

    doxygen(f, |f| {
        f.writeln("@brief ")?;
        docstring_print(f, &text("Converts the enum to a string"), lib)?;
        f.writeln("@param value Enum to convert")?;
        f.writeln("@returns String representation")
    })?;
    f.writeln(&format!(
        "static const char* {}_{}_to_string({} value)",
        &lib.settings.c_ffi_prefix,
        handle.name,
        handle.to_c_type()
    ))?;
    blocked(f, |f| {
        f.writeln("switch (value)")?;
        blocked(f, |f| {
            for variant in &handle.variants {
                f.writeln(&format!(
                    "case {}_{}_{}: return \"{}\";",
                    lib.settings.c_ffi_prefix.capital_snake_case(),
                    handle.name.capital_snake_case(),
                    variant.name.capital_snake_case(),
                    variant.name
                ))?;
            }
            f.writeln(&format!(
                "default: return \"unknown {} value\";",
                handle.name
            ))
        })
    })
}

fn write_class_declaration(
    f: &mut dyn Printer,
    handle: &ClassDeclarationHandle,
) -> FormattingResult<()> {
    f.writeln(&format!(
        "typedef struct {} {};",
        handle.to_c_type(),
        handle.to_c_type()
    ))
}

fn write_function_docs(
    f: &mut dyn Printer,
    handle: &Handle<Function<Validated>>,
    lib: &Library,
) -> FormattingResult<()> {
    doxygen(f, |f| {
        // Print top-level documentation
        doxygen_print(f, &handle.doc, lib)?;

        // Print each parameter value
        for param in &handle.parameters {
            f.writeln(&format!("@param {} ", param.name))?;
            docstring_print(f, &param.doc, lib)?;
            if let FunctionArgument::Basic(BasicType::Duration(mapping)) = param.arg_type {
                f.write(&format!(" ({})", mapping.unit()))?;
            }
        }

        fn write_error_return_doc(f: &mut dyn Printer) -> FormattingResult<()> {
            f.writeln("@return Error code")
        }

        match handle.get_signature_type() {
            SignatureType::NoErrorNoReturn => {}
            SignatureType::NoErrorWithReturn(ret, doc) => {
                f.writeln("@return ")?;
                docstring_print(f, &doc, lib)?;
                if let FunctionReturnValue::Basic(BasicType::Duration(mapping)) = ret {
                    f.write(&format!(" ({})", mapping.unit()))?;
                }
            }
            SignatureType::ErrorNoReturn(_) => {
                write_error_return_doc(f)?;
            }
            SignatureType::ErrorWithReturn(_, ret, doc) => {
                f.writeln("@param out ")?;
                docstring_print(f, &doc, lib)?;
                if let FunctionReturnValue::Basic(BasicType::Duration(mapping)) = ret {
                    f.write(&format!(" ({})", mapping.unit()))?;
                }
                write_error_return_doc(f)?;
            }
        }

        Ok(())
    })
}

fn write_function(
    f: &mut dyn Printer,
    handle: &Handle<Function<Validated>>,
    lib: &Library,
) -> FormattingResult<()> {
    write_function_docs(f, handle, lib)?;

    if let Some(error_type) = &handle.error_type {
        f.writeln(&format!(
            "{} {}_{}(",
            error_type.inner.to_c_type(),
            &lib.settings.c_ffi_prefix,
            handle.name
        ))?;
    } else {
        f.writeln(&format!(
            "{} {}_{}(",
            handle.return_type.to_c_type(),
            &lib.settings.c_ffi_prefix,
            handle.name
        ))?;
    }

    f.write(
        &handle
            .parameters
            .iter()
            .map(|param| format!("{} {}", param.arg_type.to_c_type(), param.name))
            .collect::<Vec<String>>()
            .join(", "),
    )?;

    if handle.error_type.is_some() {
        if let FunctionReturnType::Type(x, _) = &handle.return_type {
            if !handle.parameters.is_empty() {
                f.write(", ")?;
                f.write(&format!("{}* out", x.to_c_type()))?;
            }
        }
    }

    f.write(");")
}

fn write_interface(
    f: &mut dyn Printer,
    handle: &Handle<Interface<Validated>>,
    lib: &Library,
) -> FormattingResult<()> {
    doxygen(f, |f| doxygen_print(f, &handle.doc, lib))?;

    let struct_name = handle.to_c_type();

    let ctx_variable_name = lib.settings.interface.context_variable_name.clone();
    let destroy_func_name = lib.settings.interface.destroy_func_name.clone();

    f.writeln(&format!("typedef struct {}", struct_name))?;
    f.writeln("{")?;
    indented(f, |f| {
        for cb in &handle.callbacks {
            f.newline()?;

            // Print the documentation
            doxygen(f, |f| {
                // Print top-level documentation
                doxygen_print(f, &cb.doc, lib)?;

                // Print each argument value
                for arg in &cb.arguments {
                    f.writeln(&format!("@param {} ", arg.name))?;
                    docstring_print(f, &arg.doc, lib)?;
                }

                f.writeln(&format!("@param {} ", ctx_variable_name))?;
                docstring_print(f, &text("Context data"), lib)?;

                // Print return documentation
                if let CallbackReturnType::Type(_, doc) = &cb.return_type {
                    f.writeln("@return ")?;
                    docstring_print(f, doc, lib)?;
                }

                Ok(())
            })?;

            f.newline()?;

            // Print function signature
            f.write(&format!("{} (*{})(", cb.return_type.to_c_type(), cb.name))?;

            f.write(&chelpers::callback_parameters(cb))?;

            f.write(");")?;
        }

        doxygen(f, |f| {
            f.writeln(
                "@brief Callback when the underlying owner doesn't need the interface anymore",
            )?;
            f.writeln("@param arg Context data")
        })?;
        f.writeln(&format!("void (*{})(void* arg);", destroy_func_name))?;

        doxygen(f, |f| f.writeln("@brief Context data"))?;
        f.writeln(&format!("void* {};", ctx_variable_name))?;

        Ok(())
    })?;
    f.writeln(&format!("}} {};", struct_name))?;

    f.newline()?;

    // Write init helper
    doxygen(f, |f| {
        f.writeln("@brief ")?;
        docstring_print(f, &text("Initialize an instance of the interface"), lib)?;
        for cb in &handle.callbacks {
            f.writeln(&format!("@param {} ", cb.name))?;
            docstring_print(f, &cb.doc.brief, lib)?;
        }
        f.writeln(&format!(
            "@param {} Callback when the underlying owner doesn't need the interface anymore",
            destroy_func_name
        ))?;
        f.writeln(&format!("@param {} Context data", ctx_variable_name))?;
        Ok(())
    })?;
    f.writeln(&format!(
        "static {} {}_{}_init(",
        struct_name, &lib.settings.c_ffi_prefix, handle.name
    ))?;
    indented(f, |f| {
        for cb in &handle.callbacks {
            f.writeln(&format!("{} (*{})(", cb.return_type.to_c_type(), cb.name,))?;

            f.write(&chelpers::callback_parameters(cb))?;
            f.write("),")?;
        }

        f.writeln(&format!("void (*{})(void* arg),", destroy_func_name))?;
        f.writeln(&format!("void* {}", ctx_variable_name))?;

        Ok(())
    })?;
    f.writeln(")")?;

    blocked(f, |f| {
        f.writeln(&format!("{} _return_value = {{", struct_name))?;
        indented(f, |f| {
            for cb in &handle.callbacks {
                f.writeln(&format!("{},", cb.name))?;
            }
            f.writeln(&format!("{},", destroy_func_name))?;
            f.writeln(ctx_variable_name.as_ref())
        })?;
        f.writeln("};")?;
        f.writeln("return _return_value;")
    })?;

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
        .join(platform_location.platform.as_string())
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
            platform_location.platform.as_string(),
            platform_location.bin_filename(&config.ffi_name)
        ))?;
        f.writeln(&format!(
            "IMPORTED_IMPLIB \"${{prefix}}/lib/{}/{}\"",
            platform_location.platform.as_string(),
            platform_location.dyn_lib_filename(&config.ffi_name)
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
            platform_location.platform.as_string(),
            platform_location.static_lib_filename(&config.ffi_name)
        ))?;
        f.writeln("INTERFACE_INCLUDE_DIRECTORIES \"${prefix}/include\"")?;
        f.writeln(&format!(
            "INTERFACE_LINK_LIBRARIES \"{}\"",
            link_deps.join(";")
        ))
    })?;
    f.writeln(")")?;

    f.writeln(&format!(
        "set({}_CPP_FILE ${{prefix}}/src/{}.cpp CACHE STRING \"CPP implementation\" FORCE)",
        lib.settings.name.capital_snake_case(),
        lib.settings.name
    ))?;
    f.newline()
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
