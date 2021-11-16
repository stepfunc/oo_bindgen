use oo_bindgen::model::*;

pub fn define(lib: &mut LibraryBuilder) -> BackTraced<()> {
    let collection_without_reserve =
        lib.define_collection("string_collection", StringType, false)?;

    let collection_with_reserve =
        lib.define_collection("string_collection_with_reserve", StringType, true)?;

    // Define test method
    let collection_size_method = lib
        .define_function("collection_size")?
        .param("col", collection_without_reserve.clone(), "Collection")?
        .returns(BasicType::U32, "Size of the collection")?
        .doc("Get the size of a collection")?
        .build_static("get_size")?;

    let collection_get_method = lib
        .define_function("collection_get")?
        .param("col", collection_without_reserve, "Collection")?
        .param("idx", BasicType::U32, "Index")?
        .returns(StringType, "Value")?
        .doc("Get an item from the collection")?
        .build_static("get_value")?;

    let collection_with_reserve_size_method = lib
        .define_function("collection_with_reserve_size")?
        .param("col", collection_with_reserve.clone(), "Collection")?
        .returns(BasicType::U32, "Size of the collection")?
        .doc("Get the size of a collection")?
        .build_static("get_size_with_reserve")?;

    let collection_with_reserve_get_method = lib
        .define_function("collection_with_reserve_get")?
        .param("col", collection_with_reserve, "Collection")?
        .param("idx", BasicType::U32, "Index")?
        .returns(StringType, "Value")?
        .doc("Get an item from the collection")?
        .build_static("get_value_with_reserve")?;

    lib.define_static_class("string_collection_test_methods")?
        .static_method(collection_size_method)?
        .static_method(collection_get_method)?
        .static_method(collection_with_reserve_size_method)?
        .static_method(collection_with_reserve_get_method)?
        .doc("Collection helper functions")?
        .build()?;

    Ok(())
}
