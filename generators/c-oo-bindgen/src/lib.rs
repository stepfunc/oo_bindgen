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
use oo_bindgen::class::*;
use oo_bindgen::constants::{ConstantSetHandle, ConstantValue, Representation};
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
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

mod chelpers;
mod ctype;
//mod cpp;
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
    generate_c_header(lib, include_path)?;

    // TODO - Create the C++ header
    // crate::cpp::generate_cpp_header(lib, &include_path)?;
    // crate::cpp::generate_cpp_impl(lib, &include_path)?;

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
                Statement::StructDefinition(st) => match st {
                    StructType::FStruct(x) => write_struct_definition(f, x, lib)?,
                    StructType::RStruct(x) => write_struct_definition(f, x, lib)?,
                    StructType::CStruct(x) => write_struct_definition(f, x, lib)?,
                    StructType::UStruct(x) => write_struct_definition(f, x, lib)?,
                },
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

fn write_struct_definition<T>(
    f: &mut dyn Printer,
    handle: &Handle<Struct<T>>,
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
    f.writeln(&format!(
        "typedef struct {}",
        handle.to_c_type(&lib.c_ffi_prefix)
    ))?;
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
                element.field_type.to_c_type(&lib.c_ffi_prefix),
                element.name.to_snake_case(),
            ))?;
        }
        Ok(())
    })?;
    f.writeln(&format!("}} {};", handle.to_c_type(&lib.c_ffi_prefix)))?;

    // user should never try to initialize opaque structs, so don't suggest this is OK
    if handle.visibility != Visibility::Private {
        f.newline()?;
        for c in &handle.constructors {
            write_struct_constructor(f, lib, c, handle)?;
            f.newline()?;
        }
    }

    Ok(())
}

fn write_struct_constructor<T>(
    f: &mut dyn Printer,
    lib: &Library,
    constructor: &Constructor,
    handle: &Handle<Struct<T>>,
) -> FormattingResult<()>
where
    T: StructFieldType + CType + TypeExtractor,
{
    doxygen(f, |f| {
        f.writeln("@brief ")?;
        docstring_print(f, &constructor.doc.brief, lib)?;

        if !constructor.values.is_empty() {
            f.newline()?;
            f.writeln("@note")?;
            for value in &constructor.values {
                f.writeln(&format!("{} is initialized to {}", value.name, value.value))?;
            }
            f.newline()?;
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
        .filter(|f| !constructor.values.iter().any(|cf| cf.name == f.name))
        .map(|el| {
            format!(
                "{} {}",
                el.field_type.to_c_type(&lib.c_ffi_prefix),
                el.name.to_snake_case()
            )
        })
        .collect::<Vec<String>>()
        .join(", ");

    f.writeln(&format!(
        "static {} {}_{}_{}({})",
        handle.to_c_type(&lib.c_ffi_prefix),
        &lib.c_ffi_prefix,
        handle.name().to_snake_case(),
        constructor.name.to_snake_case(),
        params
    ))?;

    blocked(f, |f| {
        f.writeln(&format!("return ({})", handle.to_c_type(&lib.c_ffi_prefix)))?;
        f.writeln("{")?;
        for field in &handle.fields {
            let value: String = match constructor.values.iter().find(|x| x.name == field.name) {
                Some(x) => match &x.value {
                    ValidatedConstructorDefault::Bool(x) => x.to_string(),
                    ValidatedConstructorDefault::Numeric(x) => match x {
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
                    ValidatedConstructorDefault::Duration(t, x) => t.get_value_string(*x),
                    ValidatedConstructorDefault::Enum(x, variant) => {
                        format!(
                            "{}_{}_{}",
                            lib.c_ffi_prefix.to_shouty_snake_case(),
                            x.name.to_shouty_snake_case(),
                            variant.to_shouty_snake_case()
                        )
                    }
                    ValidatedConstructorDefault::String(x) => format!("\"{}\"", x),
                    ValidatedConstructorDefault::DefaultStruct(handle, _, name) => {
                        format!(
                            "{}_{}_{}()",
                            &lib.c_ffi_prefix,
                            handle.name().to_snake_case(),
                            name.to_snake_case(),
                        )
                    }
                },
                None => field.name.to_snake_case(),
            };
            indented(f, |f| {
                f.writeln(&format!(".{} = {},", field.name.to_snake_case(), value))
            })?;
        }
        f.writeln("};")?;
        Ok(())
    })?;

    Ok(())
}

/* TODO
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
                el.field_type.to_any_type().to_c_type(&lib.c_ffi_prefix),
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
                    AnyType::Bool(default) => match default {
                        None => el.name.to_snake_case(),
                        Some(false) => "false".to_string(),
                        Some(true) => "true".to_string(),
                    },
                    AnyType::Uint8(default) => match default {
                        None => el.name.to_snake_case(),
                        Some(value) => value.to_string(),
                    },
                    AnyType::Sint8(default) => match default {
                        None => el.name.to_snake_case(),
                        Some(value) => value.to_string(),
                    },
                    AnyType::Uint16(default) => match default {
                        None => el.name.to_snake_case(),
                        Some(value) => value.to_string(),
                    },
                    AnyType::Sint16(default) => match default {
                        None => el.name.to_snake_case(),
                        Some(value) => value.to_string(),
                    },
                    AnyType::Uint32(default) => match default {
                        None => el.name.to_snake_case(),
                        Some(value) => value.to_string(),
                    },
                    AnyType::Sint32(default) => match default {
                        None => el.name.to_snake_case(),
                        Some(value) => value.to_string(),
                    },
                    AnyType::Uint64(default) => match default {
                        None => el.name.to_snake_case(),
                        Some(value) => value.to_string(),
                    },
                    AnyType::Sint64(default) => match default {
                        None => el.name.to_snake_case(),
                        Some(value) => value.to_string(),
                    },
                    AnyType::Float(default) => match default {
                        None => el.name.to_snake_case(),
                        Some(value) => format!("{:.}f", value),
                    },
                    AnyType::Double(default) => match default {
                        None => el.name.to_snake_case(),
                        Some(value) => format!("{:.}", value),
                    },
                    AnyType::String(default) => match default {
                        None => el.name.to_snake_case(),
                        Some(value) => format!("\"{}\"", value),
                    },
                    AnyType::Struct(handle) => {
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
                    AnyType::StructRef(_) => el.name.to_snake_case(),
                    AnyType::Enum(field) => match &field.default_variant {
                        None => el.name.to_snake_case(),
                        Some(value) => match field.handle.find_variant_by_name(value.as_str()) {
                            Some(variant) => format!(
                                "{}_{}_{}",
                                lib.c_ffi_prefix.to_shouty_snake_case(),
                                field.handle.name.to_shouty_snake_case(),
                                variant.name.to_shouty_snake_case()
                            ),
                            None => panic!("Variant {} not found in {}", value, field.handle.name),
                        },
                    },
                    AnyType::ClassRef(_) => el.name.to_snake_case(),
                    AnyType::Interface(_) => el.name.to_snake_case(),
                    AnyType::Iterator(_) => el.name.to_snake_case(),
                    AnyType::Collection(_) => el.name.to_snake_case(),
                    AnyType::Duration(mapping, default) => match default {
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
*/

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
                    handle.function.name
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
                    param.arg_type.to_c_type(&lib.c_ffi_prefix),
                    param.name.to_snake_case()
                )
            })
            .collect::<Vec<String>>()
            .join(", "),
    )?;

    if handle.error_type.is_some() {
        if let FReturnType::Type(x, _) = &handle.return_type {
            if !handle.parameters.is_empty() {
                f.write(", ")?;
                f.write(&format!("{}* out", x.to_c_type(&lib.c_ffi_prefix)))?;
            }
        }
    }

    f.write(");")
}

fn write_interface(
    f: &mut dyn Printer,
    handle: &InterfaceHandle,
    lib: &Library,
) -> FormattingResult<()> {
    doxygen(f, |f| doxygen_print(f, &handle.doc, lib))?;

    let struct_name = handle.to_c_type(&lib.c_ffi_prefix);

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

                f.writeln(&format!("@param {} ", CTX_VARIABLE_NAME))?;
                docstring_print(f, &"Context data".into(), lib)?;

                // Print return documentation
                if let CReturnType::Type(_, doc) = &cb.return_type {
                    f.writeln("@return ")?;
                    docstring_print(f, doc, lib)?;
                }

                Ok(())
            })?;

            f.newline()?;

            // Print function signature
            f.write(&format!(
                "{} (*{})(",
                cb.return_type.to_c_type(&lib.c_ffi_prefix),
                cb.name.to_snake_case(),
            ))?;

            f.write(&chelpers::callback_parameters(lib, cb))?;

            f.write(");")?;
        }

        doxygen(f, |f| {
            f.writeln(
                "@brief Callback when the underlying owner doesn't need the interface anymore",
            )?;
            f.writeln("@param arg Context data")
        })?;
        f.writeln(&format!("void (*{})(void* arg);", DESTROY_FUNC_NAME))?;

        doxygen(f, |f| f.writeln("@brief Context data"))?;
        f.writeln(&format!("void* {};", CTX_VARIABLE_NAME))?;

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
        for cb in &handle.callbacks {
            f.writeln(&format!("@param {} ", cb.name.to_snake_case()))?;
            docstring_print(f, &cb.doc.brief, lib)?;
        }
        f.writeln(&format!(
            "@param {} Callback when the underlying owner doesn't need the interface anymore",
            DESTROY_FUNC_NAME.to_snake_case()
        ))?;
        f.writeln(&format!(
            "@param {} Context data",
            CTX_VARIABLE_NAME.to_snake_case()
        ))?;
        Ok(())
    })?;
    f.writeln(&format!(
        "static {} {}_{}_init(",
        struct_name,
        &lib.c_ffi_prefix,
        handle.name.to_snake_case()
    ))?;
    indented(f, |f| {
        for cb in &handle.callbacks {
            f.writeln(&format!(
                "{} (*{})(",
                cb.return_type.to_c_type(&lib.c_ffi_prefix),
                cb.name.to_snake_case(),
            ))?;

            f.write(&chelpers::callback_parameters(lib, cb))?;
            f.write("),")?;
        }

        f.writeln(&format!("void (*{})(void* arg),", DESTROY_FUNC_NAME))?;
        f.writeln(&format!("void* {}", CTX_VARIABLE_NAME.to_snake_case()))?;

        Ok(())
    })?;
    f.writeln(")")?;

    blocked(f, |f| {
        f.writeln(&format!("{} result = ", struct_name))?;
        blocked(f, |f| {
            for cb in &handle.callbacks {
                f.writeln(&format!(
                    ".{} = {},",
                    cb.name.to_snake_case(),
                    cb.name.to_snake_case()
                ))?;
            }

            f.writeln(&format!(
                ".{} = {},",
                DESTROY_FUNC_NAME.to_snake_case(),
                DESTROY_FUNC_NAME.to_snake_case()
            ))?;

            f.writeln(&format!(
                ".{} = {},",
                CTX_VARIABLE_NAME.to_snake_case(),
                CTX_VARIABLE_NAME.to_snake_case()
            ))?;

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
