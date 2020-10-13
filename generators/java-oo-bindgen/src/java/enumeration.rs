use super::doc::*;
use super::*;
use heck::{CamelCase, ShoutySnakeCase};
use oo_bindgen::native_enum::*;

pub(crate) fn generate(
    f: &mut impl Printer,
    native_enum: &NativeEnumHandle,
    lib: &Library,
) -> FormattingResult<()> {
    let enum_name = native_enum.name.to_camel_case();

    // Documentation
    documentation(f, |f| javadoc_print(f, &native_enum.doc, lib))?;

    // Enum definition
    f.writeln(&format!("public enum {}", enum_name))?;
    blocked(f, |f| {
        // Write the variants
        for variant in &native_enum.variants {
            documentation(f, |f| javadoc_print(f, &variant.doc, lib))?;
            f.writeln(&format!("{},", variant.name.to_shouty_snake_case()))?;
        }
        f.write(";")?;

        Ok(())
    })
}
