use clap::{App, Arg};
use oo_bindgen::platforms::*;
use oo_bindgen::Library;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

include!(concat!(env!("OUT_DIR"), "/paths.rs"));

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
                .help("Build Java (JNA) bindings"),
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
        .get_matches();

    let run_tests = !matches.is_present("no-tests");

    let run_c = matches.is_present("c");
    let run_dotnet = matches.is_present("dotnet");
    let run_java = matches.is_present("java");
    let run_all = !run_c && !run_dotnet && !run_java;

    let package_src = matches.value_of("package");

    if run_c || run_all {
        let builder = run_builder::<CBindingBuilder>(&settings, run_tests, package_src);

        if matches.is_present("doxygen") {
            builder.build_doxygen();
        }
    }
    /*if run_dotnet || run_all {
        run_builder::<DotnetBindingBuilder>(&settings, run_tests, package_src);
    }*/
    if run_java || run_all {
        run_builder::<JavaBindingBuilder>(&settings, run_tests, package_src);
    }
}

fn ffi_path() -> PathBuf {
    [TARGET_DIR, "deps"].iter().collect()
}

fn run_builder<'a, B: BindingBuilder<'a>>(
    settings: &'a BindingBuilderSettings,
    run_tests: bool,
    package_src: Option<&str>,
) -> B {
    let mut platforms = PlatformLocations::new();
    if let Some(package_src) = package_src {
        let platform_path = [package_src, Platform::Linux.to_string()]
            .iter()
            .collect::<PathBuf>();
        if platform_path.is_dir() {
            platforms.add(Platform::Linux, platform_path);
        }

        let platform_path = [package_src, Platform::Win64.to_string()]
            .iter()
            .collect::<PathBuf>();
        if platform_path.is_dir() {
            platforms.add(Platform::Win64, platform_path);
        }

        let platform_path = [package_src, Platform::Win32.to_string()]
            .iter()
            .collect::<PathBuf>();
        if platform_path.is_dir() {
            platforms.add(Platform::Win32, platform_path);
        }
    } else {
        platforms.add(Platform::current(), ffi_path());
    }

    if platforms.is_empty() {
        panic!("No platforms found!");
    }

    let mut builder = B::new(settings, platforms);
    builder.generate();

    if package_src.is_none() {
        builder.build();
        if run_tests {
            builder.test();
        }
    } else {
        builder.package();
    }

    builder
}

pub struct BindingBuilderSettings<'a> {
    pub ffi_name: &'a str,
    pub destination_path: &'a Path,
    pub library: &'a Library,
}

trait BindingBuilder<'a> {
    fn new(settings: &'a BindingBuilderSettings<'a>, platforms: PlatformLocations) -> Self;
    fn generate(&mut self);
    fn build(&mut self);
    fn test(&mut self);
    fn package(&mut self);
}

struct CBindingBuilder<'a> {
    settings: &'a BindingBuilderSettings<'a>,
    platforms: PlatformLocations,
}

impl<'a> CBindingBuilder<'a> {
    fn build_doxygen(&self) {
        let mut platforms = PlatformLocations::new();
        platforms.add(Platform::current(), ffi_path());

        let config = c_oo_bindgen::CBindgenConfig {
            output_dir: self.output_dir(),
            ffi_name: self.settings.ffi_name.to_owned(),
            platforms,
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
    fn new(settings: &'a BindingBuilderSettings<'a>, platforms: PlatformLocations) -> Self {
        Self {
            settings,
            platforms,
        }
    }

    fn generate(&mut self) {
        let mut platforms = PlatformLocations::new();
        platforms.add(Platform::current(), ffi_path());

        let config = c_oo_bindgen::CBindgenConfig {
            output_dir: self.output_dir(),
            ffi_name: self.settings.ffi_name.to_owned(),
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
        // Already done in build
    }
}
/*
struct DotnetBindingBuilder<'a> {
    settings: &'a BindingBuilderSettings<'a>,
    platforms: PlatformLocations,
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
    fn new(settings: &'a BindingBuilderSettings<'a>, platforms: PlatformLocations) -> Self {
        Self {
            settings,
            platforms,
        }
    }

    fn generate(&mut self) {
        // Clear/create generated files
        let build_dir = self.build_dir();
        if build_dir.exists() {
            fs::remove_dir_all(&build_dir).unwrap();
        }
        fs::create_dir_all(&build_dir).unwrap();

        let config = dotnet_oo_bindgen::DotnetBindgenConfig {
            output_dir: build_dir,
            ffi_name: self.settings.ffi_name.to_owned(),
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
        // Run unit tests
        let result = Command::new("dotnet")
            .current_dir(&self.output_dir())
            .arg("pack")
            .arg("--configuration")
            .arg("Release")
            .arg("--output")
            .arg("nupkg")
            .status()
            .unwrap();
        assert!(result.success());
    }
}*/

struct JavaBindingBuilder<'a> {
    settings: &'a BindingBuilderSettings<'a>,
    platforms: PlatformLocations,
}

impl<'a> JavaBindingBuilder<'a> {
    fn output_dir(&self) -> PathBuf {
        let mut output_dir = PathBuf::from(self.settings.destination_path);
        output_dir.push("java");
        output_dir
    }

    fn build_dir(&self) -> PathBuf {
        let mut output_dir = self.output_dir();
        output_dir.push(self.settings.library.name.to_owned());
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
    fn new(settings: &'a BindingBuilderSettings<'a>, platforms: PlatformLocations) -> Self {
        Self {
            settings,
            platforms,
        }
    }

    fn generate(&mut self) {
        // Clear/create generated files
        let build_dir = self.build_dir();
        if build_dir.exists() {
            fs::remove_dir_all(&build_dir).unwrap();
        }
        fs::create_dir_all(&build_dir).unwrap();

        let config = java_oo_bindgen::JavaBindgenConfig {
            output_dir: build_dir,
            ffi_name: self.settings.ffi_name.to_owned(),
            group_id: "io.stepfunc".to_string(),
            platforms: self.platforms.clone(),
        };

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
