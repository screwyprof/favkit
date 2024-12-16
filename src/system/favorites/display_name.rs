use core_services::{CFString, CFStringRef};

use crate::system::core_foundation::{Raw, Safe};

pub(crate) type RawDisplayName = Raw<CFStringRef>;
pub(crate) type DisplayName = Safe<CFString>;

impl From<DisplayName> for String {
    fn from(name: DisplayName) -> Self {
        name.0.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core_services::TCFType;

    #[test]
    fn should_return_none_for_null_string() {
        let string_ref: CFStringRef = std::ptr::null_mut();
        let raw = Raw::from(string_ref);
        assert!(Option::<DisplayName>::from(raw).is_none());
    }

    #[test]
    fn should_convert_valid_string_to_display_name() {
        let valid = CFString::new("Documents");
        let string_ref = valid.as_concrete_TypeRef();
        let display_name = Option::<DisplayName>::from(Raw::from(string_ref)).unwrap();
        assert_eq!(String::from(display_name), "Documents");
    }
}
