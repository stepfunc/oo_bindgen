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
    safe_packed_borrows,
    while_true,
    bare_trait_objects
)]

use heck::KebabCase;
use oo_bindgen::platforms::*;
use oo_bindgen::*;
use std::path::PathBuf;

mod java;

pub use java::generate_java_bindings;

pub struct JavaBindgenConfig {
    pub output_dir: PathBuf,
    pub ffi_name: String,
    pub group_id: String,
    pub platforms: PlatformLocations,
}

impl JavaBindgenConfig {
    fn source_dir(&self, lib: &Library) -> PathBuf {
        let mut result = self.output_dir.clone();
        result.extend(&["src", "main", "java"]);
        for dir in self.group_id.split('.') {
            result.push(dir);
        }
        result.push(&lib.name.to_kebab_case());
        result
    }

    fn resource_dir(&self) -> PathBuf {
        let mut result = self.output_dir.clone();
        result.extend(&["src", "main", "resources"]);
        result
    }
}
