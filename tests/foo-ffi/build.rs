use rust_oo_bindgen::RustCodegen;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    match foo_schema::build_lib() {
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(-1);
        }
        Ok(lib) => {
            RustCodegen::new(&lib).generate().unwrap();
        }
    }
}
