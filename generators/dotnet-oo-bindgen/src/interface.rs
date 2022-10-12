use oo_bindgen::backend::*;
use oo_bindgen::model::*;

use crate::conversion::{base_functor_type, full_functor_type, TypeInfo};
use crate::doc::{docstring_print, xmldoc_print};
use crate::formatting::{documentation, namespaced};
use crate::helpers::call_dotnet_function;
use crate::{print_imports, print_license, TargetFramework};

pub(crate) fn generate(
    f: &mut dyn Printer,
    interface: &InterfaceType<Validated>,
    lib: &Library,
    framework: TargetFramework,
) -> FormattingResult<()> {
    let interface_name = format!("I{}", interface.name().camel_case());

    let destroy_func_name = lib.settings.interface.destroy_func_name.clone();
    let ctx_variable_name = lib.settings.interface.context_variable_name.clone();

    print_license(f, &lib.info.license_description)?;
    print_imports(f)?;
    f.newline()?;

    let is_private = interface
        .untyped()
        .get_functional_callback()
        .map(|cb| cb.functional_transform.enabled())
        .unwrap_or(false);
    let visibility = if is_private { "internal" } else { "public" };

    namespaced(f, &lib.settings.name, |f| {
        documentation(f, |f| {
            // Print top-level documentation
            xmldoc_print(f, interface.doc())
        })?;

        f.writeln(&format!("{} interface {}", visibility, interface_name))?;
        blocked(f, |f| {
            // Write each required method
            interface.untyped().callbacks.iter().try_for_each(|func| {
                // Documentation
                documentation(f, |f| {
                    // Print top-level documentation
                    xmldoc_print(f, &func.doc)?;
                    f.newline()?;

                    // Print each parameter value
                    for arg in &func.arguments {
                        f.writeln(&format!("<param name=\"{}\">", arg.name.mixed_case()))?;
                        docstring_print(f, &arg.doc)?;
                        f.write("</param>")?;
                    }

                    // Print return value
                    if let Some(doc) = &func.return_type.get_doc() {
                        f.writeln("<returns>")?;
                        docstring_print(f, doc)?;
                        f.write("</returns>")?;
                    }

                    Ok(())
                })?;

                // Callback signature
                f.writeln(&format!(
                    "{} {}(",
                    func.return_type.get_dotnet_type(),
                    func.name.camel_case()
                ))?;
                f.write(
                    &func
                        .arguments
                        .iter()
                        .map(|arg| {
                            format!(
                                "{} {}",
                                arg.arg_type.get_dotnet_type(),
                                arg.name.mixed_case()
                            )
                        })
                        .collect::<Vec<String>>()
                        .join(", "),
                )?;
                if func.default_implementation.is_some() && !framework.supports_default_interface_methods() {
                    tracing::warn!("Method {}::{} has a default implementation defined, but it cannot be supported in C# 7.X", interface.name().camel_case(), func.name.camel_case());
                }
                f.write(");")
            })
        })?;

        f.newline()?;

        // Write the Action<>/Func<> based implementation if it's a functional interface
        if let Some(callback) = interface.untyped().get_functional_callback() {
            namespaced(f, "functional", |f| {
                generate_functional_helpers(f, interface.untyped(), callback)
            })?;
            f.newline()?;
        }

        // write a Task-based implementation if it's a future interface
        if let InterfaceType::Future(fi) = interface {
            let class_name = fi.interface.name.camel_case();
            let value_type = fi.value_type.get_dotnet_type();
            let success_method_name = fi
                .interface
                .settings
                .future
                .success_callback_method_name
                .camel_case();

            f.writeln(&format!(
                "internal class {}: {}",
                class_name, interface_name
            ))?;
            blocked(f, |f| {
                f.writeln(&format!(
                    "private TaskCompletionSource<{}> tcs = new TaskCompletionSource<{}>();",
                    value_type, value_type
                ))?;
                f.newline()?;
                f.writeln(&format!(
                    "internal {}(TaskCompletionSource<{}> tcs)",
                    class_name, value_type
                ))?;
                blocked(f, |f| f.writeln("this.tcs = tcs;"))?;
                f.newline()?;
                f.writeln(&format!(
                    "void {}.{}({} value)",
                    interface_name, success_method_name, value_type
                ))?;
                blocked(f, |f| f.writeln("Task.Run(() => tcs.SetResult(value));"))?;
                f.newline()?;

                if let Some(err) = fi.error_type.get() {
                    let error_method_name = fi
                        .interface
                        .settings
                        .future
                        .failure_callback_method_name
                        .camel_case();
                    f.writeln(&format!(
                        "void {}.{}({} err)",
                        interface_name,
                        error_method_name,
                        err.inner.get_dotnet_type()
                    ))?;
                    blocked(f, |f| {
                        f.writeln(&format!(
                            "Task.Run(() => tcs.SetException(new {}(err)));",
                            err.exception_name.camel_case()
                        ))
                    })?;
                }

                Ok(())
            })?;
            f.newline()?;
        }

        // Create the native adapter
        f.writeln("[StructLayout(LayoutKind.Sequential)]")?;
        f.writeln(&format!("internal struct {}NativeAdapter", interface_name))?;
        blocked(f, |f| {
            // Define each delegate type
            for cb in &interface.untyped().callbacks {
                f.writeln("[UnmanagedFunctionPointer(CallingConvention.Cdecl)]")?; // C calling convetion
                f.writeln(&format!(
                    "private delegate {} {}_delegate(",
                    cb.return_type.get_native_type(),
                    cb.name
                ))?;
                f.write(
                    &cb.arguments
                        .iter()
                        .map(|arg| {
                            format!(
                                "{} {}",
                                arg.arg_type.get_native_type(),
                                arg.name.mixed_case()
                            )
                        })
                        .chain(std::iter::once(format!(
                            "IntPtr {}",
                            lib.settings.interface.context_variable_name
                        )))
                        .collect::<Vec<String>>()
                        .join(", "),
                )?;
                f.write(");")?;
                f.writeln(&format!(
                    "private static {}_delegate {}_static_delegate = {}NativeAdapter.{}_cb;",
                    cb.name, cb.name, interface_name, cb.name
                ))?;
            }

            f.writeln("[UnmanagedFunctionPointer(CallingConvention.Cdecl)]")?; // C calling convetion
            f.writeln(&format!(
                "private delegate void {}_delegate(IntPtr arg);",
                destroy_func_name
            ))?;

            f.writeln(&format!(
                "private static {}_delegate {}_static_delegate = {}NativeAdapter.{}_cb;",
                destroy_func_name, destroy_func_name, interface_name, destroy_func_name
            ))?;

            f.newline()?;

            // Define each structure element that will be marshalled
            for cb in &interface.untyped().callbacks {
                f.writeln(&format!("private {}_delegate {};", cb.name, cb.name))?;
            }

            f.writeln(&format!(
                "private {}_delegate {};",
                destroy_func_name, destroy_func_name
            ))?;
            f.writeln(&format!("public IntPtr {};", ctx_variable_name))?;

            f.newline()?;

            // Define the constructor
            f.writeln(&format!(
                "internal {}NativeAdapter({} impl)",
                interface_name, interface_name
            ))?;
            blocked(f, |f| {
                f.writeln("var _handle = GCHandle.Alloc(impl);")?;
                f.newline()?;

                for cb in &interface.untyped().callbacks {
                    f.writeln(&format!(
                        "this.{} = {}NativeAdapter.{}_static_delegate;",
                        cb.name, interface_name, cb.name
                    ))?;

                    f.newline()?;
                }

                f.writeln(&format!(
                    "this.{} = {}NativeAdapter.{}_static_delegate;",
                    destroy_func_name, interface_name, destroy_func_name
                ))?;

                f.writeln(&format!(
                    "this.{} = GCHandle.ToIntPtr(_handle);",
                    ctx_variable_name
                ))?;
                Ok(())
            })?;

            // Define each delegate function
            for cb in &interface.untyped().callbacks {
                f.writeln(&format!(
                    "internal static {} {}_cb(",
                    cb.return_type.get_native_type(),
                    cb.name
                ))?;
                f.write(
                    &cb.arguments
                        .iter()
                        .map(|arg| {
                            format!(
                                "{} {}",
                                arg.arg_type.get_native_type(),
                                arg.name.mixed_case()
                            )
                        })
                        .chain(std::iter::once(format!("IntPtr {}", ctx_variable_name)))
                        .collect::<Vec<String>>()
                        .join(", "),
                )?;
                f.write(")")?;

                blocked(f, |f| {
                    f.writeln(&format!(
                        "var _handle = GCHandle.FromIntPtr({});",
                        ctx_variable_name
                    ))?;
                    f.writeln(&format!("var _impl = ({})_handle.Target;", interface_name))?;
                    call_dotnet_function(f, cb, "return ")
                })?;

                f.newline()?;
            }

            // destroy delegate
            f.writeln(&format!(
                "internal static void {}_cb(IntPtr arg)",
                destroy_func_name
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

pub(crate) fn generate_interface_implementation(
    f: &mut dyn Printer,
    interface: &Handle<Interface<Validated>>,
    cb: &CallbackFunction<Validated>,
) -> FormattingResult<()> {
    let functor_type = full_functor_type(cb);

    f.writeln(&format!(
        "internal class Implementation: I{}",
        interface.name.camel_case()
    ))?;
    blocked(f, |f| {
        f.writeln(&format!("private readonly {} action;", functor_type))?;
        f.newline()?;

        // constructor
        f.writeln(&format!("internal Implementation({} action)", functor_type))?;
        blocked(f, |f| f.writeln("this.action = action;"))?;

        f.newline()?;

        f.writeln(&format!(
            "public {} {}(",
            cb.return_type.get_dotnet_type(),
            cb.name.camel_case()
        ))?;
        f.write(
            &cb.arguments
                .iter()
                .map(|param| {
                    format!(
                        "{} {}",
                        param.arg_type.get_dotnet_type(),
                        param.name.mixed_case()
                    )
                })
                .collect::<Vec<_>>()
                .join(", "),
        )?;
        f.write(")")?;
        blocked(f, |f| {
            f.newline()?;

            if !cb.return_type.is_none() {
                f.write("return ")?;
            }

            let params = cb
                .arguments
                .iter()
                .map(|param| param.name.mixed_case())
                .collect::<Vec<_>>()
                .join(", ");

            f.write(&format!("this.action.Invoke({});", params))
        })
    })
}

pub(crate) fn generate_functional_helpers(
    f: &mut dyn Printer,
    interface: &Handle<Interface<Validated>>,
    cb: &CallbackFunction<Validated>,
) -> FormattingResult<()> {
    let interface_name = format!("I{}", interface.name.camel_case());
    let class_name = interface.name.camel_case();
    let functor_type = full_functor_type(cb);

    let visibility = if cb.functional_transform.enabled() {
        "internal"
    } else {
        "public"
    };

    documentation(f, |f| {
        f.writeln("<summary>")?;
        f.writeln(&format!(
            "Provides a method to create an implementation of {} from a functor",
            interface_name
        ))?;
        f.writeln("</summary>")
    })?;
    f.writeln(&format!("{} static class {}", visibility, class_name))?;
    blocked(f, |f| {
        f.newline()?;
        // write the private implementation class
        generate_interface_implementation(f, interface, cb)?;
        f.newline()?;

        documentation(f, |f| {
            f.writeln("<summary>")?;
            f.write(&format!(
                "Creates an instance of {} which invokes a {}",
                interface_name,
                base_functor_type(cb)
            ))?;
            f.write("</summary>")?;
            f.newline()?;
            f.writeln("<param name=\"action\">")?;
            f.writeln("Callback to execute")?;
            f.writeln("</param>")?;
            f.writeln(&format!(
                "<return>An implementation of {}</return>",
                interface_name
            ))?;
            Ok(())
        })?;
        // write the factory function
        f.writeln(&format!(
            "{} static {} create({} action)",
            visibility, interface_name, functor_type
        ))?;
        blocked(f, |f| f.writeln("return new Implementation(action);"))?;

        Ok(())
    })
}
