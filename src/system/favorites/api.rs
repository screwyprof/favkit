use core_foundation::base::kCFAllocatorDefault;
use core_services::{LSSharedFileListResolutionFlags, kLSSharedFileListFavoriteItems};

use crate::{
    favorites::FavoritesApi,
    finder::{FinderError, Result, SidebarItem, Target},
    system::api::MacOsApi,
};

use super::{
    display_name::DisplayName, handle::FavoritesHandle, item::SnapshotItem, snapshot::Snapshot,
    url::Url,
};

pub struct Favorites<'a> {
    api: &'a dyn MacOsApi,
}

impl<'a> Favorites<'a> {
    pub fn new(api: &'a dyn MacOsApi) -> Self {
        Self { api }
    }

    unsafe fn list_create(&self) -> Result<FavoritesHandle> {
        unsafe {
            let ptr = self.api.ls_shared_file_list_create(
                kCFAllocatorDefault,
                kLSSharedFileListFavoriteItems,
                std::ptr::null(),
            );
            (!ptr.is_null())
                .then_some(FavoritesHandle::from(ptr))
                .ok_or(FinderError::NullListHandle)
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

    unsafe fn copy_display_name(&self, item: SnapshotItem) -> Result<DisplayName> {
        unsafe {
            let name_ref = self
                .api
                .ls_shared_file_list_item_copy_display_name(item.into());
            DisplayName::from_ref(name_ref).ok_or(FinderError::NullDisplayNameHandle)
        }
    }

    unsafe fn copy_resolved_url(&self, item: SnapshotItem) -> Result<Url> {
        unsafe {
            let url_ref = self.api.ls_shared_file_list_item_copy_resolved_url(
                item.into(),
                LSSharedFileListResolutionFlags::default(),
                std::ptr::null_mut(),
            );
            Url::from_ref(url_ref).ok_or(FinderError::NullUrlHandle)
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
                    let display_name = self
                        .copy_display_name(item)
                        .ok()
                        .map(|name| name.to_string())
                        .filter(|name| !name.is_empty());
                    let url = self.copy_resolved_url(item)?;
                    let target = Target(url.to_string());
                    Ok(SidebarItem::new(display_name, target))
                })
                .collect::<Result<Vec<_>>>()?;

            Ok(items)
        }
    }
}
