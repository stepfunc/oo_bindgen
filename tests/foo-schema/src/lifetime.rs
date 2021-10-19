use oo_bindgen::iterator::IteratorHandle;
use oo_bindgen::types::{BasicType, STRING_TYPE};
use oo_bindgen::*;

fn define_inner_iterator(lib: &mut LibraryBuilder) -> Result<IteratorHandle, BindingError> {
    let inner_byte_iterator = lib.declare_class("InnerByteIterator")?;
    let byte_value = lib.declare_struct("ByteValue")?;
    let iterator_next_fn = lib
        .define_function("next_byte_value")
        .param("it", inner_byte_iterator, "inner byte iterator")?
        .returns(byte_value.clone(), "next byte value")?
        .doc("returns the next byte value")?
        .build()?;

    let byte_value = lib
        .define_function_return_struct(&byte_value)?
        .add("value", BasicType::Uint8, "byte")?
        .doc("item type for inner iterator")?
        .end_fields()?
        .build()?;

    lib.define_iterator_with_lifetime(&iterator_next_fn, &byte_value)
}

fn define_outer_iter(lib: &mut LibraryBuilder) -> Result<IteratorHandle, BindingError> {
    let inner_iter = define_inner_iterator(lib)?;

    let chunk_iterator = lib.declare_class("ChunkIterator")?;

    let chunk = lib.declare_struct("Chunk")?;

    let iterator_next_fn = lib
        .define_function("next_chunk")
        .param("it", chunk_iterator, "iterator over chunks of bytes")?
        .returns(chunk.clone(), "next chunk struct")?
        .doc("returns the next Chunk struct")?
        .build()?;

    let byte_values = lib
        .define_function_return_struct(&chunk)?
        .add(
            "iter",
            inner_iter,
            "inner iterator over individual byte values",
        )?
        .doc("Structure with an iterator with a lifetime")?
        .end_fields()?
        .build()?;

    lib.define_iterator_with_lifetime(&iterator_next_fn, &byte_values)
}

pub fn define(lib: &mut LibraryBuilder) -> Result<(), BindingError> {
    let outer_iter = define_outer_iter(lib)?;

    let interface = lib
        .define_interface(
            "ChunkReceiver",
            "Callback interface for chunks of a byte array",
        )
        .callback("on_chunk", "callback to bytes")?
        .param("values", outer_iter, "iterator over an iterator of bytes")?
        .returns_nothing()?
        .build()?
        .build()?;

    let invoke_fn = lib
        .define_function("iterate_string_by_chunks")
        .doc("iterate over a string by invoking the callback interface with chunks of the string")?
        .param(
            "values",
            STRING_TYPE,
            "String to pass to the callback interface",
        )?
        .param("chunk_size", BasicType::Uint32, "size of each iteration")?
        .param("callback", interface, "callback interface to invoke")?
        .returns_nothing()?
        .build()?;

    lib.define_static_class("DoubleIteratorTestHelper")
        .doc("Helper methods for the double iterator tests with lifetimes")?
        .static_method("IterateStringByChunks", &invoke_fn)?
        .build()?;

    Ok(())
}
