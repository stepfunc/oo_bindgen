use oo_bindgen::platforms::*;
use oo_bindgen::Library;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    match dnp3_schema::build_lib() {
        Ok(lib) => {
            println!("Success!");

            generate_c_lib(&lib);
            generate_dotnet_lib(&lib);
            build_and_test_dotnet_lib();
        }
        Err(err) => println!("Failed: {}", err.to_string()),
    }
}

fn generate_c_lib(lib: &Library) {
    let mut platforms = PlatformLocations::new();
    platforms.add(Platform::current(), PathBuf::from("./target/debug/deps"));

    let config = c_oo_bindgen::CBindgenConfig {
        output_dir: PathBuf::from("example/bindings/c/generated"),
        ffi_name: "dnp3_ffi".to_string(),
        platforms,
    };

    c_oo_bindgen::generate_c_package(&lib, &config).unwrap();
}

fn generate_dotnet_lib(lib: &Library) {
    let mut platforms = PlatformLocations::new();
    platforms.add(Platform::current(), PathBuf::from("./target/debug/deps"));

    let config = dotnet_oo_bindgen::DotnetBindgenConfig {
        output_dir: PathBuf::from("example/bindings/dotnet/dnp3rs"),
        ffi_name: "dnp3_ffi".to_string(),
        platforms,
    };

    dotnet_oo_bindgen::generate_dotnet_bindings(&lib, &config).unwrap();
}

fn build_and_test_dotnet_lib() {
    let build_dir = "example/bindings/dotnet";
    let result = Command::new("dotnet")
        .current_dir(&build_dir)
        .arg("build")
        .status()
        .unwrap();
    assert!(result.success());

    let result = Command::new("dotnet")
        .current_dir(&build_dir)
        .arg("test")
        .status()
        .unwrap();
    assert!(result.success());
}
