use oo_bindgen::constants::*;
use oo_bindgen::*;

pub fn define(lib: &mut LibraryBuilder) -> Result<(), BindingError> {
    lib.define_constants("SpecialValues")?
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
