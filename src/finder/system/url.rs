use core_foundation::{
    base::{kCFAllocatorDefault, TCFType},
    string::CFString,
    url::{CFURLCreateWithString, CFURL, CFURLRef},
};
use std::path::{Path, PathBuf};
use thiserror::Error;

use crate::finder::sidebar::Target;

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

impl TryFrom<String> for MacOsUrl {
    type Error = UrlError;

    fn try_from(url: String) -> Result<Self, Self::Error> {
        Self::try_from(url.as_str())
    }
}

impl From<&MacOsUrl> for String {
    fn from(url: &MacOsUrl) -> Self {
        url.0.get_string().to_string()
    }
}

impl From<MacOsUrl> for String {
    fn from(url: MacOsUrl) -> Self {
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

        Ok(match path.as_path() {
            p if dirs::document_dir().is_some_and(|d| p == d.as_path()) => 
                Target::Documents(p.to_path_buf()),
            p if dirs::download_dir().is_some_and(|d| p == d.as_path()) => 
                Target::Downloads(p.to_path_buf()),
            p if dirs::home_dir().is_some_and(|d| p == d.as_path()) => 
                Target::Home(p.to_path_buf()),
            p if p == Path::new("/Applications") => 
                Target::Applications(p.to_path_buf()),
            p => Target::UserPath(p.to_path_buf())
        })
    }
}

impl TryFrom<MacOsUrl> for Target {
    type Error = UrlError;

    fn try_from(url: MacOsUrl) -> Result<Self, Self::Error> {
        Target::try_from(url.as_ref())
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
        match target {
            Target::AirDrop(url) => MacOsUrl::try_from(url.as_str()),
            Target::Documents(path) |
            Target::Applications(path) |
            Target::Downloads(path) |
            Target::Home(path) |
            Target::UserPath(path) => {
                if let Some(path_str) = path.to_str() {
                    MacOsUrl::try_from(format!("file://{}", path_str))
                } else {
                    Err(UrlError::NonUtf8Path(path.clone()))
                }
            }
        }
    }
}

impl TryFrom<&Target> for CFURL {
    type Error = UrlError;

    fn try_from(target: &Target) -> Result<Self, Self::Error> {
        let url_str = match target {
            Target::AirDrop(_) => "nwnode://domain-AirDrop".to_string(),
            Target::UserPath(path)
            | Target::Documents(path)
            | Target::Downloads(path)
            | Target::Applications(path)
            | Target::Home(path) => format!("file://{}", path.display()),
        };
        Ok(MacOsUrl::try_from(url_str)?.0)
    }
}

impl From<&Target> for CFURLRef {
    fn from(target: &Target) -> Self {
        match target {
            Target::AirDrop(_) => {
                let url_str = "nwnode://domain-AirDrop";
                let cf_str = CFString::new(url_str);
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
            _ => {
                if let Ok(url) = CFURL::try_from(target) {
                    let ptr = url.as_concrete_TypeRef();
                    std::mem::forget(url);
                    ptr
                } else {
                    std::ptr::null()
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        
        let converted: String = url.clone().into();
        assert_eq!(converted, url_str);
        
        // Test CFURL conversions
        let cf_url = url.as_ref();
        assert_eq!(cf_url.get_string().to_string(), url_str);
        
        let mac_url = MacOsUrl::from(cf_url.clone());
        assert_eq!(String::from(&mac_url), url_str);
    }
}
