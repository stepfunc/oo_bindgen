use crate::{BindingBuilder, BindingBuilderSettings};
use oo_bindgen::backend::PlatformLocations;
use std::path::PathBuf;
use std::process::Command;

pub(crate) struct DotnetBindingBuilder {
    settings: BindingBuilderSettings,
    platforms: PlatformLocations,
    extra_files: Vec<PathBuf>,
}

impl DotnetBindingBuilder {
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
        self.settings.destination_path.join("dotnet")
    }

    fn build_dir(&self) -> PathBuf {
        let mut output_dir = self.output_dir();
        output_dir.push(self.settings.library.settings.name.to_string());
        output_dir
    }
}

impl BindingBuilder for DotnetBindingBuilder {
    fn name() -> &'static str {
        "dotnet"
    }

    fn generate(&mut self, _is_packaging: bool, generate_doxygen: bool) {
        // Clear/create generated files
        let build_dir = self.build_dir();
        if build_dir.exists() {
            std::fs::remove_dir_all(&build_dir).unwrap();
        }
        std::fs::create_dir_all(&build_dir).unwrap();

        let config = dotnet_oo_bindgen::DotnetBindgenConfig {
            output_dir: build_dir,
            ffi_name: self.settings.ffi_name,
            extra_files: self.extra_files.clone(),
            platforms: self.platforms.clone(),
            generate_doxygen,
        };

        dotnet_oo_bindgen::generate_dotnet_bindings(&self.settings.library, &config).unwrap();
    }

    fn build(&mut self) {
        let result = Command::new("dotnet")
            .current_dir(&self.output_dir())
            .arg("build")
            .arg("--configuration")
            .arg("Release")
            .status()
            .unwrap();
        assert!(result.success());
    }

    fn test(&mut self) {
        // Run unit tests
        let result = Command::new("dotnet")
            .current_dir(&self.output_dir())
            .arg("test")
            .arg("--configuration")
            .arg("Release")
            .status()
            .unwrap();
        assert!(result.success());
    }

    fn package(&mut self) {
        // Produce a nupkg
        let result = Command::new("dotnet")
            .current_dir(&self.output_dir())
            .arg("pack")
            .arg("--configuration")
            .arg("Release")
            .arg("--include-symbols")
            .arg("--output")
            .arg("nupkg")
            .status()
            .unwrap();
        assert!(result.success());
    }
}
