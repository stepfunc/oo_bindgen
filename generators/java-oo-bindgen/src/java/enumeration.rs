use super::doc::*;
use super::*;
use oo_bindgen::doc::Validated;
use oo_bindgen::enum_type::*;

pub(crate) fn generate(
    f: &mut impl Printer,
    native_enum: &Handle<Enum<Validated>>,
) -> FormattingResult<()> {
    let enum_name = native_enum.name.camel_case();

    // Documentation
    documentation(f, |f| javadoc_print(f, &native_enum.doc))?;

    // Enum definition
    f.writeln(&format!("public enum {}", enum_name))?;
    blocked(f, |f| {
        // Write the variants
        for variant in &native_enum.variants {
            documentation(f, |f| javadoc_print(f, &variant.doc))?;
            f.writeln(&format!(
                "{}({}),",
                variant.name.capital_snake_case(),
                variant.value
            ))?;
        }
        f.write(";")?;

        f.newline()?;

        f.writeln("final private int value;")?;

        f.newline()?;

        f.writeln(&format!("private {}(int value)", enum_name))?;
        blocked(f, |f| f.writeln("this.value = value;"))
    })
}
