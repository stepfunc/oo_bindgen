use std::env::var;
use std::path::PathBuf;

fn main() {
    println!(
        "cargo:rustc-env=TARGET_DIR={}",
        get_target_directory().unwrap().to_string_lossy()
    );
    println!("cargo:rustc-env=PROFILE={}", var("PROFILE").unwrap());
}

fn get_target_directory() -> Option<PathBuf> {
    let path = PathBuf::from(var("OUT_DIR").ok()?);
    Some(path.parent()?.parent()?.parent()?.to_owned())
}
