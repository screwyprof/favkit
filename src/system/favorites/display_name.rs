use std::fmt;

use crate::system::core_foundation::CFRef;
use core_foundation::string::{CFString, CFStringRef};

pub(crate) type DisplayName = CFRef<CFString>;

impl From<CFStringRef> for DisplayName {
    fn from(string_ref: CFStringRef) -> Self {
        CFRef::from_ref(string_ref)
    }
}

impl fmt::Display for DisplayName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.as_ref() {
            Some(s) => write!(f, "{}", s),
            None => write!(f, "<none>"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core_foundation::base::TCFType;

    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn should_return_none_for_null_string() -> Result<()> {
        let string_ref: CFStringRef = std::ptr::null_mut();
        assert!(DisplayName::from(string_ref).is_none());
        Ok(())
    }

    #[test]
    fn should_format_display_name_using_display_trait() -> Result<()> {
        let valid = CFString::new("Documents");
        let string_ref = valid.as_concrete_TypeRef();
        let display_name = DisplayName::from(string_ref);
        assert!(display_name.is_some());
        assert_eq!(format!("{}", display_name), "Documents");
        Ok(())
    }
}
