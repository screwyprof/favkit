use crate::finder::macos::MacOsApi;
use crate::finder::sidebar::Sidebar;
use crate::finder::sidebar_item::SidebarItem;

use core_foundation::{
    url::CFURL,
    base::TCFType,
};

use core_services::LSSharedFileListItemRef;

pub struct SidebarRepository<T: MacOsApi> {
    api: T,
}

impl<T: MacOsApi> SidebarRepository<T> {
    pub fn new(api: T) -> Self {
        Self { api }
    }

    pub fn load(&self) -> Option<Sidebar> {
        unsafe {
            let favorites_list = self.api.get_favorites_list();
            if favorites_list.is_null() {
                return None;
            }

            let mut seed = 0;
            let favorites_snapshot = self.api.get_favorites_snapshot(favorites_list, &mut seed);

            let mut favorites = Vec::new();
            let values = favorites_snapshot.get_all_values();
            for item_ref in values.iter().map(|&x| x as LSSharedFileListItemRef) {
                let url_ref = self.api.get_item_url(item_ref);
                if !url_ref.is_null() {
                    let url = CFURL::wrap_under_get_rule(url_ref);
                    if let Some(path) = url.to_path() {
                        if let Some(item) = SidebarItem::new(path) {
                            favorites.push(item);
                        }
                    }
                }
            }

            Some(Sidebar::new(favorites))
        }
    }

    pub fn save(&self, _sidebar: &Sidebar) -> Option<()> {
        // TODO: Implement save functionality when needed
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::finder::macos::test_utils::MockMacOsApi;
    use core_services::{LSSharedFileListItemRef, LSSharedFileListRef};
    use core_foundation::array::CFArray;

    #[test]
    fn loads_empty_favorites() {
        // Given
        let api = MockMacOsApi::new()
            .with_favorites_list(std::ptr::null_mut())
            .with_favorites_snapshot(CFArray::from_copyable(&[]));

        let repository = SidebarRepository::new(api);

        // When
        let sidebar = repository.load();

        // Then
        assert!(sidebar.is_none());
    }

    #[test]
    fn loads_single_favorite() {
        // Given
        let home = std::path::Path::new("/Users");
        let test_path = home;
        let url = CFURL::from_path(&test_path, true).unwrap();
        let item_ref = 1 as LSSharedFileListItemRef;
        
        let favorites_snapshot = CFArray::from_copyable(&[item_ref]);
        let api = MockMacOsApi::new()
            .with_favorites_list(1 as LSSharedFileListRef)
            .with_favorites_snapshot(favorites_snapshot)
            .with_item_url(url.as_concrete_TypeRef(), item_ref);

        let repository = SidebarRepository::new(api);

        // When
        let sidebar = repository.load().unwrap();

        // Then
        let favorites = sidebar.favorites();
        assert_eq!(favorites.iter().count(), 1);
        assert_eq!(favorites.iter().next().unwrap().path(), test_path);
    }
}
