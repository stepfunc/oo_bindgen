use oo_bindgen::iterator::IteratorHandle;
use oo_bindgen::types::{BasicType, StringType};
use oo_bindgen::*;

fn define_iterator(lib: &mut LibraryBuilder) -> BindResult<IteratorHandle> {
    // Define the iterator next function
    // Must always take a class pointer as a param and return a struct pointer
    // (null if no other value available)
    let iterator_class = lib.declare_iterator("StringIterator")?;
    let iterator_item = lib.declare_function_return_struct("StringIteratorItem")?;
    let iterator_next_fn = lib
        .define_function("iterator_next")
        .param("it", iterator_class, "Iterator")?
        .returns(iterator_item.clone(), "Iterator value")?
        .doc("Get the next value, or NULL if the iterator reached the end")?
        .build()?;

    // Define the iterator item structure
    let iterator_item = lib
        .define_function_return_struct(iterator_item)?
        .add("value", BasicType::U8, "Character value")?
        .doc("Single iterator item")?
        .end_fields()?
        .build()?;

    // Define the actual iterator
    lib.define_iterator(&iterator_next_fn, iterator_item.into())
}

pub fn define(lib: &mut LibraryBuilder) -> BindResult<()> {
    // iterators can only be used in callback arguments, so we need an interface
    let iterator = define_iterator(lib)?;

    let interface = lib
        .define_synchronous_interface("ValuesReceiver", "Callback interface for receiving values")
        .begin_callback("on_characters", "callback to receive character values")?
        .param("values", iterator, "byte value for each character")?
        .returns_nothing()?
        .end_callback()?
        .build()?;

    let invoke_fn = lib
        .define_function("invoke_callback")
        .doc("invokes the callback with an iterator over the elements of the string")?
        .param(
            "values",
            StringType,
            "String to pass to the callback interface",
        )?
        .param("callback", interface, "callback interface to invoke")?
        .returns_nothing()?
        .build()?;

    lib.define_static_class("IteratorTestHelper")
        .doc("Helper methods for the iterator tests")?
        .static_method("InvokeCallback", &invoke_fn)?
        .build()?;

    Ok(())
}
