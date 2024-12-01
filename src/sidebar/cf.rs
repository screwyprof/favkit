use core_foundation::{
    base::TCFType,
    string::{CFString, CFStringRef},
    url::CFURL,
};
use core_services::{
    LSSharedFileListCreate, LSSharedFileListItemCopyDisplayName,
    LSSharedFileListItemCopyResolvedURL, LSSharedFileListItemRef, LSSharedFileListRef,
};
use std::ptr::NonNull;

/// RAII wrapper for LSSharedFileListRef
pub struct SharedFileList {
    inner: NonNull<std::ffi::c_void>,
}

impl SharedFileList {
    /// Creates a new SharedFileList. Returns None if creation fails.
    pub unsafe fn new(list_type: CFStringRef) -> Option<Self> {
        let list = LSSharedFileListCreate(std::ptr::null(), list_type, std::ptr::null());
        NonNull::new(list.cast()).map(|inner| Self { inner })
    }

    pub fn as_raw(&self) -> LSSharedFileListRef {
        self.inner.as_ptr().cast()
    }
}

impl Drop for SharedFileList {
    fn drop(&mut self) {
        unsafe {
            core_foundation::base::CFRelease(self.inner.as_ptr());
        }
    }
}

/// Safe wrapper for Core Foundation operations
pub struct CFItem {
    item_ref: LSSharedFileListItemRef,
}

impl CFItem {
    pub fn new(item_ref: LSSharedFileListItemRef) -> Self {
        Self { item_ref }
    }

    pub fn display_name(&self) -> Option<String> {
        unsafe {
            let name_ref = LSSharedFileListItemCopyDisplayName(self.item_ref);
            if name_ref.is_null() {
                return None;
            }
            let cf_name = CFString::wrap_under_create_rule(name_ref);
            Some(cf_name.to_string())
        }
    }

    pub fn resolved_url(&self) -> Option<CFURL> {
        unsafe {
            let url_ref =
                LSSharedFileListItemCopyResolvedURL(self.item_ref, 0, std::ptr::null_mut());
            if url_ref.is_null() {
                return None;
            }
            Some(CFURL::wrap_under_create_rule(url_ref))
        }
    }
}
