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

impl<P: AsRef<Path>> From<P> for MacOsPath
where
    P: Into<PathBuf>,
{
    fn from(path: P) -> Self {
        let path_str = path.as_ref().to_str().unwrap_or("");
        let home_dir = dirs::home_dir().unwrap_or_default();
        let home_str = home_dir.to_str().unwrap_or("");

        match path_str {
            "/Applications" => Self::Location(MacOsLocation::Applications),
            p if p == format!("{}/Downloads", home_str) => Self::Location(MacOsLocation::Downloads),
            p if p == format!("{}/Desktop", home_str) => Self::Location(MacOsLocation::Desktop),
            p if p == format!("{}/Documents", home_str) => Self::Location(MacOsLocation::Documents),
            p if p == format!("{}/Applications", home_str) => Self::Location(MacOsLocation::UserApplications),
            p if p == home_str => Self::Location(MacOsLocation::Home),
            "/System/Library/CoreServices/Finder.app/Contents/Resources/MyLibraries/myDocuments.cannedSearch" => Self::Location(MacOsLocation::Recents),
            _ => Self::Custom(path.into()),
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

        // Handle special URLs
        let url_str = url_str.as_str();
        if url_str == "nwnode://domain-AirDrop" {
            return Ok(Self::Location(MacOsLocation::AirDrop));
        }

        // Handle file:// URLs
        let path_str = url_str
            .strip_prefix("file://")
            .ok_or(Error::UrlConversion)?;

        // Handle special paths
        let home_dir = dirs::home_dir().unwrap_or_default();
        let path_buf = PathBuf::from(path_str);

        let path = match &path_buf {
            p if p == &PathBuf::from("/Applications") => Self::Location(MacOsLocation::Applications),
            p if p == &home_dir.join("Downloads") => Self::Location(MacOsLocation::Downloads),
            p if p == &home_dir.join("Desktop") => Self::Location(MacOsLocation::Desktop),
            p if p == &home_dir.join("Documents") => Self::Location(MacOsLocation::Documents),
            p if p == &home_dir.join("Applications") => Self::Location(MacOsLocation::UserApplications),
            p if p == &home_dir => Self::Location(MacOsLocation::Home),
            p if p == &PathBuf::from("/System/Library/CoreServices/Finder.app/Contents/Resources/MyLibraries/myDocuments.cannedSearch") => Self::Location(MacOsLocation::Recents),
            _ => Self::Custom(path_buf),
        };

        Ok(path)
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
    fn test_display_applications() {
        let path = MacOsPath::Location(MacOsLocation::Applications);
        assert_eq!(path.to_string(), "/Applications");
    }

    #[test]
    fn test_display_airdrop() {
        let path = MacOsPath::Location(MacOsLocation::AirDrop);
        assert_eq!(path.to_string(), "nwnode://domain-AirDrop");
    }

    #[test]
    fn test_display_custom() {
        let path = MacOsPath::Custom(PathBuf::from("/Users/happygopher/Projects"));
        assert_eq!(path.to_string(), "/Users/happygopher/Projects");
    }

    #[test]
    fn test_custom_name() {
        let path = MacOsPath::Custom(PathBuf::from("/Users/happygopher/Projects"));
        assert_eq!(path.name(), "Projects");
    }
}
