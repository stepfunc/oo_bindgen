use oo_bindgen::*;
use oo_bindgen::formatting::*;
use oo_bindgen::interface::*;
use heck::{CamelCase, MixedCase};
use crate::*;

pub(crate) fn generate(f: &mut dyn Printer, interface: &InterfaceHandle, lib: &Library) -> FormattingResult<()> {
    let interface_name = interface.name.to_camel_case();

    print_license(f, &lib.license)?;

    f.writeln("using System;")?;
    f.writeln("using System.Runtime.InteropServices;")?;
    f.newline()?;

    namespaced(f, &lib.name, |f| {
        f.writeln(&format!("public interface {}", interface_name))?;

        blocked(f, |f| {
            // Write each required method
            interface.callbacks()
                .filter(|func| { func.name != interface.destroy_name })
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
        f.writeln(&format!("internal struct {}NativeAdapter", interface_name))?;
        blocked(f, |f| {
            // Define each delegate type
            for el in &interface.elements {
                match el {
                    InterfaceElement::CallbackFunction(func) => {
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
                        f.writeln(&format!("private static {}_delegate {}_static_delegate = {}NativeAdapter.{}_cb;", func.name, func.name, interface_name, func.name))?;
                    },
                    InterfaceElement::DestroyFunction(name) => {
                        f.writeln(&format!("private delegate void {}_delegate(IntPtr arg);", name))?;
                        f.writeln(&format!("private static {}_delegate {}_static_delegate = {}NativeAdapter.{}_cb;", name, name, interface_name, name))?;
                    }
                    _ => (),
                }
            }

            f.newline()?;

            // Define each structure element that will be marshalled
            for el in &interface.elements {
                match el {
                    InterfaceElement::CallbackFunction(func) => {
                        f.writeln(&format!("private {}_delegate {};", func.name, func.name))?;
                    },
                    InterfaceElement::DestroyFunction(name) => {
                        f.writeln(&format!("private {}_delegate {};", name, name))?;
                    }
                    InterfaceElement::Arg(name) => {
                        f.writeln(&format!("public IntPtr {};", name))?;
                    }
                }
            }

            f.newline()?;

            // Define the constructor
            f.writeln(&format!("internal {}NativeAdapter({} impl)", interface_name, interface_name))?;
            blocked(f, |f| {
                f.writeln("var _handle = GCHandle.Alloc(impl);")?;
                f.newline()?;

                for el in &interface.elements {
                    match el {
                        InterfaceElement::CallbackFunction(func) => {
                            f.writeln(&format!("this.{} = {}NativeAdapter.{}_static_delegate;", func.name, interface_name, func.name))?;
                        },
                        InterfaceElement::DestroyFunction(name) => {
                            f.writeln(&format!("this.{} = {}NativeAdapter.{}_static_delegate;", name, interface_name, name))?;
                        }
                        InterfaceElement::Arg(name) => {
                            f.writeln(&format!("this.{} = GCHandle.ToIntPtr(_handle);", name))?;
                        },
                    }
    
                    f.newline()?;
                }
                Ok(())
            })?;

            // Define each delegate function
            for el in &interface.elements {
                match el {
                    InterfaceElement::CallbackFunction(func) => {
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
                            f.writeln(&format!("var _impl = ({})_handle.Target;", interface_name))?;
                            call_dotnet_function(f, func, "return ")
                        })?;

                        f.newline()?;
                    },
                    InterfaceElement::DestroyFunction(name) => {
                        f.writeln(&format!("internal static void {}_cb(IntPtr arg)", name))?;

                        blocked(f, |f| {
                            f.writeln("var _handle = GCHandle.FromIntPtr(arg);")?;
                            f.writeln("_handle.Free();")
                        })?;

                        f.newline()?;
                    }
                    InterfaceElement::Arg(_) => (),
                }
            }

            Ok(())
        })
    })
}

fn call_dotnet_function(f: &mut dyn Printer, method: &CallbackFunction, return_destination: &str) -> FormattingResult<()> {
    // Write the type conversions
    for param in method.params() {
        if let Some(converter) = DotnetType(&param.param_type).conversion() {
            converter.convert_from_native(f, &param.name, &format!("var _{} = ", param.name.to_mixed_case()))?;
        }
    }

    // Call the .NET function
    f.newline()?;
    let method_name = method.name.to_camel_case();
    if let ReturnType::Type(return_type) = &method.return_type {
        if DotnetType(&return_type).conversion().is_some() {
            f.write(&format!("var _result = _impl.{}(", method_name))?;
        } else {
            f.write(&format!("{}_impl.{}(", return_destination, method_name))?;
        }
    } else {
        f.write(&format!("_impl.{}(", method_name))?;
    }

    f.write(
        &method.params()
            .map(|param| {
                DotnetType(&param.param_type).as_dotnet_arg(&param.name)
            })
            .collect::<Vec<String>>()
            .join(", ")
    )?;
    f.write(");")?;

    // Convert the result (if required)
    if let ReturnType::Type(return_type) = &method.return_type {
        if let Some(converter) = DotnetType(&return_type).conversion() {
            converter.convert_to_native(f, "_result", return_destination)?;
        }
    }

    Ok(())
}
