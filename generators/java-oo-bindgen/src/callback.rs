use crate::*;
use crate::doc::*;
use heck::{CamelCase, MixedCase};
use oo_bindgen::callback::*;
use oo_bindgen::native_function::*;

pub(crate) fn generate(
    f: &mut dyn Printer,
    callback: &OneTimeCallbackHandle,
    lib: &Library,
) -> FormattingResult<()> {
    let callback_name = callback.name.to_camel_case();

    f.writeln(&format!("public interface {}", callback_name))?;
    blocked(f, |f| {
        // Write each required method
        for func in callback.callbacks() {
            // Documentation
            documentation(f, |f| {
                // Print top-level documentation
                javadoc_print(f, &func.doc, lib)?;
                f.newline()?;

                // Print each parameter value
                for param in &func.parameters {
                    if let CallbackParameter::Parameter(param) = param {
                        f.writeln(&format!("@param {} ", param.name))?;
                        docstring_print(f, &param.doc, lib)?;
                    }
                }

                // Print return value
                if let ReturnType::Type(_, doc) = &func.return_type {
                    f.writeln("@return ")?;
                    docstring_print(f, doc, lib)?;
                }

                Ok(())
            })?;

            // Callback signature
            f.writeln(&format!(
                "{} {}(",
                func.return_type.as_java_type(),
                func.name.to_mixed_case()
            ))?;
            f.write(
                &func
                    .parameters
                    .iter()
                    .filter_map(|param| match param {
                        CallbackParameter::Parameter(param) => Some(format!(
                            "{} {}",
                            param.param_type.as_java_type(),
                            param.name.to_mixed_case()
                        )),
                        _ => None,
                    })
                    .collect::<Vec<String>>()
                    .join(", "),
            )?;
            f.write(");")?;
        }

        f.newline()?;

        // Create the native adapter
        let field_order = callback
            .elements
            .iter()
            .map(|el| match el {
                OneTimeCallbackElement::CallbackFunction(func) => {
                    format!("\"{}\"", func.name.to_mixed_case())
                }
                OneTimeCallbackElement::Arg(name) => format!("\"{}\"", name.to_mixed_case()),
            })
            .collect::<Vec<_>>()
            .join(", ");
        f.writeln(&format!(
            "@com.sun.jna.Structure.FieldOrder({{ {} }})",
            field_order
        ))?;

        f.writeln("class NativeAdapter extends com.sun.jna.Structure")?;
        blocked(f, |f| {
            // ByValue member
            f.writeln("public static class ByValue extends NativeAdapter implements com.sun.jna.Structure.ByValue")?;
            blocked(f, |f| {
                f.writeln("public ByValue() { }")?;
                f.writeln(&format!("public ByValue({} impl)", callback_name))?;
                blocked(f, |f| f.writeln("super(impl);"))
            })?;

            // Define each callback
            for el in &callback.elements {
                match el {
                    OneTimeCallbackElement::CallbackFunction(func) => {
                        f.writeln(&format!(
                            "public com.sun.jna.Callback {} = new com.sun.jna.Callback()",
                            func.name.to_mixed_case()
                        ))?;

                        f.writeln("{")?;
                        indented(f, |f| {
                            f.writeln(&format!(
                                "public {} callback(",
                                func.return_type.as_native_type()
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
                                        CallbackParameter::Arg(name) => {
                                            format!("com.sun.jna.Pointer {}", name.to_mixed_case())
                                        }
                                    })
                                    .collect::<Vec<String>>()
                                    .join(", "),
                            )?;
                            f.write(")")?;

                            blocked(f, |f| {
                                f.writeln(&format!(
                                    "{}.NativeAdapter _arg = NativeAdapter._impls.get({});",
                                    callback_name,
                                    func.arg_name.to_mixed_case(),
                                ))?;

                                f.writeln("if(_arg == null)")?;
                                blocked(f, |f| {
                                    f.writeln(&format!("throw new RuntimeException(\"Unknown callback of type \" + {}.class);", callback_name))
                                })?;

                                f.writeln(&format!(
                                    "NativeAdapter._impls.remove({});",
                                    func.arg_name.to_mixed_case()
                                ))?;
                                call_java_function(f, func, "return ")
                            })
                        })?;
                        f.writeln("};")?;
                    }
                    OneTimeCallbackElement::Arg(name) => {
                        f.writeln(&format!(
                            "public com.sun.jna.Pointer {};",
                            name.to_mixed_case()
                        ))?;
                    }
                }
            }
            f.writeln(&format!("{} _impl;", callback_name))?;

            f.newline()?;

            // No-arg constructor for JNA
            f.writeln("public NativeAdapter() { }")?;

            f.newline()?;

            // Define the constructor
            f.writeln(&format!("NativeAdapter({} impl)", callback_name))?;
            blocked(f, |f| {
                f.writeln("long _id = NativeAdapter._nextId.incrementAndGet();")?;
                f.writeln(&format!(
                    "this.{} = new com.sun.jna.Pointer(_id);",
                    callback.arg_name.to_mixed_case()
                ))?;
                f.writeln("this._impl = impl;")?;
                f.writeln(&format!(
                    "NativeAdapter._impls.put(this.{}, this);",
                    callback.arg_name.to_mixed_case()
                ))
            })?;

            f.newline()?;

            // Define the set of interfaces to keep them alive when they are in
            // the native heap
            f.writeln(&format!("final static java.util.Map<com.sun.jna.Pointer, {}.NativeAdapter> _impls = java.util.Collections.synchronizedMap(new java.util.HashMap<>());", callback_name))?;
            f.writeln("final static java.util.concurrent.atomic.AtomicLong _nextId = new java.util.concurrent.atomic.AtomicLong();")
        })
    })
}
