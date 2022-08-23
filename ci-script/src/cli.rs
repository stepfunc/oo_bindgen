use clap::{App, Arg, ArgMatches};

pub(crate) fn build() -> ArgMatches {
    App::new("oo-bindgen")
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
                .short('f')
                .long("extra-files")
                .takes_value(true)
                .help("Path to extra files to include in the generated bindings"),
        )
        .get_matches()
}
