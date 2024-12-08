use core_foundation::base::TCFType;
use core_foundation::string::CFString;
use core_foundation::url::CFURL;
use core_services::LSSharedFileListItemRef;
use crate::finder::system::api::MacOsApi;
use crate::finder::system::url::UrlError;
use crate::finder::sidebar::{Target, item::SidebarItem};
use crate::errors::FinderError;

/// Repository is responsible for loading and saving sidebar items.
pub struct Repository {
    api: Box<dyn MacOsApi>,
}

impl Repository {
    pub fn new(api: Box<dyn MacOsApi>) -> Self {
        Self { api }
    }

    pub fn load(&self) -> Result<Vec<SidebarItem>, FinderError> {
        unsafe {
            // Get the favorites list
            let favorites = self.api.get_favorites_list();
            if favorites.is_null() {
                return Err(FinderError::SystemError("Could not get favorites list".to_string()));
            }

            // Get a snapshot of the favorites
            let mut seed = 0;
            let snapshot = self.api.get_favorites_snapshot(favorites, &mut seed);
            if snapshot.as_concrete_TypeRef().is_null() {
                return Err(FinderError::SystemError("Could not get favorites snapshot".to_string()));
            }

            // Get all items from the snapshot
            let items_array = snapshot.get_all_values();
            let mut items = Vec::new();

            // Process each item
            for item in items_array {
                let item = item as LSSharedFileListItemRef;
                
                // Get the URL for this item
                let url_ref = self.api.get_item_url(item);
                if url_ref.is_null() {
                    return Err(FinderError::InvalidPath {
                        path: "/invalid/path".into(),
                        source: None,
                    });
                }

                let cfurl = CFURL::wrap_under_create_rule(url_ref);
                
                // Convert URL to target
                let target = match Target::try_from(&cfurl) {
                    Ok(target) => target,
                    Err(UrlError::PathToUrl) => {
                        return Err(FinderError::InvalidPath {
                            path: "/invalid/path".into(),
                            source: None,
                        });
                    }
                    Err(UrlError::InvalidUrl) => {
                        return Err(FinderError::UnsupportedTarget("Invalid URL format".to_string()));
                    }
                    _ => {
                        return Err(FinderError::UnsupportedTarget("Unsupported URL type".to_string()));
                    }
                };

                // Get display name
                let display_name = self.api.get_item_display_name(item);
                let item = match target {
                    Target::AirDrop(_) => SidebarItem::new(target, "AirDrop"),
                    _ => if display_name.is_null() {
                        SidebarItem::new(target, "")
                    } else {
                        let cf_string = CFString::wrap_under_get_rule(display_name);
                        SidebarItem::new(target, cf_string.to_string())
                    }
                };

                items.push(item);
            }

            Ok(items)
        }
    }
}
