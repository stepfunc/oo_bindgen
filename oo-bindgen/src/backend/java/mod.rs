use std::path::PathBuf;

pub use api::generate_java_bindings;
pub use rust::generate_java_ffi;
pub use rust::JniBindgenConfig;

use crate::backend::*;
use crate::model::Library;

mod api;
mod rust;

pub struct JavaBindgenConfig {
    /// Path to output the generated Java code
    pub java_output_dir: PathBuf,
    /// Path to the C FFI target lib (the actual Rust code, not the compiled FFI)
    pub ffi_path: PathBuf,
    /// Name of the FFI target
    pub ffi_name: &'static str,
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
        result.push(&lib.settings.name.kebab_case());
        result
    }

    fn java_resource_dir(&self) -> PathBuf {
        let mut result = self.java_output_dir.clone();
        result.extend(&["src", "main", "resources"]);
        result
    }
}
