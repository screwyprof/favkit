#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

use std::cell::Cell;
use favkit::{FinderApi, FavoritesApi};
use core_services::LSSharedFileListRef;

struct MockFavoritesApi {
    create_favorites_called: Cell<bool>,
}

impl MockFavoritesApi {
    fn new() -> Self {
        Self {
            create_favorites_called: Cell::new(false),
        }
    }

    fn was_called(&self) -> bool {
        self.create_favorites_called.get()
    }
}

impl FavoritesApi for MockFavoritesApi {
    fn create_favorites_list(&self) -> LSSharedFileListRef {
        self.create_favorites_called.set(true);
        std::ptr::null_mut()
    }
}

#[test]
fn it_delegates_to_favorites_api() {
    // Given I have a mock favorites API
    let mock = MockFavoritesApi::new();
    let finder = FinderApi::new(&mock);
    
    // When I get favorites list
    finder.get_favorites_list();
    
    // Then create_favorites_list was called
    assert!(mock.was_called());
}
