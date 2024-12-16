use core_foundation::base::kCFAllocatorDefault;
use core_services::kLSSharedFileListFavoriteItems;

use crate::{favorites::FavoritesApi, finder::Result, system::api::MacOsApi};

pub struct Favorites<'a> {
    api: &'a dyn MacOsApi,
}

impl<'a> Favorites<'a> {
    pub fn new(api: &'a dyn MacOsApi) -> Self {
        Self { api }
    }
}

impl FavoritesApi for Favorites<'_> {
    fn list_items(&self) -> Result<Vec<String>> {
        unsafe {
            let _list = self.api.ls_shared_file_list_create(
                kCFAllocatorDefault,
                kLSSharedFileListFavoriteItems,
                std::ptr::null(),
            );

            Ok(vec![])
        }
    }
}
