use oo_bindgen::types::{BasicType, StringType};
use oo_bindgen::*;

pub fn define(lib: &mut LibraryBuilder) -> BackTraced<()> {
    // Declare the class
    let string_class = lib.declare_class("string_class")?;

    // Declare each native function
    let constructor = lib
        .define_constructor(string_class.clone())?
        .doc("Create a new StringClass")?
        .build()?;

    let destructor = lib.define_destructor(string_class.clone(), "Destroy a StringClass")?;

    let echo = lib
        .define_method("echo", string_class.clone())?
        .param("value", StringType, "String to echo")?
        .returns(StringType, "Echoed string")?
        .doc("Echo a string")?
        .build()?;

    let string_length = lib
        .define_function("string_length")?
        .param("value", StringType, "String")?
        .returns(BasicType::U32, "String length")?
        .doc("Get the length of a string")?
        .build_static("get_length")?;

    // Define the class
    lib.define_class(&string_class)?
        .constructor(constructor)?
        .destructor(destructor)?
        .method(echo)?
        .static_method(string_length)?
        .disposable_destroy()?
        .doc("StringClass")?
        .build()?;

    Ok(())
}
