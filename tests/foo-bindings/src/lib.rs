use oo_bindgen::*;

mod duration;
mod enums;
mod structure;

pub fn build_lib() -> Result<Library, BindingError> {
    let mut builder = LibraryBuilder::new(
        "foo",
        semver::Version::parse("1.2.3").unwrap()
    );
    builder.description("Foo is an interesting lib")?;
    builder.license([
        "foo v1.2.3",
        "Copyright (C) 2020 Automatak LLC",
        "",
        "This is my custom license.",
        "These views are not even my own. They belong to nobody.",
        "  - Frumious Scadateer (@scadateer)",
    ].iter().map(|s| s.to_string()).collect())?;

    duration::define(&mut builder)?;
    enums::define(&mut builder)?;
    structure::define(&mut builder)?;

    Ok(builder.build())
}
