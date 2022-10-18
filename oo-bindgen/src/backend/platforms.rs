use platforms::target::OS;
use platforms::Platform;
use std::path::PathBuf;

pub trait PlatformExt {
    fn static_lib_filename<T: AsRef<str>>(&self, libname: T) -> String;
    fn dyn_lib_filename<T: AsRef<str>>(&self, libname: T) -> String;
    fn bin_filename<T: AsRef<str>>(&self, libname: T) -> String;
}

// Source for these are:
// - https://github.com/rust-lang/rust/blob/1.58.1/library/std/src/sys/mod.rs
// - https://github.com/rust-lang/rust/blob/1.58.1/library/std/src/sys/unix/env.rs
// - https://github.com/rust-lang/rust/blob/1.58.1/library/std/src/sys/windows/env.rs
// - https://github.com/rust-lang/rust/blob/1.58.1/src/tools/compiletest/src/runtest.rs
impl PlatformExt for Platform {
    fn static_lib_filename<T: AsRef<str>>(&self, libname: T) -> String {
        if self.target_os == OS::Windows {
            format!("{}.lib", libname.as_ref())
        } else {
            format!("lib{}.a", libname.as_ref())
        }
    }

    fn dyn_lib_filename<T: AsRef<str>>(&self, libname: T) -> String {
        if self.target_os == OS::Windows {
            format!("{}.dll.lib", libname.as_ref())
        } else if self.target_os == OS::MacOS {
            format!("lib{}.dylib", libname.as_ref())
        } else {
            format!("lib{}.so", libname.as_ref())
        }
    }

    fn bin_filename<T: AsRef<str>>(&self, libname: T) -> String {
        if self.target_os == OS::Windows {
            format!("{}.dll", libname.as_ref())
        } else if self.target_os == OS::MacOS {
            format!("lib{}.dylib", libname.as_ref())
        } else {
            format!("lib{}.so", libname.as_ref())
        }
    }
}

#[derive(Clone)]
pub struct PlatformLocation {
    pub platform: Platform,
    pub location: PathBuf,
}

impl PlatformLocation {
    pub fn new(platform: Platform, location: PathBuf) -> Self {
        Self { platform, location }
    }
}

#[derive(Clone)]
pub struct PlatformLocations {
    pub locations: Vec<PlatformLocation>,
}

impl PlatformLocations {
    pub fn new() -> Self {
        PlatformLocations {
            locations: Vec::new(),
        }
    }

    pub fn single(&self) -> Option<&PlatformLocation> {
        if self.locations.len() == 1 {
            self.locations.get(0)
        } else {
            None
        }
    }

    pub fn add(&mut self, platform: Platform, location: PathBuf) {
        let loc = PlatformLocation::new(platform, location);
        self.locations.push(loc);
    }

    pub fn iter(&self) -> impl Iterator<Item = &PlatformLocation> {
        self.locations.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.locations.is_empty()
    }
}

impl Default for PlatformLocations {
    fn default() -> Self {
        Self::new()
    }
}
