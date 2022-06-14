use java_oo_bindgen::generate_java_ffi;
use java_oo_bindgen::JniBindgenConfig;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=lib.rs");

    // normally you'd never want to write files here, but this crate isn't used as a dependency
    let out_path: PathBuf = Path::new(&std::env::var_os("OUT_DIR").unwrap()).join("jni.rs");

    let config = JniBindgenConfig {
        group_id: "io.stepfunc".to_string(),
        ffi_name: "foo_ffi".to_string(),
    };

    match foo_schema::build_lib() {
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(-1);
        }
        Ok(lib) => {
            generate_java_ffi(&out_path, &lib, &config).unwrap();
        }
    }
}
