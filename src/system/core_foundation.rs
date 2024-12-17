use core_foundation::{
    array::{CFArray, CFArrayRef},
    string::{CFString, CFStringRef},
    url::{CFURL, CFURLRef},
};
use core_services::{
    LSSharedFileListItemRef, LSSharedFileListRef, OpaqueLSSharedFileListItemRef,
    OpaqueLSSharedFileListRef, TCFType,
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
        (!raw.0.is_null()).then(|| unsafe { Safe(CFString::wrap_under_get_rule(raw.0)) })
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

impl<T> From<Raw<CFArrayRef>> for Option<Safe<CFArray<T>>> {
    fn from(raw: Raw<CFArrayRef>) -> Self {
        (!raw.0.is_null()).then(|| unsafe { Safe(CFArray::wrap_under_get_rule(raw.0)) })
    }
}

impl From<Raw<CFURLRef>> for Option<Safe<CFURL>> {
    fn from(raw: Raw<CFURLRef>) -> Self {
        (!raw.0.is_null()).then(|| unsafe { Safe(CFURL::wrap_under_get_rule(raw.0)) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core_foundation::url::kCFURLPOSIXPathStyle;

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

    #[test]
    fn should_return_none_for_null_array() -> Result<()> {
        let ptr: CFArrayRef = std::ptr::null();
        let raw = Raw::from(ptr);
        assert!(Option::<Safe<CFArray<LSSharedFileListItemRef>>>::from(raw).is_none());
        Ok(())
    }

    #[test]
    fn should_wrap_valid_array() -> Result<()> {
        let item: LSSharedFileListItemRef = 1 as LSSharedFileListItemRef;
        let array = CFArray::from_copyable(&[item]);
        let ptr = array.as_concrete_TypeRef();
        let raw = Raw::from(ptr);
        let wrapped = Option::<Safe<CFArray<LSSharedFileListItemRef>>>::from(raw)
            .ok_or("Failed to create Safe<CFArray>")?;
        assert_eq!(wrapped.0.len(), 1);
        Ok(())
    }

    #[test]
    fn should_return_none_for_null_url() -> Result<()> {
        let ptr: CFURLRef = std::ptr::null_mut();
        let raw = Raw::from(ptr);
        assert!(Option::<Safe<CFURL>>::from(raw).is_none());
        Ok(())
    }

    #[test]
    fn should_wrap_valid_url() -> Result<()> {
        let path = CFString::new("/test");
        let url = CFURL::from_file_system_path(path, kCFURLPOSIXPathStyle, true);
        let ptr = url.as_concrete_TypeRef();
        let raw = Raw::from(ptr);
        let wrapped = Option::<Safe<CFURL>>::from(raw).ok_or("Failed to create Safe<CFURL>")?;
        assert_eq!(wrapped.0.get_string().to_string(), "file:///test/");
        Ok(())
    }
}
