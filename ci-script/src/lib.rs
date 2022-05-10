use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use clap::{App, Arg};

use oo_bindgen::backend::*;
use oo_bindgen::model::Library;

pub fn run(settings: BindingBuilderSettings) {
    let matches = App::new("oo-bindgen")
        .arg(
            Arg::with_name("c")
                .long("c")
                .takes_value(false)
                .help("Build C bindings"),
        )
        .arg(
            Arg::with_name("dotnet")
                .long("dotnet")
                .takes_value(false)
                .help("Build .NET Core bindings"),
        )
        .arg(
            Arg::with_name("java")
                .long("java")
                .takes_value(false)
                .help("Build Java (JNI) bindings"),
        )
        .arg(
            Arg::with_name("doxygen")
                .long("doxygen")
                .takes_value(false)
                .help("Generate Doxygen documentation"),
        )
        .arg(
            Arg::with_name("no-tests")
                .long("no-tests")
                .takes_value(false)
                .help("Do not run the unit tests"),
        )
        .arg(
            Arg::with_name("package")
                .long("package")
                .takes_value(true)
                .help("Generate package with the provided modules"),
        )
        .arg(
            Arg::with_name("extra-files")
                .short("f")
                .long("extra-files")
                .takes_value(true)
                .help("Path to extra files to include in the generated bindings"),
        )
        .get_matches();

    let mut run_tests = !matches.is_present("no-tests");

    let run_c = matches.is_present("c");
    let run_dotnet = matches.is_present("dotnet");
    let run_java = matches.is_present("java");
    let run_all = !run_c && !run_dotnet && !run_java;

    let package = matches.is_present("package");
    let package_src = matches.value_of("package");

    let extra_files = matches
        .values_of("extra-files")
        .map_or(Vec::new(), |v| v.map(PathBuf::from).collect());

    let mut platforms = PlatformLocations::new();
    if let Some(package_src) = package_src {
        for entry in fs::read_dir(package_src).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                if let Some(p) = Platform::find(&entry.file_name().to_string_lossy()) {
                    platforms.add(p.clone(), entry.path());
                }
            }
        }
    } else {
        let current_platform =
            Platform::guess_current().expect("could not determine current platform");
        platforms.add(current_platform.clone(), ffi_path());

        if !current_platform.has_official_support() {
            println!(
                "WARNING: building for an unsupported platform: {}",
                current_platform.target_triple
            );
            if run_tests {
                println!("Skipping tests an unsupported platform");
                run_tests = false;
            }
        }
    }

    assert!(!platforms.is_empty(), "No platforms found!");

    if run_c || run_all {
        run_builder::<CBindingBuilder>(
            &settings,
            run_tests,
            package,
            &platforms,
            &extra_files,
            matches.is_present("doxygen"),
        );
    }
    if run_dotnet || run_all {
        run_builder::<DotnetBindingBuilder>(
            &settings,
            run_tests,
            package,
            &platforms,
            &extra_files,
            matches.is_present("doxygen"),
        );
    }
    if run_java || run_all {
        run_builder::<JavaBindingBuilder>(
            &settings,
            run_tests,
            package,
            &platforms,
            &extra_files,
            false,
        );
    }
}

fn ffi_path() -> PathBuf {
    [env!("TARGET_DIR"), "deps"].iter().collect()
}

fn run_builder<'a, B: BindingBuilder<'a>>(
    settings: &'a BindingBuilderSettings,
    run_tests: bool,
    package: bool,
    platforms: &'a PlatformLocations,
    extra_files: &[PathBuf],
    generate_doxygen: bool,
) -> B {
    let mut builder = B::new(settings, platforms, extra_files);

    builder.generate(package, generate_doxygen);

    if !package {
        if run_tests {
            builder.build();
            builder.test();
        }
    } else {
        builder.package();
    }

    builder
}

pub struct BindingBuilderSettings<'a> {
    /// FFI target name (as specified in with `cargo build -p <...>`)
    pub ffi_target_name: &'a str,
    /// Compiled FFI name (usually the same as `ffi_target_name`, but with hyphens replaced by underscores)
    pub ffi_name: &'a str,
    /// Path to the FFI target
    pub ffi_path: &'a Path,
    /// Name of the Java group (e.g. `io.stepfunc`)
    pub java_group_id: &'a str,
    /// Destination path
    pub destination_path: &'a Path,
    /// Library to build
    pub library: &'a Library,
}

trait BindingBuilder<'a> {
    fn name() -> &'static str;
    fn new(
        settings: &'a BindingBuilderSettings<'a>,
        platforms: &'a PlatformLocations,
        extra_files: &[PathBuf],
    ) -> Self;
    fn generate(&mut self, is_packaging: bool, generate_doxygen: bool);
    fn build(&mut self);
    fn test(&mut self);
    fn package(&mut self);
}

struct CBindingBuilder<'a> {
    settings: &'a BindingBuilderSettings<'a>,
    platforms: &'a PlatformLocations,
    extra_files: Vec<PathBuf>,
}

impl<'a> CBindingBuilder<'a> {
    fn output_dir(&self) -> PathBuf {
        let mut output_dir = PathBuf::from(self.settings.destination_path);
        output_dir.push("c");
        output_dir.push("generated");
        output_dir
    }

    fn build_dir(&self) -> PathBuf {
        let mut build_dir = PathBuf::from(self.settings.destination_path);
        build_dir.push("c");
        build_dir.push("build");
        build_dir
    }
}

impl<'a> BindingBuilder<'a> for CBindingBuilder<'a> {
    fn name() -> &'static str {
        "c"
    }

    fn new(
        settings: &'a BindingBuilderSettings<'a>,
        platforms: &'a PlatformLocations,
        extra_files: &[PathBuf],
    ) -> Self {
        Self {
            settings,
            platforms,
            extra_files: extra_files.to_vec(),
        }
    }

    fn generate(&mut self, _is_packaging: bool, generate_doxygen: bool) {
        for platform in self.platforms.iter() {
            let config = c_oo_bindgen::CBindgenConfig {
                output_dir: self.output_dir(),
                ffi_target_name: self.settings.ffi_target_name.to_owned(),
                ffi_name: self.settings.ffi_name.to_owned(),
                is_release: env!("PROFILE") == "release",
                extra_files: self.extra_files.clone(),
                platform_location: platform.clone(),
                generate_doxygen,
            };

            c_oo_bindgen::generate_c_package(self.settings.library, &config)
                .expect("failed to package C lib");
        }
    }

    fn build(&mut self) {
        // Clear/create build directory
        let build_dir = self.build_dir();
        if build_dir.exists() {
            fs::remove_dir_all(&build_dir).unwrap();
        }
        fs::create_dir_all(&build_dir).unwrap();

        // CMake configure
        let result = Command::new("cmake")
            .current_dir(&build_dir)
            .arg("..")
            .status()
            .unwrap();
        assert!(result.success());

        // CMake build
        let result = Command::new("cmake")
            .current_dir(&build_dir)
            .args(&["--build", ".", "--config", "Debug"])
            .status()
            .unwrap();
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

struct DotnetBindingBuilder<'a> {
    settings: &'a BindingBuilderSettings<'a>,
    platforms: &'a PlatformLocations,
    extra_files: Vec<PathBuf>,
}

impl<'a> DotnetBindingBuilder<'a> {
    fn output_dir(&self) -> PathBuf {
        let mut output_dir = PathBuf::from(self.settings.destination_path);
        output_dir.push("dotnet");
        output_dir
    }

    fn build_dir(&self) -> PathBuf {
        let mut output_dir = self.output_dir();
        output_dir.push(self.settings.library.settings.name.to_string());
        output_dir
    }
}

impl<'a> BindingBuilder<'a> for DotnetBindingBuilder<'a> {
    fn name() -> &'static str {
        "dotnet"
    }

    fn new(
        settings: &'a BindingBuilderSettings<'a>,
        platforms: &'a PlatformLocations,
        extra_files: &[PathBuf],
    ) -> Self {
        Self {
            settings,
            platforms,
            extra_files: extra_files.to_vec(),
        }
    }

    fn generate(&mut self, _is_packaging: bool, generate_doxygen: bool) {
        // Clear/create generated files
        let build_dir = self.build_dir();
        if build_dir.exists() {
            fs::remove_dir_all(&build_dir).unwrap();
        }
        fs::create_dir_all(&build_dir).unwrap();

        let config = dotnet_oo_bindgen::DotnetBindgenConfig {
            output_dir: build_dir,
            ffi_name: self.settings.ffi_name.to_owned(),
            extra_files: self.extra_files.clone(),
            platforms: self.platforms.clone(),
            generate_doxygen,
        };

        dotnet_oo_bindgen::generate_dotnet_bindings(self.settings.library, &config).unwrap();
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

struct JavaBindingBuilder<'a> {
    settings: &'a BindingBuilderSettings<'a>,
    platforms: &'a PlatformLocations,
    extra_files: Vec<PathBuf>,
}

impl<'a> JavaBindingBuilder<'a> {
    fn output_dir(&self) -> PathBuf {
        let mut output_dir = PathBuf::from(self.settings.destination_path);
        output_dir.push("java");
        output_dir
    }

    fn java_build_dir(&self) -> PathBuf {
        let mut output_dir = self.output_dir();
        output_dir.push(self.settings.library.settings.name.to_string());
        output_dir
    }

    fn rust_build_dir(&self) -> PathBuf {
        let mut output_dir = self.output_dir();
        output_dir.push(format!("{}-jni", self.settings.library.settings.name));
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

impl<'a> BindingBuilder<'a> for JavaBindingBuilder<'a> {
    fn name() -> &'static str {
        "java"
    }

    fn new(
        settings: &'a BindingBuilderSettings<'a>,
        platforms: &'a PlatformLocations,
        extra_files: &[PathBuf],
    ) -> Self {
        Self {
            settings,
            platforms,
            extra_files: extra_files.to_vec(),
        }
    }

    fn generate(&mut self, is_packaging: bool, _generate_doxygen: bool) {
        let config = java_oo_bindgen::JavaBindgenConfig {
            java_output_dir: self.java_build_dir(),
            rust_output_dir: self.rust_build_dir(),
            ffi_name: self.settings.ffi_name.to_owned(),
            ffi_path: self.settings.ffi_path.to_owned(),
            group_id: self.settings.java_group_id.to_owned(),
            extra_files: self.extra_files.clone(),
            platforms: self.platforms.clone(),
        };

        // Generate Java JNI DLL if we are not packaging
        if !is_packaging {
            // Clear/create the generated files
            let build_dir = self.rust_build_dir();
            if build_dir.exists() {
                fs::remove_dir_all(&build_dir).unwrap();
            }
            fs::create_dir_all(&build_dir).unwrap();

            // Generate the Rust code
            java_oo_bindgen::generate_java_ffi(self.settings.library, &config.to_jni_config())
                .unwrap();

            // You: You're setting the target directory to point at the workspace dir. This doesn't look right...
            // Me: Yeah, it avoids some recompilation.
            // You: But this doesn't feel safe. What are your credentials for doing such a thing?
            // Me: Here's my business card. *hands said card*. The lettering is something called "Silian Rail".
            // You: Impressive, but still, that doesn't give you the right to do that.
            // Me: Let's see Paul Allen's card.
            // You: What!?
            // Me: You know what, I got to go. I have to return some videotapes. *leaves in a hurry*
            // You: *still puzzled about the situation, but accepting this hack*
            let target_dir = pathdiff::diff_paths("./target", self.rust_build_dir()).unwrap();
            let mut cmd = Command::new("cargo");

            let current_platform = Platform::guess_current().unwrap();

            cmd.current_dir(self.rust_build_dir()).args(&[
                "build",
                "--target-dir",
                &target_dir.to_string_lossy(),
            ]);

            // When not building for the native target of the host system, the output path
            // changes. That's the only way I figured out how to properly handle this.
            if ffi_path()
                .to_string_lossy()
                .contains(current_platform.target_triple)
            {
                cmd.args(&["--target", current_platform.target_triple]);
            }

            if env!("PROFILE") == "release" {
                cmd.arg("--release");
            }

            let result = cmd.status().unwrap();
            assert!(result.success());
        }

        // Clear/create Java generated files
        let build_dir = self.java_build_dir();
        if build_dir.exists() {
            fs::remove_dir_all(&build_dir).unwrap();
        }
        fs::create_dir_all(&build_dir).unwrap();

        // Generate the Java code
        java_oo_bindgen::generate_java_bindings(self.settings.library, &config).unwrap();
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
