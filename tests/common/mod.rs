#[cfg(test)]
use favkit::sidebar::{MacOsApi, MacOsPath};
use std::cell::RefCell;

#[cfg(test)]
pub struct MockMacOsApi {
    favorites: RefCell<Vec<(String, MacOsPath)>>,
    list_favorites_called: RefCell<u32>,
}

#[cfg(test)]
impl MockMacOsApi {
    pub fn with_favorites(favorites: Vec<(String, MacOsPath)>) -> Self {
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

#[cfg(test)]
impl MacOsApi for MockMacOsApi {
    fn list_favorite_items(&self) -> Vec<(String, MacOsPath)> {
        *self.list_favorites_called.borrow_mut() += 1;
        self.favorites.borrow().clone()
    }
}
