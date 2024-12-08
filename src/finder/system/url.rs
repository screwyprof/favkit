use core_foundation::{
    base::{kCFAllocatorDefault, TCFType},
    string::CFString,
    url::{CFURLCreateWithString, CFURL},
};
use std::path::Path;
use thiserror::Error;

use crate::finder::sidebar::Target;

#[derive(Debug, Error)]
pub enum UrlError {
    #[error("Invalid URL format")]
    InvalidUrl,
    #[error("Failed to convert path to URL")]
    PathToUrl,
}

impl TryFrom<&CFURL> for Target {
    type Error = UrlError;

    fn try_from(url: &CFURL) -> Result<Self, Self::Error> {
        let url_str = url.get_string().to_string();
        
        if url_str.starts_with("nwnode://") {
            return Ok(Target::AirDrop("nwnode://domain-AirDrop".to_string()));
        }

        let path = url
            .to_path()
            .ok_or(UrlError::InvalidUrl)?;

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

impl TryFrom<&Target> for CFURL {
    type Error = UrlError;

    fn try_from(target: &Target) -> Result<Self, Self::Error> {
        let url_str = match target {
            Target::AirDrop(url) => url.clone(),
            Target::UserPath(path)
            | Target::Documents(path)
            | Target::Downloads(path)
            | Target::Applications(path)
            | Target::Home(path) => format!("file://{}", path.display()),
        };
        let cf_str = CFString::new(&url_str);
        
        unsafe {
            let url_ref = CFURLCreateWithString(
                kCFAllocatorDefault,
                cf_str.as_concrete_TypeRef(),
                std::ptr::null(),
            );
            
            if url_ref.is_null() {
                return Err(UrlError::PathToUrl);
            }
            
            Ok(CFURL::wrap_under_create_rule(url_ref))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_convert_url_to_target() {
        // Test AirDrop URL
        let cf_str = CFString::new("nwnode://domain-AirDrop");
        let url = unsafe {
            let url_ref = CFURLCreateWithString(
                kCFAllocatorDefault,
                cf_str.as_concrete_TypeRef(),
                std::ptr::null(),
            );
            CFURL::wrap_under_create_rule(url_ref)
        };
        let target = Target::try_from(&url).unwrap();
        assert!(matches!(target, Target::AirDrop(s) if s == "nwnode://domain-AirDrop"));

        // Test file URL
        let cf_str = CFString::new("file:///Applications");
        let url = unsafe {
            let url_ref = CFURLCreateWithString(
                kCFAllocatorDefault,
                cf_str.as_concrete_TypeRef(),
                std::ptr::null(),
            );
            CFURL::wrap_under_create_rule(url_ref)
        };
        let target = Target::try_from(&url).unwrap();
        assert!(matches!(target, Target::Applications(p) if p == PathBuf::from("/Applications")));
    }

    #[test]
    fn test_convert_target_to_url() {
        // Test AirDrop target
        let target = Target::AirDrop("nwnode://domain-AirDrop".to_string());
        let url = CFURL::try_from(&target).unwrap();
        assert_eq!(url.get_string().to_string(), "nwnode://domain-AirDrop");

        // Test file target
        let target = Target::Applications(PathBuf::from("/Applications"));
        let url = CFURL::try_from(&target).unwrap();
        assert_eq!(url.get_string().to_string(), "file:///Applications");
    }
}
