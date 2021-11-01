use std::path::PathBuf;

use oo_bindgen::*;

mod callback;
mod class;
mod collection;
mod constants;
mod duration;
mod enums;
mod error;
mod integer;
mod iterator;
mod lifetime;
mod opaque_struct;
mod strings;
mod structure;
mod universal_struct;

pub fn build_lib() -> Result<Library, BindingError> {
    let lib_info = LibraryInfo {
        description: "Foo is an interesting library".to_string(),
        project_url: "https://stepfunc.io/".to_string(),
        repository: "stepfunc/oo_bindgen".to_string(),
        license_name: "Custom license".to_string(),
        license_description: [
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
        license_path: PathBuf::from("LICENSE.txt"),
        developers: vec![DeveloperInfo {
            name: "Émile Grégoire".to_string(),
            email: "emile@stepfunc.io".to_string(),
            organization: "Step Function I/O".to_string(),
            organization_url: "https://stepfunc.io/".to_string(),
        }],
    };

    let mut builder = LibraryBuilder::new(
        oo_bindgen::Version::parse("1.2.3").unwrap(),
        lib_info,
        LibrarySettings::create("foo", "foo"),
    );

    structure::define(&mut builder)?;
    constants::define(&mut builder)?;
    callback::define(&mut builder)?;
    class::define(&mut builder)?;
    duration::define(&mut builder)?;
    enums::define(&mut builder)?;
    error::define(&mut builder)?;
    integer::define(&mut builder)?;
    iterator::define(&mut builder)?;
    opaque_struct::define(&mut builder)?;
    strings::define(&mut builder)?;
    lifetime::define(&mut builder)?;
    collection::define(&mut builder)?;
    universal_struct::define(&mut builder)?;

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
