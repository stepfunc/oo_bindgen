use std::env::var;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

fn main() {
    let file = File::create(format!("{}/paths.rs", var("OUT_DIR").unwrap())).unwrap();
    let mut writer = BufWriter::new(file);

    let target_dir = get_target_directory().unwrap();
    let target_dir_str = target_dir.to_str().unwrap();

    writeln!(
        &mut writer,
        "const TARGET_DIR: &str = r\"{}\";",
        target_dir_str
    )
    .unwrap();
}

fn get_target_directory() -> Option<PathBuf> {
    let path = PathBuf::from(var("OUT_DIR").ok()?);
    Some(path.parent()?.parent()?.parent()?.to_owned())
}
