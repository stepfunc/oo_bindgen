use oo_bindgen::model::*;

pub fn define(lib: &mut LibraryBuilder) -> BackTraced<()> {
    // iterators can only be used in callback arguments, so we need an interface
    let iterator = lib.define_iterator("int32_iterator", Primitive::S32)?;

    let interface = lib
        .define_interface(
            "int_value_receiver",
            "Callback interface for receiving integer values",
        )?
        .begin_callback("on_int32", "callback to receive character values")?
        .param("value", iterator, "iterator of values")?
        .enable_functional_transform()
        .end_callback()?
        .build_sync()?;

    let invoke_fn = lib
        .define_function("invoke_int32_callback")?
        .doc("invokes the callback with an iterator over the elements")?
        .param("callback", interface, "callback interface to invoke")?
        .build_static_with_same_name()?;

    lib.define_static_class("int32_iterator_test_helper")?
        .doc("Helper methods for the iterator tests")?
        .static_method(invoke_fn)?
        .build()?;

    Ok(())
}
