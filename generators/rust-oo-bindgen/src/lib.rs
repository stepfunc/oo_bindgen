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

use std::env;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use heck::CamelCase;

use oo_bindgen::backend::*;
use oo_bindgen::model::*;

use crate::rust_struct::*;
use crate::rust_type::*;
use crate::type_converter::*;

mod rust_struct;
mod rust_type;
mod type_converter;

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

    pub fn generate(self) -> FormattingResult<()> {
        let mut f = FilePrinter::new(&self.dest_path)?;

        for statement in self.library.statements() {
            match statement {
                Statement::StructDefinition(s) => match s {
                    StructType::FunctionArg(s) => self.write_struct_definition(&mut f, s)?,
                    StructType::FunctionReturn(s) => self.write_struct_definition(&mut f, s)?,
                    StructType::CallbackArg(s) => self.write_struct_definition(&mut f, s)?,
                    StructType::Universal(s) => self.write_struct_definition(&mut f, s)?,
                },
                Statement::EnumDefinition(handle) => self.write_enum_definition(&mut f, handle)?,
                Statement::FunctionDefinition(handle) => {
                    Self::write_function(&mut f, handle, &self.library.settings.c_ffi_prefix)?
                }
                Statement::InterfaceDefinition(t) => {
                    self.write_interface(&mut f, t.untyped(), t.mode())?
                }
                _ => (),
            }
            f.newline()?;
        }

        Ok(())
    }

    fn write_struct_definition<T>(
        &self,
        f: &mut dyn Printer,
        handle: &Handle<Struct<T, Validated>>,
    ) -> FormattingResult<()>
    where
        T: StructFieldType + RustType,
    {
        let struct_name = handle.name().to_camel_case();
        let c_lifetime = if handle.annotate_c_with_lifetime() {
            "<'a>"
        } else {
            ""
        };
        let rust_lifetime = if handle.annotate_rust_with_lifetime() {
            "<'a>"
        } else {
            ""
        };
        let public = "pub "; //if handle.has_conversion() { "" } else { "pub " };

        // Write the C struct with private fields (if conversion required)
        f.writeln("#[repr(C)]")?;
        f.writeln("#[derive(Clone)]")?;
        f.writeln(&format!("pub struct {}{}", struct_name, c_lifetime))?;

        blocked(f, |f| {
            for element in &handle.fields {
                f.writeln(&format!(
                    "{}{}: {},",
                    public,
                    element.name,
                    element.field_type.as_c_type()
                ))?;
            }
            Ok(())
        })?;

        f.newline()?;

        // Write accessors/mutators
        f.writeln(&format!(
            "impl{lifetime} {name}{lifetime}",
            name = struct_name,
            lifetime = c_lifetime
        ))?;
        blocked(f, |f| {
            for field in &handle.fields {
                let field_lifetime = if field.field_type.rust_requires_lifetime() {
                    "'a "
                } else {
                    ""
                };
                let fn_lifetime =
                    if field.field_type.rust_requires_lifetime() && !handle.c_requires_lifetime() {
                        "<'a>"
                    } else {
                        ""
                    };
                let ampersand = if field.field_type.is_copyable() {
                    ""
                } else {
                    "&"
                };

                // Accessor
                f.writeln("#[allow(clippy::needless_lifetimes)]")?;
                f.writeln(&format!(
                    "pub fn {name}{fn_lifetime}(&{lifetime}self) -> {ampersand}{return_type}",
                    name = field.name,
                    return_type = field.field_type.as_rust_type(),
                    fn_lifetime = fn_lifetime,
                    lifetime = field_lifetime,
                    ampersand = ampersand
                ))?;
                blocked(f, |f| {
                    if let Some(conversion) = field.field_type.conversion() {
                        if conversion.is_unsafe() {
                            f.writeln("unsafe {")?;
                        }
                        conversion.convert_from_c(
                            f,
                            &format!(
                                "{ampersand}self.{name}",
                                name = field.name,
                                ampersand = ampersand
                            ),
                            "",
                        )?;
                        if conversion.is_unsafe() {
                            f.writeln("}")?;
                        }
                        Ok(())
                    } else {
                        f.writeln(&format!(
                            "{ampersand}self.{name}",
                            name = field.name,
                            ampersand = ampersand
                        ))
                    }
                })?;

                f.newline()?;

                // Mutator
                f.writeln("#[allow(clippy::needless_lifetimes)]")?;
                f.writeln(&format!(
                    "pub fn set_{name}{fn_lifetime}(&{lifetime}mut self, value: {element_type})",
                    name = field.name,
                    element_type = field.field_type.as_rust_type(),
                    fn_lifetime = fn_lifetime,
                    lifetime = field_lifetime
                ))?;
                blocked(f, |f| {
                    if let Some(conversion) = field.field_type.conversion() {
                        conversion.convert_to_c(f, "value", &format!("self.{} = ", field.name))?;
                        f.write(";")
                    } else {
                        f.writeln(&format!("self.{} = value;", field.name))
                    }
                })?;

                f.newline()?;
            }
            Ok(())
        })?;

        // Write the Rust version with all public fields
        let rust_struct_name = format!("{}Fields", struct_name);
        if handle.has_conversion() {
            f.writeln(&format!("pub struct {}{}", rust_struct_name, rust_lifetime))?;
            blocked(f, |f| {
                for element in &handle.fields {
                    f.writeln(&format!(
                        "pub {}: {},",
                        element.name,
                        element.field_type.as_rust_type()
                    ))?;
                }
                Ok(())
            })?;

            // Write the conversion to the C representation
            f.writeln(&format!(
                "impl{rust_lifetime} From<{rust_struct_name}{rust_lifetime}> for {name}{c_lifetime}",
                name = struct_name,
                rust_struct_name = rust_struct_name,
                rust_lifetime = rust_lifetime,
                c_lifetime = c_lifetime,
            ))?;
            blocked(f, |f| {
                f.writeln(&format!("fn from(from: {}) -> Self", rust_struct_name))?;
                blocked(f, |f| {
                    f.writeln("Self")?;
                    blocked(f, |f| {
                        for element in &handle.fields {
                            if let Some(conversion) = element.field_type.conversion() {
                                conversion.convert_to_c(
                                    f,
                                    &format!("from.{}", element.name),
                                    &format!("{}: ", element.name),
                                )?;
                                f.write(",")?;
                            } else {
                                f.writeln(&format!("{name}: from.{name},", name = element.name))?;
                            }
                        }
                        Ok(())
                    })
                })
            })
        } else {
            f.writeln(&format!(
                "pub type {rust_struct_name}{lifetime} = {name}{lifetime};",
                rust_struct_name = rust_struct_name,
                name = struct_name,
                lifetime = c_lifetime
            ))
        }
    }

    fn write_enum_definition(
        &self,
        f: &mut dyn Printer,
        handle: &Handle<Enum<Validated>>,
    ) -> FormattingResult<()> {
        let enum_name = handle.name.to_camel_case();
        f.writeln("#[repr(C)]")?;
        f.writeln("#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]")?;
        f.writeln(&format!("pub enum {}", enum_name))?;
        blocked(f, |f| {
            for variant in &handle.variants {
                f.writeln(&format!(
                    "{} = {},",
                    variant.name.to_camel_case(),
                    variant.value
                ))?;
            }
            Ok(())
        })?;

        // Conversion routines
        f.writeln(&format!("impl From<{}> for std::os::raw::c_int", enum_name))?;
        blocked(f, |f| {
            f.writeln(&format!("fn from(value: {}) -> Self", enum_name))?;
            blocked(f, |f| {
                f.writeln("match value")?;
                blocked(f, |f| {
                    for variant in &handle.variants {
                        f.writeln(&format!(
                            "{}::{} => {},",
                            enum_name,
                            variant.name.to_camel_case(),
                            variant.value
                        ))?;
                    }
                    Ok(())
                })
            })
        })?;

        f.writeln(&format!("impl From<std::os::raw::c_int> for {}", enum_name))?;
        blocked(f, |f| {
            f.writeln("fn from(value: std::os::raw::c_int) -> Self")?;
            blocked(f, |f| {
                f.writeln("match value")?;
                blocked(f, |f| {
                    for variant in &handle.variants {
                        f.writeln(&format!(
                            "{} => {}::{},",
                            variant.value,
                            enum_name,
                            variant.name.to_camel_case(),
                        ))?;
                    }
                    f.writeln(&format!(
                        "_ => panic!(\"{{}} is not a variant of {}\", value),",
                        enum_name
                    ))
                })
            })
        })
    }

    fn write_function(
        f: &mut dyn Printer,
        handle: &Handle<Function<Validated>>,
        prefix: &str,
    ) -> FormattingResult<()> {
        f.writeln("#[allow(clippy::missing_safety_doc)]")?;
        f.writeln("#[no_mangle]")?;
        f.writeln(&format!(
            "pub unsafe extern \"C\" fn {}_{}(",
            prefix, handle.name
        ))?;

        f.write(
            &handle
                .arguments
                .iter()
                .map(|param| format!("{}: {}", param.name, param.arg_type.as_c_type()))
                .collect::<Vec<String>>()
                .join(", "),
        )?;

        fn write_error_return(
            f: &mut dyn Printer,
            _error: &ErrorType<Validated>,
        ) -> FormattingResult<()> {
            f.write(") -> std::os::raw::c_int")
            //f.write(&format!(") -> {}", error.inner.name.to_camel_case()))
        }

        // write the return type
        match handle.get_signature_type() {
            SignatureType::NoErrorNoReturn => {
                f.write(")")?;
            }
            SignatureType::NoErrorWithReturn(t, _) => {
                f.write(&format!(") -> {}", t.as_c_type()))?;
            }
            SignatureType::ErrorNoReturn(err) => {
                write_error_return(f, &err)?;
            }
            SignatureType::ErrorWithReturn(err, ret, _) => {
                if !handle.arguments.is_empty() {
                    f.write(", ")?;
                }
                f.write(&format!("out: *mut {}", ret.as_c_type()))?;
                write_error_return(f, &err)?;
            }
        }

        blocked(f, |f| {
            for param in &handle.arguments {
                if let Some(converter) = param.arg_type.conversion() {
                    converter.convert_from_c(f, &param.name, &format!("let {} = ", param.name))?;
                    f.write(";")?;
                }
            }

            fn basic_invocation(f: &mut dyn Printer, name: &str) -> FormattingResult<()> {
                f.writeln(&format!("crate::{}(", name))
            }

            // invoke the inner function
            match handle.get_signature_type() {
                SignatureType::NoErrorNoReturn => {
                    basic_invocation(f, &handle.name)?;
                }
                SignatureType::NoErrorWithReturn(ret, _) => {
                    if ret.has_conversion() {
                        f.writeln(&format!("let _result = crate::{}(", handle.name))?;
                    } else {
                        basic_invocation(f, &handle.name)?;
                    }
                }
                SignatureType::ErrorWithReturn(_, _, _) | SignatureType::ErrorNoReturn(_) => {
                    f.writeln(&format!("match crate::{}(", &handle.name))?;
                }
            }

            f.write(
                &handle
                    .arguments
                    .iter()
                    .map(|param| param.name.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
            )?;
            f.write(")")?;

            match handle.get_signature_type() {
                SignatureType::NoErrorNoReturn => {}
                SignatureType::NoErrorWithReturn(ret, _) => {
                    if let Some(conversion) = ret.conversion() {
                        f.write(";")?;
                        conversion.convert_to_c(f, "_result", "")?;
                    }
                }
                SignatureType::ErrorNoReturn(err) => {
                    blocked(f, |f| {
                        let converter = TypeConverter::ValidatedEnum(err.inner.clone());
                        f.writeln("Ok(()) =>")?;
                        blocked(f, |f| {
                            converter.convert_to_c(
                                f,
                                &format!("{}::Ok", err.inner.name.to_camel_case()),
                                "",
                            )
                        })?;
                        f.writeln("Err(err) =>")?;
                        blocked(f, |f| converter.convert_to_c(f, "err", ""))
                    })?;
                }
                SignatureType::ErrorWithReturn(err, result_type, _) => {
                    blocked(f, |f| {
                        let converter = TypeConverter::ValidatedEnum(err.inner.clone());
                        f.writeln("Ok(x) =>")?;
                        blocked(f, |f| {
                            if let Some(converter) = result_type.conversion() {
                                converter.convert_to_c(f, "x", "let x = ")?;
                                f.write(";")?;
                            }
                            f.writeln("out.write(x);")?;
                            converter.convert_to_c(
                                f,
                                &format!("{}::Ok", err.inner.name.to_camel_case()),
                                "",
                            )
                        })?;
                        f.writeln("Err(err) =>")?;
                        blocked(f, |f| converter.convert_to_c(f, "err", ""))
                    })?;
                }
            }

            Ok(())
        })
    }

    fn write_interface(
        &self,
        f: &mut dyn Printer,
        handle: &Interface<Validated>,
        mode: InterfaceCategory,
    ) -> FormattingResult<()> {
        let interface_name = handle.name.to_camel_case();
        // C structure
        f.writeln("#[repr(C)]")?;
        f.writeln("#[derive(Clone)]")?;
        f.writeln(&format!("pub struct {}", interface_name))?;
        blocked(f, |f| {
            for cb in &handle.callbacks {
                let lifetime = if cb.c_requires_lifetime() {
                    "for<'a> "
                } else {
                    ""
                };

                f.writeln("#[allow(clippy::needless_lifetimes)]")?;
                f.writeln(&format!(
                    "pub {name}: Option<{lifetime}extern \"C\" fn(",
                    name = cb.name,
                    lifetime = lifetime
                ))?;

                f.write(
                    &cb.arguments
                        .iter()
                        .map(|arg| format!("{}: {}", arg.name, arg.arg_type.as_c_type()))
                        .chain(std::iter::once(format!(
                            "{}: *mut std::os::raw::c_void",
                            handle.settings.interface.context_variable_name
                        )))
                        .collect::<Vec<String>>()
                        .join(", "),
                )?;

                f.write(&format!(") -> {}>,", cb.return_type.as_c_type()))?;
            }

            f.writeln(&format!(
                "pub {}: Option<extern \"C\" fn(ctx: *mut std::os::raw::c_void)>,",
                handle.settings.interface.destroy_func_name
            ))?;

            f.writeln(&format!(
                "pub {}: *mut std::os::raw::c_void,",
                handle.settings.interface.context_variable_name
            ))?;
            Ok(())
        })?;

        f.newline()?;

        self.write_callback_helpers(
            f,
            mode,
            &interface_name,
            handle.settings.clone(),
            handle.callbacks.iter(),
        )?;

        f.newline()?;

        // Drop
        f.writeln(&format!("impl Drop for {}", interface_name))?;
        blocked(f, |f| {
            f.writeln("fn drop(&mut self)")?;
            blocked(f, |f| {
                f.writeln(&format!(
                    "if let Some(cb) = self.{}",
                    handle.settings.interface.destroy_func_name
                ))?;
                blocked(f, |f| {
                    f.writeln(&format!(
                        "cb(self.{});",
                        handle.settings.interface.context_variable_name
                    ))
                })
            })
        })
    }

    fn write_callback_helpers<'b, I: Iterator<Item = &'b CallbackFunction<Validated>>>(
        &self,
        f: &mut dyn Printer,
        mode: InterfaceCategory,
        name: &str,
        settings: Rc<LibrarySettings>,
        callbacks: I,
    ) -> FormattingResult<()> {
        let generate_send_and_sync = match mode {
            InterfaceCategory::Synchronous => false,
            InterfaceCategory::Asynchronous => true,
            InterfaceCategory::Future => true,
        };

        // Send/Sync trait
        if generate_send_and_sync {
            f.writeln(&format!("unsafe impl Send for {} {{}}", name))?;
            f.writeln(&format!("unsafe impl Sync for {} {{}}", name))?;
        }

        f.newline()?;

        // Helper impl
        f.writeln(&format!("impl {}", name))?;
        blocked(f, |f| {
            for callback in callbacks {
                let lifetime = if callback.rust_requires_lifetime() {
                    "<'a>"
                } else {
                    ""
                };

                // Function signature
                f.writeln("#[allow(clippy::needless_lifetimes)]")?;
                f.writeln(&format!(
                    "pub(crate) fn {name}{lifetime}(&self, ",
                    name = callback.name,
                    lifetime = lifetime
                ))?;
                f.write(
                    &callback
                        .arguments
                        .iter()
                        .map(|arg| format!("{}: {}", arg.name, arg.arg_type.as_rust_type()))
                        .collect::<Vec<_>>()
                        .join(", "),
                )?;
                f.write(")")?;

                if let Some(value) = &callback.return_type.get_value() {
                    f.write(&format!(" -> Option<{}>", value.as_rust_type()))?;
                }

                // Function body
                blocked(f, |f| {
                    for arg in &callback.arguments {
                        if let Some(converter) = arg.arg_type.conversion() {
                            converter.convert_to_c(
                                f,
                                &arg.name,
                                &format!("let {} = ", arg.name),
                            )?;
                            f.write(";")?;
                        }
                    }

                    let params = &callback
                        .arguments
                        .iter()
                        .map(|arg| arg.name.to_string())
                        .chain(std::iter::once(format!(
                            "self.{}",
                            settings.interface.context_variable_name
                        )))
                        .collect::<Vec<_>>()
                        .join(", ");
                    let call = format!("cb({})", params);

                    if let Some(v) = &callback.return_type.get_value() {
                        f.writeln(&format!("self.{}.map(|cb| ", callback.name))?;
                        blocked(f, |f| {
                            if let Some(conversion) = v.conversion() {
                                f.writeln(&format!("let _result = {};", call))?;
                                conversion.convert_from_c(f, "_result", "")
                            } else {
                                f.writeln(&call)
                            }
                        })?;
                        f.write(")")?;
                    } else {
                        f.writeln(&format!("if let Some(cb) = self.{}", callback.name))?;
                        blocked(f, |f| f.writeln(&call))?;
                    }

                    if callback.return_type.is_none() {
                        f.write(";")?;
                    }

                    Ok(())
                })?;
            }
            Ok(())
        })
    }
}
