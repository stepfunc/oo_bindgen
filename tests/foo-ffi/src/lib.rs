#![allow(clippy::missing_safety_doc)]

pub use callback::*;
pub use class::*;
pub use collection::*;
pub use duration::*;
pub use enums::*;
pub use error::*;
pub use integer::*;
pub use iterator::*;
pub use lifetime::*;
pub use opaque_struct::*;
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
mod iterator;
mod lifetime;
mod opaque_struct;
mod strings;
mod thread_class;
mod universal;

pub mod ffi;
