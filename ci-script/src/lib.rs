pub(crate) mod builders;
pub(crate) mod cli;

// re-export this so that dependencies don't need the dotnet generator directly
pub use dotnet_oo_bindgen::TargetFramework;

use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

use oo_bindgen::backend::*;
use oo_bindgen::model::Library;

use clap::Parser;

const SUPPORTED_PLATFORMS: &[&Platform] = &[
    &platform::X86_64_PC_WINDOWS_MSVC,
    &platform::I686_PC_WINDOWS_MSVC,
    &platform::X86_64_UNKNOWN_LINUX_GNU,
    &platform::AARCH64_UNKNOWN_LINUX_GNU,
    &platform::ARMV7_UNKNOWN_LINUX_GNUEABIHF,
    &platform::ARM_UNKNOWN_LINUX_GNUEABIHF,
    &platform::ARM_UNKNOWN_LINUX_GNUEABI,
];

fn is_officially_supported(p: &Platform) -> bool {
    SUPPORTED_PLATFORMS
        .iter()
        .any(|x| x.target_triple == p.target_triple)
}

pub fn run(settings: BindingBuilderSettings) {
    let args = crate::cli::Cli::parse();

    let mut run_tests = !args.no_tests;

    // if no languages are selected, we build all of them
    let run_all = !args.build_c && !args.build_dotnet && !args.build_java;

    let package = args.package.is_some();

    let mut platforms = PlatformLocations::new();
    if let Some(package_src) = args.package {
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

        if !is_officially_supported(current_platform) {
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

    if args.build_c || run_all {
        let mut builder = crate::builders::c::CBindingBuilder::new(
            settings.clone(),
            platforms.clone(),
            &args.extra_files,
        );
        builder.run(run_tests, package, args.generate_doxygen);
    }
    if args.build_dotnet || run_all {
        let mut builder = crate::builders::dotnet::DotnetBindingBuilder::new(
            settings.clone(),
            platforms.clone(),
            &args.extra_files,
        );
        builder.run(run_tests, package, args.generate_doxygen);
    }
    if args.build_java || run_all {
        let mut builder =
            crate::builders::java::JavaBindingBuilder::new(settings, platforms, &args.extra_files);
        builder.run(run_tests, package, args.generate_doxygen);
    }
}

fn ffi_path() -> PathBuf {
    [env!("TARGET_DIR"), "deps"].iter().collect()
}

#[derive(Clone)]
pub struct BindingBuilderSettings {
    /// FFI target name (as specified in with `cargo build -p <...>`)
    pub ffi_target_name: &'static str,
    /// JNI target name (as specified in with `cargo build -p <...>`)
    pub jni_target_name: &'static str,
    /// Compiled FFI name (usually the same as `ffi_target_name`, but with hyphens replaced by underscores)
    pub ffi_name: &'static str,
    /// Path to the FFI target
    pub ffi_path: PathBuf,
    /// Name of the Java group (e.g. `io.stepfunc`)
    pub java_group_id: &'static str,
    /// Destination path
    pub destination_path: PathBuf,
    /// .NET target framework
    pub net_target_framework: TargetFramework,
    /// Library to build
    pub library: Rc<Library>,
}

trait BindingBuilder {
    fn name() -> &'static str;
    fn generate(&mut self, is_packaging: bool, generate_doxygen: bool);
    fn build(&mut self);
    fn test(&mut self);
    fn package(&mut self);

    /// run the builder
    fn run(&mut self, run_tests: bool, package: bool, generate_doxygen: bool) {
        let span = tracing::info_span!("generate()", lang = Self::name());
        span.in_scope(|| {
            tracing::info!("begin");
            self.generate(package, generate_doxygen);
            tracing::info!("end");
        });

        if package {
            let span = tracing::info_span!("package()", lang = Self::name());
            span.in_scope(|| {
                tracing::info!("begin");
                self.package();
                tracing::info!("end");
            });
        } else if run_tests {
            let span = tracing::info_span!("build()", lang = Self::name());
            span.in_scope(|| {
                tracing::info!("begin");
                self.build();
                tracing::info!("end");
            });
            let span = tracing::info_span!("test()", lang = Self::name());
            span.in_scope(|| {
                tracing::info!("begin");
                self.test();
                tracing::info!("end");
            });
        }
    }
}
