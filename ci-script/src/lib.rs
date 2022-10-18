pub(crate) mod builders;
pub(crate) mod cli;

// re-export this so that dependencies don't need the dotnet generator directly
pub use dotnet_oo_bindgen::TargetFramework;

use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

use oo_bindgen::backend::*;
use oo_bindgen::model::Library;

use crate::cli::Args;

pub fn run(settings: BindingBuilderSettings) {
    let args = Args::get();

    let (options, platforms) = get_platforms(&args);

    assert!(!platforms.is_empty(), "No platforms found!");

    for p in platforms.iter() {
        tracing::info!("Platform {} in {}", p.platform, p.location.display());
    }

    if args.build_c {
        let mut builder = crate::builders::c::CBindingBuilder::new(
            settings.clone(),
            platforms.clone(),
            &args.extra_files,
        );
        builder.run(options);
    }
    if args.build_dotnet {
        let mut builder = crate::builders::dotnet::DotnetBindingBuilder::new(
            settings.clone(),
            args.target_framework,
            platforms.clone(),
            &args.extra_files,
        );
        builder.run(options);
    }
    if args.build_java {
        let mut builder =
            crate::builders::java::JavaBindingBuilder::new(settings, platforms, &args.extra_files);
        builder.run(options);
    }
}

fn get_platforms(args: &Args) -> (RunOptions, PlatformLocations) {
    let mut platforms = PlatformLocations::new();
    let options = if let Some(dir) = &args.package_dir {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                if let Some(p) = Platform::find(&entry.file_name().to_string_lossy()) {
                    platforms.add(p.clone(), entry.path());
                }
            }
        }
        RunOptions {
            test: false,
            package: true,
            docs: false,
        }
    } else {
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
                let platform =
                    Platform::guess_current().expect("Could not determine current platform");
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

        platforms.add(platform.clone(), artifact_dir);

        RunOptions {
            test: !args.no_tests,
            package: false,
            docs: args.generate_doxygen,
        }
    };
    (options, platforms)
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
    /// Library to build
    pub library: Rc<Library>,
}

/// options for an invocation of a binding builder
#[derive(Copy, Clone, Debug)]
pub struct RunOptions {
    /// run the tests
    pub test: bool,
    /// package the library if this is a separate step from generation
    pub package: bool,
    /// generate the docs if this is a separate step
    pub docs: bool,
}

impl RunOptions {
    pub fn new() -> Self {
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

trait BindingBuilder {
    fn name() -> &'static str;
    fn generate(&mut self, is_packaging: bool, generate_docs: bool);
    fn build(&mut self);
    fn test(&mut self);
    fn package(&mut self);

    /// run the builder
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
