use oo_bindgen::native_function::*;
use oo_bindgen::types::BasicType;
use oo_bindgen::*;

pub fn define(lib: &mut LibraryBuilder) -> Result<(), BindingError> {
    let uint8_echo = lib
        .declare_native_function("uint8_echo")?
        .param("value", BasicType::Uint8, "Uint8")?
        .return_type(ReturnType::new(BasicType::Uint8, "Uint8"))?
        .doc("Echo a Uint8 value")?
        .build()?;

    let sint8_echo = lib
        .declare_native_function("sint8_echo")?
        .param("value", BasicType::Sint8, "Sint8")?
        .return_type(ReturnType::new(BasicType::Sint8, "Sint8"))?
        .doc("Echo a Sint8 value")?
        .build()?;

    let uint16_echo = lib
        .declare_native_function("uint16_echo")?
        .param("value", BasicType::Uint16, "Uint16")?
        .return_type(ReturnType::new(BasicType::Uint16, "Uint16"))?
        .doc("Echo a Uint16 value")?
        .build()?;

    let sint16_echo = lib
        .declare_native_function("sint16_echo")?
        .param("value", BasicType::Sint16, "Sint16")?
        .return_type(ReturnType::new(BasicType::Sint16, "Sint16"))?
        .doc("Echo a Sint16 value")?
        .build()?;

    let uint32_echo = lib
        .declare_native_function("uint32_echo")?
        .param("value", BasicType::Uint32, "Uint32")?
        .return_type(ReturnType::new(BasicType::Uint32, "Uint32"))?
        .doc("Echo a Uint32 value")?
        .build()?;

    let sint32_echo = lib
        .declare_native_function("sint32_echo")?
        .param("value", BasicType::Sint32, "Sint32")?
        .return_type(ReturnType::new(BasicType::Sint32, "Sint32"))?
        .doc("Echo a Sint32 value")?
        .build()?;

    let uint64_echo = lib
        .declare_native_function("uint64_echo")?
        .param("value", BasicType::Uint64, "Uint64")?
        .return_type(ReturnType::new(BasicType::Uint64, "Uint64"))?
        .doc("Echo a Uint64 value")?
        .build()?;

    let sint64_echo = lib
        .declare_native_function("sint64_echo")?
        .param("value", BasicType::Sint64, "Sint64")?
        .return_type(ReturnType::new(BasicType::Sint64, "Sint64"))?
        .doc("Echo a Sint64 value")?
        .build()?;

    // Declare static class
    lib.define_static_class("IntegerEchoFunctions")?
        .static_method("Uint8Echo", &uint8_echo)?
        .static_method("Sint8Echo", &sint8_echo)?
        .static_method("Uint16Echo", &uint16_echo)?
        .static_method("Sint16Echo", &sint16_echo)?
        .static_method("Uint32Echo", &uint32_echo)?
        .static_method("Sint32Echo", &sint32_echo)?
        .static_method("Uint64Echo", &uint64_echo)?
        .static_method("Sint64Echo", &sint64_echo)?
        .doc("Enum echo functions")?
        .build()?;

    Ok(())
}
