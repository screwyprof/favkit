use core_foundation::{
    array::CFArray,
    base::{TCFType, kCFAllocatorDefault},
};
use core_services::{LSSharedFileListItemRef, kLSSharedFileListFavoriteItems};

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
            let list = self.api.ls_shared_file_list_create(
                kCFAllocatorDefault,
                kLSSharedFileListFavoriteItems,
                std::ptr::null(),
            );

            let mut seed: u32 = 0;
            let array_ref = self.api.ls_shared_file_list_copy_snapshot(list, &mut seed);
            let _array = CFArray::<LSSharedFileListItemRef>::wrap_under_get_rule(array_ref);

            Ok(vec![])
        }
    }
}
