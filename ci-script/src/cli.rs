use clap::Parser;
use dotnet_oo_bindgen::TargetFramework;
use std::path::PathBuf;

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
    /// Path to where the compiled FFI/JNI shared libraries reside or a directory with multiple target triple dirs if packaging
    #[arg(long = "artifact-dir", short = 'a')]
    pub(crate) artifact_dir: PathBuf,
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
    #[arg(long = "package", short = 'k', default_value_t = false)]
    pub(crate) package: bool,
    /// Path(s) to extra files to include in the generated bindings
    #[arg(short = 'f', long = "extra-files")]
    pub(crate) extra_files: Vec<PathBuf>,
}
