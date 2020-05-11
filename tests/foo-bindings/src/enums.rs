use oo_bindgen::*;
use oo_bindgen::native_function::*;

pub fn define(lib: &mut LibraryBuilder) -> Result<(), BindingError> {
    let enum_zero_to_five = lib.define_native_enum("EnumZeroToFive")?
        .variant("Zero")?
        .variant("One")?
        .variant("Two")?
        .variant("Three")?
        .variant("Four")?
        .variant("Five")?
        .build();

    lib.declare_native_function("enum_zero_to_five_echo")?
        .param("value", Type::Enum(enum_zero_to_five.clone()))?
        .return_type(ReturnType::Type(Type::Enum(enum_zero_to_five.clone())))?
        .build()?;

    Ok(())
}