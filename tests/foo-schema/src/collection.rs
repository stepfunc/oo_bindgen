use oo_bindgen::types::{BasicType, StringType};
use oo_bindgen::*;

pub fn define(lib: &mut LibraryBuilder) -> Result<(), BindingError> {
    let collection_without_reserve =
        lib.define_collection("string_collection", StringType, false)?;

    let collection_with_reserve =
        lib.define_collection("string_collection_with_reserve", StringType, true)?;

    // Define test method
    let collection_size_func = lib
        .define_function("collection_size")?
        .param("col", collection_without_reserve.clone(), "Collection")?
        .returns(BasicType::U32, "Size of the collection")?
        .doc("Get the size of a collection")?
        .build()?;

    let collection_get_func = lib
        .define_function("collection_get")?
        .param("col", collection_without_reserve, "Collection")?
        .param("idx", BasicType::U32, "Index")?
        .returns(StringType, "Value")?
        .doc("Get an item from the collection")?
        .build()?;

    let collection_with_reserve_size_func = lib
        .define_function("collection_with_reserve_size")?
        .param("col", collection_with_reserve.clone(), "Collection")?
        .returns(BasicType::U32, "Size of the collection")?
        .doc("Get the size of a collection")?
        .build()?;

    let collection_with_reserve_get_func = lib
        .define_function("collection_with_reserve_get")?
        .param("col", collection_with_reserve, "Collection")?
        .param("idx", BasicType::U32, "Index")?
        .returns(StringType, "Value")?
        .doc("Get an item from the collection")?
        .build()?;

    lib.define_static_class("string_collection_test_methods")?
        .static_method("get_size", &collection_size_func)?
        .static_method("get_value", &collection_get_func)?
        .static_method("get_size_with_reserve", &collection_with_reserve_size_func)?
        .static_method("get_value_with_reserve", &collection_with_reserve_get_func)?
        .doc("Collection helper functions")?
        .build()?;

    Ok(())
}
