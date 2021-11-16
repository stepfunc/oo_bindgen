// re-export this so users don't have to know it isn't part of the crate itself
pub use semver::Version;

pub use back_traced::*;
pub use builder::class::*;
pub use builder::constants::*;
pub use builder::enums::*;
pub use builder::error_type::*;
pub use builder::function::*;
pub use builder::interface::*;
pub use builder::library::*;
pub use builder::structs::*;
pub use class::*;
pub use collection::*;
pub use constants::*;
pub use doc::*;
pub use enum_type::*;
pub use error_type::*;
pub use errors::*;
pub use function::*;
pub use handle::*;
pub use interface::*;
pub use iterator::*;
pub use library::*;
pub use name::*;
pub use return_type::*;
pub use structs::callback_argument_struct::*;
pub use structs::common::*;
pub use structs::function_argument_struct::*;
pub use structs::function_return_struct::*;
pub use structs::universal_struct::*;
pub use types::*;

mod back_traced;
mod builder {
    pub(crate) mod class;
    pub(crate) mod constants;
    pub(crate) mod enums;
    pub(crate) mod error_type;
    pub(crate) mod function;
    pub(crate) mod interface;
    pub(crate) mod library;
    pub(crate) mod structs;
}
mod class;
mod collection;
mod constants;
mod enum_type;
mod error_type;
mod errors;
mod function;
mod handle;
mod interface;
mod iterator;
mod library;
mod name;
mod return_type;
mod structs {
    pub(crate) mod callback_argument_struct;
    pub(crate) mod common;
    pub(crate) mod function_argument_struct;
    pub(crate) mod function_return_struct;
    pub(crate) mod universal_struct;
}
mod doc;
mod types;
