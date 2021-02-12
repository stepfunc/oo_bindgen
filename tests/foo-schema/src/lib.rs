use oo_bindgen::*;

mod callback;
mod class;
mod collection;
mod constants;
mod duration;
mod enums;
mod integer;
mod iterator;
mod lifetime;
mod strings;
mod structure;

pub fn build_lib() -> Result<Library, BindingError> {
    let mut builder = LibraryBuilder::new("foo", oo_bindgen::Version::parse("1.2.3").unwrap());
    builder.description("Foo is an interesting lib")?;
    builder.license(
        [
            "foo v1.2.3",
            "Copyright (C) 2020-2021 Step Function I/O",
            "",
            "This is my custom license.",
            "These views are not even my own. They belong to nobody.",
            "  - Frumious Scadateer (@scadateer)",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect(),
    )?;

    constants::define(&mut builder)?;
    callback::define(&mut builder)?;
    class::define(&mut builder)?;
    duration::define(&mut builder)?;
    enums::define(&mut builder)?;
    integer::define(&mut builder)?;
    iterator::define(&mut builder)?;
    strings::define(&mut builder)?;
    structure::define(&mut builder)?;
    lifetime::define(&mut builder)?;
    collection::define(&mut builder)?;

    builder.build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_lib() {
        build_lib().unwrap();
    }
}
