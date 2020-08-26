use crate::*;
use heck::{CamelCase, ShoutySnakeCase};
use oo_bindgen::native_enum::*;

pub(crate) fn generate(
    f: &mut impl Printer,
    native_enum: &NativeEnumHandle,
    lib: &Library,
) -> FormattingResult<()> {
    let enum_name = native_enum.name.to_camel_case();

    // Documentation
    if !native_enum.doc.is_empty() {
        documentation(f, |f| {
            f.newline()?;
            doc_print(f, &native_enum.doc, lib)?;
            Ok(())
        })?;
    }

    // Enum definition
    f.writeln(&format!("public enum {}", enum_name))?;
    blocked(f, |f| {
        // Write the variants
        for variant in &native_enum.variants {
            documentation(f, |f| {
                f.newline()?;
                doc_print(f, &variant.doc, lib)?;
                Ok(())
            })?;
            f.writeln(&format!("{},", variant.name.to_shouty_snake_case()))?;
        }
        f.write(";")?;

        f.newline()?;

        // Write the conversion routines
        f.writeln(&format!("static int toNative({} value)", enum_name))?;
        blocked(f, |f| {
            f.writeln("switch(value)")?;
            blocked(f, |f| {
                for variant in &native_enum.variants {
                    f.writeln(&format!(
                        "case {}: return {};",
                        variant.name.to_shouty_snake_case(),
                        variant.value
                    ))?;
                }
                f.writeln(&format!(
                    "default: throw new RuntimeException(\"Unknown {} value: \" + value);",
                    enum_name
                ))
            })
        })?;

        f.newline()?;

        f.writeln(&format!("static {} fromNative(int value)", enum_name))?;
        blocked(f, |f| {
            f.writeln("switch(value)")?;
            blocked(f, |f| {
                for variant in &native_enum.variants {
                    f.writeln(&format!(
                        "case {}: return {};",
                        variant.value,
                        variant.name.to_shouty_snake_case()
                    ))?;
                }
                f.writeln(&format!(
                    "default: throw new RuntimeException(\"Unknown {} value: \" + value);",
                    enum_name
                ))
            })
        })?;

        Ok(())
    })
}
