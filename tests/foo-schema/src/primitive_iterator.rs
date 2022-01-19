use oo_bindgen::model::*;

pub fn define(lib: &mut LibraryBuilder) -> BackTraced<()> {
    // iterators can only be used in callback arguments, so we need an interface
    let iterator = lib.define_iterator("range_iterator", Primitive::U32)?;

    let interface = lib
        .define_interface(
            "range_receiver",
            "Callback interface for receiving a range of u32",
        )?
        .begin_callback("on_range", "callback to receive a range of u32")?
        .param("values", iterator, "iterator of values")?
        .enable_functional_transform()
        .end_callback()?
        .build_sync()?;

    let invoke_fn = lib
        .define_function("invoke_range_callback")?
        .doc("invokes the callback with an iterator over the elements")?
        .param("min", Primitive::U32, "minimum value of the range")?
        .param("max", Primitive::U32, "maximum value of the range")?
        .param("callback", interface, "callback interface to invoke")?
        .build_static_with_same_name()?;

    lib.define_static_class("range_iterator_test_helper")?
        .doc("Helper methods for the iterator tests")?
        .static_method(invoke_fn)?
        .build()?;

    Ok(())
}
