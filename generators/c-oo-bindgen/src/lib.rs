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
use heck::{ShoutySnakeCase, SnakeCase};
use oo_bindgen::callback::*;
use oo_bindgen::class::*;
use oo_bindgen::constants::{ConstantSetHandle, ConstantValue, Representation};
use oo_bindgen::doc::*;
use oo_bindgen::formatting::*;
use oo_bindgen::native_enum::*;
use oo_bindgen::native_function::*;
use oo_bindgen::any_struct::*;
use oo_bindgen::platforms::*;
use oo_bindgen::struct_common::{StructDeclarationHandle, Visibility};
use oo_bindgen::types::{AnyType, BasicType, DurationType};
use oo_bindgen::*;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

mod chelpers;
mod cpp;
mod doc;
mod formatting;

trait CFormatting {
    fn to_c_type(&self, prefix: &str) -> String;
}

impl CFormatting for StructType {
    fn to_c_type(&self, prefix: &str) -> String {
        match self {
            StructType::Any(x) => x.to_c_type(prefix),
            StructType::FStruct(_, x) => x.to_c_type(prefix),
        }
    }
}

impl CFormatting for StructDeclarationHandle {
    fn to_c_type(&self, prefix: &str) -> String {
        format!("{}_{}_t", prefix.to_snake_case(), self.name.to_snake_case())
    }
}

impl CFormatting for AnyStructHandle {
    fn to_c_type(&self, prefix: &str) -> String {
        format!(
            "{}_{}_t",
            prefix.to_snake_case(),
            self.name().to_snake_case()
        )
    }
}

impl CFormatting for EnumHandle {
    fn to_c_type(&self, prefix: &str) -> String {
        format!("{}_{}_t", prefix.to_snake_case(), self.name.to_snake_case())
    }
}

impl CFormatting for ClassDeclarationHandle {
    fn to_c_type(&self, prefix: &str) -> String {
        format!("{}_{}_t", prefix.to_snake_case(), self.name.to_snake_case())
    }
}

impl CFormatting for Interface {
    fn to_c_type(&self, prefix: &str) -> String {
        format!("{}_{}_t", prefix.to_snake_case(), self.name.to_snake_case())
    }
}

impl CFormatting for Symbol {
    fn to_c_type(&self, prefix: &str) -> String {
        match self {
            Symbol::Function(handle) => format!("{}_{}", prefix.to_snake_case(), handle.name),
            Symbol::Struct(handle) => handle.declaration().to_c_type(prefix),
            Symbol::Enum(handle) => handle.to_c_type(prefix),
            Symbol::Class(handle) => handle.declaration().to_c_type(prefix),
            Symbol::StaticClass(_) => panic!("static classes cannot be referenced in C"),
            Symbol::Interface(handle) => handle.to_c_type(prefix),
            Symbol::Iterator(handle) => handle.iter_type.to_c_type(prefix),
            Symbol::Collection(handle) => handle.collection_type.to_c_type(prefix),
        }
    }
}

impl CFormatting for BasicType {
    fn to_c_type(&self, prefix: &str) -> String {
        match self {
            Self::Bool => "bool".to_string(),
            Self::Uint8 => "uint8_t".to_string(),
            Self::Sint8 => "int8_t".to_string(),
            Self::Uint16 => "uint16_t".to_string(),
            Self::Sint16 => "int16_t".to_string(),
            Self::Uint32 => "uint32_t".to_string(),
            Self::Sint32 => "int32_t".to_string(),
            Self::Uint64 => "uint64_t".to_string(),
            Self::Sint64 => "int64_t".to_string(),
            Self::Float => "float".to_string(),
            Self::Double => "double".to_string(),
            Self::Duration(_) => "uint64_t".to_string(),
            Self::Enum(handle) => handle.to_c_type(prefix),
        }
    }
}

impl CFormatting for AnyType {
    fn to_c_type(&self, prefix: &str) -> String {
        match self {
            AnyType::Basic(b) => b.to_c_type(prefix),
            AnyType::String => "const char*".to_string(),
            AnyType::Struct(handle) => handle.to_c_type(prefix),
            AnyType::StructRef(handle) => format!("{}*", handle.to_c_type(prefix)),
            AnyType::ClassRef(handle) => format!("{}*", handle.to_c_type(prefix)),
            AnyType::Interface(handle) => handle.to_c_type(prefix),
            AnyType::Iterator(handle) => format!("{}*", handle.iter_type.to_c_type(prefix)),
            AnyType::Collection(handle) => {
                format!("{}*", handle.collection_type.to_c_type(prefix))
            }
        }
    }
}

impl CFormatting for ReturnType {
    fn to_c_type(&self, prefix: &str) -> String {
        match self {
            ReturnType::Void => "void".to_string(),
            ReturnType::Type(return_type, _) => return_type.to_c_type(prefix),
        }
    }
}

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
    generate_c_header(lib, include_path.clone())?;

    // Create the C++ header
    crate::cpp::generate_cpp_header(lib, &include_path)?;
    crate::cpp::generate_cpp_impl(lib, &include_path)?;

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
            .write_all(&format!("PROJECT_NAME = {}\n", lib.name).into_bytes())
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

fn generate_c_header<P: AsRef<Path>>(lib: &Library, path: P) -> FormattingResult<()> {
    let uppercase_name = lib.c_ffi_prefix.to_uppercase();

    // Open file
    fs::create_dir_all(&path)?;
    let filename = path.as_ref().join(format!("{}.h", lib.name));
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
                    f.writeln(&format!(
                        "typedef struct {} {};",
                        handle.to_c_type(&lib.c_ffi_prefix),
                        handle.to_c_type(&lib.c_ffi_prefix)
                    ))?;
                }
                Statement::StructDefinition(handle) => {
                    write_struct_definition(f, handle.get_any_struct(), lib)?
                }
                Statement::EnumDefinition(handle) => write_enum_definition(f, handle, lib)?,
                Statement::ClassDeclaration(handle) => write_class_declaration(f, handle, lib)?,
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
            "#define {}_{}_{} {}",
            lib.c_ffi_prefix.to_shouty_snake_case(),
            handle.name.to_shouty_snake_case(),
            item.name.to_shouty_snake_case(),
            get_constant_value(item.value)
        ))?;
    }
    Ok(())
}

fn write_struct_definition(
    f: &mut dyn Printer,
    handle: &AnyStructHandle,
    lib: &Library,
) -> FormattingResult<()> {
    let doc = match handle.visibility {
        Visibility::Public => handle.doc.clone(),
        Visibility::Private => handle
            .doc
            .clone()
            .warning("This struct should never be initialized or modified by user code"),
    };

    doxygen(f, |f| doxygen_print(f, &doc, lib))?;

    // Write the struct definition
    f.writeln(&format!(
        "typedef struct {}",
        handle.to_c_type(&lib.c_ffi_prefix)
    ))?;
    f.writeln("{")?;
    indented(f, |f| {
        for element in &handle.fields {
            doxygen(f, |f| {
                doxygen_print(f, &element.doc, lib)?;

                let default_value = match &element.field_type {
                    AnyStructFieldType::Bool(default) => default.map(|x| x.to_string()),
                    AnyStructFieldType::Uint8(default) => default.map(|x| x.to_string()),
                    AnyStructFieldType::Sint8(default) => default.map(|x| x.to_string()),
                    AnyStructFieldType::Uint16(default) => default.map(|x| x.to_string()),
                    AnyStructFieldType::Sint16(default) => default.map(|x| x.to_string()),
                    AnyStructFieldType::Uint32(default) => default.map(|x| x.to_string()),
                    AnyStructFieldType::Sint32(default) => default.map(|x| x.to_string()),
                    AnyStructFieldType::Uint64(default) => default.map(|x| x.to_string()),
                    AnyStructFieldType::Sint64(default) => default.map(|x| x.to_string()),
                    AnyStructFieldType::Float(default) => default.map(|x| x.to_string()),
                    AnyStructFieldType::Double(default) => default.map(|x| x.to_string()),
                    AnyStructFieldType::String(default) => {
                        default.clone().map(|x| format!("\"{}\"", x))
                    }
                    AnyStructFieldType::Struct(_) => None,
                    AnyStructFieldType::StructRef(_) => None,
                    AnyStructFieldType::Enum(handle, default) => default.clone().map(|x| {
                        format!(
                            "@ref {}_{}_{}",
                            lib.c_ffi_prefix.to_shouty_snake_case(),
                            handle.name.to_shouty_snake_case(),
                            x.to_shouty_snake_case()
                        )
                    }),
                    AnyStructFieldType::ClassRef(_) => None,
                    AnyStructFieldType::Interface(_) => None,
                    AnyStructFieldType::Iterator(_) => None,
                    AnyStructFieldType::Collection(_) => None,
                    AnyStructFieldType::Duration(mapping, default) => {
                        default.map(|x| mapping.get_value_string(x))
                    }
                };

                if let Some(default_value) = default_value {
                    f.writeln(&format!("@note Default value is {}", default_value))?;
                }

                if let AnyStructFieldType::Duration(mapping, _) = &element.field_type {
                    f.writeln(&format!("@note The unit is {}", mapping.unit()))?;
                }

                Ok(())
            })?;
            f.writeln(&format!(
                "{} {};",
                element
                    .field_type
                    .to_all_types()
                    .to_c_type(&lib.c_ffi_prefix),
                element.name.to_snake_case(),
            ))?;
        }
        Ok(())
    })?;
    f.writeln(&format!("}} {};", handle.to_c_type(&lib.c_ffi_prefix)))?;

    // user should never try to initialize opaque structs, so don't suggest this is OK
    if handle.visibility != Visibility::Private {
        f.newline()?;
        write_struct_initializer(f, handle, lib)?;
    }

    Ok(())
}

fn write_struct_initializer(
    f: &mut dyn Printer,
    handle: &AnyStructHandle,
    lib: &Library,
) -> FormattingResult<()> {
    doxygen(f, |f| {
        f.writeln("@brief ")?;
        docstring_print(
            f,
            &format!("Initialize {{struct:{}}} to default values", handle.name()).into(),
            lib,
        )?;

        for param in handle.fields().filter(|el| !el.field_type.has_default()) {
            f.writeln(&format!("@param {} ", param.name.to_snake_case()))?;
            docstring_print(f, &param.doc.brief, lib)?;
        }

        f.writeln("@returns ")?;
        docstring_print(
            f,
            &format!("New instance of {{struct:{}}}", handle.name()).into(),
            lib,
        )?;

        Ok(())
    })?;

    let params = handle
        .fields()
        .filter(|el| !el.field_type.has_default())
        .map(|el| {
            format!(
                "{} {}",
                el.field_type.to_all_types().to_c_type(&lib.c_ffi_prefix),
                el.name.to_snake_case()
            )
        })
        .collect::<Vec<String>>()
        .join(", ");

    f.writeln(&format!(
        "static {} {}_{}_init({})",
        handle.to_c_type(&lib.c_ffi_prefix),
        &lib.c_ffi_prefix,
        handle.name().to_snake_case(),
        params
    ))?;
    blocked(f, |f| {
        f.writeln(&format!("return ({})", handle.to_c_type(&lib.c_ffi_prefix)))?;
        f.writeln("{")?;
        indented(f, |f| {
            for el in handle.fields() {
                let value = match &el.field_type {
                    AnyStructFieldType::Bool(default) => match default {
                        None => el.name.to_snake_case(),
                        Some(false) => "false".to_string(),
                        Some(true) => "true".to_string(),
                    },
                    AnyStructFieldType::Uint8(default) => match default {
                        None => el.name.to_snake_case(),
                        Some(value) => value.to_string(),
                    },
                    AnyStructFieldType::Sint8(default) => match default {
                        None => el.name.to_snake_case(),
                        Some(value) => value.to_string(),
                    },
                    AnyStructFieldType::Uint16(default) => match default {
                        None => el.name.to_snake_case(),
                        Some(value) => value.to_string(),
                    },
                    AnyStructFieldType::Sint16(default) => match default {
                        None => el.name.to_snake_case(),
                        Some(value) => value.to_string(),
                    },
                    AnyStructFieldType::Uint32(default) => match default {
                        None => el.name.to_snake_case(),
                        Some(value) => value.to_string(),
                    },
                    AnyStructFieldType::Sint32(default) => match default {
                        None => el.name.to_snake_case(),
                        Some(value) => value.to_string(),
                    },
                    AnyStructFieldType::Uint64(default) => match default {
                        None => el.name.to_snake_case(),
                        Some(value) => value.to_string(),
                    },
                    AnyStructFieldType::Sint64(default) => match default {
                        None => el.name.to_snake_case(),
                        Some(value) => value.to_string(),
                    },
                    AnyStructFieldType::Float(default) => match default {
                        None => el.name.to_snake_case(),
                        Some(value) => format!("{:.}f", value),
                    },
                    AnyStructFieldType::Double(default) => match default {
                        None => el.name.to_snake_case(),
                        Some(value) => format!("{:.}", value),
                    },
                    AnyStructFieldType::String(default) => match default {
                        None => el.name.to_snake_case(),
                        Some(value) => format!("\"{}\"", value),
                    },
                    AnyStructFieldType::Struct(handle) => {
                        if handle.all_fields_have_defaults() {
                            format!(
                                "{}_{}_init()",
                                &lib.c_ffi_prefix,
                                handle.name().to_snake_case()
                            )
                        } else {
                            el.name.to_snake_case()
                        }
                    }
                    AnyStructFieldType::StructRef(_) => el.name.to_snake_case(),
                    AnyStructFieldType::Enum(handle, default) => match default {
                        None => el.name.to_snake_case(),
                        Some(value) => match handle.find_variant_by_name(value) {
                            Some(variant) => format!(
                                "{}_{}_{}",
                                lib.c_ffi_prefix.to_shouty_snake_case(),
                                handle.name.to_shouty_snake_case(),
                                variant.name.to_shouty_snake_case()
                            ),
                            None => panic!("Variant {} not found in {}", value, handle.name),
                        },
                    },
                    AnyStructFieldType::ClassRef(_) => el.name.to_snake_case(),
                    AnyStructFieldType::Interface(_) => el.name.to_snake_case(),
                    AnyStructFieldType::Iterator(_) => el.name.to_snake_case(),
                    AnyStructFieldType::Collection(_) => el.name.to_snake_case(),
                    AnyStructFieldType::Duration(mapping, default) => match default {
                        None => el.name.to_snake_case(),
                        Some(value) => match mapping {
                            DurationType::Milliseconds => value.as_millis().to_string(),
                            DurationType::Seconds => value.as_secs().to_string(),
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
    handle: &EnumHandle,
    lib: &Library,
) -> FormattingResult<()> {
    doxygen(f, |f| doxygen_print(f, &handle.doc, lib))?;

    f.writeln(&format!(
        "typedef enum {}",
        handle.to_c_type(&lib.c_ffi_prefix)
    ))?;
    f.writeln("{")?;
    indented(f, |f| {
        for variant in &handle.variants {
            doxygen(f, |f| doxygen_print(f, &variant.doc, lib))?;
            f.writeln(&format!(
                "{}_{}_{} = {},",
                lib.c_ffi_prefix.to_shouty_snake_case(),
                handle.name.to_shouty_snake_case(),
                variant.name.to_shouty_snake_case(),
                variant.value
            ))?;
        }
        Ok(())
    })?;
    f.writeln(&format!("}} {};", handle.to_c_type(&lib.c_ffi_prefix)))?;

    f.newline()?;

    doxygen(f, |f| {
        f.writeln("@brief ")?;
        docstring_print(
            f,
            &format!("Converts a {{enum:{}}} to a string", handle.name).into(),
            lib,
        )?;
        f.writeln("@param value Enum to convert")?;
        f.writeln("@returns String representation")
    })?;
    f.writeln(&format!(
        "static const char* {}_{}_to_string({} value)",
        &lib.c_ffi_prefix,
        handle.name.to_snake_case(),
        handle.to_c_type(&lib.c_ffi_prefix)
    ))?;
    blocked(f, |f| {
        f.writeln("switch (value)")?;
        blocked(f, |f| {
            for variant in &handle.variants {
                f.writeln(&format!(
                    "case {}_{}_{}: return \"{}\";",
                    lib.c_ffi_prefix.to_shouty_snake_case(),
                    handle.name.to_shouty_snake_case(),
                    variant.name.to_shouty_snake_case(),
                    variant.name
                ))?;
            }
            f.writeln(&format!("default: return \"Unknown{}\";", handle.name))
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
                    "Iterator of {{struct:{}}}. See @ref {}_{}.",
                    handle.item_type.name(),
                    lib.c_ffi_prefix,
                    handle.native_func.name
                )),
                lib,
            )
        })?,
        Some(Symbol::Collection(handle)) => doxygen(f, |f| {
            doxygen_print(
                f,
                &Doc::from(&*format!(
                    "Collection of {}. See @ref {}_{} and @ref {}_{}.",
                    handle.item_type.to_c_type(&lib.c_ffi_prefix),
                    lib.c_ffi_prefix,
                    handle.add_func.name,
                    lib.c_ffi_prefix,
                    handle.delete_func.name
                )),
                lib,
            )
        })?,
        _ => (),
    }

    f.writeln(&format!(
        "typedef struct {} {};",
        handle.to_c_type(&lib.c_ffi_prefix),
        handle.to_c_type(&lib.c_ffi_prefix)
    ))
}

fn write_function_docs(
    f: &mut dyn Printer,
    handle: &FunctionHandle,
    lib: &Library,
) -> FormattingResult<()> {
    doxygen(f, |f| {
        // Print top-level documentation
        doxygen_print(f, &handle.doc, lib)?;

        // Print each parameter value
        for param in &handle.parameters {
            f.writeln(&format!("@param {} ", param.name))?;
            docstring_print(f, &param.doc, lib)?;
            if let FArgument::Basic(BasicType::Duration(mapping)) = param.arg_type {
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
                if let AnyType::Basic(BasicType::Duration(mapping)) = ret {
                    f.write(&format!(" ({})", mapping.unit()))?;
                }
            }
            SignatureType::ErrorNoReturn(_) => {
                write_error_return_doc(f)?;
            }
            SignatureType::ErrorWithReturn(_, ret, doc) => {
                f.writeln("@param out ")?;
                docstring_print(f, &doc, lib)?;
                if let AnyType::Basic(BasicType::Duration(mapping)) = ret {
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
    handle: &FunctionHandle,
    lib: &Library,
) -> FormattingResult<()> {
    write_function_docs(f, handle, lib)?;

    if let Some(error_type) = &handle.error_type {
        f.writeln(&format!(
            "{} {}_{}(",
            error_type.inner.to_c_type(&lib.c_ffi_prefix),
            &lib.c_ffi_prefix,
            handle.name
        ))?;
    } else {
        f.writeln(&format!(
            "{} {}_{}(",
            handle.return_type.to_c_type(&lib.c_ffi_prefix),
            &lib.c_ffi_prefix,
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
                    AnyType::from(param.arg_type.clone()).to_c_type(&lib.c_ffi_prefix),
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
                f.write(&format!("{}* out", x.to_c_type(&lib.c_ffi_prefix)))?;
            }
        }
    }

    f.write(");")
}

fn write_interface(f: &mut dyn Printer, handle: &Interface, lib: &Library) -> FormattingResult<()> {
    doxygen(f, |f| doxygen_print(f, &handle.doc, lib))?;

    let struct_name = handle.to_c_type(&lib.c_ffi_prefix);

    f.writeln(&format!("typedef struct {}", struct_name))?;
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
                        handle.return_type.to_c_type(&lib.c_ffi_prefix),
                        handle.name.to_snake_case(),
                    ))?;

                    f.write(&chelpers::callback_parameters(lib, handle))?;

                    f.write(");")?;
                }
                InterfaceElement::DestroyFunction(name) => {
                    doxygen(f, |f| {
                        f.writeln("@brief Callback when the underlying owner doesn't need the interface anymore")?;
                        f.writeln("@param arg Context data")
                    })?;
                    f.writeln(&format!("void (*{})(void* arg);", name))?;
                }
            }
        }
        Ok(())
    })?;
    f.writeln(&format!("}} {};", struct_name))?;

    f.newline()?;

    // Write init helper
    doxygen(f, |f| {
        f.writeln("@brief ")?;
        docstring_print(
            f,
            &format!("Initialize a {{interface:{}}} interface", handle.name).into(),
            lib,
        )?;
        for element in &handle.elements {
            match element {
                InterfaceElement::Arg(name) => {
                    f.writeln(&format!("@param {} Context data", name.to_snake_case()))?;
                }
                InterfaceElement::CallbackFunction(handle) => {
                    f.writeln(&format!("@param {} ", handle.name.to_snake_case()))?;
                    docstring_print(f, &handle.doc.brief, lib)?;
                }
                InterfaceElement::DestroyFunction(name) => {
                    f.writeln(&format!("@param {} Callback when the underlying owner doesn't need the interface anymore", name.to_snake_case()))?;
                }
            }
        }
        Ok(())
    })?;
    f.writeln(&format!(
        "static {} {}_{}_init(",
        struct_name,
        &lib.c_ffi_prefix,
        handle.name.to_snake_case()
    ))?;
    indented(f, |f| {
        for (idx, element) in handle.elements.iter().enumerate() {
            match element {
                InterfaceElement::Arg(name) => {
                    f.writeln(&format!("void* {}", name.to_snake_case()))?;
                }
                InterfaceElement::CallbackFunction(handle) => {
                    f.writeln(&format!(
                        "{} (*{})(",
                        handle.return_type.to_c_type(&lib.c_ffi_prefix),
                        handle.name.to_snake_case(),
                    ))?;

                    f.write(&chelpers::callback_parameters(lib, handle))?;
                    f.write(")")?;
                }
                InterfaceElement::DestroyFunction(name) => {
                    f.writeln(&format!("void (*{})(void* arg)", name))?;
                }
            }
            if idx + 1 < handle.elements.len() {
                f.write(",")?;
            }
        }
        Ok(())
    })?;
    f.writeln(")")?;

    blocked(f, |f| {
        f.writeln(&format!("{} result = ", struct_name))?;
        blocked(f, |f| {
            for element in &handle.elements {
                match element {
                    InterfaceElement::Arg(name) => {
                        f.writeln(&format!(
                            ".{} = {},",
                            name.to_snake_case(),
                            name.to_snake_case()
                        ))?;
                    }
                    InterfaceElement::CallbackFunction(handle) => {
                        f.writeln(&format!(
                            ".{} = {},",
                            handle.name.to_snake_case(),
                            handle.name.to_snake_case()
                        ))?;
                    }
                    InterfaceElement::DestroyFunction(name) => {
                        f.writeln(&format!(
                            ".{} = {},",
                            name.to_snake_case(),
                            name.to_snake_case()
                        ))?;
                    }
                }
            }
            Ok(())
        })?;
        f.write(";")?;
        f.writeln("return result;")
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
    let filename = cmake_path.join(format!("{}-config.cmake", lib.name));
    let mut f = FilePrinter::new(filename)?;

    let link_deps = get_link_dependencies(config);

    // Prefix used everywhere else
    f.writeln("set(prefix \"${CMAKE_CURRENT_LIST_DIR}/../\")")?;
    f.newline()?;

    // Write dynamic library version
    f.writeln(&format!("add_library({} SHARED IMPORTED GLOBAL)", lib.name))?;
    f.writeln(&format!("set_target_properties({} PROPERTIES", lib.name))?;
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
        lib.name
    ))?;
    f.writeln(&format!(
        "set_target_properties({}_static PROPERTIES",
        lib.name
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
    f.writeln(")")
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

    if !output.status.success() {
        panic!("failed to get the link dependencies");
    }

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
