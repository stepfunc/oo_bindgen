use super::doc::*;
use super::*;
use heck::{CamelCase, ShoutySnakeCase};
use oo_bindgen::constants::*;

pub(crate) fn generate(
    f: &mut impl Printer,
    set: &ConstantSetHandle,
    lib: &Library,
) -> FormattingResult<()> {
    fn get_type_as_string(value: &ConstantValue) -> &'static str {
        match value {
            ConstantValue::U8(_, _) => "UByte",
        }
    }

    fn get_value_as_string(value: &ConstantValue) -> String {
        match value {
            ConstantValue::U8(x, Representation::Hex) => format!("UByte.valueOf(0x{:02X?})", x),
        }
    }

    let set_name = set.name.to_camel_case();

    // Documentation
    documentation(f, |f| javadoc_print(f, &set.doc, lib))?;

    // class definition
    f.writeln(&format!("public final class {}", set_name))?;
    blocked(f, |f| {
        f.writeln("// not constructable")?;
        f.writeln(&format!("private {}() {{}}", set_name))?;

        // Write the values
        for constant in &set.values {
            documentation(f, |f| javadoc_print(f, &constant.doc, lib))?;
            f.writeln(&format!(
                "public static final {} {} = {};",
                get_type_as_string(&constant.value),
                constant.name.to_shouty_snake_case(),
                get_value_as_string(&constant.value)
            ))?;
        }
        Ok(())
    })
}
