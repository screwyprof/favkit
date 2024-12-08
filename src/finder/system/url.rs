use core_foundation::{
    base::{kCFAllocatorDefault, TCFType},
    string::CFString,
    url::{CFURLCreateWithString, CFURL, CFURLRef},
};
use std::path::PathBuf;
use thiserror::Error;

use crate::finder::sidebar::Target;
use super::special_dirs::MacOsPath;

#[derive(Debug, Error)]
pub enum UrlError {
    #[error("URL contains invalid characters: {0}")]
    InvalidFormat(String),

    #[error("Path contains non-UTF8 characters: {0}")]
    NonUtf8Path(PathBuf),

    #[error("Core Foundation returned null when creating URL from '{0}'. The URL may be malformed or use an unsupported scheme")]
    CoreFoundationError(String),
}

/// A safe wrapper around Core Foundation URL operations
#[derive(Debug, Clone)]
pub struct MacOsUrl(CFURL);

impl MacOsUrl {
    /// Gets the path from the URL if it exists
    pub fn to_path(&self) -> Option<PathBuf> {
        self.0.to_path()
    }

    /// Returns true if this URL points to AirDrop
    pub fn is_airdrop(&self) -> bool {
        self.to_string().starts_with("nwnode://")
    }

    /// Creates a MacOsUrl from a raw CFURLRef pointer
    /// 
    /// # Safety
    /// The caller must ensure that url_ref is a valid CFURLRef pointer
    pub unsafe fn from_ref(url_ref: CFURLRef) -> Self {
        Self(CFURL::wrap_under_create_rule(url_ref))
    }
}

impl TryFrom<&str> for MacOsUrl {
    type Error = UrlError;

    fn try_from(url: &str) -> Result<Self, Self::Error> {
        // Check for scheme://
        if !url.contains("://") {
            return Err(UrlError::InvalidFormat("URL must contain ://".into()));
        }

        let cf_str = CFString::new(url);
        unsafe {
            let url_ref = CFURLCreateWithString(
                kCFAllocatorDefault,
                cf_str.as_concrete_TypeRef(),
                std::ptr::null(),
            );
            
            if url_ref.is_null() {
                return Err(UrlError::CoreFoundationError(url.to_string()));
            }
            
            Ok(MacOsUrl::from_ref(url_ref))
        }
    }
}

impl std::fmt::Display for MacOsUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref().get_string())
    }
}

impl AsRef<CFURL> for MacOsUrl {
    fn as_ref(&self) -> &CFURL {
        &self.0
    }
}

impl TryFrom<&CFURL> for Target {
    type Error = UrlError;

    fn try_from(url: &CFURL) -> Result<Self, Self::Error> {
        let mac_url = MacOsUrl(url.clone());
        if mac_url.is_airdrop() {
            return Ok(Target::AirDrop("nwnode://domain-AirDrop".to_string()));
        }

        let path = url.to_path()
            .ok_or(UrlError::InvalidFormat("Failed to convert URL to path".into()))?;

        Target::try_from(MacOsPath::from(path))
            .map_err(UrlError::InvalidFormat)
    }
}

impl TryFrom<&MacOsUrl> for Target {
    type Error = UrlError;

    fn try_from(url: &MacOsUrl) -> Result<Self, Self::Error> {
        Target::try_from(url.as_ref())
    }
}

impl TryFrom<&Target> for MacOsUrl {
    type Error = UrlError;

    fn try_from(target: &Target) -> Result<Self, Self::Error> {
        MacOsUrl::try_from(target.to_string().as_str())
    }
}

impl From<CFURLRef> for MacOsUrl {
    /// # Safety
    /// The caller must ensure that url_ref is a valid CFURLRef pointer
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn from(url_ref: CFURLRef) -> Self {
        // SAFETY: We trust that Core Foundation gives us valid pointers
        unsafe { Self::from_ref(url_ref) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_convert_url_to_target() {
        // Test AirDrop URL
        let url = MacOsUrl::try_from("nwnode://domain-AirDrop").unwrap();
        let target = Target::try_from(&url).unwrap();
        assert!(matches!(target, Target::AirDrop(s) if s == "nwnode://domain-AirDrop"));

        // Test file URL
        let url = MacOsUrl::try_from("file:///Applications").unwrap();
        let target = Target::try_from(&url).unwrap();
        assert!(matches!(target, Target::Applications(p) if p == Path::new("/Applications")));
    }

    #[test]
    fn test_convert_target_to_url() {
        // Test AirDrop target
        let target = Target::AirDrop("nwnode://domain-AirDrop".to_string());
        let url = MacOsUrl::try_from(&target).unwrap();
        assert_eq!(url.to_string(), "nwnode://domain-AirDrop");

        // Test file target
        let target = Target::Applications(PathBuf::from("/Applications"));
        let url = MacOsUrl::try_from(&target).unwrap();
        assert_eq!(url.to_string(), "file:///Applications");
    }
}
