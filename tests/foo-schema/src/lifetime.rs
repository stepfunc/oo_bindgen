use oo_bindgen::model::*;

fn define_inner_iterator(lib: &mut LibraryBuilder) -> BackTraced<AbstractIteratorHandle> {
    let byte_value = lib.declare_function_return_struct("byte_value")?;
    let byte_value = lib
        .define_function_return_struct(byte_value)?
        .add("value", BasicType::U8, "byte")?
        .doc("item type for inner iterator")?
        .end_fields()?
        .build()?;

    let iterator = lib.define_iterator_with_lifetime("inner_byte_iterator", byte_value)?;

    Ok(iterator)
}

fn define_outer_iter(lib: &mut LibraryBuilder) -> BackTraced<AbstractIteratorHandle> {
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

    let iterator = lib.define_iterator_with_lifetime("chunk_iterator", chunk)?;

    Ok(iterator)
}

pub fn define(lib: &mut LibraryBuilder) -> BackTraced<()> {
    let outer_iter = define_outer_iter(lib)?;

    let interface = lib
        .define_interface(
            "chunk_receiver",
            "Callback interface for chunks of a byte array",
        )?
        .begin_callback("on_chunk", "callback to bytes")?
        .param("values", outer_iter, "iterator over an iterator of bytes")?
        .enable_functional_transform()
        .end_callback()?
        .build_sync()?;

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
        .build_static_with_same_name()?;

    lib.define_static_class("double_iterator_test_helper")?
        .doc("Helper methods for the double iterator tests with lifetimes")?
        .static_method(invoke_fn)?
        .build()?;

    Ok(())
}
