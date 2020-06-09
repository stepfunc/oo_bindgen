use oo_bindgen::*;
use oo_bindgen::callback::*;
use oo_bindgen::formatting::*;
use heck::{CamelCase, MixedCase};
use crate::*;

pub(crate) fn generate(f: &mut dyn Printer, cb: &OneTimeCallbackHandle, lib: &Library) -> FormattingResult<()> {
    let cb_name = cb.name.to_camel_case();

    print_license(f, &lib.license)?;

    f.writeln("using System;")?;
    f.writeln("using System.Runtime.InteropServices;")?;
    f.newline()?;

    namespaced(f, &lib.name, |f| {
        f.writeln(&format!("public interface {}", cb_name))?;

        blocked(f, |f| {
            // Write each required method
            cb.callbacks()
                .map(|func| {
                    f.writeln(&format!("{} {}(", DotnetReturnType(&func.return_type).as_dotnet_type(), func.name.to_camel_case()))?;
                    f.write(
                        &func.parameters.iter()
                            .filter_map(|param| {
                                match param {
                                    CallbackParameter::Parameter(param) => Some(format!("{} {}", DotnetType(&param.param_type).as_dotnet_type(), param.name.to_mixed_case())),
                                    _ => None
                                }
                            })
                            .collect::<Vec<String>>()
                            .join(", ")
                    )?;
                    f.write(");")
                }).collect()
        })?;

        f.newline()?;

        // Create the native adapter
        f.writeln("[StructLayout(LayoutKind.Sequential)]")?;
        f.writeln(&format!("internal struct {}NativeAdapter", cb_name))?;
        blocked(f, |f| {
            // Define each delegate type
            for el in &cb.elements {
                match el {
                    OneTimeCallbackElement::CallbackFunction(func) => {
                        f.writeln(&format!("private delegate {} {}_delegate(", DotnetReturnType(&func.return_type).as_native_type(), func.name))?;
                        f.write(
                            &func.parameters.iter()
                                .map(|param| {
                                    match param {
                                        CallbackParameter::Parameter(param) => format!("{} {}", DotnetType(&param.param_type).as_native_type(), param.name.to_mixed_case()),
                                        CallbackParameter::Arg(name) => format!("IntPtr {}", name),
                                    }
                                })
                                .collect::<Vec<String>>()
                                .join(", ")
                        )?;
                        f.write(");")?;
                        f.writeln(&format!("private static {}_delegate {}_static_delegate = {}NativeAdapter.{}_cb;", func.name, func.name, cb_name, func.name))?;
                    },
                    OneTimeCallbackElement::Arg(_) => (),
                }
            }

            f.newline()?;

            // Define each structure element that will be marshalled
            for el in &cb.elements {
                match el {
                    OneTimeCallbackElement::CallbackFunction(func) => {
                        f.writeln(&format!("private {}_delegate {};", func.name, func.name))?;
                    },
                    OneTimeCallbackElement::Arg(name) => {
                        f.writeln(&format!("public IntPtr {};", name))?;
                    }
                }
            }

            f.newline()?;

            // Define the constructor
            f.writeln(&format!("internal {}NativeAdapter({} impl)", cb_name, cb_name))?;
            blocked(f, |f| {
                f.writeln("var _handle = GCHandle.Alloc(impl);")?;
                f.newline()?;

                for el in &cb.elements {
                    match el {
                        OneTimeCallbackElement::CallbackFunction(func) => {
                            f.writeln(&format!("this.{} = {}NativeAdapter.{}_static_delegate;", func.name, cb_name, func.name))?;
                        },
                        OneTimeCallbackElement::Arg(name) => {
                            f.writeln(&format!("this.{} = GCHandle.ToIntPtr(_handle);", name))?;
                        },
                    }
    
                    f.newline()?;
                }
                Ok(())
            })?;

            // Define each delegate function
            for el in &cb.elements {
                match el {
                    OneTimeCallbackElement::CallbackFunction(func) => {
                        f.writeln(&format!("internal static {} {}_cb(", DotnetReturnType(&func.return_type).as_native_type(), func.name))?;
                        f.write(
                            &func.parameters.iter()
                                .map(|param| {
                                    match param {
                                        CallbackParameter::Parameter(param) => format!("{} {}", DotnetType(&param.param_type).as_native_type(), param.name.to_mixed_case()),
                                        CallbackParameter::Arg(name) => format!("IntPtr {}", name),
                                    }
                                })
                                .collect::<Vec<String>>()
                                .join(", ")
                        )?;
                        f.write(")")?;

                        blocked(f, |f| {
                            f.writeln(&format!("var _handle = GCHandle.FromIntPtr({});", func.arg_name))?;
                            f.writeln(&format!("var _impl = ({})_handle.Target;", cb_name))?;
                            call_dotnet_function(f, func, "_result = ")?;
                            f.writeln("_handle.Free();")?;

                            if !func.return_type.is_void() {
                                f.writeln("return _result;")?;
                            }
                            Ok(())
                        })?;

                        f.newline()?;
                    },
                    OneTimeCallbackElement::Arg(_) => (),
                }
            }

            Ok(())
        })
    })
}
