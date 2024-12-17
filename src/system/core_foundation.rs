use core_services::{
    CFString, CFStringRef, LSSharedFileListItemRef, LSSharedFileListRef,
    OpaqueLSSharedFileListItemRef, OpaqueLSSharedFileListRef, TCFType,
};

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

impl From<Raw<*mut OpaqueLSSharedFileListRef>> for Option<Safe<LSSharedFileListRef>> {
    fn from(raw: Raw<*mut OpaqueLSSharedFileListRef>) -> Self {
        (!raw.0.is_null()).then_some(Safe(raw.0))
    }
}

impl From<Raw<*mut OpaqueLSSharedFileListItemRef>> for Option<Safe<LSSharedFileListItemRef>> {
    fn from(raw: Raw<*mut OpaqueLSSharedFileListItemRef>) -> Self {
        (!raw.0.is_null()).then_some(Safe(raw.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn should_return_none_for_null_string() -> Result<()> {
        let ptr: CFStringRef = std::ptr::null_mut();
        let raw = Raw::from(ptr);
        assert!(Option::<Safe<CFString>>::from(raw).is_none());
        Ok(())
    }

    #[test]
    fn should_wrap_valid_string() -> Result<()> {
        let string = CFString::new("test");
        let ptr = string.as_concrete_TypeRef();
        let raw = Raw::from(ptr);
        let wrapped =
            Option::<Safe<CFString>>::from(raw).ok_or("Failed to create Safe<CFString>")?;
        assert_eq!(wrapped.0.to_string(), "test");
        Ok(())
    }

    #[test]
    fn should_return_none_for_null_list() -> Result<()> {
        let ptr = std::ptr::null_mut();
        let raw = Raw::from(ptr);
        assert!(Option::<Safe<LSSharedFileListRef>>::from(raw).is_none());
        Ok(())
    }

    #[test]
    fn should_wrap_valid_list() -> Result<()> {
        let list_ref = 1 as *mut OpaqueLSSharedFileListRef;
        let raw = Raw::from(list_ref);
        let wrapped = Option::<Safe<LSSharedFileListRef>>::from(raw)
            .ok_or("Failed to create Safe<LSSharedFileListRef>")?;
        assert_eq!(wrapped.0, list_ref);
        Ok(())
    }

    #[test]
    fn should_return_none_for_null_item() -> Result<()> {
        let ptr = std::ptr::null_mut();
        let raw = Raw::from(ptr);
        assert!(Option::<Safe<LSSharedFileListItemRef>>::from(raw).is_none());
        Ok(())
    }

    #[test]
    fn should_wrap_valid_item() -> Result<()> {
        let item_ref = 1 as *mut OpaqueLSSharedFileListItemRef;
        let raw = Raw::from(item_ref);
        let wrapped = Option::<Safe<LSSharedFileListItemRef>>::from(raw)
            .ok_or("Failed to create Safe<LSSharedFileListItemRef>")?;
        assert_eq!(wrapped.0, item_ref);
        Ok(())
    }
}
