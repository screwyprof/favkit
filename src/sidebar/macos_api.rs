use core_foundation::{
    array::{CFArray, CFArrayRef},
    base::TCFType,
    string::{CFString, CFStringRef},
    url::{CFURLGetString, CFURLRef, CFURL},
};
use core_services::{
    kLSSharedFileListFavoriteItems, LSSharedFileListCopySnapshot, LSSharedFileListCreate,
    LSSharedFileListItemCopyDisplayName, LSSharedFileListItemCopyResolvedURL,
    LSSharedFileListItemRef, LSSharedFileListRef,
};
use std::ptr;

pub trait RawMacOsApi {
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

impl RawMacOsApi for RealMacOsApi {
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

pub struct MacOsApi<T: RawMacOsApi> {
    raw: T,
}

impl<T: RawMacOsApi> MacOsApi<T> {
    pub fn new(raw: T) -> Self {
        Self { raw }
    }

    pub fn list_favorite_items(&self) -> Vec<(String, String)> {
        unsafe {
            let favorites_list = self.raw.create_favorites_list();
            if favorites_list.is_null() {
                return vec![];
            }

            let mut seed = 0;
            let items_ref = self.raw.copy_snapshot(favorites_list, &mut seed);
            let items = CFArray::<*const std::ffi::c_void>::wrap_under_create_rule(items_ref);

            items
                .iter()
                .filter_map(|item_ref| {
                    let item_ref = *item_ref as LSSharedFileListItemRef;

                    // Get item name
                    let name_ref = self.raw.copy_display_name(item_ref);
                    if name_ref.is_null() {
                        return None;
                    }
                    let name = CFString::wrap_under_create_rule(name_ref);

                    // Get item URL
                    let url_ref = self.raw.copy_resolved_url(item_ref);
                    if url_ref.is_null() {
                        return None;
                    }
                    let url = CFURL::wrap_under_create_rule(url_ref);
                    let path =
                        CFString::wrap_under_get_rule(CFURLGetString(url.as_concrete_TypeRef()));

                    Some((name.to_string(), path.to_string()))
                })
                .collect()
        }
    }
}

impl Default for MacOsApi<RealMacOsApi> {
    fn default() -> Self {
        Self::new(RealMacOsApi)
    }
}
