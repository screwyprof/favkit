use core_foundation::{array::CFArray, base::TCFType, string::CFString};
use core_services::LSSharedFileListItemRef;
use std::{convert::TryFrom, ffi::c_void};

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
            // Create the favorites list (we own this reference)
            let favorites_list = self.api.create_favorites_list();
            if favorites_list.is_null() {
                return vec![];
            }

            // Get a snapshot of the items (we own this reference)
            let mut seed = 0;
            let items_ref = self.api.copy_snapshot(favorites_list, &mut seed);

            // Wrap the array with create_rule since we own it
            let items = CFArray::<*const c_void>::wrap_under_create_rule(items_ref);
            let values = items.get_all_values();

            // Process each item
            let mut result = Vec::with_capacity(values.len());
            for &item_ref in values.iter() {
                let item_ref = item_ref as *mut c_void as LSSharedFileListItemRef;

                // Get item name (we own this reference)
                let name_ref = self.api.copy_display_name(item_ref);

                // Get item URL (we own this reference)
                let url_ref = self.api.copy_resolved_url(item_ref);
                if url_ref.is_null() {
                    continue;
                }
                // Wrap with create_rule since we own it from Copy*
                let url = core_foundation::url::CFURL::wrap_under_create_rule(url_ref);

                // Convert URL to MacOsPath using our safe wrapper
                let url_wrapper = CFURLWrapper::from(&url);
                let name = if name_ref.is_null() {
                    None
                } else {
                    Some(CFString::wrap_under_create_rule(name_ref))
                };

                if let Ok(item) = SidebarItem::try_from((url_wrapper, name)) {
                    result.push(item);
                }
            }

            // All Core Foundation objects are dropped here, releasing their memory
            result
        }
    }
}

impl<T: MacOsApi + Default> Default for SidebarApi<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}
