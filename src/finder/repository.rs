use core_services::LSSharedFileListItemRef;

use crate::{
    errors::FinderError,
    finder::{
        sidebar::{target::Target, item::SidebarItem},
        system::api::MacOsApi,
    },
};

use core_foundation::array::CFArray;

/// Repository is responsible for loading and saving sidebar items.
pub struct Repository {
    api: Box<dyn MacOsApi>,
}

impl Repository {
    pub fn new(api: Box<dyn MacOsApi>) -> Self {
        Self { api }
    }

    /// Load all sidebar items from the favorites list
    pub fn load(&self) -> Result<Vec<SidebarItem>, FinderError> {
        let snapshot = self.get_favorites_snapshot()?;
        let items = snapshot
            .get_all_values()
            .iter()
            .filter_map(|item| self.process_item(*item as LSSharedFileListItemRef))
            .collect();

        Ok(items)
    }

    /// Get a snapshot of the favorites list
    fn get_favorites_snapshot(&self) -> Result<CFArray<LSSharedFileListItemRef>, FinderError> {
        // SAFETY: We trust that Core Foundation provides a valid favorites list
        let favorites = unsafe { self.api.get_favorites_list()? };

        let mut seed = 0;
        // SAFETY: We trust that Core Foundation provides a valid snapshot from a valid favorites list
        unsafe { self.api.get_favorites_snapshot(favorites, &mut seed) }
    }

    /// Process a single item from the favorites list
    fn process_item(&self, item: LSSharedFileListItemRef) -> Option<SidebarItem> {
        // Get URL first
        let url = unsafe { self.api.get_item_url(item) }?;

        // Try to convert URL to target
        let target = Target::try_from(&url).ok()?;

        // Handle AirDrop specially - it will have empty display name from macOS
        if let Target::AirDrop(_) = target {
            return Some(SidebarItem::new(target, "AirDrop"));
        }

        // Get display name for non-AirDrop items
        let display_name = unsafe { self.api.get_item_display_name(item) };

        // For other items, require a non-empty display name
        display_name
            .filter(|name| !name.is_empty())
            .map(|name| SidebarItem::new(target, &name))
    }
}
