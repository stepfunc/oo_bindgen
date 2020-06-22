use oo_bindgen::native_function::*;
use oo_bindgen::*;

pub fn define(lib: &mut LibraryBuilder) -> Result<(), BindingError> {
    // Define the iterator next function
    // Must always take a class pointer as a param and return a struct pointer
    // (null if no other value available)
    let iterator_class = lib.declare_class("StringIterator")?;
    let iterator_item = lib.declare_native_struct("StringIteratorItem")?;
    let iterator_next_fn = lib
        .declare_native_function("iterator_next")?
        .param("it", Type::ClassRef(iterator_class.clone()))?
        .return_type(ReturnType::Type(Type::StructRef(iterator_item.clone())))?
        .build()?;

    // Define the iterator item structure
    let iterator_item = lib
        .define_native_struct(&iterator_item)?
        .add("value", Type::Uint8)?
        .build();

    // Define the actual iterator
    let iterator = lib.define_iterator(&iterator_next_fn, &iterator_item)?;

    // Define test method
    let iterate_string_fn = lib
        .declare_native_function("iterator_create")?
        .param("value", Type::String)?
        .return_type(ReturnType::Type(Type::Iterator(iterator)))?
        .build()?;
    let iterator_destroy_fn = lib
        .declare_native_function("iterator_destroy")?
        .param("it", Type::ClassRef(iterator_class.clone()))?
        .return_type(ReturnType::Void)?
        .build()?;
    lib.define_class(&iterator_class)?
        .static_method("IterateString", &iterate_string_fn)?
        .destructor(&iterator_destroy_fn)?
        .build();

    Ok(())
}
