use favkit::sidebar::{MacOsApi, MacOsPath};
use std::sync::Mutex;

pub struct MockMacOsApi {
    favorites: Mutex<Vec<(String, MacOsPath)>>,
}

impl MockMacOsApi {
    pub fn with_favorites(favorites: Vec<(String, MacOsPath)>) -> Self {
        Self {
            favorites: Mutex::new(favorites),
        }
    }
}

impl MacOsApi for MockMacOsApi {
    fn list_favorite_items(&self) -> Vec<(String, MacOsPath)> {
        self.favorites.lock().unwrap().clone()
    }
}
