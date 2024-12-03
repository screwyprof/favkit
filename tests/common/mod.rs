use favkit::sidebar::{MacOsApi, MacOsPath, SidebarItem};
use std::cell::RefCell;

/// A mock implementation of MacOsApi for testing.
pub struct MockMacOsApi {
    favorites: RefCell<Vec<SidebarItem>>,
    list_favorites_called: RefCell<u32>,
}

impl MockMacOsApi {
    pub fn with_favorites(favorites: Vec<SidebarItem>) -> Self {
        Self {
            favorites: RefCell::new(favorites),
            list_favorites_called: RefCell::new(0),
        }
    }

    #[allow(dead_code)]
    pub fn list_favorites_call_count(&self) -> u32 {
        *self.list_favorites_called.borrow()
    }
}

impl MacOsApi for MockMacOsApi {
    fn list_favorite_items(&self) -> Vec<(String, MacOsPath)> {
        *self.list_favorites_called.borrow_mut() += 1;
        self.favorites
            .borrow()
            .iter()
            .map(|item| (item.name().to_string(), item.path().clone()))
            .collect()
    }
}
