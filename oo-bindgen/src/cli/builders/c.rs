use crate::backend::PlatformLocations;

use std::path::PathBuf;
use std::process::Command;

use crate::cli::{BindingBuilder, BindingBuilderSettings};

pub(crate) struct CBindingBuilder {
    settings: BindingBuilderSettings,
    platforms: PlatformLocations,
    extra_files: Vec<PathBuf>,
}

impl CBindingBuilder {
    pub(crate) fn new(
        settings: BindingBuilderSettings,
        platforms: PlatformLocations,
        extra_files: &[PathBuf],
    ) -> Self {
        Self {
            settings,
            platforms,
            extra_files: extra_files.to_vec(),
        }
    }

    fn output_dir(&self) -> PathBuf {
        self.settings.destination_path.join("c/generated")
    }

    fn build_dir(&self) -> PathBuf {
        self.settings.destination_path.join("c/build")
    }
}

impl BindingBuilder for CBindingBuilder {
    fn name() -> &'static str {
        "c"
    }

    fn generate(&mut self, _is_packaging: bool, generate_doxygen: bool) {
        let config = crate::backend::c::CBindgenConfig {
            output_dir: self.output_dir(),
            ffi_target_name: self.settings.ffi_target_name,
            ffi_name: self.settings.ffi_name,
            extra_files: self.extra_files.clone(),
            platform_locations: self.platforms.clone(),
            generate_doxygen,
        };

        crate::backend::c::generate_c_package(&self.settings.library, &config)
            .expect("failed to package C lib");
    }

    fn build(&mut self) {
        // Clear/create build directory
        let build_dir = self.build_dir();
        if build_dir.exists() {
            std::fs::remove_dir_all(&build_dir).unwrap();
        }
        std::fs::create_dir_all(&build_dir).unwrap();

        // CMake configure
        let result = Command::new("cmake")
            .current_dir(&build_dir)
            .arg("..")
            .status()
            .expect("cmake failed");
        assert!(result.success());

        // CMake build
        let result = Command::new("cmake")
            .current_dir(&build_dir)
            .args(&["--build", ".", "--config", "Debug"])
            .status()
            .expect("cmake failed");
        assert!(result.success());
    }

    fn test(&mut self) {
        // Run unit tests
        let result = Command::new("ctest")
            .current_dir(&self.build_dir())
            .args(&[".", "-C", "Debug"])
            .status()
            .unwrap();
        assert!(result.success());
    }

    fn package(&mut self) {
        // Already done in generate
    }
}
