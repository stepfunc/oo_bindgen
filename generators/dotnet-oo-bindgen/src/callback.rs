use crate::*;
use heck::{CamelCase, MixedCase};
use oo_bindgen::callback::*;

pub(crate) fn generate(
    f: &mut dyn Printer,
    cb: &OneTimeCallbackHandle,
    lib: &Library,
) -> FormattingResult<()> {
    let cb_name = format!("I{}", cb.name.to_camel_case());

    print_license(f, &lib.license)?;
    print_imports(f)?;
    f.newline()?;

    namespaced(f, &lib.name, |f| {
        documentation(f, |f| {
            // Print top-level documentation
            xmldoc_print(f, &cb.doc, lib)
        })?;

        f.writeln(&format!("public interface {}", cb_name))?;
        blocked(f, |f| {
            // Write each required method
            cb.callbacks().try_for_each(|func| {
                // Documentation
                documentation(f, |f| {
                    // Print top-level documentation
                    xmldoc_print(f, &func.doc, lib)?;
                    f.newline()?;

                    // Print each parameter value
                    for param in &func.parameters {
                        if let CallbackParameter::Parameter(param) = param {
                            f.writeln(&format!("<param name=\"{}\">", param.name.to_mixed_case()))?;
                            docstring_print(f, &param.doc, lib)?;
                            f.write("</param>")?;
                        }
                    }

                    // Print return value
                    if let ReturnType::Type(_, doc) = &func.return_type {
                        f.writeln("<returns>")?;
                        docstring_print(f, doc, lib)?;
                        f.write("</returns>")?;
                    }

                    Ok(())
                })?;

                // Callback signature
                f.writeln(&format!(
                    "{} {}(",
                    func.return_type.as_dotnet_type(),
                    func.name.to_camel_case()
                ))?;
                f.write(
                    &func
                        .params()
                        .map(|param| {
                            format!(
                                "{} {}",
                                param.param_type.as_dotnet_type(),
                                param.name.to_mixed_case()
                            )
                        })
                        .collect::<Vec<String>>()
                        .join(", "),
                )?;
                f.write(");")
            })
        })?;

        f.newline()?;

        // Write the Action<>/Func<> based implementation if it's a functional interface
        if cb.is_functional() {
            generate_functional_callback(
                f,
                &cb_name,
                &cb.name.to_camel_case(),
                cb.callbacks().next().unwrap(),
            )?;
            f.newline()?;
        }

        // Create the native adapter
        f.writeln("[StructLayout(LayoutKind.Sequential)]")?;
        f.writeln(&format!("internal struct {}NativeAdapter", cb_name))?;
        blocked(f, |f| {
            // Define each delegate type
            for el in &cb.elements {
                match el {
                    OneTimeCallbackElement::CallbackFunction(func) => {
                        f.writeln(&format!(
                            "private delegate {} {}_delegate(",
                            func.return_type.as_native_type(),
                            func.name
                        ))?;
                        f.write(
                            &func
                                .parameters
                                .iter()
                                .map(|param| match param {
                                    CallbackParameter::Parameter(param) => format!(
                                        "{} {}",
                                        param.param_type.as_native_type(),
                                        param.name.to_mixed_case()
                                    ),
                                    CallbackParameter::Arg(name) => format!("IntPtr {}", name),
                                })
                                .collect::<Vec<String>>()
                                .join(", "),
                        )?;
                        f.write(");")?;
                        f.writeln(&format!("private static {}_delegate {}_static_delegate = {}NativeAdapter.{}_cb;", func.name, func.name, cb_name, func.name))?;
                    }
                    OneTimeCallbackElement::Arg(_) => (),
                }
            }

            f.newline()?;

            // Define each structure element that will be marshalled
            for el in &cb.elements {
                match el {
                    OneTimeCallbackElement::CallbackFunction(func) => {
                        f.writeln(&format!("private {}_delegate {};", func.name, func.name))?;
                    }
                    OneTimeCallbackElement::Arg(name) => {
                        f.writeln(&format!("public IntPtr {};", name))?;
                    }
                }
            }

            f.newline()?;

            // Define the constructor
            f.writeln(&format!(
                "internal {}NativeAdapter({} impl)",
                cb_name, cb_name
            ))?;
            blocked(f, |f| {
                f.writeln("var _handle = GCHandle.Alloc(impl);")?;
                f.newline()?;

                for el in &cb.elements {
                    match el {
                        OneTimeCallbackElement::CallbackFunction(func) => {
                            f.writeln(&format!(
                                "this.{} = {}NativeAdapter.{}_static_delegate;",
                                func.name, cb_name, func.name
                            ))?;
                        }
                        OneTimeCallbackElement::Arg(name) => {
                            f.writeln(&format!("this.{} = GCHandle.ToIntPtr(_handle);", name))?;
                        }
                    }

                    f.newline()?;
                }
                Ok(())
            })?;

            // Define each delegate function
            for el in &cb.elements {
                match el {
                    OneTimeCallbackElement::CallbackFunction(func) => {
                        f.writeln(&format!(
                            "internal static {} {}_cb(",
                            func.return_type.as_dotnet_type(),
                            func.name
                        ))?;
                        f.write(
                            &func
                                .parameters
                                .iter()
                                .map(|param| match param {
                                    CallbackParameter::Parameter(param) => format!(
                                        "{} {}",
                                        param.param_type.as_native_type(),
                                        param.name.to_mixed_case()
                                    ),
                                    CallbackParameter::Arg(name) => format!("IntPtr {}", name),
                                })
                                .collect::<Vec<String>>()
                                .join(", "),
                        )?;
                        f.write(")")?;

                        blocked(f, |f| {
                            f.writeln(&format!(
                                "var _handle = GCHandle.FromIntPtr({});",
                                func.arg_name
                            ))?;
                            f.writeln(&format!("var _impl = ({})_handle.Target;", cb_name))?;
                            call_dotnet_function(f, func, "var _result = ")?;
                            f.writeln("_handle.Free();")?;

                            if !func.return_type.is_void() {
                                f.writeln("return _result;")?;
                            }
                            Ok(())
                        })?;

                        f.newline()?;
                    }
                    OneTimeCallbackElement::Arg(_) => (),
                }
            }

            f.newline()?;

            // Write the conversion routine
            f.writeln(&format!(
                "internal static {} FromNative(IntPtr self)",
                cb_name
            ))?;
            blocked(f, |f| {
                f.writeln("if (self != IntPtr.Zero)")?;
                blocked(f, |f| {
                    f.writeln("var handle = GCHandle.FromIntPtr(self);")?;
                    f.writeln(&format!("return handle.Target as {};", cb_name))
                })?;
                f.writeln("else")?;
                blocked(f, |f| f.writeln("return null;"))
            })
        })
    })
}
