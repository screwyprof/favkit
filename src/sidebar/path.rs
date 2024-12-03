use std::ops::Deref;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq)]
pub enum MacOsLocation {
    Applications,
    UserApplications,
    Desktop,
    Documents,
    Downloads,
    Home,
    Custom(PathBuf),
}

impl MacOsLocation {
    pub fn to_path_buf(&self) -> PathBuf {
        match self {
            Self::Applications => PathBuf::from("/Applications"),
            Self::UserApplications => dirs::home_dir().unwrap_or_default().join("Applications"),
            Self::Desktop => dirs::desktop_dir().unwrap_or_default(),
            Self::Documents => dirs::document_dir().unwrap_or_default(),
            Self::Downloads => dirs::download_dir().unwrap_or_default(),
            Self::Home => dirs::home_dir().unwrap_or_default(),
            Self::Custom(path) => path.clone(),
        }
    }
}

impl From<&str> for MacOsLocation {
    fn from(s: &str) -> Self {
        match s {
            "/Applications" => Self::Applications,
            s if s.starts_with("~/Applications") => Self::UserApplications,
            s if s.starts_with("~/Desktop") => Self::Desktop,
            s if s.starts_with("~/Documents") => Self::Documents,
            s if s.starts_with("~/Downloads") => Self::Downloads,
            s if s.starts_with("~/") => Self::Home,
            s => Self::Custom(PathBuf::from(s)),
        }
    }
}

impl From<&Path> for MacOsLocation {
    fn from(path: &Path) -> Self {
        path.to_str()
            .map_or_else(|| Self::Custom(path.to_path_buf()), |s| s.into())
    }
}

#[derive(Debug, Clone)]
pub struct MacOsPath {
    location: MacOsLocation,
    path: PathBuf,
}

impl MacOsPath {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let path = path.as_ref();
        let location: MacOsLocation = path.into();
        let path = location.to_path_buf();
        Self { location, path }
    }

    pub fn from_location(location: MacOsLocation) -> Self {
        let path = location.to_path_buf();
        Self { location, path }
    }

    pub fn location(&self) -> &MacOsLocation {
        &self.location
    }
}

impl<T: AsRef<str>> From<T> for MacOsPath {
    fn from(s: T) -> Self {
        let location: MacOsLocation = s.as_ref().into();
        Self::from_location(location)
    }
}

impl From<MacOsLocation> for MacOsPath {
    fn from(location: MacOsLocation) -> Self {
        Self::from_location(location)
    }
}

impl Deref for MacOsPath {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl AsRef<Path> for MacOsPath {
    fn as_ref(&self) -> &Path {
        &self.path
    }
}

impl<P: AsRef<Path>> PartialEq<P> for MacOsPath {
    fn eq(&self, other: &P) -> bool {
        self.path == other.as_ref()
    }
}

impl std::fmt::Display for MacOsPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path.display())
    }
}
