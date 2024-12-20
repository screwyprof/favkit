use std::fmt;

use core_foundation::{
    base::TCFType,
    url::{CFURL, CFURLRef},
};

use crate::system::{
    core_foundation::CFRef,
    favorites::errors::{FavoritesError, Result},
};

#[derive(Debug, Clone)]
pub struct Url(CFRef<CFURL>);

impl TryFrom<CFURLRef> for Url {
    type Error = FavoritesError;

    fn try_from(url_ref: CFURLRef) -> Result<Self> {
        CFRef::try_from_ref(url_ref)
            .map(Self)
            .map_err(|_| FavoritesError::NullUrlHandle)
    }
}

impl From<&Url> for CFURLRef {
    fn from(url: &Url) -> Self {
        url.0.as_concrete_TypeRef()
    }
}

impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.get_string())
    }
}

#[cfg(test)]
mod tests {
    use core_foundation::{base::TCFType, string::CFString, url::kCFURLPOSIXPathStyle};

    use super::*;

    const EXAMPLE_URL: &str = "file:///Users/user/Documents";

    #[test]
    fn should_fail_when_url_is_null() {
        // Arrange
        let cf_url_ref: CFURLRef = std::ptr::null_mut();

        // Act & Assert
        assert!(Url::try_from(cf_url_ref).is_err());
    }

    #[test]
    fn should_wrap_url() -> Result<()> {
        // Arrange
        let cf_string = CFString::new(EXAMPLE_URL);
        let cf_url = CFURL::from_file_system_path(cf_string, kCFURLPOSIXPathStyle, false);
        let cf_url_ref = cf_url.as_concrete_TypeRef();

        // Act
        let _url = Url::try_from(cf_url_ref)?;

        // Assert
        Ok(())
    }

    #[test]
    fn should_unwrap_url() -> Result<()> {
        // Arrange
        let cf_string = CFString::new(EXAMPLE_URL);
        let cf_url = CFURL::from_file_system_path(cf_string, kCFURLPOSIXPathStyle, false);
        let cf_url_ref = cf_url.as_concrete_TypeRef();
        let url = Url::try_from(cf_url_ref)?;

        // Act
        let result_ref: CFURLRef = (&url).into();

        // Assert
        assert!(!result_ref.is_null());
        Ok(())
    }

    #[test]
    fn should_convert_to_string() -> Result<()> {
        // Arrange
        let cf_string = CFString::new(EXAMPLE_URL);
        let cf_url = CFURL::from_file_system_path(cf_string, kCFURLPOSIXPathStyle, false);
        let cf_url_ref = cf_url.as_concrete_TypeRef();
        let url = Url::try_from(cf_url_ref)?;

        // Act
        let result = url.to_string();

        // Assert
        assert_eq!(result, EXAMPLE_URL);
        Ok(())
    }
}
