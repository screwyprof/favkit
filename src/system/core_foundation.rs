use core_foundation::{
    array::{CFArray, CFArrayRef},
    string::{CFString, CFStringRef},
    url::{CFURL, CFURLRef},
};
use core_services::{LSSharedFileListItemRef, LSSharedFileListRef, TCFType};

// Core Foundation handles
pub(crate) struct CFStringHandle(pub(crate) CFString);
pub(crate) struct CFArrayHandle<T>(pub(crate) CFArray<T>);
pub(crate) struct CFURLHandle(pub(crate) CFURL);

// Raw pointer handles
pub(crate) struct LSSharedFileListHandle(pub(crate) LSSharedFileListRef);
pub(crate) struct LSSharedFileListItemHandle(pub(crate) LSSharedFileListItemRef);

// Core Foundation conversions
impl CFStringHandle {
    pub(crate) fn from_ref(raw: CFStringRef) -> Option<Self> {
        (!raw.is_null()).then(|| unsafe { Self(CFString::wrap_under_get_rule(raw)) })
    }
}

impl<T> CFArrayHandle<T> {
    pub(crate) fn from_ref(raw: CFArrayRef) -> Option<Self> {
        (!raw.is_null()).then(|| unsafe { Self(CFArray::wrap_under_get_rule(raw)) })
    }
}

impl CFURLHandle {
    pub(crate) fn from_ref(raw: CFURLRef) -> Option<Self> {
        (!raw.is_null()).then(|| unsafe { Self(CFURL::wrap_under_get_rule(raw)) })
    }
}

// Raw pointer conversions
impl LSSharedFileListHandle {
    pub(crate) fn from_ref(raw: *mut core_services::OpaqueLSSharedFileListRef) -> Option<Self> {
        (!raw.is_null()).then_some(Self(raw))
    }
}

impl LSSharedFileListItemHandle {
    pub(crate) fn from_ref(raw: *mut core_services::OpaqueLSSharedFileListItemRef) -> Option<Self> {
        (!raw.is_null()).then_some(Self(raw))
    }
}

#[cfg(test)]
mod tests {
    use core_foundation::url::kCFURLPOSIXPathStyle;
    use core_services::{OpaqueLSSharedFileListItemRef, OpaqueLSSharedFileListRef};

    use super::*;

    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn should_return_none_for_null_string() -> Result<()> {
        let ptr: CFStringRef = std::ptr::null_mut();
        assert!(CFStringHandle::from_ref(ptr).is_none());
        Ok(())
    }

    #[test]
    fn should_wrap_valid_string() -> Result<()> {
        let string = CFString::new("test");
        let ptr = string.as_concrete_TypeRef();
        let wrapped = CFStringHandle::from_ref(ptr).ok_or("Failed to create CFStringHandle")?;
        assert_eq!(wrapped.0.to_string(), "test");
        Ok(())
    }

    #[test]
    fn should_return_none_for_null_list() -> Result<()> {
        let ptr = std::ptr::null_mut();
        assert!(LSSharedFileListHandle::from_ref(ptr).is_none());
        Ok(())
    }

    #[test]
    fn should_wrap_valid_list() -> Result<()> {
        let list_ref = 1 as *mut OpaqueLSSharedFileListRef;
        let wrapped = LSSharedFileListHandle::from_ref(list_ref)
            .ok_or("Failed to create LSSharedFileListHandle")?;
        assert_eq!(wrapped.0, list_ref);
        Ok(())
    }

    #[test]
    fn should_return_none_for_null_item() -> Result<()> {
        let ptr = std::ptr::null_mut();
        assert!(LSSharedFileListItemHandle::from_ref(ptr).is_none());
        Ok(())
    }

    #[test]
    fn should_wrap_valid_item() -> Result<()> {
        let item_ref = 1 as *mut OpaqueLSSharedFileListItemRef;
        let wrapped = LSSharedFileListItemHandle::from_ref(item_ref)
            .ok_or("Failed to create LSSharedFileListItemHandle")?;
        assert_eq!(wrapped.0, item_ref);
        Ok(())
    }

    #[test]
    fn should_return_none_for_null_array() -> Result<()> {
        let ptr: CFArrayRef = std::ptr::null();
        assert!(CFArrayHandle::<LSSharedFileListItemRef>::from_ref(ptr).is_none());
        Ok(())
    }

    #[test]
    fn should_wrap_valid_array() -> Result<()> {
        let item: LSSharedFileListItemRef = 1 as LSSharedFileListItemRef;
        let array = CFArray::from_copyable(&[item]);
        let ptr = array.as_concrete_TypeRef();
        let wrapped = CFArrayHandle::<LSSharedFileListItemRef>::from_ref(ptr)
            .ok_or("Failed to create CFArrayHandle")?;
        assert_eq!(wrapped.0.len(), 1);
        Ok(())
    }

    #[test]
    fn should_return_none_for_null_url() -> Result<()> {
        let ptr: CFURLRef = std::ptr::null_mut();
        assert!(CFURLHandle::from_ref(ptr).is_none());
        Ok(())
    }

    #[test]
    fn should_wrap_valid_url() -> Result<()> {
        let path = CFString::new("/test");
        let url = CFURL::from_file_system_path(path, kCFURLPOSIXPathStyle, true);
        let ptr = url.as_concrete_TypeRef();
        let wrapped = CFURLHandle::from_ref(ptr).ok_or("Failed to create CFURLHandle")?;
        assert_eq!(wrapped.0.get_string().to_string(), "file:///test/");
        Ok(())
    }
}
