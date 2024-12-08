use core_foundation::{array::CFArray, base::TCFType, string::CFStringRef, url::CFURLRef};
use core_services::{
    kLSSharedFileListFavoriteItems, LSSharedFileListCopySnapshot, LSSharedFileListCreate,
    LSSharedFileListItemCopyDisplayName, LSSharedFileListItemCopyResolvedURL,
    LSSharedFileListItemRef, LSSharedFileListRef,
};
use std::ptr;

pub trait MacOsApi {
    /// Creates a reference to the system's favorites list.
    ///
    /// # Safety
    /// This function is unsafe because it interacts with Core Foundation APIs that require manual memory management.
    /// The caller must ensure that the returned LSSharedFileListRef is properly released when no longer needed.
    unsafe fn get_favorites_list(&self) -> LSSharedFileListRef;

    /// Gets a snapshot of the current state of the favorites list.
    ///
    /// # Safety
    /// This function is unsafe because it interacts with Core Foundation APIs that require manual memory management.
    /// The caller must ensure that:
    /// - The list parameter is a valid LSSharedFileListRef
    /// - The returned CFArray is properly released when no longer needed
    unsafe fn get_favorites_snapshot(
        &self,
        list: LSSharedFileListRef,
        seed: &mut u32,
    ) -> CFArray<LSSharedFileListItemRef>;

    /// Gets the display name of a favorites list item.
    ///
    /// # Safety
    /// This function is unsafe because it interacts with Core Foundation APIs that require manual memory management.
    /// The caller must ensure that:
    /// - The item parameter is a valid LSSharedFileListItemRef
    /// - The returned CFStringRef is properly released when no longer needed
    unsafe fn get_item_display_name(&self, item: LSSharedFileListItemRef) -> CFStringRef;

    /// Gets the resolved URL of a favorites list item.
    ///
    /// # Safety
    /// This function is unsafe because it interacts with Core Foundation APIs that require manual memory management.
    /// The caller must ensure that:
    /// - The item parameter is a valid LSSharedFileListItemRef
    /// - The returned CFURLRef is properly released when no longer needed
    unsafe fn get_item_url(&self, item: LSSharedFileListItemRef) -> CFURLRef;
}

pub struct RealMacOsApi;

impl RealMacOsApi {
    pub fn new() -> Self {
        Self
    }
}

impl Default for RealMacOsApi {
    fn default() -> Self {
        Self::new()
    }
}

impl MacOsApi for RealMacOsApi {
    unsafe fn get_favorites_list(&self) -> LSSharedFileListRef {
        LSSharedFileListCreate(ptr::null(), kLSSharedFileListFavoriteItems, ptr::null())
    }

    unsafe fn get_favorites_snapshot(
        &self,
        list: LSSharedFileListRef,
        seed: &mut u32,
    ) -> CFArray<LSSharedFileListItemRef> {
        let array_ref = LSSharedFileListCopySnapshot(list, seed);
        CFArray::wrap_under_create_rule(array_ref)
    }

    unsafe fn get_item_display_name(&self, item: LSSharedFileListItemRef) -> CFStringRef {
        LSSharedFileListItemCopyDisplayName(item)
    }

    unsafe fn get_item_url(&self, item: LSSharedFileListItemRef) -> CFURLRef {
        LSSharedFileListItemCopyResolvedURL(item, 0, ptr::null_mut())
    }
}
