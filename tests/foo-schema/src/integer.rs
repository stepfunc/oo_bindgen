use oo_bindgen::model::*;

pub fn define(lib: &mut LibraryBuilder) -> BackTraced<()> {
    let uint8_echo = lib
        .define_function("uint8_echo")?
        .param("value", BasicType::U8, "Uint8")?
        .returns(BasicType::U8, "Uint8")?
        .doc("Echo a Uint8 value")?
        .build_static_with_same_name()?;

    let sint8_echo = lib
        .define_function("sint8_echo")?
        .param("value", BasicType::S8, "Sint8")?
        .returns(BasicType::S8, "Sint8")?
        .doc("Echo a Sint8 value")?
        .build_static_with_same_name()?;

    let uint16_echo = lib
        .define_function("uint16_echo")?
        .param("value", BasicType::U16, "Uint16")?
        .returns(BasicType::U16, "Uint16")?
        .doc("Echo a Uint16 value")?
        .build_static_with_same_name()?;

    let sint16_echo = lib
        .define_function("sint16_echo")?
        .param("value", BasicType::S16, "Sint16")?
        .returns(BasicType::S16, "Sint16")?
        .doc("Echo a Sint16 value")?
        .build_static_with_same_name()?;

    let uint32_echo = lib
        .define_function("uint32_echo")?
        .param("value", BasicType::U32, "Uint32")?
        .returns(BasicType::U32, "Uint32")?
        .doc("Echo a Uint32 value")?
        .build_static_with_same_name()?;

    let sint32_echo = lib
        .define_function("sint32_echo")?
        .param("value", BasicType::S32, "Sint32")?
        .returns(BasicType::S32, "Sint32")?
        .doc("Echo a Sint32 value")?
        .build_static_with_same_name()?;

    let uint64_echo = lib
        .define_function("uint64_echo")?
        .param("value", BasicType::U64, "Uint64")?
        .returns(BasicType::U64, "Uint64")?
        .doc("Echo a Uint64 value")?
        .build_static_with_same_name()?;

    let sint64_echo = lib
        .define_function("sint64_echo")?
        .param("value", BasicType::S64, "Sint64")?
        .returns(BasicType::S64, "Sint64")?
        .doc("Echo a Sint64 value")?
        .build_static_with_same_name()?;

    // Declare static class
    lib.define_static_class("integer_echo_functions")?
        .static_method(uint8_echo)?
        .static_method(sint8_echo)?
        .static_method(uint16_echo)?
        .static_method(sint16_echo)?
        .static_method(uint32_echo)?
        .static_method(sint32_echo)?
        .static_method(uint64_echo)?
        .static_method(sint64_echo)?
        .doc("Enum echo functions")?
        .build()?;

    Ok(())
}
