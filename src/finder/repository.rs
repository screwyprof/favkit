use core_foundation::base::TCFType;
use core_foundation::string::CFString;
use core_services::LSSharedFileListItemRef;
use super::macos::MacOsApi;
use super::macos_url::url_to_target;
use super::sidebar::Sidebar;
use super::sidebar_item::SidebarItem;

/// Repository is responsible for loading and saving sidebar items.
pub struct Repository {
    api: Box<dyn MacOsApi>,
}

impl Repository {
    pub fn new(api: Box<dyn MacOsApi>) -> Self {
        Self { api }
    }

    pub fn load(&self) -> Sidebar {
        let mut items = Vec::new();

        unsafe {
            let favorites = self.api.get_favorites_list();
            if favorites.is_null() {
                return Sidebar::new(items);
            }

            let mut seed = 0;
            let snapshot = self.api.get_favorites_snapshot(favorites, &mut seed);
            let items_array = snapshot.get_all_values();

            for item in items_array {
                let item = item as LSSharedFileListItemRef;
                let url_ref = self.api.get_item_url(item);
                if url_ref.is_null() {
                    continue;
                }

                let target = url_to_target(url_ref);
                let display_name = self.api.get_item_display_name(item);
                
                let item = if display_name.is_null() {
                    SidebarItem::new(target)
                } else {
                    let cf_string = CFString::wrap_under_get_rule(display_name);
                    SidebarItem::with_display_name(target, cf_string.to_string())
                };

                items.push(item);
            }
        }

        Sidebar::new(items)
    }
}
