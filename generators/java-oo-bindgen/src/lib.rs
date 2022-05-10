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

use std::path::PathBuf;


pub use java::generate_java_bindings;
use oo_bindgen::backend::*;
use oo_bindgen::model::Library;
pub use crate::rust::generate_java_ffi;
pub use crate::rust::JniBindgenConfig;

mod java;
mod rust;

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
    pub fn to_jni_config(&self) -> JniBindgenConfig {
        JniBindgenConfig {
            rust_output_dir: self.rust_output_dir.clone(),
            group_id: self.group_id.clone(),
            ffi_name: self.ffi_name.clone(),
        }
    }

    fn java_source_dir(&self, lib: &Library) -> PathBuf {
        let mut result = self.java_output_dir.clone();
        result.extend(&["src", "main", "java"]);
        for dir in self.group_id.split('.') {
            result.push(dir);
        }
        result.push(&lib.settings.name.kebab_case());
        result
    }

    fn java_resource_dir(&self) -> PathBuf {
        let mut result = self.java_output_dir.clone();
        result.extend(&["src", "main", "resources"]);
        result
    }
}
