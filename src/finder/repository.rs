use super::{
    sidebar::Sidebar,
    sidebar_item::SidebarItem,
    macos::MacOsApi,
};
use core_foundation::base::TCFType;
use core_services::LSSharedFileListItemRef;

pub struct Repository {
    api: Box<dyn MacOsApi>,
}

impl Repository {
    pub fn new(api: Box<dyn MacOsApi>) -> Self {
        Self { api }
    }

    pub fn load(&self) -> Sidebar {
        let mut seed = 0u32;
        let items = unsafe {
            // Get the favorites list
            let list = self.api.get_favorites_list();
            
            // Get a snapshot of the current state
            let snapshot = self.api.get_favorites_snapshot(list, &mut seed);
            
            // Convert each item in the snapshot to a SidebarItem
            let mut items = Vec::new();
            let len = snapshot.len();
            
            // Use as_concrete_TypeRef() to get the underlying array pointer
            let array_ref = snapshot.as_concrete_TypeRef();
            
            for i in 0..len {
                // Use core_foundation array functions directly
                let item_ref = core_foundation::array::CFArrayGetValueAtIndex(array_ref, i) as LSSharedFileListItemRef;
                
                // Get the URL for this item
                let url = self.api.get_item_url(item_ref);
                
                // Convert the URL to a Target and create a SidebarItem
                if !url.is_null() {
                    let target = self.api.url_to_target(url);
                    items.push(SidebarItem::new(target));
                }
            }

            items
        };

        Sidebar::new(items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::finder::target::Target;
    use std::path::PathBuf;
    use crate::finder::macos::MacOsApi;
    use core_foundation::{
        array::CFArray,
        string::CFStringRef,
        url::CFURLRef,
    };
    use core_services::{LSSharedFileListItemRef, LSSharedFileListRef};
    use std::ptr;

    struct TestMacOsApi {
        favorites: Vec<Target>,
    }

    impl TestMacOsApi {
        fn new(favorites: Vec<Target>) -> Self {
            Self { favorites }
        }
    }

    impl MacOsApi for TestMacOsApi {
        unsafe fn get_favorites_list(&self) -> LSSharedFileListRef {
            ptr::null_mut()
        }

        unsafe fn get_favorites_snapshot(
            &self,
            _list: LSSharedFileListRef,
            _seed: &mut u32,
        ) -> CFArray<LSSharedFileListItemRef> {
            let items: Vec<LSSharedFileListItemRef> = (1..=self.favorites.len())
                .map(|i| (i as *mut std::ffi::c_void) as LSSharedFileListItemRef)
                .collect();
            CFArray::from_copyable(&items)
        }

        unsafe fn get_item_display_name(&self, _item: LSSharedFileListItemRef) -> CFStringRef {
            ptr::null_mut()
        }

        unsafe fn get_item_url(&self, item: LSSharedFileListItemRef) -> CFURLRef {
            item as *const _ as CFURLRef
        }

        unsafe fn url_to_target(&self, url: CFURLRef) -> Target {
            if url.is_null() {
                return Target::Home(dirs::home_dir().unwrap_or_default());
            }
            let item = url as LSSharedFileListItemRef;
            let index = item as usize - 1;
            self.favorites.get(index).cloned().unwrap_or_else(|| {
                Target::CustomPath(PathBuf::from("/unknown"))
            })
        }
    }

    #[test]
    fn test_repository_loads_favorites() {
        // Given
        let favorites = vec![
            Target::Home(PathBuf::from("/Users/test")),
            Target::Desktop(PathBuf::from("/Users/test/Desktop")),
        ];
        let api = TestMacOsApi::new(favorites.clone());
        let repository = Repository::new(Box::new(api));

        // When
        let sidebar = repository.load();

        // Then
        let loaded_favorites: Vec<Target> = sidebar
            .favorites()
            .iter()
            .map(|item| item.target().clone())
            .collect();
        assert_eq!(loaded_favorites, favorites);
    }
}
