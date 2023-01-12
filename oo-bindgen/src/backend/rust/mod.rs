use std::env;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use heck::ToUpperCamelCase;

use crate::backend::*;
use crate::model::*;

use crate::backend::rust::rust_struct::RustStruct;
use crate::backend::rust::rust_type::RustType;

use crate::backend::rust::rust_type::LifetimeInfo;
use crate::backend::rust::type_converter::TypeConverter;

mod rust_struct;
mod rust_type;
mod type_converter;

/// Generate the FFI (C ABI) interface for the library.
///
/// This layer consists of all the public symbols that will be exported by the shared library.
/// The user must then go and write a number of functions referenced by the FFI functions to glue
/// the C API layer to the underlying rust crate.
///
/// This function is typically called from a build.rs script
pub fn generate_ffi(library: &Library) -> FormattingResult<()> {
    RustCodegen::new(library).generate()
}

struct RustCodegen<'a> {
    library: &'a Library,
    dest_path: PathBuf,
}

impl<'a> RustCodegen<'a> {
    fn new(lib: &'a Library) -> Self {
        RustCodegen {
            library: lib,
            dest_path: Path::new(&env::var_os("OUT_DIR").unwrap()).join("ffi.rs"),
        }
    }

    fn write_promise_module(f: &mut dyn Printer) -> FormattingResult<()> {
        let promise = include_str!("../../../static/rust/promise.rs");
        f.writeln("#[allow(dead_code)]")?;
        f.writeln("pub(crate) mod promise {")?;
        indented(f, |f| {
            for line in promise.lines() {
                f.writeln(line)?;
            }
            Ok(())
        })?;
        f.writeln("}")?;
        Ok(())
    }

    fn generate(self) -> FormattingResult<()> {
        let mut f = FilePrinter::new(&self.dest_path)?;

        Self::write_promise_module(&mut f)?;

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
                    self.write_interface(&mut f, t.untyped(), t.mode())?;
                    if let InterfaceType::Future(t) = t {
                        self.write_future_helpers(&mut f, t)?;
                    }
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
        let struct_name = handle.name().to_upper_camel_case();
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
        f.writeln(&format!("pub struct {struct_name}{c_lifetime}"))?;

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
        f.writeln(&format!("impl{c_lifetime} {struct_name}{c_lifetime}"))?;
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
        let rust_struct_name = format!("{struct_name}Fields");
        if handle.has_conversion() {
            f.writeln(&format!("pub struct {rust_struct_name}{rust_lifetime}"))?;
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
                "impl{rust_lifetime} From<{rust_struct_name}{rust_lifetime}> for {struct_name}{c_lifetime}",
            ))?;
            blocked(f, |f| {
                f.writeln(&format!("fn from(from: {rust_struct_name}) -> Self"))?;
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
                "pub type {rust_struct_name}{c_lifetime} = {struct_name}{c_lifetime};"
            ))
        }
    }

    fn write_enum_definition(
        &self,
        f: &mut dyn Printer,
        handle: &Handle<Enum<Validated>>,
    ) -> FormattingResult<()> {
        let enum_name = handle.name.to_upper_camel_case();
        f.writeln("#[repr(C)]")?;
        f.writeln("#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd)]")?;
        f.writeln(&format!("pub enum {enum_name}"))?;
        blocked(f, |f| {
            for variant in &handle.variants {
                f.writeln(&format!(
                    "{} = {},",
                    variant.name.to_upper_camel_case(),
                    variant.value
                ))?;
            }
            Ok(())
        })?;

        // Conversion routines
        f.writeln(&format!("impl From<{enum_name}> for std::os::raw::c_int"))?;
        blocked(f, |f| {
            f.writeln(&format!("fn from(value: {enum_name}) -> Self"))?;
            blocked(f, |f| {
                f.writeln("match value")?;
                blocked(f, |f| {
                    for variant in &handle.variants {
                        f.writeln(&format!(
                            "{}::{} => {},",
                            enum_name,
                            variant.name.to_upper_camel_case(),
                            variant.value
                        ))?;
                    }
                    Ok(())
                })
            })
        })?;

        f.writeln(&format!("impl From<std::os::raw::c_int> for {enum_name}"))?;
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
                            variant.name.to_upper_camel_case(),
                        ))?;
                    }
                    f.writeln(&format!(
                        "_ => panic!(\"{{value}} is not a variant of {enum_name}\"),"
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
                f.writeln(&format!("crate::{name}("))
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
                                &format!("{}::Ok", err.inner.name.to_upper_camel_case()),
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
                                &format!("{}::Ok", err.inner.name.to_upper_camel_case()),
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
        let interface_name = handle.name.to_upper_camel_case();
        // C structure
        f.writeln("#[repr(C)]")?;
        f.writeln("#[derive(Clone)]")?;
        f.writeln(&format!("pub struct {interface_name}"))?;
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
        f.writeln(&format!("impl Drop for {interface_name}"))?;
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
        })?;

        Ok(())
    }

    fn write_future_helpers(
        &self,
        f: &mut dyn Printer,
        handle: &FutureInterface<Validated>,
    ) -> FormattingResult<()> {
        let name = handle.interface.name.to_upper_camel_case();
        f.writeln(&format!(
            "impl crate::ffi::promise::FutureInterface for {name}"
        ))?;
        blocked(f, |f| {
            f.writeln(&format!(
                "type Value = {};",
                handle.value_type.as_rust_type()
            ))?;
            f.writeln(&format!(
                "type Error = {};",
                handle.error_type.inner.name.to_upper_camel_case()
            ))?;

            f.newline()?;
            f.writeln("fn success(&self, value: Self::Value) {")?;
            indented(f, |f| f.writeln("self.on_complete(value);"))?;
            f.writeln("}")?;

            f.newline()?;
            f.writeln("fn error(&self, value: Self::Error) {")?;
            indented(f, |f| f.writeln("self.on_failure(value);"))?;
            f.writeln("}")?;
            Ok(())
        })?;
        Ok(())
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
            f.writeln(&format!("unsafe impl Send for {name} {{}}"))?;
            f.writeln(&format!("unsafe impl Sync for {name} {{}}"))?;
        }

        f.newline()?;

        // Helper impl
        f.writeln(&format!("impl {name}"))?;
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
                    let call = format!("cb({params})");

                    if let Some(v) = &callback.return_type.get_value() {
                        f.writeln(&format!("self.{}.map(|cb| ", callback.name))?;
                        blocked(f, |f| {
                            if let Some(conversion) = v.conversion() {
                                f.writeln(&format!("let _result = {call};"))?;
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
