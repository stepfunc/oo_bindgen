use std::path::PathBuf;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Platform {
    Win64,
    Win32,
    Linux,
}

impl Platform {
    pub fn current() -> Self {
        if cfg!(target_os = "windows") && cfg!(target_pointer_width = "32") {
            return Self::Win32;
        }
        if cfg!(target_os = "windows") && cfg!(target_pointer_width = "64") {
            return Self::Win64;
        }
        if cfg!(target_os = "linux") && cfg!(target_pointer_width = "64") {
            return Self::Linux;
        }

        unimplemented!("Current platform is not supported")
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            Self::Win64 => "win-x64",
            Self::Win32 => "win-x32",
            Self::Linux => "linux",
        }
    }
}

pub struct PlatformLocation {
    pub platform: Platform,
    pub location: PathBuf,
}

impl PlatformLocation {
    pub fn new(platform: Platform, location: PathBuf) -> Self {
        Self { platform, location }
    }

    pub fn lib_filename<T: AsRef<str>>(&self, libname: T) -> String {
        match self.platform {
            Platform::Win64 | Platform::Win32 => format!("{}.dll.lib", libname.as_ref()),
            Platform::Linux => format!("lib{}.so", libname.as_ref()),
        }
    }

    pub fn bin_filename<T: AsRef<str>>(&self, libname: T) -> String {
        match self.platform {
            Platform::Win64 | Platform::Win32 => format!("{}.dll", libname.as_ref()),
            Platform::Linux => format!("lib{}.so", libname.as_ref()),
        }
    }
}

#[derive(Clone)]
pub struct PlatformLocations {
    pub win64: Option<PathBuf>,
    pub win32: Option<PathBuf>,
    pub linux: Option<PathBuf>,
}

impl PlatformLocations {
    pub fn new() -> Self {
        PlatformLocations {
            win64: None,
            win32: None,
            linux: None,
        }
    }

    pub fn add(&mut self, platform: Platform, location: PathBuf) {
        match platform {
            Platform::Win64 => self.win64 = Some(location),
            Platform::Win32 => self.win32 = Some(location),
            Platform::Linux => self.linux = Some(location),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = PlatformLocation> {
        let mut vec = Vec::new();
        if let Some(loc) = &self.win64 {
            vec.push(PlatformLocation::new(Platform::Win64, loc.clone()))
        }
        if let Some(loc) = &self.win32 {
            vec.push(PlatformLocation::new(Platform::Win32, loc.clone()))
        }
        if let Some(loc) = &self.linux {
            vec.push(PlatformLocation::new(Platform::Linux, loc.clone()))
        }
        vec.into_iter()
    }

    pub fn is_empty(&self) -> bool {
        self.win64.is_none() && self.win32.is_none() && self.linux.is_none()
    }
}

impl Default for PlatformLocations {
    fn default() -> Self {
        Self::new()
    }
}
