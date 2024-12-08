use crate::errors::FinderError;
use crate::finder::sidebar::item::SidebarItem;
use crate::finder::sidebar::Target;
use crate::finder::system::api::MacOsApi;
use crate::finder::system::url::MacOsUrl;
use core_foundation::{array::CFArray, base::TCFType, string::CFString};
use core_services::LSSharedFileListItemRef;

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
        let favorites = unsafe { self.api.get_favorites_list() };
        if favorites.is_null() {
            return Err(FinderError::SystemError("Could not get favorites list".into()));
        }

        let mut seed = 0;
        // SAFETY: We trust that Core Foundation provides a valid snapshot from a valid favorites list
        let snapshot = unsafe { self.api.get_favorites_snapshot(favorites, &mut seed) };
        if snapshot.as_concrete_TypeRef().is_null() {
            return Err(FinderError::SystemError("Could not get favorites snapshot".into()));
        }

        Ok(snapshot)
    }

    /// Process a single item from the favorites list
    fn process_item(&self, item: LSSharedFileListItemRef) -> Option<SidebarItem> {
        // Get target from URL
        let target = self.get_target_from_item(item)?;

        // Handle AirDrop specially - it always has "AirDrop" as display name
        if let Target::AirDrop(_) = target {
            return Some(SidebarItem::new(target, "AirDrop"));
        }

        // Get display name for other items
        let name = self.get_display_name(item)?;
        if name.is_empty() {
            None // Skip items with empty display name
        } else {
            Some(SidebarItem::new(target, &name))
        }
    }

    /// Get the target from an item's URL
    fn get_target_from_item(&self, item: LSSharedFileListItemRef) -> Option<Target> {
        // SAFETY: We trust that Core Foundation provides a valid URL for the item
        let url = unsafe { 
            self.api
                .get_item_url(item)
                .as_ref()
                .and_then(|url_ref| MacOsUrl::from_nullable_ref(url_ref))
        }?;
        
        Target::try_from(url).ok()
    }

    /// Get the display name for an item
    fn get_display_name(&self, item: LSSharedFileListItemRef) -> Option<String> {
        // SAFETY: We trust that Core Foundation provides a valid display name pointer
        let display_name = unsafe { self.api.get_item_display_name(item) };
        if display_name.is_null() {
            return None;
        }

        // SAFETY: We've checked that display_name is not null
        let cf_string = unsafe { CFString::wrap_under_create_rule(display_name) };
        Some(cf_string.to_string())
    }
}
