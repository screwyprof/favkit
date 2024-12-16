use core_services::{CFString, CFStringRef, TCFType};

pub(crate) struct Raw<T>(pub(crate) T);

impl<T> From<T> for Raw<T> {
    fn from(ptr: T) -> Self {
        Self(ptr)
    }
}

pub(crate) struct Safe<T>(pub(crate) T);

impl From<Raw<CFStringRef>> for Option<Safe<CFString>> {
    fn from(raw: Raw<CFStringRef>) -> Self {
        if raw.0.is_null() {
            None
        } else {
            // SAFETY: We've checked that the pointer is not null
            Some(Safe(unsafe { CFString::wrap_under_get_rule(raw.0) }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_none_for_null_pointer() {
        let ptr: CFStringRef = std::ptr::null_mut();
        let raw = Raw::from(ptr);
        assert!(Option::<Safe<CFString>>::from(raw).is_none());
    }

    #[test]
    fn should_wrap_valid_pointer() {
        let string = CFString::new("test");
        let ptr = string.as_concrete_TypeRef();
        let raw = Raw::from(ptr);
        assert!(Option::<Safe<CFString>>::from(raw).is_some());
    }
}
