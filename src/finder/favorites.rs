use core_foundation::base::kCFAllocatorDefault;
use core_services::{LSSharedFileListCreate, kLSSharedFileListFavoriteItems};

#[derive(Default)]
pub struct Favorites;

impl Favorites {
    pub fn new() -> Self {
        Self
    }
}

impl super::FavoritesApi for Favorites {
    fn create_favorites_list(&self) -> super::LSSharedFileListRef {
        unsafe {
            LSSharedFileListCreate(
                kCFAllocatorDefault,
                kLSSharedFileListFavoriteItems,
                std::ptr::null(),
            )
        }
    }
}
