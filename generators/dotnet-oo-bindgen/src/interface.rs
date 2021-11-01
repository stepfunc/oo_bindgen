use crate::helpers::call_dotnet_function;
use crate::*;
use heck::{CamelCase, MixedCase};
use oo_bindgen::interface::*;

pub(crate) fn generate(
    f: &mut dyn Printer,
    interface: &InterfaceHandle,
    lib: &Library,
) -> FormattingResult<()> {
    let interface_name = format!("I{}", interface.name.to_camel_case());

    print_license(f, &lib.info.license_description)?;
    print_imports(f)?;
    f.newline()?;

    namespaced(f, &lib.settings.name, |f| {
        documentation(f, |f| {
            // Print top-level documentation
            xmldoc_print(f, &interface.doc, lib)
        })?;

        f.writeln(&format!("public interface {}", interface_name))?;
        blocked(f, |f| {
            // Write each required method
            interface.callbacks.iter().try_for_each(|func| {
                // Documentation
                documentation(f, |f| {
                    // Print top-level documentation
                    xmldoc_print(f, &func.doc, lib)?;
                    f.newline()?;

                    // Print each parameter value
                    for arg in &func.arguments {
                        f.writeln(&format!("<param name=\"{}\">", arg.name.to_mixed_case()))?;
                        docstring_print(f, &arg.doc, lib)?;
                        f.write("</param>")?;
                    }

                    // Print return value
                    if let CallbackReturnType::Type(_, doc) = &func.return_type {
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
                        .arguments
                        .iter()
                        .map(|arg| {
                            format!(
                                "{} {}",
                                arg.arg_type.as_dotnet_type(),
                                arg.name.to_mixed_case()
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
        if interface.is_functional() {
            generate_functional_callback(f, interface, interface.callbacks.first().unwrap(), lib)?;
            f.newline()?;
        }

        // Create the native adapter
        f.writeln("[StructLayout(LayoutKind.Sequential)]")?;
        f.writeln(&format!("internal struct {}NativeAdapter", interface_name))?;
        blocked(f, |f| {
            // Define each delegate type
            for cb in &interface.callbacks {
                f.writeln(&format!(
                    "private delegate {} {}_delegate(",
                    cb.return_type.as_native_type(),
                    cb.name
                ))?;
                f.write(
                    &cb.arguments
                        .iter()
                        .map(|arg| {
                            format!(
                                "{} {}",
                                arg.arg_type.as_native_type(),
                                arg.name.to_mixed_case()
                            )
                        })
                        .chain(std::iter::once(format!("IntPtr {}", CTX_VARIABLE_NAME)))
                        .collect::<Vec<String>>()
                        .join(", "),
                )?;
                f.write(");")?;
                f.writeln(&format!(
                    "private static {}_delegate {}_static_delegate = {}NativeAdapter.{}_cb;",
                    cb.name, cb.name, interface_name, cb.name
                ))?;
            }

            f.writeln(&format!(
                "private delegate void {}_delegate(IntPtr arg);",
                DESTROY_FUNC_NAME
            ))?;

            f.writeln(&format!(
                "private static {}_delegate {}_static_delegate = {}NativeAdapter.{}_cb;",
                DESTROY_FUNC_NAME, DESTROY_FUNC_NAME, interface_name, DESTROY_FUNC_NAME
            ))?;

            f.newline()?;

            // Define each structure element that will be marshalled
            for cb in &interface.callbacks {
                f.writeln(&format!("private {}_delegate {};", cb.name, cb.name))?;
            }

            f.writeln(&format!(
                "private {}_delegate {};",
                DESTROY_FUNC_NAME, DESTROY_FUNC_NAME
            ))?;
            f.writeln(&format!("public IntPtr {};", CTX_VARIABLE_NAME))?;

            f.newline()?;

            // Define the constructor
            f.writeln(&format!(
                "internal {}NativeAdapter({} impl)",
                interface_name, interface_name
            ))?;
            blocked(f, |f| {
                f.writeln("var _handle = GCHandle.Alloc(impl);")?;
                f.newline()?;

                for cb in &interface.callbacks {
                    f.writeln(&format!(
                        "this.{} = {}NativeAdapter.{}_static_delegate;",
                        cb.name, interface_name, cb.name
                    ))?;

                    f.newline()?;
                }

                f.writeln(&format!(
                    "this.{} = {}NativeAdapter.{}_static_delegate;",
                    DESTROY_FUNC_NAME, interface_name, DESTROY_FUNC_NAME
                ))?;

                f.writeln(&format!(
                    "this.{} = GCHandle.ToIntPtr(_handle);",
                    CTX_VARIABLE_NAME
                ))?;
                Ok(())
            })?;

            // Define each delegate function
            for cb in &interface.callbacks {
                f.writeln(&format!(
                    "internal static {} {}_cb(",
                    cb.return_type.as_native_type(),
                    cb.name
                ))?;
                f.write(
                    &cb.arguments
                        .iter()
                        .map(|arg| {
                            format!(
                                "{} {}",
                                arg.arg_type.as_native_type(),
                                arg.name.to_mixed_case()
                            )
                        })
                        .chain(std::iter::once(format!("IntPtr {}", CTX_VARIABLE_NAME)))
                        .collect::<Vec<String>>()
                        .join(", "),
                )?;
                f.write(")")?;

                blocked(f, |f| {
                    f.writeln(&format!(
                        "var _handle = GCHandle.FromIntPtr({});",
                        CTX_VARIABLE_NAME
                    ))?;
                    f.writeln(&format!("var _impl = ({})_handle.Target;", interface_name))?;
                    call_dotnet_function(f, cb, "return ")
                })?;

                f.newline()?;
            }

            // destroy delegate
            f.writeln(&format!(
                "internal static void {}_cb(IntPtr arg)",
                DESTROY_FUNC_NAME
            ))?;

            blocked(f, |f| {
                f.writeln("var _handle = GCHandle.FromIntPtr(arg);")?;
                f.writeln("_handle.Free();")
            })?;

            f.newline()?;

            f.newline()?;

            // Write the conversion routine
            f.writeln(&format!(
                "internal static {} FromNative(IntPtr self)",
                interface_name
            ))?;
            blocked(f, |f| {
                f.writeln("if (self != IntPtr.Zero)")?;
                blocked(f, |f| {
                    f.writeln("var handle = GCHandle.FromIntPtr(self);")?;
                    f.writeln(&format!("return handle.Target as {};", interface_name))
                })?;
                f.writeln("else")?;
                blocked(f, |f| f.writeln("return null;"))
            })
        })
    })
}

pub(crate) fn generate_functional_callback(
    f: &mut dyn Printer,
    interface: &InterfaceHandle,
    function: &CallbackFunction,
    lib: &Library,
) -> FormattingResult<()> {
    let interface_name = format!("I{}", interface.name.to_camel_case());
    let class_name = interface.name.to_camel_case();

    // Build the Action<>/Func<> signature
    let param_types = function
        .arguments
        .iter()
        .map(|param| param.arg_type.as_dotnet_type())
        .collect::<Vec<_>>()
        .join(", ");
    let action_type = match &function.return_type {
        CallbackReturnType::Type(return_type, _) => {
            if param_types.is_empty() {
                format!("Func<{}>", return_type.as_dotnet_type())
            } else {
                format!("Func<{}, {}>", param_types, return_type.as_dotnet_type())
            }
        }
        CallbackReturnType::Void => {
            if param_types.is_empty() {
                "Action".to_string()
            } else {
                format!("Action<{}>", param_types)
            }
        }
    };

    documentation(f, |f| {
        f.writeln("<summary>")?;
        docstring_print(
            f,
            &format!("Functional adapter of {{interface:{}}}", interface.name).into(),
            lib,
        )?;
        f.write("</summary>")
    })?;
    f.writeln(&format!("public class {} : {}", class_name, interface_name))?;
    blocked(f, |f| {
        f.writeln(&format!("private readonly {} action;", action_type))?;

        f.newline()?;

        // Write the constructor
        documentation(f, |f| {
            f.writeln("<summary>")?;
            f.write("Functional constructor")?;
            f.write("</summary>")?;
            f.newline()?;
            f.writeln("<param name=\"action\">")?;
            f.writeln("Callback to execute")?;
            f.writeln("</param>")?;
            Ok(())
        })?;
        f.writeln(&format!("public {}({} action)", class_name, action_type))?;
        blocked(f, |f| f.writeln("this.action = action;"))?;

        f.newline()?;

        // Write the required method
        documentation(f, |f| {
            xmldoc_print(f, &function.doc, lib)?;
            f.newline()?;

            // Print each parameter value
            for arg in &function.arguments {
                f.writeln(&format!("<param name=\"{}\">", arg.name.to_mixed_case()))?;
                docstring_print(f, &arg.doc, lib)?;
                f.write("</param>")?;
            }

            // Print return value
            if let CallbackReturnType::Type(_, doc) = &function.return_type {
                f.writeln("<returns>")?;
                docstring_print(f, doc, lib)?;
                f.write("</returns>")?;
            }
            Ok(())
        })?;
        f.writeln(&format!(
            "public {} {}(",
            function.return_type.as_dotnet_type(),
            function.name.to_camel_case()
        ))?;
        f.write(
            &function
                .arguments
                .iter()
                .map(|param| {
                    format!(
                        "{} {}",
                        param.arg_type.as_dotnet_type(),
                        param.name.to_mixed_case()
                    )
                })
                .collect::<Vec<_>>()
                .join(", "),
        )?;
        f.write(")")?;
        blocked(f, |f| {
            f.newline()?;

            if !function.return_type.is_void() {
                f.write("return ")?;
            }

            let params = function
                .arguments
                .iter()
                .map(|param| param.name.to_mixed_case())
                .collect::<Vec<_>>()
                .join(", ");

            f.write(&format!("this.action.Invoke({});", params))
        })
    })
}
