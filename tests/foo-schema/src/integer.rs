use oo_bindgen::model::*;

pub fn define(lib: &mut LibraryBuilder) -> BackTraced<()> {
    let uint8_echo = lib
        .define_function("uint8_echo")?
        .param("value", Primitive::U8, "Uint8")?
        .returns(Primitive::U8, "Uint8")?
        .doc("Echo a Uint8 value")?
        .build_static_with_same_name()?;

    let sint8_echo = lib
        .define_function("sint8_echo")?
        .param("value", Primitive::S8, "Sint8")?
        .returns(Primitive::S8, "Sint8")?
        .doc("Echo a Sint8 value")?
        .build_static_with_same_name()?;

    let uint16_echo = lib
        .define_function("uint16_echo")?
        .param("value", Primitive::U16, "Uint16")?
        .returns(Primitive::U16, "Uint16")?
        .doc("Echo a Uint16 value")?
        .build_static_with_same_name()?;

    let sint16_echo = lib
        .define_function("sint16_echo")?
        .param("value", Primitive::S16, "Sint16")?
        .returns(Primitive::S16, "Sint16")?
        .doc("Echo a Sint16 value")?
        .build_static_with_same_name()?;

    let uint32_echo = lib
        .define_function("uint32_echo")?
        .param("value", Primitive::U32, "Uint32")?
        .returns(Primitive::U32, "Uint32")?
        .doc("Echo a Uint32 value")?
        .build_static_with_same_name()?;

    let sint32_echo = lib
        .define_function("sint32_echo")?
        .param("value", Primitive::S32, "Sint32")?
        .returns(Primitive::S32, "Sint32")?
        .doc("Echo a Sint32 value")?
        .build_static_with_same_name()?;

    let uint64_echo = lib
        .define_function("uint64_echo")?
        .param("value", Primitive::U64, "Uint64")?
        .returns(Primitive::U64, "Uint64")?
        .doc("Echo a Uint64 value")?
        .build_static_with_same_name()?;

    let sint64_echo = lib
        .define_function("sint64_echo")?
        .param("value", Primitive::S64, "Sint64")?
        .returns(Primitive::S64, "Sint64")?
        .doc("Echo a Sint64 value")?
        .build_static_with_same_name()?;

    let boolean_echo = lib
        .define_function("bool_echo")?
        .param("value", Primitive::Bool, "Bool")?
        .returns(Primitive::Bool, "Bool")?
        .doc("Echo a boolean value")?
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
        .static_method(boolean_echo)?
        .doc("Integer echo functions")?
        .build()?;

    Ok(())
}
