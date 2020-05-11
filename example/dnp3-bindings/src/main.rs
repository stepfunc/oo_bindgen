use dnp3_bindings::build_lib;
use oo_bindgen::Library;
use oo_bindgen::platforms::*;
use std::path::PathBuf;

fn main() {
    match build_lib() {
        Ok(lib) => {
            println!("Success!");

            generate_c_lib(&lib);
            generate_dotnet_lib(&lib);
        },
        Err(err) => println!("Failed: {}", err.to_string())
    }
}

fn generate_c_lib(lib: &Library) {
    let mut platforms = PlatformLocations::new();
    platforms.add(Platform::current(), PathBuf::from("./target/debug/deps"));

    let config = c_oo_bindgen::CBindgenConfig {
        output_dir: PathBuf::from("result/c"),
        ffi_name: "dnp3_ffi".to_string(),
        platforms: platforms,
    };

    c_oo_bindgen::generate_c_package(&lib, &config).unwrap();
}

fn generate_dotnet_lib(lib: &Library) {
    let mut platforms = PlatformLocations::new();
    platforms.add(Platform::current(), PathBuf::from("./target/debug/deps"));

    let config = dotnet_oo_bindgen::DotnetBindgenConfig {
        output_dir: PathBuf::from("result/dotnet/dnp3rs"),
        ffi_name: "dnp3_ffi".to_string(),
        platforms,
    };

    dotnet_oo_bindgen::generate_dotnet_bindings(&lib, &config).unwrap();
}
