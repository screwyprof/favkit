use core_foundation::{
    array::CFArray,
    string::CFStringRef,
    url::CFURLRef,
    base::TCFType,
};
use core_services::{
    LSSharedFileListCopySnapshot, LSSharedFileListCreate, LSSharedFileListItemCopyDisplayName,
    LSSharedFileListItemCopyResolvedURL, LSSharedFileListRef, kLSSharedFileListFavoriteItems,
    LSSharedFileListItemRef,
};
use std::ptr;

use super::macos::MacOsApi;

/// Implementation of the MacOS API for the system
pub struct SystemMacOsApi;

impl SystemMacOsApi {
    /// Creates a new instance of the system MacOS API
    pub fn new() -> Self {
        Self
    }
}

impl Default for SystemMacOsApi {
    fn default() -> Self {
        Self::new()
    }
}

impl MacOsApi for SystemMacOsApi {
    /// Gets a reference to the system's favorites list
    /// 
    /// # Safety
    /// 
    /// This function is unsafe because it interacts with Core Foundation APIs that require manual memory management.
    /// The caller must ensure that the returned LSSharedFileListRef is properly released when no longer needed.
    unsafe fn get_favorites_list(&self) -> LSSharedFileListRef {
        LSSharedFileListCreate(
            ptr::null_mut(),
            kLSSharedFileListFavoriteItems,
            ptr::null_mut(),
        )
    }

    /// Gets a snapshot of the current state of the favorites list
    /// 
    /// # Safety
    /// 
    /// This function is unsafe because it interacts with Core Foundation APIs that require manual memory management.
    /// The caller must ensure that:
    /// - The list parameter is a valid LSSharedFileListRef
    /// - The returned CFArray is properly released when no longer needed
    unsafe fn get_favorites_snapshot(
        &self,
        list: LSSharedFileListRef,
        seed: &mut u32,
    ) -> CFArray<LSSharedFileListItemRef> {
        let array_ref = LSSharedFileListCopySnapshot(list, seed);
        CFArray::wrap_under_create_rule(array_ref)
    }

    /// Gets the display name of a favorites list item
    /// 
    /// # Safety
    /// 
    /// This function is unsafe because it interacts with Core Foundation APIs that require manual memory management.
    /// The caller must ensure that:
    /// - The item parameter is a valid LSSharedFileListItemRef
    /// - The returned CFStringRef is properly released when no longer needed
    unsafe fn get_item_display_name(&self, item: LSSharedFileListItemRef) -> CFStringRef {
        LSSharedFileListItemCopyDisplayName(item)
    }

    /// Gets the resolved URL of a favorites list item
    /// 
    /// # Safety
    /// 
    /// This function is unsafe because it interacts with Core Foundation APIs that require manual memory management.
    /// The caller must ensure that:
    /// - The item parameter is a valid LSSharedFileListItemRef
    /// - The returned CFURLRef is properly released when no longer needed
    unsafe fn get_item_url(&self, item: LSSharedFileListItemRef) -> CFURLRef {
        LSSharedFileListItemCopyResolvedURL(item, 0, ptr::null_mut())
    }
}
