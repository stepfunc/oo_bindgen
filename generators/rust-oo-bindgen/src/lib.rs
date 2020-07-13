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

use formatting::*;
use oo_bindgen::callback::*;
use oo_bindgen::formatting::*;
use oo_bindgen::native_enum::*;
use oo_bindgen::native_function::*;
use oo_bindgen::native_struct::*;
use oo_bindgen::*;
use std::env;
use std::fmt::Display;
use std::path::{Path, PathBuf};

mod formatting;

pub struct RustCodegen<'a> {
    library: &'a Library,
    dest_path: PathBuf,
}

impl<'a> RustCodegen<'a> {
    pub fn new(lib: &'a Library) -> Self {
        RustCodegen {
            library: lib,
            dest_path: Path::new(&env::var_os("OUT_DIR").unwrap()).join("ffi.rs"),
        }
    }

    pub fn destination<P: AsRef<Path>>(&mut self, dest: P) -> &mut Self {
        self.dest_path = dest.as_ref().to_owned();
        self
    }

    pub fn generate(self) -> FormattingResult<()> {
        let mut f = FilePrinter::new(&self.dest_path)?;

        // Some standard includes
        f.writeln("use std::os::raw::{c_char, c_void};")?;

        f.newline()?;

        for statement in self.library.into_iter() {
            match statement {
                Statement::NativeStructDefinition(handle) => {
                    self.write_struct_definition(&mut f, handle)?
                }
                Statement::EnumDefinition(handle) => self.write_enum_definition(&mut f, handle)?,
                Statement::NativeFunctionDeclaration(handle) => {
                    self.write_function(&mut f, handle)?
                }
                Statement::InterfaceDefinition(handle) => self.write_interface(&mut f, handle)?,
                Statement::OneTimeCallbackDefinition(handle) => {
                    self.write_one_time_callback(&mut f, handle)?
                }
                _ => (),
            }
            f.newline()?;
        }

        Ok(())
    }

    fn write_struct_definition(
        &self,
        f: &mut dyn Printer,
        handle: &NativeStructHandle,
    ) -> FormattingResult<()> {
        f.writeln("#[repr(C)]")?;
        f.writeln("#[derive(Clone)]")?;
        f.writeln(&format!("pub struct {}", handle.name()))?;
        blocked(f, |f| {
            for element in &handle.elements {
                f.writeln(&format!(
                    "pub {}: {},",
                    element.name,
                    RustType(&element.element_type)
                ))?;
            }
            Ok(())
        })
    }

    fn write_enum_definition(
        &self,
        f: &mut dyn Printer,
        handle: &NativeEnumHandle,
    ) -> FormattingResult<()> {
        f.writeln("#[repr(C)]")?;
        f.writeln("#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]")?;
        f.writeln(&format!("pub enum {}", handle.name))?;
        blocked(f, |f| {
            for variant in &handle.variants {
                f.writeln(&format!("{} = {},", variant.name, variant.value))?;
            }
            Ok(())
        })
    }

    fn write_function(
        &self,
        f: &mut dyn Printer,
        handle: &NativeFunctionHandle,
    ) -> FormattingResult<()> {
        f.writeln("/// # Safety")?;
        f.writeln("///")?;
        f.writeln("/// Clippy requires safety documentation for public unsafe functions")?;
        f.writeln("#[no_mangle]")?;
        f.writeln(&format!("pub unsafe extern \"C\" fn {}(", handle.name))?;

        f.write(
            &handle
                .parameters
                .iter()
                .map(|param| format!("{}: {}", param.name, RustType(&param.param_type)))
                .collect::<Vec<String>>()
                .join(", "),
        )?;

        if handle.return_type.is_void() {
            f.write(")")?;
        } else {
            f.write(&format!(") -> {}", RustReturnType(&handle.return_type)))?;
        }

        blocked(f, |f| {
            f.writeln(&format!("crate::{}(", handle.name))?;

            f.write(
                &handle
                    .parameters
                    .iter()
                    .map(|param| param.name.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
            )?;

            f.write(")")
        })
    }

    fn write_interface(&self, f: &mut dyn Printer, handle: &Interface) -> FormattingResult<()> {
        f.writeln("#[repr(C)]")?;
        f.writeln("#[derive(Clone)]")?;
        f.writeln(&format!("pub struct {}", handle.name))?;
        blocked(f, |f| {
            for element in &handle.elements {
                match element {
                    InterfaceElement::Arg(name) => {
                        f.writeln(&format!("pub {}: *mut c_void,", name))?
                    }
                    InterfaceElement::CallbackFunction(handle) => {
                        f.newline()?;
                        f.write(&format!("pub {}: Option<extern \"C\" fn(", handle.name))?;

                        f.write(
                            &handle
                                .parameters
                                .iter()
                                .map(|param| match param {
                                    CallbackParameter::Arg(name) => {
                                        format!("{}: *mut c_void", name)
                                    }
                                    CallbackParameter::Parameter(param) => {
                                        format!("{}: {}", param.name, RustType(&param.param_type))
                                    }
                                })
                                .collect::<Vec<String>>()
                                .join(", "),
                        )?;

                        f.write(&format!(") -> {}>,", RustReturnType(&handle.return_type)))?;
                    }
                    InterfaceElement::DestroyFunction(name) => {
                        f.writeln(&format!(
                            "pub {}: Option<extern \"C\" fn(data: *mut c_void)>,",
                            name
                        ))?;
                    }
                }
            }
            Ok(())
        })
    }

    fn write_one_time_callback(
        &self,
        f: &mut dyn Printer,
        handle: &OneTimeCallbackHandle,
    ) -> FormattingResult<()> {
        f.writeln("#[repr(C)]")?;
        f.writeln("#[derive(Clone)]")?;
        f.writeln(&format!("pub struct {}", handle.name))?;
        blocked(f, |f| {
            for element in &handle.elements {
                match element {
                    OneTimeCallbackElement::Arg(name) => {
                        f.writeln(&format!("pub {}: *mut c_void,", name))?
                    }
                    OneTimeCallbackElement::CallbackFunction(handle) => {
                        f.newline()?;
                        f.write(&format!("pub {}: Option<extern \"C\" fn(", handle.name))?;

                        f.write(
                            &handle
                                .parameters
                                .iter()
                                .map(|param| match param {
                                    CallbackParameter::Arg(name) => {
                                        format!("{}: *mut c_void", name)
                                    }
                                    CallbackParameter::Parameter(param) => {
                                        format!("{}: {}", param.name, RustType(&param.param_type))
                                    }
                                })
                                .collect::<Vec<String>>()
                                .join(", "),
                        )?;

                        f.write(&format!(") -> {}>,", RustReturnType(&handle.return_type)))?;
                    }
                }
            }
            Ok(())
        })
    }
}

struct RustReturnType<'a>(&'a ReturnType);

impl<'a> Display for RustReturnType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.0 {
            ReturnType::Void => write!(f, "()"),
            ReturnType::Type(return_type, _) => write!(f, "{}", RustType(&return_type)),
        }
    }
}

struct RustType<'a>(&'a Type);

impl<'a> Display for RustType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.0 {
            Type::Bool => write!(f, "bool"),
            Type::Uint8 => write!(f, "u8"),
            Type::Sint8 => write!(f, "i8"),
            Type::Uint16 => write!(f, "u16"),
            Type::Sint16 => write!(f, "i16"),
            Type::Uint32 => write!(f, "u32"),
            Type::Sint32 => write!(f, "i32"),
            Type::Uint64 => write!(f, "u64"),
            Type::Sint64 => write!(f, "i64"),
            Type::Float => write!(f, "f32"),
            Type::Double => write!(f, "f64"),
            Type::String => write!(f, "*const c_char"),
            Type::Struct(handle) => write!(f, "{}", handle.name()),
            Type::StructRef(handle) => write!(f, "*const {}", handle.name),
            Type::Enum(handle) => write!(f, "{}", handle.name),
            Type::ClassRef(handle) => write!(f, "*mut crate::{}", handle.name),
            Type::Interface(handle) => write!(f, "{}", handle.name),
            Type::OneTimeCallback(handle) => write!(f, "{}", handle.name),
            Type::Iterator(handle) => write!(f, "*mut crate::{}", handle.name()),
            Type::Duration(mapping) => match mapping {
                DurationMapping::Milliseconds | DurationMapping::Seconds => write!(f, "u64"),
                DurationMapping::SecondsFloat => write!(f, "f32"),
            },
        }
    }
}
