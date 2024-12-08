use core_foundation::{
    string::CFString,
    url::{CFURL, kCFURLPOSIXPathStyle},
};
use std::convert::TryFrom;
use thiserror::Error;

use crate::finder::sidebar::Target;

#[derive(Debug, Error)]
pub enum UrlError {
    #[error("Failed to convert path to URL")]
    PathToUrl,
    #[error("Invalid URL format")]
    InvalidUrl,
    #[error("Failed to get path from URL")]
    UrlToPath,
}

impl TryFrom<&CFURL> for Target {
    type Error = UrlError;

    fn try_from(url: &CFURL) -> Result<Self, Self::Error> {
        // First check for special URLs
        let cf_string = url.get_string();
        let url_str = cf_string.to_string();
        
        if url_str == "nwnode://domain-AirDrop" {
            return Ok(Target::AirDrop(url_str));
        }

        // Then handle regular paths
        if let Some(path) = url.to_path() {
            return Ok(Target::UserPath(path));
        }

        Err(UrlError::InvalidUrl)
    }
}

impl TryFrom<&Target> for CFURL {
    type Error = UrlError;

    fn try_from(target: &Target) -> Result<Self, Self::Error> {
        match target {
            Target::AirDrop(url_str) => {
                CFURL::from_path(url_str, false).ok_or(UrlError::InvalidUrl)
            }
            Target::UserPath(path) => {
                let path_str = path.to_str().ok_or(UrlError::PathToUrl)?;
                let cf_str = CFString::new(path_str);
                Ok(CFURL::from_file_system_path(
                    cf_str,
                    kCFURLPOSIXPathStyle,
                    true,
                ))
            }
            Target::Documents(path) => {
                let path_str = path.to_str().ok_or(UrlError::PathToUrl)?;
                let cf_str = CFString::new(path_str);
                Ok(CFURL::from_file_system_path(
                    cf_str,
                    kCFURLPOSIXPathStyle,
                    true,
                ))
            }
            Target::Downloads(path) => {
                let path_str = path.to_str().ok_or(UrlError::PathToUrl)?;
                let cf_str = CFString::new(path_str);
                Ok(CFURL::from_file_system_path(
                    cf_str,
                    kCFURLPOSIXPathStyle,
                    true,
                ))
            }
            Target::Applications(path) => {
                let path_str = path.to_str().ok_or(UrlError::PathToUrl)?;
                let cf_str = CFString::new(path_str);
                Ok(CFURL::from_file_system_path(
                    cf_str,
                    kCFURLPOSIXPathStyle,
                    true,
                ))
            }
            Target::Home(path) => {
                let path_str = path.to_str().ok_or(UrlError::PathToUrl)?;
                let cf_str = CFString::new(path_str);
                Ok(CFURL::from_file_system_path(
                    cf_str,
                    kCFURLPOSIXPathStyle,
                    true,
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_convert_airdrop_url_to_target() {
        let path = "nwnode://domain-AirDrop";
        let url = CFURL::from_path(path, false).unwrap();
        let target = Target::try_from(&url).unwrap();
        
        assert!(matches!(target, Target::AirDrop(url_str) if url_str == path));
    }

    #[test]
    fn test_convert_path_url_to_target() {
        let path = "/Users/test/Documents";
        let cf_str = CFString::new(path);
        let url = CFURL::from_file_system_path(cf_str, kCFURLPOSIXPathStyle, true);
        let target = Target::try_from(&url).unwrap();
        
        assert!(matches!(target, Target::UserPath(p) if p == PathBuf::from(path)));
    }

    #[test]
    fn test_convert_target_to_url() {
        let path = "nwnode://domain-AirDrop";
        let target = Target::AirDrop(path.to_string());
        let url = CFURL::try_from(&target).unwrap();
        let url_path = url.to_path().unwrap();
        
        assert_eq!(url_path.to_str().unwrap(), path);
    }

    #[test]
    fn test_convert_path_target_to_url() {
        let path = "/Users/test/Documents";
        let target = Target::UserPath(PathBuf::from(path));
        let url = CFURL::try_from(&target).unwrap();
        let path_from_url = url.to_path().unwrap();
        
        assert_eq!(path_from_url, PathBuf::from(path));
    }
}
