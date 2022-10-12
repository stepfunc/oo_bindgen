use std::path::PathBuf;

use oo_bindgen::model::*;

mod callback;
mod class;
mod collection;
mod constants;
mod duration;
mod enums;
mod error;
mod integer;
mod interface_defaults;
mod iterator;
mod lifetime;
mod opaque_struct;
mod primitive_iterator;
mod primitive_pointer;
mod strings;
mod structure;
mod thread_class;
mod universal_struct;

pub fn build_lib() -> BackTraced<Library> {
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
        logo_png: include_bytes!("../../../sfio_logo.png"),
    };

    let settings = LibrarySettings::create(
        "foo",
        "foo",
        ClassSettings::default(),
        IteratorSettings::default(),
        CollectionSettings::default(),
        FutureSettings::default(),
        InterfaceSettings::default(),
    )?;

    let mut builder = LibraryBuilder::new(Version::parse("1.2.3").unwrap(), lib_info, settings);

    structure::define(&mut builder)?;
    constants::define(&mut builder)?;
    callback::define(&mut builder)?;
    class::define(&mut builder)?;
    duration::define(&mut builder)?;
    enums::define(&mut builder)?;
    error::define(&mut builder)?;
    integer::define(&mut builder)?;
    interface_defaults::define(&mut builder)?;
    iterator::define(&mut builder)?;
    opaque_struct::define(&mut builder)?;
    primitive_iterator::define(&mut builder)?;
    primitive_pointer::define(&mut builder)?;
    strings::define(&mut builder)?;
    lifetime::define(&mut builder)?;
    collection::define(&mut builder)?;
    universal_struct::define(&mut builder)?;
    thread_class::define(&mut builder)?;

    let library = builder.build()?;

    Ok(library)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_lib() {
        build_lib().unwrap();
    }
}
