use core_foundation::string::{CFString, CFStringRef};
use core_services::TCFType;
use std::fmt;

use crate::system::{
    core_foundation::CFRef,
    favorites::errors::{FavoritesError, Result},
};

#[derive(Debug)]
pub struct DisplayName(CFRef<CFString>);

impl TryFrom<CFStringRef> for DisplayName {
    type Error = FavoritesError;

    fn try_from(string_ref: CFStringRef) -> Result<Self> {
        CFRef::try_from_ref(string_ref)
            .map(Self)
            .map_err(|_| FavoritesError::NullDisplayNameHandle)
    }
}

impl From<&DisplayName> for CFStringRef {
    fn from(display_name: &DisplayName) -> Self {
        display_name.0.as_concrete_TypeRef()
    }
}

impl fmt::Display for DisplayName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core_foundation::base::TCFType;

    const EXAMPLE_DISPLAY_NAME: &str = "Documents";

    #[test]
    fn should_fail_when_display_name_is_null() {
        // Arrange
        let cf_string_ref: CFStringRef = std::ptr::null_mut();

        // Act & Assert
        assert!(DisplayName::try_from(cf_string_ref).is_err());
    }

    #[test]
    fn should_wrap_display_name() -> Result<()> {
        // Arrange
        let cf_string = CFString::new(EXAMPLE_DISPLAY_NAME);
        let cf_string_ref = cf_string.as_concrete_TypeRef();

        // Act
        let _display_name = DisplayName::try_from(cf_string_ref)?;

        // Assert
        Ok(())
    }

    #[test]
    fn should_unwrap_display_name() -> Result<()> {
        // Arrange
        let cf_string = CFString::new(EXAMPLE_DISPLAY_NAME);
        let cf_string_ref = cf_string.as_concrete_TypeRef();
        let display_name = DisplayName::try_from(cf_string_ref)?;

        // Act
        let result_ref: CFStringRef = (&display_name).into();

        // Assert
        assert!(!result_ref.is_null());
        Ok(())
    }

    #[test]
    fn should_convert_to_string() -> Result<()> {
        // Arrange
        let cf_string = CFString::new(EXAMPLE_DISPLAY_NAME);
        let cf_string_ref = cf_string.as_concrete_TypeRef();
        let display_name = DisplayName::try_from(cf_string_ref)?;

        // Act
        let result = display_name.to_string();

        // Assert
        assert_eq!(result, EXAMPLE_DISPLAY_NAME);
        Ok(())
    }
}
