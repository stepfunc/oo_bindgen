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

use heck::KebabCase;
use oo_bindgen::platforms::*;
use oo_bindgen::*;
use std::path::PathBuf;

mod java;
mod rust;

pub use java::generate_java_bindings;
pub use rust::generate_java_ffi;

pub struct JavaBindgenConfig {
    /// Path to output the generated Java code
    pub java_output_dir: PathBuf,
    /// Path to output the generated Rust code
    pub rust_output_dir: PathBuf,
    /// Path to the C FFI target lib (the actual Rust code, not the compiled FFI)
    pub ffi_path: PathBuf,
    /// Name of the FFI target
    pub ffi_name: String,
    /// Maven group id (e.g. io.stepfunc)
    pub group_id: String,
    /// Extra files to include in the distribution
    pub extra_files: Vec<PathBuf>,
    /// Platforms to include
    pub platforms: PlatformLocations,
}

impl JavaBindgenConfig {
    fn java_source_dir(&self, lib: &Library) -> PathBuf {
        let mut result = self.java_output_dir.clone();
        result.extend(&["src", "main", "java"]);
        for dir in self.group_id.split('.') {
            result.push(dir);
        }
        result.push(&lib.settings.name.to_kebab_case());
        result
    }

    fn java_resource_dir(&self) -> PathBuf {
        let mut result = self.java_output_dir.clone();
        result.extend(&["src", "main", "resources"]);
        result
    }

    fn java_ffi_name(&self) -> String {
        let mut result = self.ffi_name.clone();
        result.push_str("-java");
        result
    }

    fn rust_source_dir(&self) -> PathBuf {
        let mut result = self.rust_output_dir.clone();
        result.extend(&["src"]);
        result
    }

    fn java_signature_path(&self, libname: &str) -> String {
        let mut result = self.group_id.replace(".", "/");
        result.push('/');
        result.push_str(libname);
        result
    }
}
