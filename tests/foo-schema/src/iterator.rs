use oo_bindgen::native_function::*;
use oo_bindgen::types::BasicType;
use oo_bindgen::*;

pub fn define(lib: &mut LibraryBuilder) -> Result<(), BindingError> {
    // Define the iterator next function
    // Must always take a class pointer as a param and return a struct pointer
    // (null if no other value available)
    let iterator_class = lib.declare_class("StringIterator")?;
    let iterator_item = lib.declare_native_struct("StringIteratorItem")?;
    let iterator_next_fn = lib
        .declare_native_function("iterator_next")?
        .param("it", iterator_class.clone(), "Iterator")?
        .return_type(ReturnType::new(
            iterator_item.clone(),
            "Iterator value",
        ))?
        .doc("Get the next value, or NULL if the iterator reached the end")?
        .build()?;

    // Define the iterator item structure
    let iterator_item = lib
        .define_native_struct(&iterator_item)?
        .add("value", BasicType::Uint8, "Character value")?
        .doc("Single iterator item")?
        .build()?;

    // Define the actual iterator
    let iterator = lib.define_iterator(&iterator_next_fn, &iterator_item)?;

    // Define test method
    let iterate_string_fn = lib
        .declare_native_function("iterator_create")?
        .param("value", Type::String, "String to iterate on")?
        .return_type(ReturnType::new(iterator, "New iterator"))?
        .doc("Create an iterator")?
        .build()?;
    let iterator_destroy_fn = lib
        .declare_native_function("iterator_destroy")?
        .param("it", iterator_class.clone(), "Iterator")?
        .return_type(ReturnType::Void)?
        .doc("Destroy an iterator")?
        .build()?;
    lib.define_class(&iterator_class)?
        .static_method("IterateString", &iterate_string_fn)?
        .destructor(&iterator_destroy_fn)?
        .doc("IterateString functions")?
        .build()?;

    Ok(())
}
