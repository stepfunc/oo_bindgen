#![allow(clippy::missing_safety_doc)]

mod callback;
mod class;
mod collection;
mod duration;
mod enums;
mod error;
mod integer;
mod iterator;
mod lifetime;
mod opaque_struct;
mod strings;
mod structure;

pub use callback::*;
pub use class::*;
pub use collection::*;
pub use duration::*;
pub use enums::*;
pub use error::*;
pub use integer::*;
pub use iterator::*;
pub use lifetime::*;
pub(crate) use opaque_struct::*;
pub use strings::*;
pub(crate) use structure::*;

pub mod ffi;

static VERSION: &str = concat!("1.2.3", "\0");

fn version() -> &'static std::ffi::CStr {
    unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(VERSION.as_bytes()) }
}
