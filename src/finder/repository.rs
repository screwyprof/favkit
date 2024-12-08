use crate::errors::FinderError;
use crate::finder::sidebar::item::SidebarItem;
use crate::finder::sidebar::Target;
use crate::finder::system::api::MacOsApi;
use crate::finder::system::url::MacOsUrl;
use core_foundation::{base::TCFType, string::CFString};
use core_services::LSSharedFileListItemRef;

/// Repository is responsible for loading and saving sidebar items.
pub struct Repository {
    api: Box<dyn MacOsApi>,
}

impl Repository {
    pub fn new(api: Box<dyn MacOsApi>) -> Self {
        Self { api }
    }

    pub fn load(&self) -> Result<Vec<SidebarItem>, FinderError> {
        // Get the favorites list
        // SAFETY: We trust that Core Foundation provides a valid favorites list
        let favorites = unsafe { self.api.get_favorites_list() };
        if favorites.is_null() {
            return Err(FinderError::SystemError(
                "Could not get favorites list".to_string(),
            ));
        }

        // Get a snapshot of the favorites
        let mut seed = 0;
        // SAFETY: We trust that Core Foundation provides a valid snapshot from a valid favorites list
        let snapshot = unsafe { self.api.get_favorites_snapshot(favorites, &mut seed) };
        if snapshot.as_concrete_TypeRef().is_null() {
            return Err(FinderError::SystemError(
                "Could not get favorites snapshot".to_string(),
            ));
        }

        // Get all items from the snapshot
        let items_array = snapshot.get_all_values();
        let mut items = Vec::new();
        println!("Processing {} items", items_array.len());

        // Process each item
        for (idx, item) in items_array.iter().enumerate() {
            let item = *item as LSSharedFileListItemRef;

            // Get the URL for this item
            // SAFETY: We trust that Core Foundation provides a valid URL for the item
            let url_ref = unsafe { self.api.get_item_url(item) };
            // SAFETY: We trust that Core Foundation provides either null or valid URL pointers
            let url = unsafe { MacOsUrl::from_url_ref(url_ref) };
            let target = match url {
                Some(url) => Target::try_from(url)?,
                None => continue, // Skip items with no URL
            };
            println!("Item {} Target: {:?}", idx, target);

            // Get display name
            // SAFETY: We trust that Core Foundation provides a valid display name pointer
            let display_name = unsafe { self.api.get_item_display_name(item) };
            let item = if display_name.is_null() {
                match target {
                    Target::AirDrop(_) => Some(SidebarItem::new(target, "AirDrop")),
                    _ => None, // Skip other items with no display name
                }
            } else {
                // SAFETY: We trust that Core Foundation provides a valid display name string
                let cf_string = unsafe { CFString::wrap_under_create_rule(display_name) };
                let name = cf_string.to_string();
                if name.is_empty() {
                    match target {
                        Target::AirDrop(_) => Some(SidebarItem::new(target, "AirDrop")),
                        _ => None, // Skip other items with empty display name
                    }
                } else {
                    println!("Item {} display name: {}", idx, name);
                    Some(SidebarItem::new(target, &name))
                }
            };

            if let Some(item) = item {
                println!("Created item {}: {:?}", idx, item);
                items.push(item);
            }
        }

        println!("Final items: {:?}", items);
        Ok(items)
    }
}
