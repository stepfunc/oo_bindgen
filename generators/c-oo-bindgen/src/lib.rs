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
    safe_packed_borrows,
    while_true,
    bare_trait_objects
)]

use crate::doc::*;
use crate::formatting::*;
use heck::{CamelCase, ShoutySnakeCase, SnakeCase};
use oo_bindgen::callback::*;
use oo_bindgen::class::*;
use oo_bindgen::constants::{ConstantSetHandle, ConstantValue, Representation};
use oo_bindgen::doc::*;
use oo_bindgen::formatting::*;
use oo_bindgen::native_enum::*;
use oo_bindgen::native_function::*;
use oo_bindgen::native_struct::*;
use oo_bindgen::platforms::*;
use oo_bindgen::*;
use std::fmt::Display;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

mod doc;
mod formatting;

trait CFormatting {
    fn to_c_type(&self) -> String;
}

impl CFormatting for NativeStructDeclarationHandle {
    fn to_c_type(&self) -> String {
        format!("{}_t", self.name.to_snake_case())
    }
}

impl CFormatting for NativeStructHandle {
    fn to_c_type(&self) -> String {
        format!("{}_t", self.name().to_snake_case())
    }
}

impl CFormatting for NativeEnumHandle {
    fn to_c_type(&self) -> String {
        format!("{}_t", self.name.to_snake_case())
    }
}

impl CFormatting for ClassDeclarationHandle {
    fn to_c_type(&self) -> String {
        format!("{}_t", self.name.to_snake_case())
    }
}

impl CFormatting for Interface {
    fn to_c_type(&self) -> String {
        format!("{}_t", self.name.to_snake_case())
    }
}

impl CFormatting for OneTimeCallbackHandle {
    fn to_c_type(&self) -> String {
        format!("{}_t", self.name.to_snake_case())
    }
}

impl CFormatting for Symbol {
    fn to_c_type(&self) -> String {
        match self {
            Symbol::NativeFunction(handle) => handle.name.to_owned(),
            Symbol::Struct(handle) => handle.declaration().to_c_type(),
            Symbol::Enum(handle) => handle.to_c_type(),
            Symbol::Class(handle) => handle.declaration().to_c_type(),
            Symbol::StaticClass(_) => panic!("static classes cannot be referenced in C"),
            Symbol::Interface(handle) => handle.to_c_type(),
            Symbol::OneTimeCallback(handle) => handle.to_c_type(),
            Symbol::Iterator(handle) => handle.iter_type.to_c_type(),
            Symbol::Collection(handle) => handle.collection_type.to_c_type(),
        }
    }
}

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
        fs::copy(
            p.location.join(&lib_filename),
            destination_path.join(&lib_filename),
        )?;

        // Copy DLL on Windows
        let bin_filename = p.bin_filename(&config.ffi_name);
        let destination_path = lib_path.join(p.platform.to_string());
        fs::create_dir_all(&destination_path)?;
        fs::copy(
            p.location.join(&bin_filename),
            destination_path.join(&bin_filename),
        )?;
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
            .write_all(&format!("PROJECT_NAME = {}\n", lib.name).into_bytes())
            .unwrap();
        stdin
            .write_all(&format!("PROJECT_NUMBER = {}\n", lib.version.to_string()).into_bytes())
            .unwrap();
        stdin.write_all(b"HTML_OUTPUT = doc\n").unwrap();
        stdin.write_all(b"GENERATE_LATEX = NO\n").unwrap();
        stdin.write_all(b"INPUT = include\n").unwrap();
    }

    command.wait()?;

    Ok(())
}

fn generate_c_header<P: AsRef<Path>>(lib: &Library, path: P) -> FormattingResult<()> {
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
        for statement in lib.into_iter() {
            match statement {
                Statement::Constants(handle) => write_constants_definition(f, handle, lib)?,
                Statement::NativeStructDeclaration(handle) => {
                    f.writeln(&format!(
                        "typedef struct {} {};",
                        handle.to_c_type(),
                        handle.to_c_type()
                    ))?;
                }
                Statement::NativeStructDefinition(handle) => {
                    write_struct_definition(f, handle, lib)?
                }
                Statement::EnumDefinition(handle) => write_enum_definition(f, handle, lib)?,
                Statement::ClassDeclaration(handle) => write_class_declaration(f, handle, lib)?,
                Statement::NativeFunctionDeclaration(handle) => write_function(f, handle, lib)?,
                Statement::InterfaceDefinition(handle) => write_interface(f, handle, lib)?,
                Statement::OneTimeCallbackDefinition(handle) => {
                    write_one_time_callback(f, handle, lib)?
                }
                _ => (),
            }
            f.newline()?;
        }

        Ok(())
    })
}

fn write_constants_definition(
    f: &mut dyn Printer,
    handle: &ConstantSetHandle,
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
            "#define {}_{} {}",
            handle.name.to_shouty_snake_case(),
            item.name.to_shouty_snake_case(),
            get_constant_value(item.value)
        ))?;
    }
    Ok(())
}

fn write_struct_definition(
    f: &mut dyn Printer,
    handle: &NativeStructHandle,
    lib: &Library,
) -> FormattingResult<()> {
    let doc = match handle.struct_type {
        NativeStructType::Public => handle.doc.clone(),
        NativeStructType::Opaque => handle
            .doc
            .clone()
            .warning("This struct should never be initialized or modified by user code"),
    };

    doxygen(f, |f| doxygen_print(f, &doc, lib))?;

    // Write the struct definition
    f.writeln(&format!("typedef struct {}", handle.to_c_type()))?;
    f.writeln("{")?;
    indented(f, |f| {
        for element in &handle.elements {
            doxygen(f, |f| {
                doxygen_print(f, &element.doc, lib)?;

                let default_value = match &element.element_type {
                    StructElementType::Bool(default) => default.map(|x| x.to_string()),
                    StructElementType::Uint8(default) => default.map(|x| x.to_string()),
                    StructElementType::Sint8(default) => default.map(|x| x.to_string()),
                    StructElementType::Uint16(default) => default.map(|x| x.to_string()),
                    StructElementType::Sint16(default) => default.map(|x| x.to_string()),
                    StructElementType::Uint32(default) => default.map(|x| x.to_string()),
                    StructElementType::Sint32(default) => default.map(|x| x.to_string()),
                    StructElementType::Uint64(default) => default.map(|x| x.to_string()),
                    StructElementType::Sint64(default) => default.map(|x| x.to_string()),
                    StructElementType::Float(default) => default.map(|x| x.to_string()),
                    StructElementType::Double(default) => default.map(|x| x.to_string()),
                    StructElementType::String(default) => {
                        default.clone().map(|x| format!("\"{}\"", x))
                    }
                    StructElementType::Struct(_) => None,
                    StructElementType::StructRef(_) => None,
                    StructElementType::Enum(handle, default) => default.clone().map(|x| {
                        format!("@ref {}_{}", handle.name.to_camel_case(), x.to_camel_case())
                    }),
                    StructElementType::ClassRef(_) => None,
                    StructElementType::Interface(_) => None,
                    StructElementType::OneTimeCallback(_) => None,
                    StructElementType::Iterator(_) => None,
                    StructElementType::Collection(_) => None,
                    StructElementType::Duration(mapping, default) => {
                        default.map(|x| mapping.get_value_string(x))
                    }
                };

                if let Some(default_value) = default_value {
                    f.writeln(&format!("@note Default value is {}", default_value))?;
                }

                if let StructElementType::Duration(mapping, _) = &element.element_type {
                    f.writeln(&format!("@note The unit is {}", mapping.unit()))?;
                }

                Ok(())
            })?;
            f.writeln(&format!(
                "{} {};",
                CType(&element.element_type.to_type()),
                element.name.to_snake_case(),
            ))?;
        }
        Ok(())
    })?;
    f.writeln(&format!("}} {};", handle.to_c_type()))?;

    // user should never try to initialize opaque structs, so don't suggest this is OK
    if handle.struct_type != NativeStructType::Opaque {
        f.newline()?;
        write_struct_initializer(f, handle)?;
    }

    Ok(())
}

fn write_struct_initializer(
    f: &mut dyn Printer,
    handle: &NativeStructHandle,
) -> FormattingResult<()> {
    let params = handle
        .elements()
        .filter(|el| !el.element_type.has_default())
        .map(|el| {
            format!(
                "{} {}",
                CType(&el.element_type.to_type()),
                el.name.to_snake_case()
            )
        })
        .collect::<Vec<String>>()
        .join(", ");

    f.writeln(&format!(
        "static {} {}_init({})",
        handle.to_c_type(),
        handle.name().to_snake_case(),
        params
    ))?;
    blocked(f, |f| {
        f.writeln(&format!("return ({})", handle.to_c_type()))?;
        f.writeln("{")?;
        indented(f, |f| {
            for el in handle.elements() {
                let value = match &el.element_type {
                    StructElementType::Bool(default) => match default {
                        None => el.name.to_snake_case(),
                        Some(false) => "false".to_string(),
                        Some(true) => "true".to_string(),
                    },
                    StructElementType::Uint8(default) => match default {
                        None => el.name.to_snake_case(),
                        Some(value) => value.to_string(),
                    },
                    StructElementType::Sint8(default) => match default {
                        None => el.name.to_snake_case(),
                        Some(value) => value.to_string(),
                    },
                    StructElementType::Uint16(default) => match default {
                        None => el.name.to_snake_case(),
                        Some(value) => value.to_string(),
                    },
                    StructElementType::Sint16(default) => match default {
                        None => el.name.to_snake_case(),
                        Some(value) => value.to_string(),
                    },
                    StructElementType::Uint32(default) => match default {
                        None => el.name.to_snake_case(),
                        Some(value) => value.to_string(),
                    },
                    StructElementType::Sint32(default) => match default {
                        None => el.name.to_snake_case(),
                        Some(value) => value.to_string(),
                    },
                    StructElementType::Uint64(default) => match default {
                        None => el.name.to_snake_case(),
                        Some(value) => value.to_string(),
                    },
                    StructElementType::Sint64(default) => match default {
                        None => el.name.to_snake_case(),
                        Some(value) => value.to_string(),
                    },
                    StructElementType::Float(default) => match default {
                        None => el.name.to_snake_case(),
                        Some(value) => format!("{:.}f", value),
                    },
                    StructElementType::Double(default) => match default {
                        None => el.name.to_snake_case(),
                        Some(value) => format!("{:.}", value),
                    },
                    StructElementType::String(default) => match default {
                        None => el.name.to_snake_case(),
                        Some(value) => format!("\"{}\"", value),
                    },
                    StructElementType::Struct(handle) => {
                        if handle.is_default_constructed() {
                            format!("{}_init()", handle.name().to_snake_case())
                        } else {
                            el.name.to_snake_case()
                        }
                    }
                    StructElementType::StructRef(_) => el.name.to_snake_case(),
                    StructElementType::Enum(handle, default) => match default {
                        None => el.name.to_snake_case(),
                        Some(value) => match handle.find_variant_by_name(value) {
                            Some(variant) => format!(
                                "{}_{}",
                                handle.name.to_camel_case(),
                                variant.name.to_camel_case()
                            ),
                            None => panic!("Variant {} not found in {}", value, handle.name),
                        },
                    },
                    StructElementType::ClassRef(_) => el.name.to_snake_case(),
                    StructElementType::Interface(_) => el.name.to_snake_case(),
                    StructElementType::OneTimeCallback(_) => el.name.to_snake_case(),
                    StructElementType::Iterator(_) => el.name.to_snake_case(),
                    StructElementType::Collection(_) => el.name.to_snake_case(),
                    StructElementType::Duration(mapping, default) => match default {
                        None => el.name.to_snake_case(),
                        Some(value) => match mapping {
                            DurationMapping::Milliseconds => value.as_millis().to_string(),
                            DurationMapping::Seconds => value.as_secs().to_string(),
                            DurationMapping::SecondsFloat => value.as_secs_f32().to_string(),
                        },
                    },
                };
                f.writeln(&format!(".{} = {},", el.name.to_snake_case(), value))?;
            }
            Ok(())
        })?;
        f.writeln("};")
    })
}

fn write_enum_definition(
    f: &mut dyn Printer,
    handle: &NativeEnumHandle,
    lib: &Library,
) -> FormattingResult<()> {
    doxygen(f, |f| doxygen_print(f, &handle.doc, lib))?;

    f.writeln(&format!("typedef enum {}", handle.to_c_type()))?;
    f.writeln("{")?;
    indented(f, |f| {
        for variant in &handle.variants {
            doxygen(f, |f| doxygen_print(f, &variant.doc, lib))?;
            f.writeln(&format!(
                "{}_{} = {},",
                handle.name.to_camel_case(),
                variant.name.to_camel_case(),
                variant.value
            ))?;
        }
        Ok(())
    })?;
    f.writeln(&format!("}} {};", handle.to_c_type()))?;

    f.newline()?;

    f.writeln(&format!(
        "static const char* {}_to_string({} value)",
        handle.name,
        handle.to_c_type()
    ))?;
    blocked(f, |f| {
        f.writeln("switch (value)")?;
        blocked(f, |f| {
            for variant in &handle.variants {
                f.writeln(&format!(
                    "case {}_{}: return \"{}\";",
                    handle.name.to_camel_case(),
                    variant.name.to_camel_case(),
                    variant.name
                ))?;
            }
            f.writeln("default: return \"\";")
        })
    })
}

fn write_class_declaration(
    f: &mut dyn Printer,
    handle: &ClassDeclarationHandle,
    lib: &Library,
) -> FormattingResult<()> {
    match lib.symbol(&handle.name) {
        Some(Symbol::Class(handle)) => doxygen(f, |f| doxygen_print(f, &handle.doc, lib))?,
        Some(Symbol::Iterator(handle)) => doxygen(f, |f| {
            doxygen_print(
                f,
                &Doc::from(&*format!(
                    "Iterator of {{struct:{}}}. See @ref {}.",
                    handle.item_type.name(),
                    handle.native_func.name
                )),
                lib,
            )
        })?,
        Some(Symbol::Collection(handle)) => doxygen(f, |f| {
            doxygen_print(
                f,
                &Doc::from(&*format!(
                    "Collection of {}. See @ref {} and @ref {}.",
                    CType(&handle.item_type).to_string(),
                    handle.add_func.name,
                    handle.delete_func.name
                )),
                lib,
            )
        })?,
        _ => (),
    }

    f.writeln(&format!(
        "typedef struct {} {};",
        handle.to_c_type(),
        handle.to_c_type()
    ))
}

fn write_function_docs(
    f: &mut dyn Printer,
    handle: &NativeFunctionHandle,
    lib: &Library,
) -> FormattingResult<()> {
    doxygen(f, |f| {
        // Print top-level documentation
        doxygen_print(f, &handle.doc, lib)?;

        // Print each parameter value
        for param in &handle.parameters {
            f.writeln(&format!("@param {} ", param.name))?;
            docstring_print(f, &param.doc, lib)?;
            if let Type::Duration(mapping) = param.param_type {
                f.write(&format!(" ({})", mapping.unit()))?;
            }
        }

        fn write_error_return_doc(f: &mut dyn Printer) -> FormattingResult<()> {
            f.writeln("@return Error code")
        }

        match handle.get_type() {
            NativeFunctionType::NoErrorNoReturn => {}
            NativeFunctionType::NoErrorWithReturn(ret, doc) => {
                f.writeln("@return ")?;
                docstring_print(f, &doc, lib)?;
                if let Type::Duration(mapping) = ret {
                    f.write(&format!(" ({})", mapping.unit()))?;
                }
            }
            NativeFunctionType::ErrorNoReturn(_) => {
                write_error_return_doc(f)?;
            }
            NativeFunctionType::ErrorWithReturn(_, ret, doc) => {
                f.writeln("@param out ")?;
                docstring_print(f, &doc, lib)?;
                if let Type::Duration(mapping) = ret {
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
    handle: &NativeFunctionHandle,
    lib: &Library,
) -> FormattingResult<()> {
    write_function_docs(f, handle, lib)?;

    if let Some(error_type) = &handle.error_type {
        f.writeln(&format!(
            "{} {}(",
            CType(&Type::Enum(error_type.inner.clone())),
            handle.name
        ))?;
    } else {
        f.writeln(&format!(
            "{} {}(",
            CReturnType(&handle.return_type),
            handle.name
        ))?;
    }

    f.write(
        &handle
            .parameters
            .iter()
            .map(|param| {
                format!(
                    "{} {}",
                    CType(&param.param_type),
                    param.name.to_snake_case()
                )
            })
            .collect::<Vec<String>>()
            .join(", "),
    )?;

    if handle.error_type.is_some() {
        if let ReturnType::Type(x, _) = &handle.return_type {
            if !handle.parameters.is_empty() {
                f.write(", ")?;
                f.write(&format!("{}* out", CType(x)))?;
            }
        }
    }

    f.write(");")
}

fn write_interface(f: &mut dyn Printer, handle: &Interface, lib: &Library) -> FormattingResult<()> {
    doxygen(f, |f| doxygen_print(f, &handle.doc, lib))?;

    f.writeln(&format!("typedef struct {}", handle.to_c_type()))?;
    f.writeln("{")?;
    indented(f, |f| {
        for element in &handle.elements {
            match element {
                InterfaceElement::Arg(name) => {
                    doxygen(f, |f| f.writeln("@brief Context data"))?;
                    f.writeln(&format!("void* {};", name))?
                }
                InterfaceElement::CallbackFunction(handle) => {
                    f.newline()?;

                    // Print the documentation
                    doxygen(f, |f| {
                        // Print top-level documentation
                        doxygen_print(f, &handle.doc, lib)?;

                        // Print each parameter value
                        for param in &handle.parameters {
                            match param {
                                CallbackParameter::Arg(name) => {
                                    f.writeln(&format!("@param {} ", name))?;
                                    docstring_print(f, &"Context data".into(), lib)?;
                                }
                                CallbackParameter::Parameter(param) => {
                                    f.writeln(&format!("@param {} ", param.name))?;
                                    docstring_print(f, &param.doc, lib)?;
                                }
                            }
                        }

                        // Print return documentation
                        if let ReturnType::Type(_, doc) = &handle.return_type {
                            f.writeln("@return ")?;
                            docstring_print(f, doc, lib)?;
                        }

                        Ok(())
                    })?;

                    f.newline()?;

                    // Print function signature
                    f.write(&format!(
                        "{} (*{})(",
                        CReturnType(&handle.return_type),
                        handle.name.to_snake_case(),
                    ))?;

                    f.write(
                        &handle
                            .parameters
                            .iter()
                            .map(|param| match param {
                                CallbackParameter::Arg(_) => "void*".to_string(),
                                CallbackParameter::Parameter(param) => {
                                    format!("{}", CType(&param.param_type))
                                }
                            })
                            .collect::<Vec<String>>()
                            .join(", "),
                    )?;

                    f.write(");")?;
                }
                InterfaceElement::DestroyFunction(name) => {
                    doxygen(f, |f| {
                        f.writeln("@brief Callback when the underlying owner doesn't need the callback anymore")?;
                        f.writeln("@param arg Context data")
                    })?;
                    f.writeln(&format!("void (*{})(void* arg);", name))?;
                }
            }
        }
        Ok(())
    })?;
    f.writeln(&format!("}} {};", handle.to_c_type()))
}

fn write_one_time_callback(
    f: &mut dyn Printer,
    handle: &OneTimeCallbackHandle,
    lib: &Library,
) -> FormattingResult<()> {
    doxygen(f, |f| doxygen_print(f, &handle.doc, lib))?;

    f.writeln(&format!("typedef struct {}", handle.to_c_type()))?;
    f.writeln("{")?;
    indented(f, |f| {
        for element in &handle.elements {
            match element {
                OneTimeCallbackElement::Arg(name) => {
                    doxygen(f, |f| f.writeln("@brief Context data"))?;
                    f.writeln(&format!("void* {};", name))?
                }
                OneTimeCallbackElement::CallbackFunction(handle) => {
                    f.newline()?;

                    // Print the documentation
                    doxygen(f, |f| {
                        // Print top-level documentation
                        doxygen_print(f, &handle.doc, lib)?;

                        // Print each parameter value
                        for param in &handle.parameters {
                            match param {
                                CallbackParameter::Arg(name) => {
                                    f.writeln(&format!("@param {} ", name))?;
                                    docstring_print(f, &"Context data".into(), lib)?;
                                }
                                CallbackParameter::Parameter(param) => {
                                    f.writeln(&format!("@param {} ", param.name))?;
                                    docstring_print(f, &param.doc, lib)?;
                                }
                            }
                        }

                        // Print return documentation
                        if let ReturnType::Type(_, doc) = &handle.return_type {
                            f.writeln("@return ")?;
                            docstring_print(f, doc, lib)?;
                        }

                        Ok(())
                    })?;

                    f.newline()?;

                    // Print function signature
                    f.write(&format!(
                        "{} (*{})(",
                        CReturnType(&handle.return_type),
                        handle.name.to_snake_case(),
                    ))?;

                    f.write(
                        &handle
                            .parameters
                            .iter()
                            .map(|param| match param {
                                CallbackParameter::Arg(_) => "void*".to_string(),
                                CallbackParameter::Parameter(param) => {
                                    format!("{}", CType(&param.param_type))
                                }
                            })
                            .collect::<Vec<String>>()
                            .join(", "),
                    )?;

                    f.write(");")?;
                }
            }
        }
        Ok(())
    })?;
    f.writeln(&format!("}} {};", handle.to_c_type()))
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

    let mut is_first_if = true;

    // Windows platform
    if let Some(p) = &config.platforms.win64 {
        f.writeln("if(WIN32 AND CMAKE_SIZEOF_VOID_P EQUAL 8)")?;
        is_first_if = false;

        indented(&mut f, |f| {
            f.writeln(&format!("add_library({} SHARED IMPORTED GLOBAL)", lib.name))?;
            f.writeln(&format!("set_target_properties({} PROPERTIES", lib.name))?;
            indented(f, |f| {
                f.writeln(&format!(
                    "IMPORTED_LOCATION \"${{prefix}}/lib/{}/{}\"",
                    p.platform.to_string(),
                    p.bin_filename(&config.ffi_name)
                ))?;
                f.writeln(&format!(
                    "IMPORTED_IMPLIB \"${{prefix}}/lib/{}/{}\"",
                    p.platform.to_string(),
                    p.lib_filename(&config.ffi_name)
                ))?;
                f.writeln("INTERFACE_INCLUDE_DIRECTORIES \"${prefix}/include\"")
            })?;
            f.writeln(")")
        })?;
    }

    const LINUX_PLATFORM_CHECK: &str = "UNIX AND CMAKE_SIZEOF_VOID_P EQUAL 8";

    // Linux dynamic lib
    if config.platforms.linux.is_some() || config.platforms.linux_musl.is_some() {
        if is_first_if {
            f.writeln(&format!("if({})", LINUX_PLATFORM_CHECK))?;
            //is_first_if = false;
        } else {
            f.writeln(&format!("elseif({})", LINUX_PLATFORM_CHECK))?;
        }

        if let Some(p) = &config.platforms.linux {
            indented(&mut f, |f| {
                f.writeln(&format!("add_library({} SHARED IMPORTED GLOBAL)", lib.name))?;
                f.writeln(&format!("set_target_properties({} PROPERTIES", lib.name))?;
                indented(f, |f| {
                    f.writeln(&format!(
                        "IMPORTED_LOCATION \"${{prefix}}/lib/{}/{}\"",
                        p.platform.to_string(),
                        p.bin_filename(&config.ffi_name)
                    ))?;
                    f.writeln("INTERFACE_INCLUDE_DIRECTORIES \"${prefix}/include\"")
                })?;
                f.writeln(")")
            })?;
        }

        if let Some(p) = &config.platforms.linux_musl {
            indented(&mut f, |f| {
                f.writeln(&format!(
                    "add_library({}_static STATIC IMPORTED GLOBAL)",
                    lib.name
                ))?;
                f.writeln(&format!(
                    "set_target_properties({}_static PROPERTIES",
                    lib.name
                ))?;
                indented(f, |f| {
                    f.writeln(&format!(
                        "IMPORTED_LOCATION \"${{prefix}}/lib/{}/{}\"",
                        p.platform.to_string(),
                        p.bin_filename(&config.ffi_name)
                    ))?;
                    f.writeln("INTERFACE_INCLUDE_DIRECTORIES \"${prefix}/include\"")
                })?;
                f.writeln(")")
            })?;
        }
    }

    // Write error message if platform not found
    f.writeln("else()")?;
    indented(&mut f, |f| {
        f.writeln("message(FATAL_ERROR \"Platform not supported\")")
    })?;
    f.writeln("endif()")
}

struct CReturnType<'a>(&'a ReturnType);

impl<'a> Display for CReturnType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.0 {
            ReturnType::Void => write!(f, "void"),
            ReturnType::Type(return_type, _) => write!(f, "{}", CType(&return_type)),
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
            Type::String => write!(f, "const char*"),
            Type::Struct(handle) => write!(f, "{}", handle.to_c_type()),
            Type::StructRef(handle) => write!(f, "{}*", handle.to_c_type()),
            Type::Enum(handle) => write!(f, "{}", handle.to_c_type()),
            Type::ClassRef(handle) => write!(f, "{}*", handle.to_c_type()),
            Type::Interface(handle) => write!(f, "{}", handle.to_c_type()),
            Type::OneTimeCallback(handle) => write!(f, "{}", handle.to_c_type()),
            Type::Iterator(handle) => write!(f, "{}*", handle.iter_type.to_c_type()),
            Type::Collection(handle) => write!(f, "{}*", handle.collection_type.to_c_type()),
            Type::Duration(mapping) => match mapping {
                DurationMapping::Milliseconds | DurationMapping::Seconds => write!(f, "uint64_t"),
                DurationMapping::SecondsFloat => write!(f, "float"),
            },
        }
    }
}
