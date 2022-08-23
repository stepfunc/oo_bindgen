use crate::{BindingBuilder, BindingBuilderSettings};
use oo_bindgen::backend::PlatformLocations;
use std::path::PathBuf;
use std::process::Command;

pub(crate) struct JavaBindingBuilder {
    settings: BindingBuilderSettings,
    platforms: PlatformLocations,
    extra_files: Vec<PathBuf>,
}

impl JavaBindingBuilder {
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
        self.settings.destination_path.join("java")
    }

    fn java_build_dir(&self) -> PathBuf {
        let mut output_dir = self.output_dir();
        output_dir.push(self.settings.library.settings.name.to_string());
        output_dir
    }

    fn maven(&self) -> Command {
        let mut command = if cfg!(windows) {
            let mut command = Command::new("cmd");
            command.args(&["/c", "mvn.cmd"]);
            command
        } else {
            Command::new("mvn")
        };

        command.current_dir(self.output_dir());
        command.arg("-B"); // No progress on CI

        command
    }
}

impl BindingBuilder for JavaBindingBuilder {
    fn name() -> &'static str {
        "java"
    }

    fn generate(&mut self, is_packaging: bool, _generate_doxygen: bool) {
        let config = java_oo_bindgen::JavaBindgenConfig {
            java_output_dir: self.java_build_dir(),
            ffi_name: self.settings.ffi_name,
            ffi_path: self.settings.ffi_path.to_owned(),
            group_id: self.settings.java_group_id.to_owned(),
            extra_files: self.extra_files.clone(),
            platforms: self.platforms.clone(),
        };

        // Generate Java JNI shared library if we are not packaging
        if !is_packaging {
            let mut cmd = Command::new("cargo");

            cmd.args(&["build", "-p", self.settings.jni_target_name]);

            if env!("PROFILE") == "release" {
                cmd.arg("--release");
            }

            let result = cmd.status().unwrap();
            assert!(result.success());
        }

        // Clear/create Java generated files
        let build_dir = self.java_build_dir();
        if build_dir.exists() {
            std::fs::remove_dir_all(&build_dir).unwrap();
        }
        std::fs::create_dir_all(&build_dir).unwrap();

        // Generate the Java code
        java_oo_bindgen::generate_java_bindings(&self.settings.library, &config).unwrap();
    }

    fn build(&mut self) {
        let result = self.maven().arg("compile").status().unwrap();
        assert!(result.success());
    }

    fn test(&mut self) {
        let result = self.maven().arg("verify").status().unwrap();
        assert!(result.success());
    }

    fn package(&mut self) {
        let result = self
            .maven()
            .arg("package")
            .arg("-DskipTests")
            .status()
            .unwrap();
        assert!(result.success());
    }
}
