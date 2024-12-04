use core_foundation::{array::CFArray, base::TCFType, string::CFString, url::CFURL};
use core_services::{LSSharedFileListItemRef, OpaqueLSSharedFileListItemRef};

use super::{
    macos_api::MacOsApi,
    path::{CFURLWrapper, MacOsPath},
    SidebarItem,
};
use crate::error::{Error, Result};

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
    pub fn list_favorite_items(&self) -> Result<Vec<SidebarItem>> {
        unsafe {
            // Get the list of items
            let items = self.get_favorites_array()?;

            // Convert items to SidebarItems
            let items = items
                .get_all_values()
                .iter()
                .filter_map(|&item_ref| {
                    debug_assert!(!item_ref.is_null(), "item_ref should not be null");
                    let item_ref = item_ref as *const OpaqueLSSharedFileListItemRef;
                    let item_ref = item_ref as LSSharedFileListItemRef;
                    self.convert_item_ref(item_ref).ok()
                })
                .collect();

            Ok(items)
        }
    }

    /// Gets the array of favorite items from the macOS API.
    ///
    /// # Safety
    /// Caller must ensure proper memory management of returned CFArray.
    unsafe fn get_favorites_array(&self) -> Result<CFArray<LSSharedFileListItemRef>> {
        let favorites_list = self.api.get_favorites_list();
        if favorites_list.is_null() {
            return Err(Error::GetFavoritesList);
        }

        let mut seed = 0;
        let array = self.api.get_favorites_snapshot(favorites_list, &mut seed);
        Ok(array)
    }

    /// Converts a single item reference to a SidebarItem.
    ///
    /// # Safety
    /// Caller must ensure item_ref is valid and handle memory management of returned objects.
    unsafe fn convert_item_ref(&self, item_ref: LSSharedFileListItemRef) -> Result<SidebarItem> {
        debug_assert!(!item_ref.is_null(), "item_ref should not be null");

        // Get item name
        let name = self.api.get_item_display_name(item_ref);
        let name = if name.is_null() {
            return Err(Error::GetDisplayName);
        } else {
            Some(CFString::wrap_under_create_rule(name))
        };

        // Get item URL
        let url_ref = self.api.get_item_url(item_ref);
        if url_ref.is_null() {
            return Err(Error::GetItemUrl);
        }
        let url = CFURL::wrap_under_create_rule(url_ref);

        // Convert to SidebarItem
        SidebarItem::builder()
            .path(MacOsPath::try_from(CFURLWrapper::from(&url))?)
            .name(name.map(|n| n.to_string()).unwrap_or_else(String::new))
            .build()
    }
}

impl<T: MacOsApi + Default> Default for SidebarApi<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}
