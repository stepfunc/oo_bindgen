use oo_bindgen::model::*;

pub fn define(lib: &mut LibraryBuilder) -> BackTraced<()> {
    lib.define_constants("special_values")?
        .add(
            "one",
            ConstantValue::U8(1, Representation::Hex),
            "the value 1",
        )?
        .add(
            "two",
            ConstantValue::U8(2, Representation::Hex),
            "the value 2",
        )?
        .doc("some special values")?
        .build()?;

    Ok(())
}
