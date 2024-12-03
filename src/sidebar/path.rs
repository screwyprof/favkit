use core_foundation::{
    base::TCFType,
    string::CFString,
    url::{kCFURLPOSIXPathStyle, CFURLGetString, CFURL},
};
use std::{
    convert::TryFrom,
    path::{Path, PathBuf},
};

use crate::error::{Error, Result};

// Newtype wrapper for URL conversion
pub(crate) struct CFURLWrapper<'a>(&'a CFURL);

impl CFURLWrapper<'_> {
    fn get_url_string(&self) -> Option<String> {
        let url_str =
            unsafe { CFString::wrap_under_get_rule(CFURLGetString(self.0.as_concrete_TypeRef())) };
        Some(url_str.to_string())
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
    Custom(PathBuf),
}

impl MacOsLocation {
    fn path(&self) -> PathBuf {
        match self {
            Self::Applications => PathBuf::from("/Applications"),
            Self::UserApplications => dirs::home_dir().unwrap_or_default().join("Applications"),
            Self::Desktop => dirs::desktop_dir().unwrap_or_default(),
            Self::Documents => dirs::document_dir().unwrap_or_default(),
            Self::Downloads => dirs::download_dir().unwrap_or_default(),
            Self::Home => dirs::home_dir().unwrap_or_default(),
            Self::AirDrop => PathBuf::from("nwnode://domain-AirDrop"), // Special case
            Self::Custom(path) => path.clone(),
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
            Self::Custom(path) => path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MacOsPath {
    location: MacOsLocation,
}

impl MacOsPath {
    pub fn location(&self) -> &MacOsLocation {
        &self.location
    }

    pub fn path(&self) -> PathBuf {
        self.location.path()
    }
}

impl<P: AsRef<Path>> From<P> for MacOsPath
where
    P: Into<PathBuf>,
{
    fn from(path: P) -> Self {
        let path_str = path.as_ref().to_str().unwrap_or("");
        let location = match path_str {
            "/Applications" => MacOsLocation::Applications,
            p if p.ends_with("/Downloads") => MacOsLocation::Downloads,
            p if p.ends_with("/Desktop") => MacOsLocation::Desktop,
            p if p.ends_with("/Documents") => MacOsLocation::Documents,
            p if p.ends_with("/Applications") && p != "/Applications" => {
                MacOsLocation::UserApplications
            }
            "nwnode://domain-AirDrop" => MacOsLocation::AirDrop,
            _ => MacOsLocation::Custom(path.into()),
        };
        Self { location }
    }
}

impl From<MacOsLocation> for MacOsPath {
    fn from(location: MacOsLocation) -> Self {
        Self { location }
    }
}

impl TryFrom<CFURLWrapper<'_>> for MacOsPath {
    type Error = Error;

    fn try_from(wrapper: CFURLWrapper) -> Result<Self> {
        // First try to get the filesystem path directly
        if let Some(path) = wrapper.0.to_path() {
            return Ok(Self::from(path));
        }

        // If that fails, try to handle the URL string
        let url_str = wrapper.get_url_string().ok_or(Error::UrlConversion)?;

        // Handle special URLs (like AirDrop)
        if url_str == "nwnode://domain-AirDrop" {
            return Ok(Self::from(MacOsLocation::AirDrop));
        }

        // Handle file:// URLs
        if let Some(path) = url_str.strip_prefix("file://") {
            return Ok(Self::from(path));
        }

        // For any other URLs, store them as-is
        Ok(Self::from(url_str))
    }
}

impl TryFrom<&MacOsPath> for CFURL {
    type Error = Error;

    fn try_from(path: &MacOsPath) -> Result<Self> {
        match path.location() {
            MacOsLocation::AirDrop => {
                // For AirDrop, create a URL directly
                let url_str = CFString::new("nwnode://domain-AirDrop");
                Ok(CFURL::from_file_system_path(
                    url_str,
                    kCFURLPOSIXPathStyle,
                    false,
                ))
            }
            _ => {
                let path = path.path();
                let path_str = CFString::new(&path.to_string_lossy());
                Ok(CFURL::from_file_system_path(
                    path_str,
                    kCFURLPOSIXPathStyle,
                    path.is_dir(),
                ))
            }
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
        let path = MacOsPath::from(MacOsLocation::Applications);
        assert_eq!(path.to_string(), "/Applications");
    }

    #[test]
    fn test_display_airdrop() {
        let path = MacOsPath::from(MacOsLocation::AirDrop);
        assert_eq!(path.to_string(), "nwnode://domain-AirDrop");
    }

    #[test]
    fn test_display_custom() {
        let path = MacOsPath::from("/Users/happygopher/Projects");
        assert_eq!(path.to_string(), "/Users/happygopher/Projects");
    }

    #[test]
    fn test_custom_name() {
        let path = MacOsPath::from("/Users/happygopher/Projects");
        assert_eq!(path.location().name(), "Projects");
    }
}
