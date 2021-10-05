//use oo_bindgen::types::{BasicType, STRING_TYPE};
use oo_bindgen::*;

pub fn define(_lib: &mut LibraryBuilder) -> Result<(), BindingError> {
    // TODO - this whole iterator test must be rewritten

    /*
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
            .define_any_struct(&iterator_item)?
            .add("value", BasicType::Uint8, "Character value")?
            .doc("Single iterator item")?
            .build()?;

        // Define the actual iterator
        let iterator = lib.define_iterator(&iterator_next_fn, &iterator_item)?;

        // Define test method
        let iterator_create_fn = lib
            .define_function("iterator_create")
            .param("value", STRING_TYPE, "String to iterate on")?
            .returns(iterator, "New iterator")?
            .doc("Create an iterator")?
            .build()?;
        let iterator_destroy_fn = lib
            .define_function("iterator_destroy")
            .param("it", iterator_class.clone(), "Iterator")?
            .returns_nothing()?
            .doc("Destroy an iterator")?
            .build()?;

        lib.define_class(&iterator_class)?
            .static_method("IterateString", &iterator_create_fn)?
            .destructor(&iterator_destroy_fn)?
            .doc("IterateString functions")?
            .build()?;
    */
    Ok(())
}
