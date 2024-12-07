use core_foundation::{
    array::CFArray,
    base::TCFType,
    string::CFStringRef,
    url::{CFURL, CFURLRef},
};
use core_services::{
    kLSSharedFileListFavoriteItems, LSSharedFileListCopySnapshot, LSSharedFileListCreate,
    LSSharedFileListItemCopyDisplayName, LSSharedFileListItemCopyResolvedURL,
    LSSharedFileListItemRef, LSSharedFileListRef,
};
use std::ptr;

use super::macos::MacOsApi;
use super::target::Target;

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

    /// Convert a CFURLRef to a Target
    /// 
    /// # Safety
    /// 
    /// The caller must ensure that:
    /// - The `url` parameter is either null or a valid CFURLRef
    /// - The CFURLRef must remain valid for the duration of this function call
    unsafe fn url_to_target(&self, url: CFURLRef) -> Target {
        if url.is_null() {
            return Target::Home(dirs::home_dir().unwrap_or_default());
        }

        let url = CFURL::wrap_under_get_rule(url);
        
        // Special handling for AirDrop
        let cf_string = url.get_string();
        let url_string = cf_string.to_string();
        if url_string.contains("nwnode://domain-AirDrop") {
            return Target::AirDrop(url_string);
        }

        // Handle regular file paths
        let path = url.to_path().unwrap_or_default();
        let path_str = path.to_string_lossy();
        Target::from_path(path_str.as_ref())
    }
}
