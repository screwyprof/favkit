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

    /// Creates a MacOsUrl from a raw CFURLRef pointer, returning None if the pointer is null
    /// 
    /// # Safety
    /// The caller must ensure that url_ref is either null or a valid CFURLRef pointer
    pub unsafe fn from_nullable_ref(url_ref: CFURLRef) -> Option<Self> {
        if url_ref.is_null() {
            None
        } else {
            Some(Self(CFURL::wrap_under_create_rule(url_ref)))
        }
    }

    /// Creates a MacOsUrl from a raw CFURLRef pointer
    /// 
    /// # Safety
    /// The caller must ensure that url_ref is a valid CFURLRef pointer
    pub unsafe fn from_ref(url_ref: CFURLRef) -> Self {
        Self(CFURL::wrap_under_create_rule(url_ref))
    }

    /// Creates a MacOsUrl from a raw CFURLRef pointer.
    /// Returns None if the pointer is null.
    /// 
    /// # Safety
    /// The caller must ensure that url_ref is either null or a valid CFURLRef pointer
    pub unsafe fn from_url_ref(url_ref: CFURLRef) -> Option<Self> {
        if url_ref.is_null() {
            None
        } else {
            Some(Self::from_ref(url_ref))
        }
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

impl From<&MacOsUrl> for String {
    fn from(url: &MacOsUrl) -> Self {
        url.0.get_string().to_string()
    }
}

impl AsRef<CFURL> for MacOsUrl {
    fn as_ref(&self) -> &CFURL {
        &self.0
    }
}

impl From<CFURL> for MacOsUrl {
    fn from(url: CFURL) -> Self {
        MacOsUrl(url)
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

impl TryFrom<&CFURL> for Target {
    type Error = UrlError;

    fn try_from(url: &CFURL) -> Result<Self, Self::Error> {
        let mac_url = MacOsUrl::from(url.clone());
        let url_str: String = mac_url.as_ref().get_string().to_string();
        
        if url_str.starts_with("nwnode://") {
            return Ok(Target::AirDrop("nwnode://domain-AirDrop".to_string()));
        }

        let path = mac_url
            .to_path()
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
        MacOsUrl::try_from(String::from(target).as_str())
    }
}

impl TryFrom<&Target> for CFURL {
    type Error = UrlError;

    fn try_from(target: &Target) -> Result<Self, Self::Error> {
        Ok(MacOsUrl::try_from(target)?.0)
    }
}

impl From<&Target> for CFURLRef {
    fn from(target: &Target) -> Self {
        let cf_str = CFString::new(&String::from(target));
        let url_ref = unsafe {
            CFURLCreateWithString(
                kCFAllocatorDefault,
                cf_str.as_concrete_TypeRef(),
                std::ptr::null(),
            )
        };
        std::mem::forget(cf_str);
        url_ref
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
        let target = Target::try_from(url.as_ref()).unwrap();
        assert!(matches!(target, Target::AirDrop(s) if s == "nwnode://domain-AirDrop"));

        // Test file URL
        let url = MacOsUrl::try_from("file:///Applications").unwrap();
        let target = Target::try_from(url.as_ref()).unwrap();
        assert!(matches!(target, Target::Applications(p) if p == Path::new("/Applications")));
    }

    #[test]
    fn test_convert_target_to_url() {
        // Test AirDrop target
        let target = Target::AirDrop("any string".to_string()); // The input string doesn't matter
        let url = MacOsUrl::from(CFURL::try_from(&target).unwrap());
        let url_str: String = (&url).into();
        assert_eq!(url_str, "nwnode://domain-AirDrop");

        // Test file target
        let target = Target::Applications(Path::new("/Applications").to_path_buf());
        let url = MacOsUrl::from(CFURL::try_from(&target).unwrap());
        let url_str: String = (&url).into();
        assert_eq!(url_str, "file:///Applications");
    }

    #[test]
    fn test_macos_url_creation() {
        // Valid URLs
        assert!(MacOsUrl::try_from("file:///Applications").is_ok());
        assert!(MacOsUrl::try_from("nwnode://domain-AirDrop").is_ok());
        assert!(MacOsUrl::try_from("http://example.com").is_ok());
        assert!(MacOsUrl::try_from("https://example.com").is_ok());
        assert!(MacOsUrl::try_from("custom://anything").is_ok());
        
        // Invalid URLs
        assert!(MacOsUrl::try_from("not a url").is_err());
    }

    #[test]
    fn test_url_conversions() {
        let url_str = "file:///Applications";
        let url = MacOsUrl::try_from(url_str).unwrap();
        
        // Test String conversions
        let converted: String = (&url).into();
        assert_eq!(converted, url_str);
        
        let converted: String = (&url.clone()).into();
        assert_eq!(converted, url_str);
        
        // Test CFURL conversions
        let cf_url = url.as_ref();
        assert_eq!(cf_url.get_string().to_string(), url_str);
        
        let mac_url = MacOsUrl::from(cf_url.clone());
        assert_eq!(String::from(&mac_url), url_str);
    }
}
