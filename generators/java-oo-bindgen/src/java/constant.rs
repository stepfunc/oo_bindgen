use super::doc::*;
use super::*;
use oo_bindgen::constants::*;
use oo_bindgen::doc::Validated;

pub(crate) fn generate(
    f: &mut impl Printer,
    set: &Handle<ConstantSet<Validated>>,
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

    let set_name = set.name.camel_case();

    // Documentation
    documentation(f, |f| javadoc_print(f, &set.doc))?;

    // class definition
    f.writeln(&format!("public final class {}", set_name))?;
    blocked(f, |f| {
        f.writeln("// not constructable")?;
        f.writeln(&format!("private {}() {{}}", set_name))?;

        // Write the values
        for constant in &set.values {
            documentation(f, |f| javadoc_print(f, &constant.doc))?;
            f.writeln(&format!(
                "public static final {} {} = {};",
                get_type_as_string(&constant.value),
                constant.name.capital_snake_case(),
                get_value_as_string(&constant.value)
            ))?;
        }
        Ok(())
    })
}
