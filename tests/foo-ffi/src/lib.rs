#![allow(clippy::missing_safety_doc)]

extern crate alloc;

pub use callback::*;
pub use class::*;
pub use collection::*;
pub use duration::*;
pub use enums::*;
pub use error::*;
pub use integer::*;
pub(crate) use interface_defaults::*;
pub use iterator::*;
pub use lifetime::*;
pub use opaque_struct::*;
pub use primitive_iterator::*;
pub use primitive_pointers::*;
pub use strings::*;
pub use thread_class::*;
use universal::*;

mod callback;
mod class;
mod collection;
mod duration;
mod enums;
mod error;
mod integer;
mod interface_defaults;
mod iterator;
mod lifetime;
mod opaque_struct;
mod primitive_iterator;
mod primitive_pointers;
mod strings;
mod thread_class;
mod universal;

#[allow(clippy::extra_unused_lifetimes)]
pub mod ffi;

static VERSION: &str = concat!("1.2.3", "\0");

fn version() -> &'static std::ffi::CStr {
    unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(VERSION.as_bytes()) }
}
