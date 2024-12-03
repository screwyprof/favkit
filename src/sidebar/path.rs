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
    pub fn into_path_buf(self) -> PathBuf {
        match self {
            Self::Applications => "/Applications".into(),
            Self::UserApplications => dirs::home_dir().unwrap_or_default().join("Applications"),
            Self::Desktop => dirs::desktop_dir().unwrap_or_default(),
            Self::Documents => dirs::document_dir().unwrap_or_default(),
            Self::Downloads => dirs::download_dir().unwrap_or_default(),
            Self::Home => dirs::home_dir().unwrap_or_default(),
            Self::Custom(path) => path,
        }
    }
}

impl<P: AsRef<Path>> From<P> for MacOsLocation {
    fn from(path: P) -> Self {
        match path.as_ref().to_str() {
            Some("/Applications") => Self::Applications,
            Some(p) if p.starts_with("~/Applications") => Self::UserApplications,
            Some(p) if p.starts_with("~/Desktop") => Self::Desktop,
            Some(p) if p.starts_with("~/Documents") => Self::Documents,
            Some(p) if p.starts_with("~/Downloads") => Self::Downloads,
            Some("~/") => Self::Home,
            _ => Self::Custom(path.as_ref().to_path_buf()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MacOsPath {
    location: MacOsLocation,
    path: PathBuf,
}

impl MacOsPath {
    pub fn location(&self) -> &MacOsLocation {
        &self.location
    }
}

impl<P: AsRef<Path>> From<P> for MacOsPath
where
    P: Into<PathBuf>,
{
    fn from(path: P) -> Self {
        let path_buf: PathBuf = path.into();
        let location: MacOsLocation = path_buf.as_path().into();
        let path = location.clone().into_path_buf();
        Self { location, path }
    }
}

impl From<MacOsLocation> for MacOsPath {
    fn from(location: MacOsLocation) -> Self {
        let path = location.clone().into_path_buf();
        Self { location, path }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_well_known_locations_from_str() {
        assert_eq!(
            MacOsLocation::from("/Applications"),
            MacOsLocation::Applications
        );
        assert_eq!(
            MacOsLocation::from("~/Applications"),
            MacOsLocation::UserApplications
        );
        assert_eq!(MacOsLocation::from("~/Desktop"), MacOsLocation::Desktop);
        assert_eq!(MacOsLocation::from("~/Documents"), MacOsLocation::Documents);
        assert_eq!(MacOsLocation::from("~/Downloads"), MacOsLocation::Downloads);
        assert_eq!(MacOsLocation::from("~/"), MacOsLocation::Home);
    }

    #[test]
    fn test_custom_location_from_str() {
        let expected_path = PathBuf::from("/Users/test/Custom");
        let location = MacOsLocation::from("/Users/test/Custom");
        assert_eq!(location, MacOsLocation::Custom(expected_path));
    }

    #[test]
    fn test_location_into_path_buf() {
        assert_eq!(
            MacOsLocation::Applications.into_path_buf(),
            PathBuf::from("/Applications")
        );
        assert!(MacOsLocation::Downloads
            .into_path_buf()
            .ends_with("Downloads"));

        let custom_path = PathBuf::from("/custom/path");
        assert_eq!(
            MacOsLocation::Custom(custom_path.clone()).into_path_buf(),
            custom_path
        );
    }

    #[test]
    fn test_path_from_location() {
        let path = MacOsPath::from(MacOsLocation::Applications);
        assert_eq!(path.location(), &MacOsLocation::Applications);
        assert_eq!(path.as_ref(), Path::new("/Applications"));

        let custom_path = PathBuf::from("/custom");
        let path = MacOsPath::from(MacOsLocation::Custom(custom_path.clone()));
        assert_eq!(path.location(), &MacOsLocation::Custom(custom_path.clone()));
        assert_eq!(path.as_ref(), custom_path);
    }

    #[test]
    fn test_path_from_str() {
        let applications: MacOsPath = "/Applications".into();
        let downloads: MacOsPath = "~/Downloads".into();
        let custom: MacOsPath = "/custom/path".into();

        assert_eq!(applications.location(), &MacOsLocation::Applications);
        assert_eq!(downloads.location(), &MacOsLocation::Downloads);
        assert_eq!(
            custom.location(),
            &MacOsLocation::Custom(PathBuf::from("/custom/path"))
        );
    }

    #[test]
    fn test_path_equality() {
        let applications = MacOsPath::from(MacOsLocation::Applications);
        let same_applications = MacOsPath::from(MacOsLocation::Applications);
        let downloads = MacOsPath::from(MacOsLocation::Downloads);

        assert_eq!(applications, same_applications);
        assert_ne!(applications, downloads);
        assert_eq!(applications, Path::new("/Applications"));
    }

    #[test]
    fn test_path_display() {
        let applications = MacOsPath::from(MacOsLocation::Applications);
        assert_eq!(applications.to_string(), "/Applications");

        let custom = MacOsPath::from(MacOsLocation::Custom(PathBuf::from("/custom/path")));
        assert_eq!(custom.to_string(), "/custom/path");
    }
}
