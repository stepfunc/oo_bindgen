use dnp3_bindings::build_lib;
use oo_bindgen::Library;
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
    let config = c_oo_bindgen::CBindgenConfig {
        output_dir: PathBuf::from("result/c"),
    };

    c_oo_bindgen::generate_c_header(&lib, &config).unwrap();
}

fn generate_dotnet_lib(lib: &Library) {
    let config = dotnet_oo_bindgen::DotnetBindgenConfig {
        output_dir: PathBuf::from("result/dotnet"),
        ffi_name: "dnp3_ffi".to_string(),
        compiled_ffi_dir: PathBuf::from(".\\target\\debug")
    };

    dotnet_oo_bindgen::generate_dotnet_bindings(&lib, &config).unwrap();
}
