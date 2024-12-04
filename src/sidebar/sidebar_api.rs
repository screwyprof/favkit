use core_foundation::{array::CFArray, base::TCFType, string::CFString, url::CFURL};
use core_services::{LSSharedFileListItemRef, OpaqueLSSharedFileListItemRef};
use std::convert::TryFrom;

use super::{macos_api::MacOsApi, path::CFURLWrapper, SidebarItem};

/// A high-level API for interacting with the macOS Finder sidebar.
/// Handles Core Foundation memory management internally.
pub struct SidebarApi<T: MacOsApi> {
    api: T,
}

impl<T: MacOsApi> SidebarApi<T> {
    pub fn new(api: T) -> Self {
        Self { api }
    }

    /// Lists all favorite items from the Finder sidebar.
    ///
    /// # Safety
    /// This method handles Core Foundation memory management by:
    /// 1. Using wrap_under_create_rule for objects returned by Copy* functions
    /// 2. Letting Rust's drop semantics handle cleanup through TCFType
    /// 3. Early returns with null checks to prevent invalid memory access
    pub fn list_favorite_items(&self) -> Vec<SidebarItem> {
        unsafe {
            // Get the list of items
            let items = match self.get_favorites_array() {
                Some(items) => items,
                None => return vec![],
            };

            // Convert items to SidebarItems
            items
                .get_all_values()
                .iter()
                .filter_map(|&item_ref| {
                    let item_ref = item_ref as *const OpaqueLSSharedFileListItemRef;
                    let item_ref = item_ref as LSSharedFileListItemRef;
                    self.convert_item_ref(item_ref)
                })
                .collect()
        }
    }

    /// Gets the array of favorite items from the macOS API.
    ///
    /// # Safety
    /// Caller must ensure proper memory management of returned CFArray.
    unsafe fn get_favorites_array(&self) -> Option<CFArray<LSSharedFileListItemRef>> {
        let favorites_list = self.api.create_favorites_list();
        if favorites_list.is_null() {
            return None;
        }

        let mut seed = 0;
        Some(self.api.copy_snapshot(favorites_list, &mut seed))
    }

    /// Converts a single item reference to a SidebarItem.
    ///
    /// # Safety
    /// Caller must ensure item_ref is valid and handle memory management of returned objects.
    unsafe fn convert_item_ref(&self, item_ref: LSSharedFileListItemRef) -> Option<SidebarItem> {
        // Get item name
        let name = self.api.copy_display_name(item_ref);
        let name = if name.is_null() {
            None
        } else {
            Some(CFString::wrap_under_create_rule(name))
        };

        // Get item URL
        let url_ref = self.api.copy_resolved_url(item_ref);
        if url_ref.is_null() {
            return None;
        }
        let url = CFURL::wrap_under_create_rule(url_ref);

        // Convert to SidebarItem
        SidebarItem::try_from((CFURLWrapper::from(&url), name)).ok()
    }
}

impl<T: MacOsApi + Default> Default for SidebarApi<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}
