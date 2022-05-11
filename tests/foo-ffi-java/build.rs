use java_oo_bindgen::generate_java_ffi;
use java_oo_bindgen::JniBindgenConfig;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // normally you'd never want to write files here, but this crate isn't used as a dependency
    let out_dir: PathBuf =
        Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("src/generated");

    let config = JniBindgenConfig {
        rust_output_dir: out_dir,
        group_id: "io.stepfunc".to_string(),
        ffi_name: "foo_ffi".to_string(),
    };

    match foo_schema::build_lib() {
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(-1);
        }
        Ok(lib) => {
            println!("writing the lib!");
            generate_java_ffi(&lib, &config).unwrap();
            println!("success!");
        }
    }
}
