use std::path::PathBuf;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Platform {
    Win64,
    Linux,
    LinuxMusl,
}

impl Platform {
    pub fn current() -> Self {
        if cfg!(target_os = "windows") && cfg!(target_pointer_width = "64") {
            return Self::Win64;
        }
        if cfg!(target_os = "linux")
            && cfg!(target_pointer_width = "64")
            && !cfg!(target_env = "musl")
        {
            return Self::Linux;
        }
        if cfg!(target_os = "linux")
            && cfg!(target_pointer_width = "64")
            && cfg!(target_env = "musl")
        {
            return Self::LinuxMusl;
        }

        unimplemented!("Current platform is not supported")
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            Self::Win64 => "win-x64",
            Self::Linux => "linux",
            Self::LinuxMusl => "linux-musl",
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

    pub fn lib_filename<T: AsRef<str>>(&self, libname: T) -> String {
        match self.platform {
            Platform::Win64 => format!("{}.dll.lib", libname.as_ref()),
            Platform::Linux => format!("lib{}.so", libname.as_ref()),
            Platform::LinuxMusl => format!("lib{}.a", libname.as_ref()),
        }
    }

    pub fn bin_filename<T: AsRef<str>>(&self, libname: T) -> String {
        match self.platform {
            Platform::Win64 => format!("{}.dll", libname.as_ref()),
            Platform::Linux => format!("lib{}.so", libname.as_ref()),
            Platform::LinuxMusl => format!("lib{}.a", libname.as_ref()),
        }
    }
}

#[derive(Clone)]
pub struct PlatformLocations {
    pub win64: Option<PlatformLocation>,
    pub linux: Option<PlatformLocation>,
    pub linux_musl: Option<PlatformLocation>,
}

impl PlatformLocations {
    pub fn new() -> Self {
        PlatformLocations {
            win64: None,
            linux: None,
            linux_musl: None,
        }
    }

    pub fn add(&mut self, platform: Platform, location: PathBuf) {
        let loc = PlatformLocation::new(platform, location);
        match platform {
            Platform::Win64 => self.win64 = Some(loc),
            Platform::Linux => self.linux = Some(loc),
            Platform::LinuxMusl => self.linux_musl = Some(loc),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = PlatformLocation> {
        let mut vec = Vec::new();
        if let Some(loc) = &self.win64 {
            vec.push(loc.clone())
        }
        if let Some(loc) = &self.linux {
            vec.push(loc.clone())
        }
        if let Some(loc) = &self.linux_musl {
            vec.push(loc.clone())
        }
        vec.into_iter()
    }

    pub fn is_empty(&self) -> bool {
        self.win64.is_none() && self.linux.is_none() && self.linux_musl.is_none()
    }

    pub fn has_dynamic_lib(&self) -> bool {
        self.win64.is_some() || self.linux.is_some()
    }
}

impl Default for PlatformLocations {
    fn default() -> Self {
        Self::new()
    }
}
