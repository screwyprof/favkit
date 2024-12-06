use crate::finder::macos::MacOsApi;
use crate::finder::sidebar::Sidebar;
use crate::finder::sidebar_item::SidebarItem;

use core_foundation::{
    url::CFURL,
    base::TCFType,
};

use core_services::LSSharedFileListItemRef;

use dirs;

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
            println!("favorites_list is null: {}", favorites_list.is_null());
            if favorites_list.is_null() {
                return None;
            }

            let mut seed = 0;
            let favorites_snapshot = self.api.get_favorites_snapshot(favorites_list, &mut seed);
            let values = favorites_snapshot.get_all_values();
            println!("snapshot values count: {}", values.len());

            let mut favorites = Vec::new();
            for item_ref in values.iter().map(|&x| x as LSSharedFileListItemRef) {
                println!("processing item_ref: {:?}", item_ref);
                let url_ref = self.api.get_item_url(item_ref);
                println!("url_ref is null: {}", url_ref.is_null());
                if !url_ref.is_null() {
                    let url = CFURL::wrap_under_get_rule(url_ref);
                    println!("got url: {:?}", url);
                    
                    let url_str = url.get_string().to_string();
                    println!("url string: {}", url_str);
                    
                    // Handle special URLs first
                    if url_str == "~/" {
                        println!("Found home URL");
                        if let Some(sidebar_item) = SidebarItem::new("~/") {
                            println!("Created home sidebar item");
                            favorites.push(sidebar_item);
                            continue;
                        } else {
                            println!("Failed to create home sidebar item");
                        }
                    }
                    
                    // Special handling for AirDrop URL (trim trailing slash)
                    let normalized_url = url_str.trim_end_matches('/');
                    println!("normalized url: {}", normalized_url);
                    
                    if normalized_url == "nwnode://domain-AirDrop" {
                        println!("Found AirDrop URL");
                        if let Some(sidebar_item) = SidebarItem::new("nwnode://domain-AirDrop") {
                            println!("Created AirDrop sidebar item");
                            favorites.push(sidebar_item);
                            continue;
                        } else {
                            println!("Failed to create AirDrop sidebar item");
                        }
                    }
                    
                    // Handle regular file paths
                    if let Some(path) = url.to_path() {
                        println!("got path: {:?}", path);
                        // Convert real paths to our special paths
                        if let Some(home_dir) = dirs::home_dir() {
                            if path == home_dir {
                                if let Some(sidebar_item) = SidebarItem::new("~/") {
                                    favorites.push(sidebar_item);
                                    continue;
                                }
                            }
                        }
                        // Try the path as-is as fallback
                        if let Some(sidebar_item) = SidebarItem::new(path) {
                            favorites.push(sidebar_item);
                        }
                    } else {
                        println!("failed to get path from url");
                    }
                }
            }

            println!("final favorites count: {}", favorites.len());
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
        let airdrop = "nwnode://domain-AirDrop";
        // Create URL with a trailing slash to match macOS behavior
        let url = CFURL::from_path(format!("{}/", airdrop), false).expect("Failed to create URL");
        println!("Test URL: {:?}", url);
        println!("Test URL string: {:?}", url.get_string().to_string());
        
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
        assert_eq!(favorites.iter().next().unwrap().path().to_str().unwrap(), airdrop);
    }
}
