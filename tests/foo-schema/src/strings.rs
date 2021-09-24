use oo_bindgen::native_function::*;
use oo_bindgen::types::BasicType;
use oo_bindgen::*;

pub fn define(lib: &mut LibraryBuilder) -> Result<(), BindingError> {
    // Declare the class
    let stringclass = lib.declare_class("StringClass")?;

    // Declare each native function
    let stringclass_new_func = lib
        .declare_native_function("string_new")?
        .returns(stringclass.clone(), "New StringClass")?
        .doc("Create a new StringClass")?
        .build()?;

    let stringclass_destroy_func = lib
        .declare_native_function("string_destroy")?
        .param("stringclass", stringclass.clone(), "StringClass")?
        .returns_nothing()?
        .doc("Destroy a StringClass")?
        .build()?;

    let stringclass_echo_func = lib
        .declare_native_function("string_echo")?
        .param("stringclass", stringclass.clone(), "StringClass")?
        .param("value", Type::String, "String to echo")?
        .returns(Type::String, "Echoed string")?
        .doc("Echo a string")?
        .build()?;

    let stringclass_length_func = lib
        .declare_native_function("string_length")?
        .param("value", Type::String, "String")?
        .returns(BasicType::Uint32, "String length")?
        .doc("Get the string length")?
        .build()?;

    // Define the class
    let _testclass = lib
        .define_class(&stringclass)?
        .constructor(&stringclass_new_func)?
        .destructor(&stringclass_destroy_func)?
        .method("Echo", &stringclass_echo_func)?
        .static_method("GetLength", &stringclass_length_func)?
        .disposable_destroy()?
        .doc("StringClass")?
        .build()?;

    Ok(())
}
