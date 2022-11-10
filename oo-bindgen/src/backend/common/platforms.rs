use platforms::target::OS;
use platforms::Platform;
use std::path::PathBuf;

pub(crate) trait PlatformExt {
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
        } else if self.target_os == OS::MacOS || self.target_os == OS::iOS {
            format!("lib{}.dylib", libname.as_ref())
        } else {
            format!("lib{}.so", libname.as_ref())
        }
    }

    fn bin_filename<T: AsRef<str>>(&self, libname: T) -> String {
        if self.target_os == OS::Windows {
            format!("{}.dll", libname.as_ref())
        } else if self.target_os == OS::MacOS || self.target_os == OS::iOS {
            format!("lib{}.dylib", libname.as_ref())
        } else {
            format!("lib{}.so", libname.as_ref())
        }
    }
}

#[derive(Clone)]
pub(crate) struct PlatformLocation {
    pub(crate) platform: Platform,
    pub(crate) location: PathBuf,
}

impl PlatformLocation {
    pub(crate) fn new(platform: Platform, location: PathBuf) -> Self {
        Self { platform, location }
    }
}

#[derive(Clone)]
pub(crate) struct PlatformLocations {
    pub(crate) locations: Vec<PlatformLocation>,
}

impl PlatformLocations {
    pub(crate) fn new() -> Self {
        PlatformLocations {
            locations: Vec::new(),
        }
    }

    pub(crate) fn add(&mut self, platform: Platform, location: PathBuf) {
        let loc = PlatformLocation::new(platform, location);
        self.locations.push(loc);
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &PlatformLocation> {
        self.locations.iter()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.locations.is_empty()
    }
}

impl Default for PlatformLocations {
    fn default() -> Self {
        Self::new()
    }
}
