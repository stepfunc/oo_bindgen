use std::path::PathBuf;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Platform {
    /// x86_64-pc-windows-msvc
    WinX64Msvc,
    /// x86_64-unknown-linux-gnu
    LinuxX64Gnu,
    /// x86_64-unknown-linux-musl
    ///
    /// This should not be used. It doesn't use the system musl lib,
    /// so the final binary is a mix of multiple musl libs and exhibits
    /// undefined behaviours.
    LinuxX64Musl,
    /// arm-unknown-linux-gnueabi
    LinuxArm6Gnueabi,
    /// arm-unknown-linux-gnueabihf
    LinuxArm6GnueabiHf,
    /// armv7-unknown-linux-gnueabihf
    LinuxArm7GnueabiHf,
    /// aarch64-unknown-linux-gnu
    LinuxArm8Gnu,
}

impl Platform {
    pub fn from_target_triple(target_triple: &str) -> Option<Self> {
        match target_triple {
            "x86_64-pc-windows-msvc" => Some(Self::WinX64Msvc),
            "x86_64-unknown-linux-gnu" => Some(Self::LinuxX64Gnu),
            "x86_64-unknown-linux-musl" => Some(Self::LinuxX64Musl),
            "arm-unknown-linux-gnueabi" => Some(Self::LinuxArm6Gnueabi),
            "arm-unknown-linux-gnueabihf" => Some(Self::LinuxArm6GnueabiHf),
            "armv7-unknown-linux-gnueabihf" => Some(Self::LinuxArm7GnueabiHf),
            "aarch64-unknown-linux-gnu" => Some(Self::LinuxArm8Gnu),
            _ => None,
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            Self::WinX64Msvc => "x86_64-pc-windows-msvc",
            Self::LinuxX64Gnu => "x86_64-unknown-linux-gnu",
            Self::LinuxX64Musl => "x86_64-unknown-linux-musl",
            Self::LinuxArm6Gnueabi => "arm-unknown-linux-gnueabi",
            Self::LinuxArm6GnueabiHf => "arm-unknown-linux-gnueabihf",
            Self::LinuxArm7GnueabiHf => "armv7-unknown-linux-gnueabihf",
            Self::LinuxArm8Gnu => "aarch64-unknown-linux-gnu",
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
            Platform::WinX64Msvc => format!("{}.dll.lib", libname.as_ref()),
            _ => format!("lib{}.so", libname.as_ref()),
        }
    }

    pub fn bin_filename<T: AsRef<str>>(&self, libname: T) -> String {
        match self.platform {
            Platform::WinX64Msvc => format!("{}.dll", libname.as_ref()),
            _ => format!("lib{}.so", libname.as_ref()),
        }
    }
}

#[derive(Clone)]
pub struct PlatformLocations {
    pub win_x64_msvc: Option<PlatformLocation>,
    pub linux_x64_gnu: Option<PlatformLocation>,
    pub linux_x64_musl: Option<PlatformLocation>,
    pub linux_arm6_gnueabi: Option<PlatformLocation>,
    pub linux_arm6_gnueabi_hf: Option<PlatformLocation>,
    pub linux_arm7_gnueabi: Option<PlatformLocation>,
    pub linux_arm7_gnueabi_hf: Option<PlatformLocation>,
    pub linux_arm8_gnu: Option<PlatformLocation>,
}

impl PlatformLocations {
    pub fn new() -> Self {
        PlatformLocations {
            win_x64_msvc: None,
            linux_x64_gnu: None,
            linux_x64_musl: None,
            linux_arm6_gnueabi: None,
            linux_arm6_gnueabi_hf: None,
            linux_arm7_gnueabi: None,
            linux_arm7_gnueabi_hf: None,
            linux_arm8_gnu: None,
        }
    }

    pub fn add(&mut self, platform: Platform, location: PathBuf) {
        let loc = PlatformLocation::new(platform, location);
        match platform {
            Platform::WinX64Msvc => self.win_x64_msvc = Some(loc),
            Platform::LinuxX64Gnu => self.linux_x64_gnu = Some(loc),
            Platform::LinuxX64Musl => self.linux_x64_musl = Some(loc),
            Platform::LinuxArm6Gnueabi => self.linux_arm6_gnueabi = Some(loc),
            Platform::LinuxArm6GnueabiHf => self.linux_arm6_gnueabi_hf = Some(loc),
            Platform::LinuxArm7GnueabiHf => self.linux_arm7_gnueabi_hf = Some(loc),
            Platform::LinuxArm8Gnu => self.linux_arm8_gnu = Some(loc),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = PlatformLocation> {
        let mut vec = Vec::new();
        if let Some(loc) = &self.win_x64_msvc {
            vec.push(loc.clone())
        }
        if let Some(loc) = &self.linux_x64_gnu {
            vec.push(loc.clone())
        }
        if let Some(loc) = &self.linux_x64_musl {
            vec.push(loc.clone())
        }
        if let Some(loc) = &self.linux_arm6_gnueabi {
            vec.push(loc.clone())
        }
        if let Some(loc) = &self.linux_arm6_gnueabi_hf {
            vec.push(loc.clone())
        }
        if let Some(loc) = &self.linux_arm7_gnueabi {
            vec.push(loc.clone())
        }
        if let Some(loc) = &self.linux_arm7_gnueabi_hf {
            vec.push(loc.clone())
        }
        if let Some(loc) = &self.linux_arm8_gnu {
            vec.push(loc.clone())
        }
        vec.into_iter()
    }

    pub fn is_empty(&self) -> bool {
        self.iter().count() == 0
    }

    pub fn has_dynamic_lib(&self) -> bool {
        self.win_x64_msvc.is_some() || self.linux_x64_gnu.is_some()
    }
}

impl Default for PlatformLocations {
    fn default() -> Self {
        Self::new()
    }
}
