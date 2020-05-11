use oo_bindgen::*;
use oo_bindgen::native_function::*;

pub fn define(lib: &mut LibraryBuilder) -> Result<(), BindingError> {
    // Define each enum
    let enum_zero_to_five = lib.define_native_enum("EnumZeroToFive")?
        .push("Zero")?
        .push("One")?
        .push("Two")?
        .push("Three")?
        .push("Four")?
        .push("Five")?
        .build();

    let enum_one_to_six = lib.define_native_enum("EnumOneToSix")?
        .variant("One", 1)?
        .push("Two")?
        .push("Three")?
        .push("Four")?
        .push("Five")?
        .push("Six")?
        .build();

    let enum_disjoint = lib.define_native_enum("EnumDisjoint")?
        .variant("Five", 5)?
        .variant("One", 1)?
        .variant("Twenty", 20)?
        .variant("Four", 4)?
        .variant("Seven", 7)?
        .variant("Two", 2)?
        .build();

    // Declare each echo function
    let enum_zero_to_five_echo_function = lib.declare_native_function("enum_zero_to_five_echo")?
        .param("value", Type::Enum(enum_zero_to_five.clone()))?
        .return_type(ReturnType::Type(Type::Enum(enum_zero_to_five.clone())))?
        .build()?;

    let enum_one_to_six_echo_function = lib.declare_native_function("enum_one_to_six_echo")?
        .param("value", Type::Enum(enum_one_to_six.clone()))?
        .return_type(ReturnType::Type(Type::Enum(enum_one_to_six.clone())))?
        .build()?;

    let enum_disjoint_echo_function = lib.declare_native_function("enum_disjoint_echo")?
        .param("value", Type::Enum(enum_disjoint.clone()))?
        .return_type(ReturnType::Type(Type::Enum(enum_disjoint.clone())))?
        .build()?;

    let class = lib.declare_class("EnumEchoFunctions")?;
    lib.define_class(&class)?
        .static_method("EnumZeroToFiveEcho", &enum_zero_to_five_echo_function)?
        .static_method("EnumOneToSixEcho", &enum_one_to_six_echo_function)?
        .static_method("EnumDisjointEcho", &enum_disjoint_echo_function)?
        .build();

    Ok(())
}
