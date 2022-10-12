use super::doc::*;
use super::*;

trait ToConstantValue {
    fn get_constant_value(&self) -> String;
}

impl ToConstantValue for PrimitiveValue {
    fn get_constant_value(&self) -> String {
        match self {
            PrimitiveValue::Bool(x) => x.to_string(),
            PrimitiveValue::U8(x) => format!("UByte.valueOf({})", x),
            PrimitiveValue::S8(x) => x.to_string(),
            PrimitiveValue::U16(x) => format!("UShort.valueOf({})", x),
            PrimitiveValue::S16(x) => x.to_string(),
            PrimitiveValue::U32(x) => format!("UInteger.valueOf({})", x),
            PrimitiveValue::S32(x) => x.to_string(),
            PrimitiveValue::U64(x) => format!("ULong.valueOf({})", x),
            PrimitiveValue::S64(x) => x.to_string(),
            PrimitiveValue::Float(x) => x.to_string(),
            PrimitiveValue::Double(x) => x.to_string(),
        }
    }
}

impl ToConstantValue for DurationValue {
    fn get_constant_value(&self) -> String {
        match self {
            DurationValue::Milliseconds(x) => format!("java.time.Duration.ofMillis({})", x),
            DurationValue::Seconds(x) => format!("java.time.Duration.ofSeconds({})", x),
        }
    }
}

impl ToConstantValue for BasicValue {
    fn get_constant_value(&self) -> String {
        match self {
            BasicValue::Primitive(x) => x.get_constant_value(),
            BasicValue::Duration(x) => x.get_constant_value(),
            BasicValue::Enum(x) => {
                format!(
                    "{}.{}",
                    x.handle.name.camel_case(),
                    x.variant.name.capital_snake_case()
                )
            }
        }
    }
}

impl ToConstantValue for DefaultCallbackReturnValue {
    fn get_constant_value(&self) -> String {
        match self {
            DefaultCallbackReturnValue::Basic(x) => x.get_constant_value(),
            DefaultCallbackReturnValue::InitializedStruct(x) => {
                match x.initializer.initializer_type {
                    InitializerType::Normal => format!("new {}()", x.handle.name().camel_case()),
                    InitializerType::Static => format!(
                        "{}.{}()",
                        x.handle.name().camel_case(),
                        x.initializer.name.mixed_case()
                    ),
                }
            }
        }
    }
}

pub(crate) fn generate(
    f: &mut dyn Printer,
    interface: &Handle<Interface<Validated>>,
) -> FormattingResult<()> {
    let interface_name = interface.name.camel_case();

    documentation(f, |f| {
        // Print top-level documentation
        javadoc_print(f, &interface.doc)
    })?;

    if interface.is_functional() {
        f.writeln("@FunctionalInterface")?;
    }
    f.writeln(&format!("public interface {}", interface_name))?;
    blocked(f, |f| {
        // Write each required method
        for func in interface.callbacks.iter() {
            let constant_return_value = func
                .default_implementation
                .as_ref()
                .map(|x| x.get_constant_value());

            // Documentation
            documentation(f, |f| {
                // Print top-level documentation
                javadoc_print(f, &func.doc)?;
                f.newline()?;

                if let Some(v) = &constant_return_value {
                    f.writeln(&format!(
                        "<p>The default implementation of this method returns '{}'</p>",
                        v
                    ))?;
                    f.newline()?;
                }

                // Print each argument value
                for arg in &func.arguments {
                    f.writeln(&format!("@param {} ", arg.name.mixed_case()))?;
                    docstring_print(f, &arg.doc)?;
                }

                // Print return value
                if let Some(doc) = &func.return_type.get_doc() {
                    f.writeln("@return ")?;
                    docstring_print(f, doc)?;
                }

                Ok(())
            })?;

            let modifier = if func.default_implementation.is_some() {
                "default "
            } else {
                ""
            };

            // Callback signature
            f.writeln(&format!(
                "{}{} {}(",
                modifier,
                func.return_type.as_java_primitive(),
                func.name.mixed_case()
            ))?;
            f.write(
                &func
                    .arguments
                    .iter()
                    .map(|arg| {
                        format!(
                            "{} {}",
                            arg.arg_type.as_java_primitive(),
                            arg.name.mixed_case()
                        )
                    })
                    .collect::<Vec<String>>()
                    .join(", "),
            )?;

            match constant_return_value {
                None => f.write(");")?,
                Some(v) => {
                    f.write(")")?;
                    blocked(f, |f| f.writeln(&format!("return {};", v)))?;
                }
            }
            f.newline()?;
        }

        Ok(())
    })
}
