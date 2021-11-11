use oo_bindgen::iterator::IteratorHandle;
use oo_bindgen::types::{BasicType, StringType};
use oo_bindgen::*;

fn define_iterator(lib: &mut LibraryBuilder) -> BackTraced<IteratorHandle> {
    // Define the iterator item structure
    let iterator_item = lib.declare_function_return_struct("string_iterator_item")?;
    let iterator_item = lib
        .define_function_return_struct(iterator_item)?
        .add("value", BasicType::U8, "Character value")?
        .doc("Single iterator item")?
        .end_fields()?
        .build()?;

    // Define the actual iterator
    let iterator = lib.define_iterator("string_iterator", iterator_item)?;

    Ok(iterator)
}

pub fn define(lib: &mut LibraryBuilder) -> BackTraced<()> {
    // iterators can only be used in callback arguments, so we need an interface
    let iterator = define_iterator(lib)?;

    let interface = lib
        .define_synchronous_interface("values_receiver", "Callback interface for receiving values")?
        .begin_callback("on_characters", "callback to receive character values")?
        .param("values", iterator, "byte value for each character")?
        .returns_nothing()?
        .enable_functional_transform()
        .end_callback()?
        .build()?;

    let invoke_fn = lib
        .define_function("invoke_callback")?
        .doc("invokes the callback with an iterator over the elements of the string")?
        .param(
            "values",
            StringType,
            "String to pass to the callback interface",
        )?
        .param("callback", interface, "callback interface to invoke")?
        .build_static_with_same_name()?;

    lib.define_static_class("iterator_test_helper")?
        .doc("Helper methods for the iterator tests")?
        .static_method(invoke_fn)?
        .build()?;

    Ok(())
}
