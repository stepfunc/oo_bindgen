use rust_oo_bindgen::RustCodegen;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let lib = foo_schema::build_lib().unwrap();
    RustCodegen::new(&lib).generate().unwrap();
}
