use oo_bindgen::model::*;

pub fn define(lib: &mut LibraryBuilder) -> BackTraced<()> {
    let class = lib.declare_class("primitive_pointers")?;

    let constructor = lib
        .define_constructor(class.clone())?
        .doc("create an instance")?
        .build()?;

    let destructor = lib.define_destructor(class.clone(), "destroy an instance")?;

    let get_bool = lib
        .define_method("get_bool", class.clone())?
        .param("value", Primitive::Bool, "value")?
        .returns(
            PrimitiveRef::new(Primitive::Bool),
            "pointer to internal value",
        )?
        .doc("return a pointer to an internal value")?
        .build()?;

    let get_u8 = lib
        .define_method("get_u8", class.clone())?
        .param("value", Primitive::U8, "value")?
        .returns(
            PrimitiveRef::new(Primitive::U8),
            "pointer to internal value",
        )?
        .doc("return a pointer to an internal value")?
        .build()?;

    let get_float = lib
        .define_method("get_float", class.clone())?
        .param("value", Primitive::Float, "value")?
        .returns(
            PrimitiveRef::new(Primitive::Float),
            "pointer to internal value",
        )?
        .doc("return a pointer to an internal value")?
        .build()?;

    let get_double = lib
        .define_method("get_double", class.clone())?
        .param("value", Primitive::Double, "value")?
        .returns(
            PrimitiveRef::new(Primitive::Double),
            "pointer to internal value",
        )?
        .doc("return a pointer to an internal value")?
        .build()?;

    lib.define_class(&class)?
        .constructor(constructor)?
        .destructor(destructor)?
        .method(get_bool)?
        .method(get_u8)?
        .method(get_float)?
        .method(get_double)?
        .doc("Provides internal mutable primitives values which can be returned by pointer")?
        .build()?;

    Ok(())
}
