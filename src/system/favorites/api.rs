use core_foundation::base::kCFAllocatorDefault;
use core_services::{
    LSSharedFileListItemRef, LSSharedFileListResolutionFlags, kLSSharedFileListFavoriteItems,
};

use crate::{
    favorites::FavoritesApi,
    finder::{FinderError, Result, SidebarItem, Target},
    system::api::MacOsApi,
};

use super::{display_name::DisplayName, handle::FavoritesHandle, snapshot::Snapshot, url::Url};

pub struct Favorites<'a> {
    api: &'a dyn MacOsApi,
}

impl<'a> Favorites<'a> {
    pub fn new(api: &'a dyn MacOsApi) -> Self {
        Self { api }
    }

    unsafe fn list_create(&self) -> Result<FavoritesHandle> {
        unsafe {
            let list_ref = self.api.ls_shared_file_list_create(
                kCFAllocatorDefault,
                kLSSharedFileListFavoriteItems,
                std::ptr::null(),
            );

            FavoritesHandle::from_ref(list_ref).ok_or(FinderError::NullListHandle)
        }
    }

    unsafe fn copy_snapshot(&self, list: FavoritesHandle) -> Result<Snapshot> {
        let mut seed: u32 = 0;
        unsafe {
            let array_ref = self
                .api
                .ls_shared_file_list_copy_snapshot(list.into(), &mut seed);

            Snapshot::from_ref(array_ref).ok_or(FinderError::NullSnapshotHandle)
        }
    }

    unsafe fn copy_display_name(&self, item: LSSharedFileListItemRef) -> Option<String> {
        unsafe {
            let name_ref = self.api.ls_shared_file_list_item_copy_display_name(item);
            DisplayName::from_ref(name_ref).map(String::from)
        }
    }

    unsafe fn copy_resolved_url(&self, item: LSSharedFileListItemRef) -> Result<String> {
        unsafe {
            let url_ref = self.api.ls_shared_file_list_item_copy_resolved_url(
                item,
                LSSharedFileListResolutionFlags::default(),
                std::ptr::null_mut(),
            );
            Url::from_ref(url_ref)
                .map(String::from)
                .ok_or(FinderError::NullUrlHandle)
        }
    }
}

impl FavoritesApi for Favorites<'_> {
    fn list_items(&self) -> Result<Vec<SidebarItem>> {
        unsafe {
            let list = self.list_create()?;
            let snapshot = self.copy_snapshot(list)?;

            let items = snapshot
                .into_iter()
                .map(|item| {
                    let item_ref = item.into();
                    let display_name = self
                        .copy_display_name(item_ref)
                        .filter(|name| !name.is_empty());
                    let target = Target(self.copy_resolved_url(item_ref)?);
                    Ok(SidebarItem::new(display_name, target))
                })
                .collect::<Result<Vec<_>>>()?;

            Ok(items)
        }
    }
}
