use core_foundation::url::{CFURL, CFURLRef};
use std::fmt;

use crate::system::{
    core_foundation::CFRef,
    favorites::errors::{FavoritesError, Result},
};

#[derive(Debug)]
pub(crate) struct Url(CFRef<CFURL>);

impl TryFrom<CFURLRef> for Url {
    type Error = FavoritesError;

    fn try_from(url_ref: CFURLRef) -> Result<Self> {
        CFRef::try_from_ref(url_ref)
            .map(Self)
            .map_err(|_| FavoritesError::NullUrlHandle)
    }
}

impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.get_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core_foundation::{base::TCFType, string::CFString, url::kCFURLPOSIXPathStyle};

    #[test]
    fn should_fail_when_resolved_url_is_null() {
        // Arrange
        let resolved_url_ref: CFURLRef = std::ptr::null_mut();

        // Act & Assert
        assert!(Url::try_from(resolved_url_ref).is_err());
    }

    #[test]
    fn should_wrap_resolved_url() -> Result<()> {
        // Arrange
        let path = CFString::new("/Users/user/Documents");
        let resolved_url = CFURL::from_file_system_path(path, kCFURLPOSIXPathStyle, true);
        let resolved_url_ref = resolved_url.as_concrete_TypeRef();

        // Act
        let _url = Url::try_from(resolved_url_ref)?;

        // Assert
        Ok(())
    }

    #[test]
    fn should_convert_to_string() -> Result<()> {
        // Arrange
        let expected_url = "file:///Users/user/Documents/";
        let path = CFString::new("/Users/user/Documents");
        let resolved_url = CFURL::from_file_system_path(path, kCFURLPOSIXPathStyle, true);
        let resolved_url_ref = resolved_url.as_concrete_TypeRef();
        let url = Url::try_from(resolved_url_ref)?;

        // Act
        let result = url.to_string();

        // Assert
        assert_eq!(result, expected_url);
        Ok(())
    }
}
