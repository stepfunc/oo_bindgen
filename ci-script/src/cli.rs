use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Cli {
    /// build the C bindings
    #[arg(long = "c", default_value_t = false)]
    pub(crate) build_c: bool,
    /// build the .NET bindings
    #[arg(long = "dotnet", default_value_t = false)]
    pub(crate) build_dotnet: bool,
    /// build the Java bindings
    #[arg(long = "java", default_value_t = false)]
    pub(crate) build_java: bool,
    /// generate doxygen documentation
    #[arg(long = "doxygen", default_value_t = false)]
    pub(crate) generate_doxygen: bool,
    /// do NOT run the unit tests
    #[arg(long = "no-tests", default_value_t = false)]
    pub(crate) no_tests: bool,
    /// Generate package from the provided directory
    pub(crate) package: Option<PathBuf>,
    /// Path(s) to extra files to include in the generated bindings
    #[arg(short = 'f', long = "extra-files")]
    pub(crate) extra_files: Vec<PathBuf>,
}
