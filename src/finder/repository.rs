use super::{
    sidebar::Sidebar,
    sidebar_item::SidebarItem,
    macos::MacOsApi,
};
use core_services::LSSharedFileListItemRef;

/// Repository is responsible for loading and saving sidebar items.
#[allow(dead_code)]
pub struct Repository {
    api: Box<dyn MacOsApi>,
}

impl Repository {
    pub fn new(api: Box<dyn MacOsApi>) -> Self {
        Self { api }
    }

    pub fn load(&self) -> Sidebar {
        unsafe {
            let favorites_list = self.api.get_favorites_list();
            if favorites_list.is_null() {
                return Sidebar::new(vec![]);
            }

            let mut seed = 0;
            let array = self.api.get_favorites_snapshot(favorites_list, &mut seed);

            let items = array
                .get_all_values()
                .iter()
                .filter_map(|&item_ref| {
                    let item_ref = item_ref as LSSharedFileListItemRef;
                    let url_ref = self.api.get_item_url(item_ref);
                    if url_ref.is_null() {
                        return None;
                    }

                    let target = self.api.url_to_target(url_ref);
                    Some(SidebarItem::new(target))
                })
                .collect();

            Sidebar::new(items)
        }
    }
}
