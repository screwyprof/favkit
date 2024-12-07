use core_foundation::base::TCFType;
use core_foundation::string::CFString;
use core_services::LSSharedFileListItemRef;
use super::macos::MacOsApi;
use super::macos_url::url_to_target;
use super::sidebar::Sidebar;
use super::sidebar_item::SidebarItem;
use super::target::Target;
use crate::errors::FinderError;

/// Repository is responsible for loading and saving sidebar items.
pub struct Repository {
    api: Box<dyn MacOsApi>,
}

impl Repository {
    pub fn new(api: Box<dyn MacOsApi>) -> Self {
        Self { api }
    }

    pub fn load(&self) -> Result<Sidebar, FinderError> {
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

                // Convert URL to target
                let target = match url_to_target(url_ref) {
                    Ok(target) => target,
                    Err(super::macos_url::UrlError::NullUrl) => {
                        return Err(FinderError::InvalidPath {
                            path: "/invalid/path".into(),
                            source: None,
                        });
                    }
                    Err(super::macos_url::UrlError::InvalidUrl(msg)) => {
                        return Err(FinderError::UnsupportedTarget(msg));
                    }
                };

                // Get display name
                let display_name = self.api.get_item_display_name(item);
                let item = if display_name.is_null() {
                    match target {
                        Target::AirDrop(_) => SidebarItem::with_display_name(target, "AirDrop"),
                        _ => SidebarItem::new(target)
                    }
                } else {
                    let cf_string = CFString::wrap_under_get_rule(display_name);
                    SidebarItem::with_display_name(target, cf_string.to_string())
                };

                items.push(item);
            }

            Ok(Sidebar::new(items))
        }
    }
}
