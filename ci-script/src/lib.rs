pub(crate) mod builders;
pub(crate) mod cli;

use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::rc::Rc;

use oo_bindgen::backend::*;
use oo_bindgen::model::Library;

use crate::cli::{Args, PackageOptions};

/// Run the binding generator
pub fn run(settings: BindingBuilderSettings) {
    let args = Args::get();

    let (options, platforms) = {
        let span = tracing::info_span!("configure()");
        span.in_scope(|| get_platforms(&args))
    };

    if args.build_c {
        let mut builder = crate::builders::c::CBindingBuilder::new(
            settings.clone(),
            platforms.cpp,
            &args.extra_files,
        );
        builder.run(options);
    }
    if args.build_dotnet {
        let mut builder = crate::builders::dotnet::DotnetBindingBuilder::new(
            settings.clone(),
            args.target_framework,
            platforms.dotnet,
            &args.extra_files,
        );
        builder.run(options);
    }
    if args.build_java {
        let mut builder = crate::builders::java::JavaBindingBuilder::new(
            settings,
            platforms.java,
            &args.extra_files,
        );
        builder.run(options);
    }
}

struct LanguagePlatforms {
    cpp: PlatformLocations,
    dotnet: PlatformLocations,
    java: PlatformLocations,
}

impl LanguagePlatforms {
    fn same(locations: PlatformLocations) -> Self {
        Self {
            cpp: locations.clone(),
            dotnet: locations.clone(),
            java: locations,
        }
    }
}

fn get_single_platform(args: &Args) -> (RunOptions, LanguagePlatforms) {
    let artifact_dir = match &args.artifact_dir {
        Some(x) => {
            tracing::info!("Artifact dir is {}", x.display());
            x.clone()
        }
        None => {
            let x: PathBuf = "./target/release".into();
            tracing::info!("No artifact dir specified, assuming: {}", x.display());
            x
        }
    };

    let platform = match &args.target_triple {
        None => {
            let platform = Platform::guess_current().expect("Could not determine current platform");
            tracing::info!(
                "No target platform specified assuming target is the host platform: {}",
                platform
            );
            platform
        }
        Some(tt) => match Platform::find(tt) {
            None => panic!("Unable to determine Platform from target triple: {}", tt),
            Some(x) => x,
        },
    };

    let mut platforms = PlatformLocations::new();
    platforms.add(platform.clone(), artifact_dir);

    let options = RunOptions {
        test: !args.no_tests,
        package: false,
        docs: args.generate_doxygen,
    };
    (options, LanguagePlatforms::same(platforms))
}

fn get_packaging_platforms(
    dir: &PathBuf,
    options: PackageOptions,
) -> (RunOptions, LanguagePlatforms) {
    let mut platforms = PlatformLocations::new();
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            if let Some(p) = Platform::find(&entry.file_name().to_string_lossy()) {
                platforms.add(p.clone(), entry.path());
            }
        }
    }

    assert!(!platforms.is_empty(), "No platforms found!");

    for p in platforms.iter() {
        tracing::info!("Platform {} in {}", p.platform, p.location.display());
    }

    let cpp = {
        let mut cpp = PlatformLocations::new();
        for p in platforms.iter() {
            if options.package_cpp(&p.platform) {
                cpp.locations.push(p.clone());
            } else {
                tracing::warn!("Ignoring available C/C++ package {}", p.platform)
            }
        }
        cpp
    };

    let dotnet = {
        let mut dotnet = PlatformLocations::new();
        for p in platforms.iter() {
            if options.package_dotnet(&p.platform) {
                dotnet.locations.push(p.clone());
            } else {
                tracing::warn!("Ignoring available .NET package {}", p.platform)
            }
        }
        dotnet
    };

    let java = {
        let mut java = PlatformLocations::new();
        for p in platforms.iter() {
            if options.package_java(&p.platform) {
                java.locations.push(p.clone());
            } else {
                tracing::warn!("Ignoring available Java package {}", p.platform)
            }
        }
        java
    };

    let options = RunOptions {
        test: false,
        package: true,
        docs: false,
    };

    (options, LanguagePlatforms { cpp, dotnet, java })
}

fn get_platforms(args: &Args) -> (RunOptions, LanguagePlatforms) {
    if let Some(dir) = &args.package_dir {
        let config_path = args
            .package_options
            .as_ref()
            .expect("You must specify the options file when packaging");
        let options: PackageOptions = {
            let file = File::open(config_path).expect("Error opening package options file");
            serde_json::from_reader(file).expect("Error reading package options JSON")
        };

        get_packaging_platforms(dir, options)
    } else {
        get_single_platform(args)
    }
}

/// Settings that control binding generation
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
    /// Library to build
    pub library: Rc<Library>,
}

/// options for an invocation of a binding builder
#[derive(Copy, Clone, Debug)]
struct RunOptions {
    /// run the tests
    pub(crate) test: bool,
    /// package the library if this is a separate step from generation
    pub(crate) package: bool,
    /// generate the docs if this is a separate step
    pub(crate) docs: bool,
}

impl RunOptions {
    pub(crate) fn new() -> Self {
        Self {
            test: false,
            package: false,
            docs: false,
        }
    }
}

impl Default for RunOptions {
    fn default() -> Self {
        Self::new()
    }
}

trait BindingBuilder: Sized {
    fn name() -> &'static str;
    fn generate(&mut self, is_packaging: bool, generate_docs: bool);
    fn build(&mut self);
    fn test(&mut self);
    fn package(&mut self);

    fn run(&mut self, options: RunOptions) {
        let span = tracing::info_span!("generate()", lang = Self::name());
        span.in_scope(|| {
            tracing::info!("begin");
            self.generate(options.package, options.docs);
            tracing::info!("end");
        });

        if options.package {
            let span = tracing::info_span!("package()", lang = Self::name());
            span.in_scope(|| {
                tracing::info!("begin");
                self.package();
                tracing::info!("end");
            });
        } else if options.test {
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
