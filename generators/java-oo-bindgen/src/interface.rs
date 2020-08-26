use crate::*;
use heck::{CamelCase, MixedCase};
use oo_bindgen::callback::*;
use oo_bindgen::native_function::*;

pub(crate) fn generate(
    f: &mut dyn Printer,
    interface: &InterfaceHandle,
    lib: &Library,
) -> FormattingResult<()> {
    let interface_name = interface.name.to_camel_case();

    documentation(f, |f| {
        // Print top-level documentation
        f.newline()?;
        doc_print(f, &interface.doc, lib)
    })?;

    f.writeln(&format!("public interface {}", interface_name))?;
    blocked(f, |f| {
        // Write each required method
        for func in interface
            .callbacks()
            .filter(|func| func.name != interface.destroy_name)
        {
            // Documentation
            documentation(f, |f| {
                // Print top-level documentation
                f.newline()?;
                doc_print(f, &func.doc, lib)?;

                // Print each parameter value
                for param in &func.parameters {
                    if let CallbackParameter::Parameter(param) = param {
                        f.writeln(&format!("@param {} ", param.name))?;
                        doc_print(f, &param.doc, lib)?;
                    }
                }

                // Print return value
                if let ReturnType::Type(_, doc) = &func.return_type {
                    f.writeln("@return ")?;
                    doc_print(f, doc, lib)?;
                }

                Ok(())
            })?;

            // Callback signature
            f.writeln(&format!(
                "{} {}(",
                JavaReturnType(&func.return_type).as_java_type(),
                func.name.to_mixed_case()
            ))?;
            f.write(
                &func
                    .parameters
                    .iter()
                    .filter_map(|param| match param {
                        CallbackParameter::Parameter(param) => Some(format!(
                            "{} {}",
                            JavaType(&param.param_type).as_java_type(),
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
        let field_order = interface
            .elements
            .iter()
            .map(|el| match el {
                InterfaceElement::CallbackFunction(func) => {
                    format!("\"{}\"", func.name.to_mixed_case())
                }
                InterfaceElement::DestroyFunction(name) => format!("\"{}\"", name.to_mixed_case()),
                InterfaceElement::Arg(name) => format!("\"{}\"", name.to_mixed_case()),
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
                f.writeln(&format!("public ByValue({} impl)", interface_name))?;
                blocked(f, |f| f.writeln("super(impl);"))
            })?;

            // Define each callback
            for el in &interface.elements {
                match el {
                    InterfaceElement::CallbackFunction(func) => {
                        f.writeln(&format!(
                            "public com.sun.jna.Callback {} = new com.sun.jna.Callback()",
                            func.name.to_mixed_case()
                        ))?;

                        f.writeln("{")?;
                        indented(f, |f| {
                            f.writeln(&format!(
                                "public {} callback(",
                                JavaReturnType(&func.return_type).as_native_type()
                            ))?;
                            f.write(
                                &func
                                    .parameters
                                    .iter()
                                    .map(|param| match param {
                                        CallbackParameter::Parameter(param) => format!(
                                            "{} {}",
                                            JavaType(&param.param_type).as_native_type(),
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
                                    interface_name,
                                    func.arg_name.to_mixed_case(),
                                ))?;
                                f.writeln("if(_arg != null)")?;
                                blocked(f, |f| call_java_function(f, func, "return "))
                            })
                        })?;
                        f.writeln("};")?;
                    }
                    InterfaceElement::DestroyFunction(name) => {
                        f.writeln(&format!(
                            "public com.sun.jna.Callback {} = new com.sun.jna.Callback()",
                            name.to_mixed_case()
                        ))?;
                        f.writeln("{")?;
                        indented(f, |f| {
                            f.writeln("public void callback(com.sun.jna.Pointer data)")?;

                            blocked(f, |f| f.writeln("NativeAdapter._impls.remove(data);"))
                        })?;
                        f.writeln("};")?;
                    }
                    InterfaceElement::Arg(name) => {
                        f.writeln(&format!(
                            "public com.sun.jna.Pointer {};",
                            name.to_mixed_case()
                        ))?;
                    }
                }
            }
            f.writeln(&format!("{} _impl;", interface_name))?;

            f.newline()?;

            // No-arg constructor for JNA
            f.writeln("public NativeAdapter() { }")?;

            f.newline()?;

            // Define the constructor
            f.writeln(&format!("NativeAdapter({} impl)", interface_name))?;
            blocked(f, |f| {
                f.writeln("long _id = NativeAdapter._nextId.incrementAndGet();")?;
                f.writeln(&format!(
                    "this.{} = new com.sun.jna.Pointer(_id);",
                    interface.arg_name.to_mixed_case()
                ))?;
                f.writeln("this._impl = impl;")?;
                f.writeln(&format!(
                    "NativeAdapter._impls.put(this.{}, this);",
                    interface.arg_name.to_mixed_case()
                ))
            })?;

            f.newline()?;

            // Define the set of interfaces to keep them alive when they are in
            // the native heap
            f.writeln(&format!("final static java.util.Map<com.sun.jna.Pointer, {}.NativeAdapter> _impls = java.util.Collections.synchronizedMap(new java.util.HashMap<>());", interface_name))?;
            f.writeln("final static java.util.concurrent.atomic.AtomicLong _nextId = new java.util.concurrent.atomic.AtomicLong();")
        })
    })
}
