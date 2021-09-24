use oo_bindgen::native_function::*;
use oo_bindgen::*;

pub fn define(lib: &mut LibraryBuilder) -> Result<(), BindingError> {
    // Define each enum
    let enum_zero_to_five = lib
        .define_native_enum("EnumZeroToFive")?
        .push("Zero", "Zero")?
        .push("One", "One")?
        .push("Two", "Two")?
        .push("Three", "Three")?
        .push("Four", "Four")?
        .push("Five", "Five")?
        .doc("0 to 5")?
        .build()?;

    let enum_one_to_six = lib
        .define_native_enum("EnumOneToSix")?
        .variant("One", 1, "One")?
        .push("Two", "Two")?
        .push("Three", "Three")?
        .push("Four", "Four")?
        .push("Five", "Five")?
        .push("Six", "Six")?
        .doc("1 to 6")?
        .build()?;

    let enum_disjoint = lib
        .define_native_enum("EnumDisjoint")?
        .variant("Five", 5, "Five")?
        .variant("One", 1, "One")?
        .variant("Twenty", 20, "Twenty")?
        .variant("Four", 4, "Four")?
        .variant("Seven", 7, "Seven")?
        .variant("Two", 2, "Two")?
        .doc("Disjoint")?
        .build()?;

    let enum_single = lib
        .define_native_enum("EnumSingle")?
        .push("Single", "Single")?
        .doc("Single")?
        .build()?;

    // Declare each echo function
    let enum_zero_to_five_echo_function = lib
        .declare_native_function("enum_zero_to_five_echo")?
        .param("value", enum_zero_to_five.clone(), "Enum value")?
        .return_type(ReturnType::new(enum_zero_to_five, "Enum value"))?
        .doc("Echo a EnumZeroToFive enum")?
        .build()?;

    let enum_one_to_six_echo_function = lib
        .declare_native_function("enum_one_to_six_echo")?
        .param("value", enum_one_to_six.clone(), "Enum value")?
        .return_type(ReturnType::new(enum_one_to_six, "Enum value"))?
        .doc("Echo a EnumOneToSix enum")?
        .build()?;

    let enum_disjoint_echo_function = lib
        .declare_native_function("enum_disjoint_echo")?
        .param("value", enum_disjoint.clone(), "Enum value")?
        .return_type(ReturnType::new(enum_disjoint, "Enum value"))?
        .doc("Echo a EnumDisjoint enum")?
        .build()?;

    let enum_single_echo_function = lib
        .declare_native_function("enum_single_echo")?
        .param("value", enum_single.clone(), "Enum value")?
        .return_type(ReturnType::new(enum_single, "Enum value"))?
        .doc("Echo a EnumSingle enum")?
        .build()?;

    // Declare static class
    lib.define_static_class("EnumEchoFunctions")?
        .static_method("EnumZeroToFiveEcho", &enum_zero_to_five_echo_function)?
        .static_method("EnumOneToSixEcho", &enum_one_to_six_echo_function)?
        .static_method("EnumDisjointEcho", &enum_disjoint_echo_function)?
        .static_method("EnumSingleEcho", &enum_single_echo_function)?
        .doc("Enum echo functions")?
        .build()?;

    Ok(())
}
