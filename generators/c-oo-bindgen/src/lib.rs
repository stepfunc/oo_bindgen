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

use oo_bindgen::*;
use oo_bindgen::formatting::*;
use oo_bindgen::interface::*;
use oo_bindgen::native_enum::*;
use oo_bindgen::native_function::*;
use oo_bindgen::native_struct::*;
use oo_bindgen::platforms::*;
use std::fmt::Display;
use std::fs;
use std::path::{Path, PathBuf};
use crate::formatting::*;

mod formatting;

pub struct CBindgenConfig {
    pub output_dir: PathBuf,
    pub ffi_name: String,
    pub platforms: PlatformLocations,
}

pub fn generate_c_package(lib: &Library, config: &CBindgenConfig) -> FormattingResult<()> {
    // Create header file
    let mut include_path = config.output_dir.clone();
    include_path.push("include");
    generate_c_header(lib, include_path)?;

    // Generate CMake config file
    generate_cmake_config(lib, config)?;

    // Copy lib files (lib and DLL on Windows, so on Linux)
    let lib_path = config.output_dir.join("lib");
    fs::create_dir_all(&lib_path)?;
    for p in config.platforms.iter() {
        let lib_filename = p.lib_filename(&config.ffi_name);
        let destination_path = lib_path.join(p.platform.to_string());
        fs::create_dir_all(&destination_path)?;
        fs::copy(p.location.join(&lib_filename), destination_path.join(&lib_filename))?;

        // Copy DLL on Windows
        let bin_filename = p.bin_filename(&config.ffi_name);
        let destination_path = lib_path.join(p.platform.to_string());
        fs::create_dir_all(&destination_path)?;
        fs::copy(p.location.join(&bin_filename), destination_path.join(&bin_filename))?;
    }

    Ok(())
}

pub fn generate_c_header<P: AsRef<Path>>(lib: &Library, path: P) -> FormattingResult<()> {
    let uppercase_name = lib.name.to_uppercase();

    // Open file
    fs::create_dir_all(&path)?;
    let filename = path.as_ref().join(format!("{}.h", lib.name));
    let mut f = FilePrinter::new(filename)?;

    // Print license
    commented(&mut f, |f| {
        for line in lib.license.iter() {
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
        f.writeln(&format!("#define {}_VERSION_MAJOR {}", uppercase_name, lib.version.major))?;
        f.writeln(&format!("#define {}_VERSION_MINOR {}", uppercase_name, lib.version.minor))?;
        f.writeln(&format!("#define {}_VERSION_PATCH {}", uppercase_name, lib.version.patch))?;
        f.writeln(&format!("#define {}_VERSION_STRING \"{}\"", uppercase_name, lib.version.to_string()))?;
        f.newline()?;

        // Standard includes needed
        f.writeln("#include <stdbool.h>")?;
        f.writeln("#include <stdint.h>")?;
        f.newline()?;

        // Iterate through each statement and print them
        for statement in lib.into_iter() {
            match statement {
                Statement::NativeStructDeclaration(handle) => {
                    f.writeln(&format!("typedef struct {} {};", handle.name, handle.name))?;
                },
                Statement::NativeStructDefinition(handle) => write_struct_definition(f, handle)?,
                Statement::EnumDefinition(handle) => write_enum_definition(f, handle)?,
                Statement::ClassDeclaration(handle) => {
                    f.writeln(&format!("typedef struct {} {};", handle.name, handle.name))?;
                }
                Statement::NativeFunctionDeclaration(handle) => write_function(f, handle)?,
                Statement::InterfaceDefinition(handle) => write_interface(f, handle)?,
                _ => (),
            }
            f.newline()?;
        }

        Ok(())
    })
}

fn write_struct_definition(f: &mut dyn Printer, handle: &NativeStructHandle) -> FormattingResult<()> {
    f.writeln(&format!("typedef struct {}", handle.name()))?;
    f.writeln("{")?;
    indented(f, |f| {
        for element in &handle.elements {
            f.writeln(&format!("{} {};", CType(&element.element_type), element.name))?;
        }
        Ok(())
    })?;
    f.writeln(&format!("}} {};", handle.name()))
}

fn write_enum_definition(f: &mut dyn Printer, handle: &NativeEnumHandle) -> FormattingResult<()> {
    f.writeln(&format!("typedef enum {}", handle.name))?;
    f.writeln("{")?;
    indented(f, |f| {
        for variant in &handle.variants {
            f.writeln(&format!("{}_{} = {},", handle.name, variant.name, variant.value))?;
        }
        Ok(())
    })?;
    f.writeln(&format!("}} {};", handle.name))
}

fn write_function(f: &mut dyn Printer, handle: &NativeFunctionHandle) -> FormattingResult<()> {
    f.newline()?;
    f.write(&format!("{} {}(", CReturnType(&handle.return_type), handle.name))?;
    
    f.write(
        &handle.parameters.iter()
            .map(|param| format!("{} {}", CType(&param.param_type), param.name))
            .collect::<Vec<String>>()
            .join(", ")
    )?;

    f.write(");")
}

fn write_interface(f: &mut dyn Printer, handle: &Interface) -> FormattingResult<()> {
    f.writeln(&format!("typedef struct {}", handle.name))?;
    f.writeln("{")?;
    indented(f, |f| {
        for element in &handle.elements {
            match element {
                InterfaceElement::Arg(name) => f.writeln(&format!("void* {};", name))?,
                InterfaceElement::CallbackFunction(handle) => {
                    f.newline()?;
                    f.write(&format!("{} (*{})(", CReturnType(&handle.return_type), handle.name))?;
                    
                    f.write(
                        &handle.parameters.iter()
                            .map(|param| {
                                match param {
                                    CallbackParameter::Arg(_) => "void*".to_string(),
                                    CallbackParameter::Parameter(param) => format!("{}", CType(&param.param_type)),
                                }
                            })
                            .collect::<Vec<String>>()
                            .join(", ")
                    )?;

                    f.write(");")?;
                },
                InterfaceElement::DestroyFunction(name) => {
                    f.writeln(&format!("void (*{})(void* arg);", name))?;
                }
            }
        }
        Ok(())
    })?;
    f.writeln(&format!("}} {};", handle.name))
}

fn generate_cmake_config(lib: &Library, config: &CBindgenConfig) -> FormattingResult<()> {
    // Create file
    let cmake_path = config.output_dir.join("cmake");
    fs::create_dir_all(&cmake_path)?;
    let filename = cmake_path.join(format!("{}-config.cmake", lib.name));
    let mut f = FilePrinter::new(filename)?;

    // Prefix used everywhere else
    f.writeln("set(prefix \"${CMAKE_CURRENT_LIST_DIR}/../\")")?;
    f.newline()?;

    // Write each platform variation
    let mut is_first_if = true;
    for p in config.platforms.iter() {
        let platform_check = match p.platform {
            Platform::Win64 => "WIN32 AND CMAKE_SIZEOF_VOID_P EQUAL 8",
            Platform::Win32 => "WIN32 AND CMAKE_SIZEOF_VOID_P EQUAL 4",
            Platform::Linux => "UNIX AND CMAKE_SIZEOF_VOID_P EQUAL 8",
        };

        if is_first_if {
            f.writeln(&format!("if({})", platform_check))?;
            is_first_if = false;
        } else {
            f.writeln(&format!("elseif({})", platform_check))?;
        }

        indented(&mut f, |f| {
            f.writeln(&format!("add_library({} SHARED IMPORTED GLOBAL)", lib.name))?;
            f.writeln(&format!("set_target_properties({} PROPERTIES", lib.name))?;
            indented(f, |f| {
                f.writeln(&format!("IMPORTED_LOCATION \"${{prefix}}/lib/{}/{}\"", p.platform.to_string(), p.bin_filename(&config.ffi_name)))?;
                f.writeln(&format!("IMPORTED_IMPLIB \"${{prefix}}/lib/{}/{}\"", p.platform.to_string(), p.lib_filename(&config.ffi_name)))?;
                f.writeln("INTERFACE_INCLUDE_DIRECTORIES \"${prefix}/include\"")
            })?;
            f.writeln(")")
        })?;
    }

    // Write error message if platform not found
    f.writeln("else()")?;
    indented(&mut f, |f| {
        f.writeln("message(FATAL_ERROR \"Platform not supported\")")
    })?;
    f.writeln("endif()")
}

struct CReturnType<'a>(&'a ReturnType);

impl <'a> Display for CReturnType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.0 {
            ReturnType::Void => write!(f, "void"),
            ReturnType::Type(return_type) => write!(f, "{}", CType(&return_type)),
        }
    }
}

struct CType<'a>(&'a Type);

impl<'a> Display for CType<'a> {
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
            Type::String => write!(f, "char*"),
            Type::Struct(handle) => write!(f, "{}", handle.name()),
            Type::StructRef(handle) => write!(f, "{}*", handle.name),
            Type::Enum(handle) => write!(f, "{}", handle.name),
            Type::ClassRef(handle) => write!(f, "{}*", handle.name),
            Type::Interface(handle) => write!(f, "{}", handle.name),
            Type::Duration(mapping) => match mapping {
                DurationMapping::Milliseconds|DurationMapping::Seconds => write!(f, "uint64_t"),
                DurationMapping::SecondsFloat => write!(f, "float"),
            }
        }
    }
}
