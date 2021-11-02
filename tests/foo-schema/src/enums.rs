use oo_bindgen::*;

pub fn define(lib: &mut LibraryBuilder) -> Result<(), BindingError> {
    // Define each enum
    let enum_zero_to_five = lib
        .define_enum("enum_zero_to_five")?
        .push("zero", "Zero")?
        .push("one", "One")?
        .push("two", "Two")?
        .push("three", "Three")?
        .push("four", "Four")?
        .push("five", "Five")?
        .doc("0 to 5")?
        .build()?;

    let enum_one_to_six = lib
        .define_enum("enum_one_to_six")?
        .variant("one", 1, "One")?
        .push("two", "Two")?
        .push("three", "Three")?
        .push("four", "Four")?
        .push("five", "Five")?
        .push("six", "Six")?
        .doc("1 to 6")?
        .build()?;

    let enum_disjoint = lib
        .define_enum("enum_disjoint")?
        .variant("five", 5, "Five")?
        .variant("one", 1, "One")?
        .variant("twenty", 20, "Twenty")?
        .variant("four", 4, "Four")?
        .variant("seven", 7, "Seven")?
        .variant("two", 2, "Two")?
        .doc("Disjoint")?
        .build()?;

    let enum_single = lib
        .define_enum("enum_single")?
        .push("single", "Single")?
        .doc("Single")?
        .build()?;

    // Declare each echo function
    let enum_zero_to_five_echo_function = lib
        .define_function("enum_zero_to_five_echo")?
        .param("value", enum_zero_to_five.clone(), "Enum value")?
        .returns(enum_zero_to_five, "Enum value")?
        .doc("Echo a EnumZeroToFive enum")?
        .build()?;

    let enum_one_to_six_echo_function = lib
        .define_function("enum_one_to_six_echo")?
        .param("value", enum_one_to_six.clone(), "Enum value")?
        .returns(enum_one_to_six, "Enum value")?
        .doc("Echo a EnumOneToSix enum")?
        .build()?;

    let enum_disjoint_echo_function = lib
        .define_function("enum_disjoint_echo")?
        .param("value", enum_disjoint.clone(), "Enum value")?
        .returns(enum_disjoint, "Enum value")?
        .doc("Echo a EnumDisjoint enum")?
        .build()?;

    let enum_single_echo_function = lib
        .define_function("enum_single_echo")?
        .param("value", enum_single.clone(), "Enum value")?
        .returns(enum_single, "Enum value")?
        .doc("Echo a EnumSingle enum")?
        .build()?;

    // Declare static class
    lib.define_static_class("enum_echo_functions")?
        .static_method("enum_zero_to_five_echo", &enum_zero_to_five_echo_function)?
        .static_method("enum_one_to_six_echo", &enum_one_to_six_echo_function)?
        .static_method("enum_disjoint_echo", &enum_disjoint_echo_function)?
        .static_method("enum_single_echo", &enum_single_echo_function)?
        .doc("Enum echo functions")?
        .build()?;

    Ok(())
}
