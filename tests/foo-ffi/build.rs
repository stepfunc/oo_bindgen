fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    match foo_schema::build_lib() {
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(-1);
        }
        Ok(lib) => {
            oo_bindgen::backend::rust::generate_ffi(&lib).unwrap();
        }
    }
}
