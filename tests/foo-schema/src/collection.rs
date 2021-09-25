use oo_bindgen::types::{AllTypes, BasicType};
use oo_bindgen::*;

pub fn define(lib: &mut LibraryBuilder) -> Result<(), BindingError> {
    let collection_class = lib.declare_class("StringCollection")?;

    // Constructor method
    let collection_create_fn = lib
        .declare_native_function("collection_create")?
        .returns(collection_class.clone(), "New collection")?
        .doc("Create a collection")?
        .build()?;

    let collection_create_with_reserve_fn = lib
        .declare_native_function("collection_create_with_reserve")?
        .param(
            "reserve",
            BasicType::Uint32,
            "Number of elements to pre-allocate",
        )?
        .returns(
            collection_class.clone(),
            "New collection (with reserve optimization)",
        )?
        .doc("Create a collection")?
        .build()?;

    // Destructor method
    let collection_destroy_fn = lib
        .declare_native_function("collection_destroy")?
        .param("col", collection_class.clone(), "Collection")?
        .returns_nothing()?
        .doc("Destroy a collection")?
        .build()?;

    let collection_add_fn = lib
        .declare_native_function("collection_add")?
        .param("col", collection_class.clone(), "Collection")?
        .param("item", AllTypes::String, "Item")?
        .returns_nothing()?
        .doc("Add an item to the collection")?
        .build()?;

    // Define the actual collection
    let collection = lib.define_collection(
        &collection_create_fn,
        &collection_destroy_fn,
        &collection_add_fn,
    )?;
    let collection_with_reserve = lib.define_collection(
        &collection_create_with_reserve_fn,
        &collection_destroy_fn,
        &collection_add_fn,
    )?;

    // Define test method
    let collection_size_func = lib
        .declare_native_function("collection_size")?
        .param("col", collection.clone(), "Collection")?
        .returns(BasicType::Uint32, "Size of the collection")?
        .doc("Get the size of a collection")?
        .build()?;

    let collection_get_func = lib
        .declare_native_function("collection_get")?
        .param("col", collection, "Collection")?
        .param("idx", BasicType::Uint32, "Index")?
        .returns(AllTypes::String, "Value")?
        .doc("Get an item from the collection")?
        .build()?;

    let collection_with_reserve_size_func = lib
        .declare_native_function("collection_with_reserve_size")?
        .param("col", collection_with_reserve.clone(), "Collection")?
        .returns(BasicType::Uint32, "Size of the collection")?
        .doc("Get the size of a collection")?
        .build()?;

    let collection_with_reserve_get_func = lib
        .declare_native_function("collection_with_reserve_get")?
        .param("col", collection_with_reserve, "Collection")?
        .param("idx", BasicType::Uint32, "Index")?
        .returns(AllTypes::String, "Value")?
        .doc("Get an item from the collection")?
        .build()?;

    lib.define_class(&collection_class)?
        .static_method("GetSize", &collection_size_func)?
        .static_method("GetValue", &collection_get_func)?
        .static_method("GetSizeWithReserve", &collection_with_reserve_size_func)?
        .static_method("GetValueWithReserve", &collection_with_reserve_get_func)?
        .doc("Collection helper functions")?
        .build()?;

    Ok(())
}
