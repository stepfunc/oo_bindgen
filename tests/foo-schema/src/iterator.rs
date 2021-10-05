//use oo_bindgen::types::{BasicType, STRING_TYPE};
use oo_bindgen::*;
use oo_bindgen::types::{BasicType, STRING_TYPE};
use oo_bindgen::iterator::IteratorHandle;

fn define_iterator(lib: &mut LibraryBuilder) -> BindResult<IteratorHandle> {
    // Define the iterator next function
    // Must always take a class pointer as a param and return a struct pointer
    // (null if no other value available)
    let iterator_class = lib.declare_class("StringIterator")?;
    let iterator_item = lib.declare_struct("StringIteratorItem")?;
    let iterator_next_fn = lib
        .define_function("iterator_next")
        .param("it", iterator_class.clone(), "Iterator")?
        .returns(iterator_item.clone(), "Iterator value")?
        .doc("Get the next value, or NULL if the iterator reached the end")?
        .build()?;


    // Define the iterator item structure
    let iterator_item = lib
        .define_rstruct(&iterator_item)?
        .add("value", BasicType::Uint8, "Character value")?
        .doc("Single iterator item")?
        .build()?;

    // Define the actual iterator
    lib.define_iterator(&iterator_next_fn, &iterator_item)
}

pub fn define(lib: &mut LibraryBuilder) -> BindResult<()> {

    // iterators can only be used in callback arguments, so we need an interface
    let iterator = define_iterator(lib)?;

    let interface = lib
        .define_interface("values_receiver", "Callback interface for receiving values")
        .callback("on_characters", "callback to receive character values")?
        .param("values", iterator, "byte value for each character")?
        .returns_nothing()?
        .build()?
        .build()?;


    let invoke_fn = lib
        .define_function("invoke_callback")
        .doc("invokes the callback with an iterator over the elements of the string")?
        .param("values", STRING_TYPE, "String to pass to the callback interface")?
        .param("callback", interface, "callback interface to invoke")?
        .returns_nothing()?
        .build()?;

    lib
        .define_static_class("IteratorTestHelper")
        .doc("Helper methods for the iterator tests")?
        .static_method("InvokeCallback", &invoke_fn)?
        .build()?;

    Ok(())
}
