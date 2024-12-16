use core_foundation::{
    array::CFArray,
    base::{TCFType, kCFAllocatorDefault},
};
use core_services::{LSSharedFileListItemRef, LSSharedFileListRef, kLSSharedFileListFavoriteItems};

use crate::{
    favorites::FavoritesApi,
    finder::{FinderError, ListErrorKind, Result},
    system::api::MacOsApi,
};

pub struct Favorites<'a> {
    api: &'a dyn MacOsApi,
}

impl<'a> Favorites<'a> {
    pub fn new(api: &'a dyn MacOsApi) -> Self {
        Self { api }
    }

    unsafe fn list_create(&self) -> Result<LSSharedFileListRef> {
        unsafe {
            let list_ref = self.api.ls_shared_file_list_create(
                kCFAllocatorDefault,
                kLSSharedFileListFavoriteItems,
                std::ptr::null(),
            );

            (!list_ref.is_null())
                .then_some(list_ref)
                .ok_or(FinderError::ListError {
                    kind: ListErrorKind::NullHandle,
                })
        }
    }

    unsafe fn copy_snapshot(&self, list: LSSharedFileListRef) -> CFArray<LSSharedFileListItemRef> {
        let mut seed: u32 = 0;
        unsafe {
            let array_ref = self.api.ls_shared_file_list_copy_snapshot(list, &mut seed);
            CFArray::wrap_under_get_rule(array_ref)
        }
    }
}

impl FavoritesApi for Favorites<'_> {
    fn list_items(&self) -> Result<Vec<String>> {
        unsafe {
            let list = self.list_create()?;
            let _array = self.copy_snapshot(list);

            Ok(vec![])
        }
    }
}
