use std::cell::RefCell;

// Re-export what we need from favkit
// Note: This module requires the test-utils feature to be enabled
use favkit::sidebar::{MacOsApi, MacOsPath, SidebarItem};

/// A mock implementation of MacOsApi for testing.
/// Keeps track of favorites and counts how many times list_favorites is called.
pub struct MockMacOsApi {
    favorites: RefCell<Vec<SidebarItem>>,
    list_favorites_called: RefCell<u32>,
}

impl MockMacOsApi {
    /// Creates a new mock with the given favorites list
    pub fn with_favorites(favorites: Vec<SidebarItem>) -> Self {
        Self {
            favorites: RefCell::new(favorites),
            list_favorites_called: RefCell::new(0),
        }
    }

    /// Returns how many times list_favorite_items was called
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
            .cloned()
            .map(SidebarItem::into_parts)
            .collect()
    }
}
