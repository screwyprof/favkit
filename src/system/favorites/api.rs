use core_foundation::base::kCFAllocatorDefault;
use core_services::kLSSharedFileListFavoriteItems;

use crate::{
    favorites::FavoritesApi,
    finder::{FinderError, Result},
    system::api::MacOsApi,
};

use super::{
    display_name::{DisplayName, RawDisplayName},
    list::{FavoritesList, RawFavoritesList},
    snapshot::{RawSnapshot, Snapshot},
};

pub struct Favorites<'a> {
    api: &'a dyn MacOsApi,
}

impl<'a> Favorites<'a> {
    pub fn new(api: &'a dyn MacOsApi) -> Self {
        Self { api }
    }

    unsafe fn list_create(&self) -> Result<FavoritesList> {
        unsafe {
            let list_ref = self.api.ls_shared_file_list_create(
                kCFAllocatorDefault,
                kLSSharedFileListFavoriteItems,
                std::ptr::null(),
            );

            Option::from(RawFavoritesList::from(list_ref)).ok_or(FinderError::NullListHandle)
        }
    }

    unsafe fn copy_snapshot(&self, list: FavoritesList) -> Result<Snapshot> {
        let mut seed: u32 = 0;
        unsafe {
            let array_ref = self
                .api
                .ls_shared_file_list_copy_snapshot(list.into(), &mut seed);

            Option::from(RawSnapshot::from(array_ref)).ok_or(FinderError::NullSnapshotHandle)
        }
    }
}

impl FavoritesApi for Favorites<'_> {
    fn list_items(&self) -> Result<Vec<Option<String>>> {
        unsafe {
            let list = self.list_create()?;
            let snapshot = self.copy_snapshot(list)?;

            let mut names = Vec::new();
            for item in &snapshot {
                let name_ref = self
                    .api
                    .ls_shared_file_list_item_copy_display_name(item.into());
                let display_name =
                    Option::<DisplayName>::from(RawDisplayName::from(name_ref)).map(String::from);
                names.push(display_name);
            }

            Ok(names)
        }
    }
}
