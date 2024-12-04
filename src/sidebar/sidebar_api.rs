use core_foundation::{array::CFArray, base::TCFType, string::CFString, url::CFURL};
use core_services::{LSSharedFileListItemRef, OpaqueLSSharedFileListItemRef};

use super::{
    macos_api::MacOsApi,
    path::{CFURLWrapper, MacOsPath},
    SidebarItem,
};
use crate::error::{Error, Result};

/// A high-level API for interacting with the macOS Finder sidebar.
///
/// This API provides a safe interface to the low-level Core Foundation operations
/// required to interact with the Finder sidebar. It handles all memory management
/// internally through RAII patterns.
///
/// # Example
/// ```no_run
/// use favkit::{Sidebar, Result};
///
/// fn main() -> Result<()> {
///     let sidebar = Sidebar::new();
///     let items = sidebar.list_items()?;
///     
///     for item in items {
///         println!("{}: {}", item.name(), item.path());
///     }
///     Ok(())
/// }
/// ```
pub struct SidebarApi<T: MacOsApi> {
    api: T,
}

impl<T: MacOsApi> SidebarApi<T> {
    /// Creates a new instance of the sidebar API with the given implementation.
    pub fn new(api: T) -> Self {
        Self { api }
    }

    /// Lists all favorite items from the Finder sidebar.
    ///
    /// This method safely handles all Core Foundation memory management internally:
    /// - Objects returned by Core Foundation's Copy* functions are wrapped using `wrap_under_create_rule`
    /// - Memory is automatically freed when objects go out of scope through Rust's drop semantics
    /// - Null pointers and invalid references are caught early and returned as errors
    ///
    /// # Returns
    /// - A vector of `SidebarItem`s representing the current favorites
    /// - An error if any Core Foundation operation fails
    ///
    /// # Example
    /// ```no_run
    /// # use favkit::{SidebarApi, RealMacOsApi};
    /// let api = SidebarApi::new(RealMacOsApi::default());
    /// match api.list_favorite_items() {
    ///     Ok(items) => {
    ///         for item in items {
    ///             println!("Found favorite: {}", item);
    ///         }
    ///     }
    ///     Err(e) => eprintln!("Failed to list favorites: {}", e),
    /// }
    /// ```
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
    /// The array is returned wrapped in `CFArray` which implements `Drop`,
    /// so it will be automatically freed when it goes out of scope.
    unsafe fn get_favorites_array(&self) -> Result<CFArray<LSSharedFileListItemRef>> {
        let favorites_list = self.api.get_favorites_list();
        if favorites_list.is_null() {
            return Err(Error::GetFavoritesList {
                reason: "system returned null favorites list",
            });
        }

        let mut seed = 0;
        let array = self.api.get_favorites_snapshot(favorites_list, &mut seed);
        Ok(array)
    }

    /// Converts a single item reference to a SidebarItem.
    ///
    /// # Safety
    /// Caller must ensure:
    /// - item_ref is a valid LSSharedFileListItemRef
    /// - Memory management of returned objects is handled properly
    ///
    /// # Returns
    /// - A SidebarItem containing the item's name and path
    /// - An error if the item's name or URL cannot be retrieved
    unsafe fn convert_item_ref(&self, item_ref: LSSharedFileListItemRef) -> Result<SidebarItem> {
        debug_assert!(!item_ref.is_null(), "item_ref should not be null");

        // Get item name
        let name = self.api.get_item_display_name(item_ref);
        let name = if name.is_null() {
            return Err(Error::GetDisplayName {
                reason: "system returned null display name",
            });
        } else {
            Some(CFString::wrap_under_create_rule(name))
        };

        // Get item URL
        let url_ref = self.api.get_item_url(item_ref);
        if url_ref.is_null() {
            return Err(Error::GetItemUrl {
                reason: "system returned null URL",
            });
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
