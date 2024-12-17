use std::fmt;

use crate::system::core_foundation::CFRef;
use core_foundation::string::CFString;

pub(crate) type DisplayName = CFRef<CFString>;

impl fmt::Display for DisplayName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core_foundation::{
        base::TCFType,
        string::{CFString, CFStringRef},
    };

    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn should_return_none_for_null_string() -> Result<()> {
        let string_ref: CFStringRef = std::ptr::null_mut();
        assert!(DisplayName::from_ref(string_ref).is_none());
        Ok(())
    }

    #[test]
    fn should_format_display_name_using_display_trait() -> Result<()> {
        let valid = CFString::new("Documents");
        let string_ref = valid.as_concrete_TypeRef();
        let display_name =
            DisplayName::from_ref(string_ref).ok_or("Failed to create DisplayName")?;
        assert_eq!(format!("{}", display_name), "Documents");
        Ok(())
    }
}
