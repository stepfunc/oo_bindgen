use oo_bindgen::*;

mod association;
mod handler;
mod logging;
mod master;
mod runtime;

pub fn build_lib() -> Result<Library, BindingError> {
    let mut builder = LibraryBuilder::new(
        "dnp3rs",
        semver::Version::parse("0.1.0").unwrap()
    );
    builder.description("Safe and fast DNP3 library")?;
    builder.license([
        "dnp3rs v0.1.0",
        "Copyright (C) 2020  Automatak LLC",
        "",
        "This program is free software: you can redistribute it and/or modify",
        "it under the terms of the GNU General Public License as published by",
        "the Free Software Foundation, either version 3 of the License, or",
        "(at your option) any later version.",
        "",
        "This program is distributed in the hope that it will be useful,",
        "but WITHOUT ANY WARRANTY; without even the implied warranty of",
        "MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the",
        "GNU General Public License for more details.",
        "",
        "You should have received a copy of the GNU General Public License",
        "along with this program.  If not, see <https://www.gnu.org/licenses/>.",
    ].iter().map(|s| s.to_string()).collect())?;

    logging::define(&mut builder)?;
    let read_handler = handler::define(&mut builder)?;
    let master_class = runtime::define(&mut builder)?;
    let association_class = master::define(&mut builder, master_class, read_handler)?;
    association::define(&mut builder, association_class)?;

    Ok(builder.build())
}
