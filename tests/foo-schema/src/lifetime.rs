use oo_bindgen::types::BasicType;
use oo_bindgen::*;

pub fn define(lib: &mut LibraryBuilder) -> Result<(), BindingError> {
    let iterator_class = lib.declare_class("IteratorWithLifeTime")?;
    let iterator_item = lib.declare_struct("IteratorItem")?;
    let iterator_next_fn = lib
        .define_function("next_value_with_lifetime")?
        .param("it", iterator_class, "Iterator")?
        .returns(iterator_item.clone(), "Iterator Value")?
        .doc("test")?
        .build()?;

    let iterator_item = lib
        .define_any_struct(&iterator_item)?
        .add("value", BasicType::Uint8, "test")?
        .doc("item type for iterator")?
        .build()?;

    let iter = lib.define_iterator_with_lifetime(&iterator_next_fn, &iterator_item)?;

    let outer = lib.declare_struct("StructWithIteratorWithLifeTime")?;

    lib.define_any_struct(&outer)?
        .add("iter", iter, "test")?
        .doc("Structure with an iterator with a lifetime")?
        .build()?;

    Ok(())
}
