use core_foundation::{
    base::TCFType,
    string::{CFString, CFStringRef},
};

pub(crate) struct RawDisplayName(CFStringRef);

impl From<CFStringRef> for RawDisplayName {
    fn from(string_ref: CFStringRef) -> Self {
        Self(string_ref)
    }
}

pub(crate) struct DisplayName(CFString);

impl From<RawDisplayName> for Option<DisplayName> {
    fn from(raw: RawDisplayName) -> Self {
        if raw.0.is_null() {
            return None;
        }

        // SAFETY: We've checked that the pointer is not null
        let cf_string = unsafe { CFString::wrap_under_get_rule(raw.0) };
        let string = cf_string.to_string();
        (!string.is_empty()).then_some(DisplayName(cf_string))
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

    #[test]
    fn should_return_none_for_null_string() {
        let string_ref = std::ptr::null();
        assert!(Option::<DisplayName>::from(RawDisplayName::from(string_ref)).is_none());
    }

    #[test]
    fn should_return_none_for_empty_string() {
        let empty = CFString::new("");
        let string_ref = empty.as_concrete_TypeRef();
        assert!(Option::<DisplayName>::from(RawDisplayName::from(string_ref)).is_none());
    }

    #[test]
    fn should_convert_valid_string_to_display_name() {
        let valid = CFString::new("Documents");
        let string_ref = valid.as_concrete_TypeRef();
        let display_name = Option::<DisplayName>::from(RawDisplayName::from(string_ref)).unwrap();
        assert_eq!(String::from(display_name), "Documents");
    }
}
