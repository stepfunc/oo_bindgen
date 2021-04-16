use clap::{App, Arg};
use oo_bindgen::platforms::*;
use oo_bindgen::Library;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

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

    let run_tests = !matches.is_present("no-tests");

    let run_c = matches.is_present("c");
    let run_dotnet = matches.is_present("dotnet");
    let run_java = matches.is_present("java");
    let run_all = !run_c && !run_dotnet && !run_java;

    let package = matches.is_present("package");
    let package_src = matches.value_of("package");

    let extra_files = matches
        .values_of("extra-files")
        .map_or(Vec::new(), |v| v.map(PathBuf::from).collect());

    if run_c || run_all {
        let builder = run_builder::<CBindingBuilder>(
            &settings,
            run_tests,
            package,
            package_src,
            &extra_files,
        );

        if matches.is_present("doxygen") {
            builder.build_doxygen();
        }
    }
    if run_dotnet || run_all {
        run_builder::<DotnetBindingBuilder>(
            &settings,
            run_tests,
            package,
            package_src,
            &extra_files,
        );
    }
    if run_java || run_all {
        run_builder::<JavaBindingBuilder>(&settings, run_tests, package, package_src, &extra_files);
    }
}

fn ffi_path() -> PathBuf {
    [env!("TARGET_DIR"), "deps"].iter().collect()
}

fn run_builder<'a, B: BindingBuilder<'a>>(
    settings: &'a BindingBuilderSettings,
    run_tests: bool,
    package: bool,
    package_src: Option<&str>,
    extra_files: &[PathBuf],
) -> B {
    let mut platforms = PlatformLocations::new();
    if let Some(package_src) = package_src {
        let mut check_platform = |platform: Platform| {
            let platform_path = [package_src, platform.to_string()]
                .iter()
                .collect::<PathBuf>();
            if platform_path.is_dir() {
                platforms.add(platform, platform_path);
            }
        };

        check_platform(Platform::WinX64Msvc);
        check_platform(Platform::LinuxX64Gnu);
        check_platform(Platform::LinuxX64Musl);
        check_platform(Platform::LinuxArm6Gnueabi);
        check_platform(Platform::LinuxArm6GnueabiHf);
        check_platform(Platform::LinuxArm7GnueabiHf);
        check_platform(Platform::LinuxArm8Gnu);
    } else {
        platforms.add(
            Platform::from_target_triple(env!("TARGET_TRIPLE")).expect("Unsupported platform"),
            ffi_path(),
        );
    }

    if platforms.is_empty() {
        panic!("No platforms found!");
    }

    let has_dynamic_libs = platforms.has_dynamic_lib();

    let mut builder = B::new(settings, platforms, extra_files);

    if B::requires_dynamic_lib() && !has_dynamic_libs {
        println!(
            "Skipping {} because it requires dynamic libraries",
            B::name()
        );
        return builder;
    }

    builder.generate(package);

    if !package && run_tests {
        builder.build();
        builder.test();
    } else {
        builder.package();
    }

    builder
}

pub struct BindingBuilderSettings<'a> {
    pub ffi_name: &'a str,
    pub ffi_path: &'a Path,
    pub java_group_id: &'a str,
    pub destination_path: &'a Path,
    pub license_path: &'a Path,
    pub library: &'a Library,
}

trait BindingBuilder<'a> {
    fn name() -> &'static str;
    fn requires_dynamic_lib() -> bool;
    fn new(
        settings: &'a BindingBuilderSettings<'a>,
        platforms: PlatformLocations,
        extra_files: &[PathBuf],
    ) -> Self;
    fn generate(&mut self, is_packaging: bool);
    fn build(&mut self);
    fn test(&mut self);
    fn package(&mut self);
}

struct CBindingBuilder<'a> {
    settings: &'a BindingBuilderSettings<'a>,
    platforms: PlatformLocations,
    extra_files: Vec<PathBuf>,
}

impl<'a> CBindingBuilder<'a> {
    fn build_doxygen(&self) {
        let config = c_oo_bindgen::CBindgenConfig {
            output_dir: self.output_dir(),
            ffi_name: self.settings.ffi_name.to_owned(),
            extra_files: Vec::new(),
            platforms: self.platforms.clone(),
        };

        c_oo_bindgen::generate_doxygen(&self.settings.library, &config)
            .expect("failed to package C lib");
    }

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

    fn requires_dynamic_lib() -> bool {
        false
    }

    fn new(
        settings: &'a BindingBuilderSettings<'a>,
        platforms: PlatformLocations,
        extra_files: &[PathBuf],
    ) -> Self {
        Self {
            settings,
            platforms,
            extra_files: extra_files.to_vec(),
        }
    }

    fn generate(&mut self, _is_packaging: bool) {
        let mut extra_files = self.extra_files.clone();
        extra_files.push(self.settings.license_path.to_owned());

        let config = c_oo_bindgen::CBindgenConfig {
            output_dir: self.output_dir(),
            ffi_name: self.settings.ffi_name.to_owned(),
            extra_files,
            platforms: self.platforms.clone(),
        };

        c_oo_bindgen::generate_c_package(&self.settings.library, &config)
            .expect("failed to package C lib");
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
            .args(&["--build", "."])
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
    platforms: PlatformLocations,
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
        output_dir.push(self.settings.library.name.to_owned());
        output_dir
    }
}

impl<'a> BindingBuilder<'a> for DotnetBindingBuilder<'a> {
    fn name() -> &'static str {
        "dotnet"
    }

    fn requires_dynamic_lib() -> bool {
        true
    }

    fn new(
        settings: &'a BindingBuilderSettings<'a>,
        platforms: PlatformLocations,
        extra_files: &[PathBuf],
    ) -> Self {
        Self {
            settings,
            platforms,
            extra_files: extra_files.to_vec(),
        }
    }

    fn generate(&mut self, _is_packaging: bool) {
        // Clear/create generated files
        let build_dir = self.build_dir();
        if build_dir.exists() {
            fs::remove_dir_all(&build_dir).unwrap();
        }
        fs::create_dir_all(&build_dir).unwrap();

        let config = dotnet_oo_bindgen::DotnetBindgenConfig {
            output_dir: build_dir,
            ffi_name: self.settings.ffi_name.to_owned(),
            license_file: self.settings.license_path.to_owned(),
            extra_files: self.extra_files.clone(),
            platforms: self.platforms.clone(),
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
    platforms: PlatformLocations,
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
        output_dir.push(self.settings.library.name.to_owned());
        output_dir
    }

    fn rust_build_dir(&self) -> PathBuf {
        let mut output_dir = self.output_dir();
        output_dir.push(format!("{}-jni", self.settings.library.name));
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

    fn requires_dynamic_lib() -> bool {
        true
    }

    fn new(
        settings: &'a BindingBuilderSettings<'a>,
        platforms: PlatformLocations,
        extra_files: &[PathBuf],
    ) -> Self {
        Self {
            settings,
            platforms,
            extra_files: extra_files.to_vec(),
        }
    }

    fn generate(&mut self, is_packaging: bool) {
        let mut extra_files = self.extra_files.clone();
        extra_files.push(self.settings.license_path.to_owned());

        let config = java_oo_bindgen::JavaBindgenConfig {
            java_output_dir: self.java_build_dir(),
            rust_output_dir: self.rust_build_dir(),
            ffi_name: self.settings.ffi_name.to_owned(),
            ffi_path: self.settings.ffi_path.to_owned(),
            group_id: self.settings.java_group_id.to_owned(),
            extra_files,
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
            java_oo_bindgen::generate_java_ffi(self.settings.library, &config).unwrap();

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
            cmd.current_dir(self.rust_build_dir()).args(&[
                "build",
                "--target-dir",
                &target_dir.to_string_lossy(),
            ]);

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
