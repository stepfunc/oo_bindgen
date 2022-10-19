use clap::Parser;
use dotnet_oo_bindgen::TargetFramework;
use std::collections::HashMap;
use std::path::PathBuf;

impl Args {
    pub fn get() -> Self {
        let mut args = crate::cli::Args::parse();
        if !(args.build_c || args.build_dotnet || args.build_java) {
            args.build_c = true;
            args.build_dotnet = true;
            args.build_java = true;
        }
        args
    }
}

use crate::Platform;
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct EnabledLanguages {
    pub(crate) cpp: bool,
    pub(crate) dotnet: bool,
    pub(crate) java: bool,
}

#[derive(Deserialize)]
pub(crate) struct PackageOptions {
    /// This limits which available target platforms are packaged for each language
    targets: HashMap<String, EnabledLanguages>,
}

impl PackageOptions {
    pub(crate) fn package_dotnet(&self, platform: &Platform) -> bool {
        self.targets
            .get(platform.target_triple)
            .map(|x| x.dotnet)
            .unwrap_or(false)
    }

    pub(crate) fn package_cpp(&self, platform: &Platform) -> bool {
        self.targets
            .get(platform.target_triple)
            .map(|x| x.cpp)
            .unwrap_or(false)
    }

    pub(crate) fn package_java(&self, platform: &Platform) -> bool {
        self.targets
            .get(platform.target_triple)
            .map(|x| x.java)
            .unwrap_or(false)
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    /// build the C bindings
    #[arg(long = "c", default_value_t = false)]
    pub(crate) build_c: bool,
    /// build the .NET bindings
    #[arg(long = "dotnet", default_value_t = false)]
    pub(crate) build_dotnet: bool,
    /// build the Java bindings
    #[arg(long = "java", default_value_t = false)]
    pub(crate) build_java: bool,
    /// Path to where the compiled FFI/JNI shared libraries reside or a directory with multiple target triple dirs if packaging.
    /// If not specified, ./release/target is assumed
    #[arg(long = "artifact-dir", short = 'a')]
    pub(crate) artifact_dir: Option<PathBuf>,
    /// Target triple to use to lookup the platform for generation, otherwise assume the HOST platform.
    #[arg(long = "target", short = 'r')]
    pub(crate) target_triple: Option<String>,
    /// Target .NET framework, which indirectly determines the C# language version
    #[arg(value_enum, short = 't', long = "target-dotnet-framework", default_value_t = TargetFramework::NetStandard2_0)]
    pub(crate) target_framework: TargetFramework,
    /// generate doxygen documentation
    #[arg(long = "doxygen", default_value_t = false)]
    pub(crate) generate_doxygen: bool,
    /// do NOT run the unit tests
    #[arg(long = "no-tests", default_value_t = false)]
    pub(crate) no_tests: bool,
    /// Generate package from the provided directory
    #[arg(long = "package", short = 'k')]
    pub(crate) package_dir: Option<PathBuf>,
    /// Generate package(s) with the following options file
    #[arg(long = "options", short = 'o')]
    pub(crate) package_options: Option<PathBuf>,
    /// Path(s) to extra files to include in the generated bindings
    #[arg(short = 'f', long = "extra-files")]
    pub(crate) extra_files: Vec<PathBuf>,
}
