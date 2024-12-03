use core_foundation::{array::CFArray, base::TCFType, string::CFString};
use core_services::LSSharedFileListItemRef;
use std::{convert::TryFrom, ffi::c_void};

use super::{
    macos_api::MacOsApi,
    path::{CFURLWrapper, MacOsPath},
    SidebarItem,
};

/// A high-level API for interacting with the macOS Finder sidebar.
/// Handles Core Foundation memory management internally.
pub struct SidebarApi<T: MacOsApi> {
    raw: T,
}

impl<T: MacOsApi> SidebarApi<T> {
    pub fn new(raw: T) -> Self {
        Self { raw }
    }

    /// Lists all favorite items from the Finder sidebar.
    ///
    /// # Safety
    /// This method handles Core Foundation memory management by:
    /// 1. Using wrap_under_create_rule for objects returned by Copy* functions
    /// 2. Letting Rust's drop semantics handle cleanup through TCFType
    /// 3. Early returns with null checks to prevent invalid memory access
    pub fn list_favorite_items(&self) -> Vec<SidebarItem> {
        println!("SidebarApi::list_favorite_items: start");
        unsafe {
            // Create the favorites list (we own this reference)
            println!("SidebarApi::list_favorite_items: creating favorites list");
            let favorites_list = self.raw.create_favorites_list();
            if favorites_list.is_null() {
                println!(
                    "SidebarApi::list_favorite_items: favorites_list is null, returning empty vec"
                );
                return vec![];
            }

            // Get a snapshot of the items (we own this reference)
            let mut seed = 0;
            println!("SidebarApi::list_favorite_items: getting snapshot");
            let items_ref = self.raw.copy_snapshot(favorites_list, &mut seed);
            println!(
                "SidebarApi::list_favorite_items: got snapshot: {:?}",
                items_ref
            );

            // Wrap the array with create_rule since we own it
            println!("SidebarApi::list_favorite_items: about to wrap array with create_rule");
            let items = CFArray::<*const c_void>::wrap_under_create_rule(items_ref);
            println!(
                "SidebarApi::list_favorite_items: wrapped array, length: {}",
                items.len()
            );

            // Process each item
            println!("SidebarApi::list_favorite_items: starting to process items");
            let mut result = Vec::new();
            for i in 0..items.len() {
                println!("SidebarApi::list_favorite_items: processing item {}", i);
                let item_ref = items.get(i).map(|item| *item as LSSharedFileListItemRef);
                let item_ref = match item_ref {
                    Some(item_ref) => item_ref,
                    None => {
                        println!("SidebarApi::list_favorite_items: failed to get item {}", i);
                        continue;
                    }
                };
                println!(
                    "SidebarApi::list_favorite_items: got item ref: {:?}",
                    item_ref
                );

                // Get item name (we own this reference)
                println!(
                    "SidebarApi::list_favorite_items: getting display name for item {}",
                    i
                );
                let name_ref = self.raw.copy_display_name(item_ref);
                if name_ref.is_null() {
                    println!(
                        "SidebarApi::list_favorite_items: name_ref is null for item {}",
                        i
                    );
                    continue;
                }

                // Wrap with create_rule since we own it from Copy*
                println!("SidebarApi::list_favorite_items: wrapping name_ref with create_rule");
                let name = CFString::wrap_under_create_rule(name_ref);
                println!(
                    "SidebarApi::list_favorite_items: got name for item {}: {}",
                    i, name
                );

                // Get item URL (we own this reference)
                println!(
                    "SidebarApi::list_favorite_items: getting resolved url for item {}",
                    i
                );
                let url_ref = self.raw.copy_resolved_url(item_ref);
                if url_ref.is_null() {
                    println!(
                        "SidebarApi::list_favorite_items: url_ref is null for item {}",
                        i
                    );
                    continue;
                }
                // Wrap with create_rule since we own it from Copy*
                println!("SidebarApi::list_favorite_items: wrapping url_ref with create_rule");
                let url = core_foundation::url::CFURL::wrap_under_create_rule(url_ref);
                println!("SidebarApi::list_favorite_items: got url for item {}", i);

                // Convert URL to MacOsPath using our safe wrapper
                println!(
                    "SidebarApi::list_favorite_items: converting url to path for item {}",
                    i
                );
                if let Ok(path) = MacOsPath::try_from(CFURLWrapper::from(&url)) {
                    println!(
                        "SidebarApi::list_favorite_items: got path for item {}: {}",
                        i, path
                    );
                    // Create a SidebarItem with owned String data
                    println!(
                        "SidebarApi::list_favorite_items: creating SidebarItem for item {}",
                        i
                    );
                    result.push(SidebarItem::new(name.to_string(), path));
                }
            }

            println!("SidebarApi::list_favorite_items: finished processing items");
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
