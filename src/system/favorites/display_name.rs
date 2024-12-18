use core_foundation::string::{CFString, CFStringRef};

use crate::{
    finder::{FinderError, Result},
    system::core_foundation::CFRef,
};

pub(crate) type DisplayName = CFRef<CFString>;

impl TryFrom<CFStringRef> for DisplayName {
    type Error = FinderError;

    fn try_from(string_ref: CFStringRef) -> Result<Self> {
        (!string_ref.is_null())
            .then(|| CFRef::try_from_ref(string_ref))
            .ok_or(FinderError::NullDisplayNameHandle)?
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core_foundation::base::TCFType;

    #[test]
    fn should_fail_when_display_name_is_null() {
        // Arrange
        let display_name_ref: CFStringRef = std::ptr::null_mut();

        // Act & Assert
        assert!(DisplayName::try_from(display_name_ref).is_err());
    }

    #[test]
    fn should_wrap_display_name() -> Result<()> {
        // Arrange
        let display_name = CFString::new("Documents");
        let display_name_ref = display_name.as_concrete_TypeRef();

        // Act
        let _display_name = DisplayName::try_from(display_name_ref)?;

        // Assert
        Ok(())
    }

    #[test]
    fn should_convert_to_string() -> Result<()> {
        // Arrange
        let expected_name = "Documents";
        let display_name = CFString::new(expected_name);
        let display_name_ref = display_name.as_concrete_TypeRef();
        let display_name = DisplayName::try_from(display_name_ref)?;

        // Act
        let result = display_name.to_string();

        // Assert
        assert_eq!(result, expected_name);
        Ok(())
    }
}
