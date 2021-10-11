#![deny(
// dead_code,
arithmetic_overflow,
invalid_type_param_default,
//missing_fragment_specifier,
mutable_transmutes,
no_mangle_const_items,
overflowing_literals,
patterns_in_fns_without_body,
pub_use_of_private_extern_crate,
unknown_crate_types,
const_err,
order_dependent_trait_objects,
illegal_floating_point_literal_pattern,
improper_ctypes,
late_bound_lifetime_arguments,
non_camel_case_types,
non_shorthand_field_patterns,
non_snake_case,
non_upper_case_globals,
no_mangle_generic_items,
private_in_public,
stable_features,
type_alias_bounds,
tyvar_behind_raw_pointer,
unconditional_recursion,
unused_comparisons,
unreachable_pub,
anonymous_parameters,
missing_copy_implementations,
// missing_debug_implementations,
// missing_docs,
trivial_casts,
trivial_numeric_casts,
unused_import_braces,
unused_qualifications,
clippy::all
)]
#![forbid(
    unsafe_code,
    //intra_doc_link_resolution_failure, broken_intra_doc_links
    unaligned_references,
    while_true,
    bare_trait_objects
)]

pub use semver::Version;

use structs::common::StructDeclarationHandle;
use structs::function_struct::FunctionArgStructHandle;

use crate::class::*;
pub use crate::doc::doc;
use crate::doc::Doc;
use crate::enum_type::*;
use crate::error_type::ErrorType;
pub use crate::errors::*;
use crate::function::*;
pub use crate::handle::*;
use crate::interface::*;
pub use crate::library::*;
use crate::types::BasicType;

pub mod class;
pub mod collection;
pub mod constants;
pub mod doc;
pub mod enum_type;
pub mod error_type;
mod errors;
pub mod formatting;
pub mod function;
mod handle;
pub mod interface;
pub mod iterator;
mod library;
pub mod platforms;
pub mod return_type;
pub mod structs;
pub mod types;
pub mod util;
