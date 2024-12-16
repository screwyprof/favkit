use core_foundation::base::kCFAllocatorDefault;
use core_services::kLSSharedFileListFavoriteItems;

use crate::{finder::Result, system::api::MacOsApi};

use super::FavoritesApi;

pub struct MacOsFavorites<'a> {
    api: &'a dyn MacOsApi,
}

impl<'a> MacOsFavorites<'a> {
    pub fn new(api: &'a dyn MacOsApi) -> Self {
        Self { api }
    }
}

impl FavoritesApi for MacOsFavorites<'_> {
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
