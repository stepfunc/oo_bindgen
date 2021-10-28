use oo_bindgen::collection::CollectionHandle;
use oo_bindgen::types::{BasicType, StringType};
use oo_bindgen::*;

fn define_string_collection_without_reserve(
    lib: &mut LibraryBuilder,
) -> Result<CollectionHandle, BindingError> {
    let collection_class = lib.declare_collection("StringCollection")?;

    // Constructor method
    let collection_create_fn = lib
        .define_function("collection_create")
        .returns(collection_class.clone(), "New collection")?
        .doc("Create a collection")?
        .build()?;

    // Destructor method
    let collection_destroy_fn = lib
        .define_function("collection_destroy")
        .param("col", collection_class.clone(), "Collection")?
        .returns_nothing()?
        .doc("Destroy a collection")?
        .build()?;

    let collection_add_fn = lib
        .define_function("collection_add")
        .param("col", collection_class, "Collection")?
        .param("item", StringType, "Item")?
        .returns_nothing()?
        .doc("Add an item to the collection")?
        .build()?;

    lib.define_collection(
        &collection_create_fn,
        &collection_destroy_fn,
        &collection_add_fn,
    )
}

fn define_string_collection_with_reserve(
    lib: &mut LibraryBuilder,
) -> Result<CollectionHandle, BindingError> {
    let collection_class = lib.declare_collection("StringCollectionWithReserve")?;

    // Constructor method
    let collection_create_with_reserve_fn = lib
        .define_function("collection_create_with_reserve")
        .param(
            "reserve",
            BasicType::U32,
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
        .define_function("collection_with_reserve_destroy")
        .param("col", collection_class.clone(), "Collection")?
        .returns_nothing()?
        .doc("Destroy a collection")?
        .build()?;

    let collection_add_fn = lib
        .define_function("collection_with_reserve_add")
        .param("col", collection_class, "Collection")?
        .param("item", StringType, "Item")?
        .returns_nothing()?
        .doc("Add an item to the collection")?
        .build()?;

    lib.define_collection(
        &collection_create_with_reserve_fn,
        &collection_destroy_fn,
        &collection_add_fn,
    )
}

pub fn define(lib: &mut LibraryBuilder) -> Result<(), BindingError> {
    let collection_without_reserve = define_string_collection_without_reserve(lib)?;
    let collection_with_reserve = define_string_collection_with_reserve(lib)?;

    // Define test method
    let collection_size_func = lib
        .define_function("collection_size")
        .param("col", collection_without_reserve.clone(), "Collection")?
        .returns(BasicType::U32, "Size of the collection")?
        .doc("Get the size of a collection")?
        .build()?;

    let collection_get_func = lib
        .define_function("collection_get")
        .param("col", collection_without_reserve, "Collection")?
        .param("idx", BasicType::U32, "Index")?
        .returns(StringType, "Value")?
        .doc("Get an item from the collection")?
        .build()?;

    let collection_with_reserve_size_func = lib
        .define_function("collection_with_reserve_size")
        .param("col", collection_with_reserve.clone(), "Collection")?
        .returns(BasicType::U32, "Size of the collection")?
        .doc("Get the size of a collection")?
        .build()?;

    let collection_with_reserve_get_func = lib
        .define_function("collection_with_reserve_get")
        .param("col", collection_with_reserve, "Collection")?
        .param("idx", BasicType::U32, "Index")?
        .returns(StringType, "Value")?
        .doc("Get an item from the collection")?
        .build()?;

    lib.define_static_class("StringCollectionTestMethods")
        .static_method("GetSize", &collection_size_func)?
        .static_method("GetValue", &collection_get_func)?
        .static_method("GetSizeWithReserve", &collection_with_reserve_size_func)?
        .static_method("GetValueWithReserve", &collection_with_reserve_get_func)?
        .doc("Collection helper functions")?
        .build()?;

    Ok(())
}
