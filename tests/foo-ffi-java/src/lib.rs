#![allow(
    clippy::unused_unit,
    clippy::useless_conversion,
    clippy::redundant_closure,
    clippy::needless_borrow,
    clippy::needless_return,
    clippy::not_unsafe_ptr_arg_deref
)]
// ^ these lints don't matter in the generated code

mod generated;
pub use generated::*;
