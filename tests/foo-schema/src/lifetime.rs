use oo_bindgen::native_function::*;
use oo_bindgen::*;

pub fn define(lib: &mut LibraryBuilder) -> Result<(), BindingError> {
    let iterator_class = lib.declare_class("IteratorWithLifeTime")?;
    let iterator_item = lib.declare_native_struct("IteratorItem")?;
    let iterator_next_fn = lib
        .declare_native_function("next_value_with_lifetime")?
        .param("it", Type::ClassRef(iterator_class), "Iterator")?
        .return_type(ReturnType::new(
            Type::StructRef(iterator_item.clone()),
            "Iterator Value",
        ))?
        .doc("test")?
        .build()?;

    let iterator_item = lib
        .define_native_struct(&iterator_item)?
        .add("value", Type::Uint8, "test")?
        .doc("item type for iterator")?
        .build()?;

    let iter = lib.define_iterator_with_lifetime(&iterator_next_fn, &iterator_item)?;

    let outer = lib.declare_native_struct("StructWithIteratorWithLifeTime")?;

    lib.define_native_struct(&outer)?
        .add("iter", Type::Iterator(iter), "test")?
        .doc("Structure with an iterator with a lifetime")?
        .build()?;

    Ok(())
}
