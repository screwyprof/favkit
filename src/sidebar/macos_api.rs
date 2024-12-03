use core_foundation::{array::CFArrayRef, string::CFStringRef, url::CFURLRef};
use core_services::{
    kLSSharedFileListFavoriteItems, LSSharedFileListCopySnapshot, LSSharedFileListCreate,
    LSSharedFileListItemCopyDisplayName, LSSharedFileListItemCopyResolvedURL,
    LSSharedFileListItemRef, LSSharedFileListRef,
};
use std::ptr;

pub trait MacOsApi {
    /// Creates a new favorites list.
    ///
    /// # Safety
    /// This function is unsafe because it interacts with Core Foundation APIs that require manual memory management.
    /// The caller must ensure that the returned LSSharedFileListRef is properly released when no longer needed.
    unsafe fn create_favorites_list(&self) -> LSSharedFileListRef;

    /// Creates a snapshot of the favorites list.
    ///
    /// # Safety
    /// This function is unsafe because it interacts with Core Foundation APIs that require manual memory management.
    /// The caller must ensure that:
    /// - The list parameter is a valid LSSharedFileListRef
    /// - The returned CFArrayRef is properly released when no longer needed
    unsafe fn copy_snapshot(&self, list: LSSharedFileListRef, seed: &mut u32) -> CFArrayRef;

    /// Gets the display name of a favorites list item.
    ///
    /// # Safety
    /// This function is unsafe because it interacts with Core Foundation APIs that require manual memory management.
    /// The caller must ensure that:
    /// - The item parameter is a valid LSSharedFileListItemRef
    /// - The returned CFStringRef is properly released when no longer needed
    unsafe fn copy_display_name(&self, item: LSSharedFileListItemRef) -> CFStringRef;

    /// Gets the resolved URL of a favorites list item.
    ///
    /// # Safety
    /// This function is unsafe because it interacts with Core Foundation APIs that require manual memory management.
    /// The caller must ensure that:
    /// - The item parameter is a valid LSSharedFileListItemRef
    /// - The returned CFURLRef is properly released when no longer needed
    unsafe fn copy_resolved_url(&self, item: LSSharedFileListItemRef) -> CFURLRef;
}

#[derive(Default)]
pub struct RealMacOsApi;

impl MacOsApi for RealMacOsApi {
    unsafe fn create_favorites_list(&self) -> LSSharedFileListRef {
        LSSharedFileListCreate(ptr::null(), kLSSharedFileListFavoriteItems, ptr::null())
    }

    unsafe fn copy_snapshot(&self, list: LSSharedFileListRef, seed: &mut u32) -> CFArrayRef {
        LSSharedFileListCopySnapshot(list, seed)
    }

    unsafe fn copy_display_name(&self, item: LSSharedFileListItemRef) -> CFStringRef {
        LSSharedFileListItemCopyDisplayName(item)
    }

    unsafe fn copy_resolved_url(&self, item: LSSharedFileListItemRef) -> CFURLRef {
        LSSharedFileListItemCopyResolvedURL(item, 0, ptr::null_mut())
    }
}
