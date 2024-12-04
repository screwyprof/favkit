use core_foundation::{
    base::TCFType,
    string::CFString,
    url::{CFURLCreateWithString, CFURLGetString, CFURL},
};
use std::{
    convert::TryFrom,
    path::{Path, PathBuf},
};

use crate::error::{Error, Result};

/// A wrapper around CFURL that provides safe conversion to our path types.
/// The lifetime parameter ensures we don't outlive the borrowed CFURL.
///
/// # Safety
/// This wrapper maintains Core Foundation's reference counting rules by:
/// 1. Never taking ownership of the wrapped CFURL (it's borrowed)
/// 2. Using wrap_under_get_rule for borrowed CFString references
/// 3. Creating owned copies of strings before returning them
pub struct CFURLWrapper<'a>(&'a CFURL);

impl CFURLWrapper<'_> {
    /// Gets the URL string from the wrapped CFURL.
    ///
    /// # Safety
    /// This is safe because:
    /// 1. CFURLGetString returns a borrowed reference that we wrap with wrap_under_get_rule
    /// 2. wrap_under_get_rule ensures proper reference counting for the borrowed CFString
    /// 3. to_string() creates an owned copy of the string data
    fn get_url_string(&self) -> Option<String> {
        unsafe {
            let url_str =
                CFString::wrap_under_get_rule(CFURLGetString(self.0.as_concrete_TypeRef()));
            Some(url_str.to_string())
        }
    }
}

impl<'a> From<&'a CFURL> for CFURLWrapper<'a> {
    fn from(url: &'a CFURL) -> Self {
        Self(url)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MacOsLocation {
    Applications,
    UserApplications,
    Desktop,
    Documents,
    Downloads,
    Home,
    AirDrop,
    Recents,
}

impl MacOsLocation {
    pub fn path(&self) -> PathBuf {
        match self {
            Self::Applications => PathBuf::from("/Applications"),
            Self::UserApplications => dirs::home_dir().unwrap_or_default().join("Applications"),
            Self::Desktop => dirs::desktop_dir().unwrap_or_default(),
            Self::Documents => dirs::document_dir().unwrap_or_default(),
            Self::Downloads => dirs::download_dir().unwrap_or_default(),
            Self::Home => dirs::home_dir().unwrap_or_default(),
            Self::AirDrop => PathBuf::from("nwnode://domain-AirDrop"),
            Self::Recents => PathBuf::from("/System/Library/CoreServices/Finder.app/Contents/Resources/MyLibraries/myDocuments.cannedSearch"),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Applications => "Applications",
            Self::UserApplications => "Applications",
            Self::Desktop => "Desktop",
            Self::Documents => "Documents",
            Self::Downloads => "Downloads",
            Self::Home => "Home",
            Self::AirDrop => "AirDrop",
            Self::Recents => "Recents",
        }
    }

    pub fn url(&self) -> String {
        match self {
            Self::AirDrop => "nwnode://domain-AirDrop".to_string(),
            _ => format!("file://{}", self.path().display()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MacOsPath {
    Location(MacOsLocation),
    Custom(PathBuf),
}

impl MacOsPath {
    fn from_path_str(path_str: &str) -> Self {
        let home_dir = dirs::home_dir().unwrap_or_default();
        let home_str = home_dir.to_str().unwrap_or("");

        match path_str {
            "/Applications" => Self::Location(MacOsLocation::Applications),
            p if p == format!("{}/Downloads", home_str) => Self::Location(MacOsLocation::Downloads),
            p if p == format!("{}/Desktop", home_str) => Self::Location(MacOsLocation::Desktop),
            p if p == format!("{}/Documents", home_str) => Self::Location(MacOsLocation::Documents),
            p if p == format!("{}/Applications", home_str) => {
                Self::Location(MacOsLocation::UserApplications)
            }
            p if p == home_str => Self::Location(MacOsLocation::Home),
            "nwnode://domain-AirDrop" => Self::Location(MacOsLocation::AirDrop),
            p if p.contains("myDocuments.cannedSearch") => Self::Location(MacOsLocation::Recents),
            _ => Self::Custom(PathBuf::from(path_str)),
        }
    }

    pub fn path(&self) -> PathBuf {
        match self {
            Self::Location(location) => location.path(),
            Self::Custom(path) => path.clone(),
        }
    }

    pub fn name(&self) -> String {
        match self {
            Self::Location(location) => location.name().to_string(),
            Self::Custom(path) => path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown")
                .to_string(),
        }
    }

    pub fn url(&self) -> String {
        match self {
            Self::Location(location) => location.url(),
            Self::Custom(path) => format!("file://{}", path.display()),
        }
    }
}

impl<P: AsRef<Path>> From<P> for MacOsPath {
    fn from(path: P) -> Self {
        let path = path.as_ref();
        let home_dir = dirs::home_dir().unwrap_or_default();

        match path {
            p if p == Path::new("/Applications") => Self::Location(MacOsLocation::Applications),
            p if p == home_dir.join("Downloads") => Self::Location(MacOsLocation::Downloads),
            p if p == home_dir.join("Desktop") => Self::Location(MacOsLocation::Desktop),
            p if p == home_dir.join("Documents") => Self::Location(MacOsLocation::Documents),
            p if p == home_dir.join("Applications") => {
                Self::Location(MacOsLocation::UserApplications)
            }
            p if p == home_dir => Self::Location(MacOsLocation::Home),
            p if p.to_str() == Some("nwnode://domain-AirDrop") => {
                Self::Location(MacOsLocation::AirDrop)
            }
            p if p
                .to_str()
                .map_or(false, |s| s.contains("myDocuments.cannedSearch")) =>
            {
                Self::Location(MacOsLocation::Recents)
            }
            _ => Self::Custom(path.to_path_buf()),
        }
    }
}

impl From<MacOsLocation> for MacOsPath {
    fn from(location: MacOsLocation) -> Self {
        Self::Location(location)
    }
}

impl TryFrom<CFURLWrapper<'_>> for MacOsPath {
    type Error = Error;

    fn try_from(wrapper: CFURLWrapper) -> Result<Self> {
        let url_str = wrapper.get_url_string().ok_or(Error::UrlConversion)?;

        // Handle file:// URLs
        let path_str = url_str.strip_prefix("file://").unwrap_or(&url_str);

        Ok(Self::from_path_str(path_str))
    }
}

impl TryFrom<&MacOsPath> for CFURL {
    type Error = Error;

    fn try_from(path: &MacOsPath) -> Result<Self> {
        let url_str = path.url();
        let cf_str = CFString::new(&url_str);
        unsafe {
            let url = CFURLCreateWithString(
                core_foundation::base::kCFAllocatorDefault,
                cf_str.as_concrete_TypeRef(),
                std::ptr::null(),
            );
            if url.is_null() {
                return Err(Error::UrlConversion);
            }
            Ok(Self::wrap_under_create_rule(url))
        }
    }
}

impl std::fmt::Display for MacOsPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path().display())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_applications() {
        let path = MacOsPath::from("/Applications");
        assert!(matches!(
            path,
            MacOsPath::Location(MacOsLocation::Applications)
        ));
        assert_eq!(path.name(), "Applications");
        assert_eq!(path.path(), PathBuf::from("/Applications"));
    }

    #[test]
    fn test_user_applications() {
        let home = dirs::home_dir().unwrap();
        let path = MacOsPath::from(home.join("Applications"));
        assert!(matches!(
            path,
            MacOsPath::Location(MacOsLocation::UserApplications)
        ));
        assert_eq!(path.name(), "Applications");
        assert_eq!(path.path(), home.join("Applications"));
    }

    #[test]
    fn test_downloads() {
        let home = dirs::home_dir().unwrap();
        let path = MacOsPath::from(home.join("Downloads"));
        assert!(matches!(
            path,
            MacOsPath::Location(MacOsLocation::Downloads)
        ));
        assert_eq!(path.name(), "Downloads");
        assert_eq!(path.path(), home.join("Downloads"));
    }

    #[test]
    fn test_desktop() {
        let home = dirs::home_dir().unwrap();
        let path = MacOsPath::from(home.join("Desktop"));
        assert!(matches!(path, MacOsPath::Location(MacOsLocation::Desktop)));
        assert_eq!(path.name(), "Desktop");
        assert_eq!(path.path(), home.join("Desktop"));
    }

    #[test]
    fn test_documents() {
        let home = dirs::home_dir().unwrap();
        let path = MacOsPath::from(home.join("Documents"));
        assert!(matches!(
            path,
            MacOsPath::Location(MacOsLocation::Documents)
        ));
        assert_eq!(path.name(), "Documents");
        assert_eq!(path.path(), home.join("Documents"));
    }

    #[test]
    fn test_home() {
        let home = dirs::home_dir().unwrap();
        let path = MacOsPath::from(&home);
        assert!(matches!(path, MacOsPath::Location(MacOsLocation::Home)));
        assert_eq!(path.name(), "Home");
        assert_eq!(path.path(), home);
    }

    #[test]
    fn test_airdrop() {
        let path = MacOsPath::from("nwnode://domain-AirDrop");
        assert!(matches!(path, MacOsPath::Location(MacOsLocation::AirDrop)));
        assert_eq!(path.name(), "AirDrop");
        assert_eq!(path.path(), PathBuf::from("nwnode://domain-AirDrop"));
    }

    #[test]
    fn test_recents() {
        let path = MacOsPath::from("/System/Library/CoreServices/Finder.app/Contents/Resources/MyLibraries/myDocuments.cannedSearch");
        assert!(matches!(path, MacOsPath::Location(MacOsLocation::Recents)));
        assert_eq!(path.name(), "Recents");
        assert_eq!(path.path(), PathBuf::from("/System/Library/CoreServices/Finder.app/Contents/Resources/MyLibraries/myDocuments.cannedSearch"));
    }

    #[test]
    fn test_custom_path() {
        let path = MacOsPath::from("/Users/happygopher/Projects");
        assert!(matches!(path, MacOsPath::Custom(_)));
        assert_eq!(path.name(), "Projects");
        assert_eq!(path.path(), PathBuf::from("/Users/happygopher/Projects"));
    }

    #[test]
    fn test_url_conversion() {
        let url = CFURL::try_from(&MacOsPath::from("/Applications")).unwrap();
        let wrapper = CFURLWrapper::from(&url);
        let path = MacOsPath::try_from(wrapper).unwrap();
        assert!(matches!(
            path,
            MacOsPath::Location(MacOsLocation::Applications)
        ));
        assert_eq!(path.name(), "Applications");
        assert_eq!(path.path(), PathBuf::from("/Applications"));
    }

    #[test]
    fn test_display() {
        let path = MacOsPath::from("/Applications");
        assert_eq!(path.to_string(), "/Applications");

        let path = MacOsPath::from("nwnode://domain-AirDrop");
        assert_eq!(path.to_string(), "nwnode://domain-AirDrop");

        let path = MacOsPath::from("/Users/happygopher/Projects");
        assert_eq!(path.to_string(), "/Users/happygopher/Projects");
    }
}
