pub(crate) mod c;
pub(crate) mod dotnet;
/// generation routines for Java bindings
pub mod java;
/// generation routines for the Rust FFI
pub mod rust;

mod common;
pub(crate) use common::*;
