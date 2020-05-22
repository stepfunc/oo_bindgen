use oo_bindgen::*;
use oo_bindgen::native_function::*;

pub fn define(lib: &mut LibraryBuilder) -> Result<(), BindingError> {
    // Declare the class
    let testclass = lib.declare_class("TestClass")?;

    // Declare each native function
    let testclass_new_func = lib.declare_native_function("testclass_new")?
        .param("value", Type::Uint32)?
        .return_type(ReturnType::Type(Type::ClassRef(testclass.clone())))?
        .build()?;

    let testclass_destroy_func = lib.declare_native_function("testclass_destroy")?
        .param("testclass", Type::ClassRef(testclass.clone()))?
        .return_type(ReturnType::Void)?
        .build()?;

    let testclass_get_value_func = lib.declare_native_function("testclass_get_value")?
        .param("testclass", Type::ClassRef(testclass.clone()))?
        .return_type(ReturnType::Type(Type::Uint32))?
        .build()?;

    let testclass_increment_value_func = lib.declare_native_function("testclass_increment_value")?
        .param("testclass", Type::ClassRef(testclass.clone()))?
        .return_type(ReturnType::Void)?
        .build()?;

    let testclass_construction_counter = lib.declare_native_function("testclass_construction_counter")?
        .return_type(ReturnType::Type(Type::Uint32))?
        .build()?;

    // Define the class
    let _testclass = lib.define_class(&testclass)?
        .constructor(&testclass_new_func)?
        .destructor(&testclass_destroy_func)?
        .method("GetValue", &testclass_get_value_func)?
        .method("IncrementValue", &testclass_increment_value_func)?
        .static_method("ConstructionCounter", &testclass_construction_counter)?
        .build();

    Ok(())
}
