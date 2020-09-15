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
        .get_matches();

    let run_tests = !matches.is_present("no-tests");

    let run_c = matches.is_present("c");
    let run_dotnet = matches.is_present("dotnet");
    let run_java = matches.is_present("java");
    let run_all = !run_c && !run_dotnet && !run_java;

    if run_c || run_all {
        run_builder::<CBindingBuilder>(&settings, run_tests);

        if matches.is_present("doxygen") {
            CBindingBuilder::new(&settings).build_doxygen();
        }
    }
    if run_dotnet || run_all {
        run_builder::<DotnetBindingBuilder>(&settings, run_tests);
    }
    if run_java || run_all {
        run_builder::<JavaBindingBuilder>(&settings, run_tests);
    }
}

fn ffi_path() -> PathBuf {
    [TARGET_DIR, "deps"].iter().collect()
}

fn run_builder<'a, B: BindingBuilder<'a>>(settings: &'a BindingBuilderSettings, run_tests: bool) {
    let mut builder = B::new(settings);
    builder.generate();
    builder.build();
    if run_tests {
        builder.test();
    }
}

pub struct BindingBuilderSettings<'a> {
    pub ffi_name: &'a str,
    pub destination_path: &'a Path,
    pub library: &'a Library,
}

trait BindingBuilder<'a> {
    fn new(settings: &'a BindingBuilderSettings<'a>) -> Self;
    fn generate(&mut self);
    fn build(&mut self);
    fn test(&mut self);
}

struct CBindingBuilder<'a>(&'a BindingBuilderSettings<'a>);

impl<'a> CBindingBuilder<'a> {
    fn build_doxygen(&self) {
        let mut platforms = PlatformLocations::new();
        platforms.add(Platform::current(), ffi_path());

        let config = c_oo_bindgen::CBindgenConfig {
            output_dir: self.output_dir(),
            ffi_name: self.0.ffi_name.to_owned(),
            platforms,
        };

        c_oo_bindgen::generate_doxygen(&self.0.library, &config).expect("failed to package C lib");
    }

    fn output_dir(&self) -> PathBuf {
        let mut output_dir = PathBuf::from(self.0.destination_path);
        output_dir.push("c");
        output_dir.push("generated");
        output_dir
    }

    fn build_dir(&self) -> PathBuf {
        let mut build_dir = PathBuf::from(self.0.destination_path);
        build_dir.push("c");
        build_dir.push("build");
        build_dir
    }
}

impl<'a> BindingBuilder<'a> for CBindingBuilder<'a> {
    fn new(settings: &'a BindingBuilderSettings<'a>) -> Self {
        Self(settings)
    }

    fn generate(&mut self) {
        let mut platforms = PlatformLocations::new();
        platforms.add(Platform::current(), ffi_path());

        let config = c_oo_bindgen::CBindgenConfig {
            output_dir: self.output_dir(),
            ffi_name: self.0.ffi_name.to_owned(),
            platforms,
        };

        c_oo_bindgen::generate_c_package(&self.0.library, &config)
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
}

struct DotnetBindingBuilder<'a>(&'a BindingBuilderSettings<'a>);

impl<'a> DotnetBindingBuilder<'a> {
    fn output_dir(&self) -> PathBuf {
        let mut output_dir = PathBuf::from(self.0.destination_path);
        output_dir.push("dotnet");
        output_dir
    }

    fn build_dir(&self) -> PathBuf {
        let mut output_dir = self.output_dir();
        output_dir.push(self.0.library.name.to_owned());
        output_dir
    }
}

impl<'a> BindingBuilder<'a> for DotnetBindingBuilder<'a> {
    fn new(settings: &'a BindingBuilderSettings<'a>) -> Self {
        Self(settings)
    }

    fn generate(&mut self) {
        // Clear/create generated files
        let build_dir = self.build_dir();
        if build_dir.exists() {
            fs::remove_dir_all(&build_dir).unwrap();
        }
        fs::create_dir_all(&build_dir).unwrap();

        let mut platforms = PlatformLocations::new();
        platforms.add(Platform::current(), ffi_path());

        let config = dotnet_oo_bindgen::DotnetBindgenConfig {
            output_dir: build_dir,
            ffi_name: self.0.ffi_name.to_owned(),
            platforms,
        };

        dotnet_oo_bindgen::generate_dotnet_bindings(self.0.library, &config).unwrap();
    }

    fn build(&mut self) {
        let result = Command::new("dotnet")
            .current_dir(&self.output_dir())
            .arg("build")
            .status()
            .unwrap();
        assert!(result.success());
    }

    fn test(&mut self) {
        // Run unit tests
        let result = Command::new("dotnet")
            .current_dir(&self.output_dir())
            .arg("test")
            .status()
            .unwrap();
        assert!(result.success());
    }
}

struct JavaBindingBuilder<'a>(&'a BindingBuilderSettings<'a>);

impl<'a> JavaBindingBuilder<'a> {
    fn output_dir(&self) -> PathBuf {
        let mut output_dir = PathBuf::from(self.0.destination_path);
        output_dir.push("java");
        output_dir
    }

    fn build_dir(&self) -> PathBuf {
        let mut output_dir = self.output_dir();
        output_dir.push(self.0.library.name.to_owned());
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
    fn new(settings: &'a BindingBuilderSettings<'a>) -> Self {
        Self(settings)
    }

    fn generate(&mut self) {
        // Clear/create generated files
        let build_dir = self.build_dir();
        if build_dir.exists() {
            fs::remove_dir_all(&build_dir).unwrap();
        }
        fs::create_dir_all(&build_dir).unwrap();

        let mut platforms = PlatformLocations::new();
        platforms.add(Platform::current(), ffi_path());

        let config = java_oo_bindgen::JavaBindgenConfig {
            output_dir: build_dir,
            ffi_name: self.0.ffi_name.to_owned(),
            group_id: "io.stepfunc".to_string(),
            platforms,
        };

        java_oo_bindgen::generate_java_bindings(self.0.library, &config).unwrap();
    }

    fn build(&mut self) {
        let result = self.maven().arg("compile").status().unwrap();
        assert!(result.success());
    }

    fn test(&mut self) {
        let result = self.maven().arg("verify").status().unwrap();
        assert!(result.success());
    }
}
