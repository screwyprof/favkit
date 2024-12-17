use std::fmt;

use crate::system::core_foundation::CFRef;
use core_foundation::string::CFString;

pub(crate) type DisplayName = CFRef<CFString>;

impl fmt::Display for DisplayName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<DisplayName> for String {
    fn from(name: DisplayName) -> Self {
        name.0.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core_foundation::string::CFStringRef;
    use core_services::{CFString, TCFType};

    #[test]
    fn should_return_none_for_null_string() {
        let string_ref: CFStringRef = std::ptr::null_mut();
        assert!(DisplayName::from_ref(string_ref).is_none());
    }

    #[test]
    fn should_convert_valid_string_to_display_name() {
        let valid = CFString::new("Documents");
        let string_ref = valid.as_concrete_TypeRef();
        let display_name = DisplayName::from_ref(string_ref).unwrap();
        assert_eq!(display_name.0.to_string(), "Documents");
    }

    #[test]
    fn should_format_display_name_using_display_trait() {
        let valid = CFString::new("Documents");
        let string_ref = valid.as_concrete_TypeRef();
        let display_name = DisplayName::from_ref(string_ref).unwrap();
        assert_eq!(format!("{}", display_name), "Documents");
    }
}
