use oo_bindgen::iterator::IteratorHandle;
use oo_bindgen::types::{BasicType, StringType};
use oo_bindgen::*;

fn define_inner_iterator(lib: &mut LibraryBuilder) -> Result<IteratorHandle, BindingError> {
    let byte_value = lib.declare_function_return_struct("byte_value")?;
    let byte_value = lib
        .define_function_return_struct(byte_value)?
        .add("value", BasicType::U8, "byte")?
        .doc("item type for inner iterator")?
        .end_fields()?
        .build()?;

    lib.define_iterator_with_lifetime("inner_byte_iterator", byte_value)
}

fn define_outer_iter(lib: &mut LibraryBuilder) -> Result<IteratorHandle, BindingError> {
    let inner_iter = define_inner_iterator(lib)?;
    let chunk = lib.declare_function_return_struct("chunk")?;
    let chunk = lib
        .define_function_return_struct(chunk)?
        .add(
            "iter",
            inner_iter,
            "inner iterator over individual byte values",
        )?
        .doc("Structure with an iterator with a lifetime")?
        .end_fields()?
        .build()?;

    lib.define_iterator_with_lifetime("chunk_iterator", chunk)
}

pub fn define(lib: &mut LibraryBuilder) -> Result<(), BindingError> {
    let outer_iter = define_outer_iter(lib)?;

    let interface = lib
        .define_synchronous_interface(
            "chunk_receiver",
            "Callback interface for chunks of a byte array",
        )?
        .begin_callback("on_chunk", "callback to bytes")?
        .param("values", outer_iter, "iterator over an iterator of bytes")?
        .returns_nothing()?
        .end_callback()?
        .build()?;

    let invoke_fn = lib
        .define_function("iterate_string_by_chunks")?
        .doc("iterate over a string by invoking the callback interface with chunks of the string")?
        .param(
            "values",
            StringType,
            "String to pass to the callback interface",
        )?
        .param("chunk_size", BasicType::U32, "size of each iteration")?
        .param("callback", interface, "callback interface to invoke")?
        .returns_nothing()?
        .build()?;

    lib.define_static_class("double_iterator_test_helper")?
        .doc("Helper methods for the double iterator tests with lifetimes")?
        .static_method("iterate_string_by_chunks", &invoke_fn)?
        .build()?;

    Ok(())
}
