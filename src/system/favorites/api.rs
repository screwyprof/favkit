use core_foundation::base::kCFAllocatorDefault;
use core_services::{LSSharedFileListResolutionFlags, kLSSharedFileListFavoriteItems};

use crate::{
    favorites::FavoritesApi,
    finder::{Result, SidebarItem, Target},
    system::api::MacOsApi,
};

use super::{
    display_name::DisplayName, handle::FavoritesHandle, snapshot::Snapshot,
    snapshot_item::SnapshotItem, url::Url,
};

pub struct Favorites<'a> {
    api: &'a dyn MacOsApi,
}

impl<'a> Favorites<'a> {
    pub fn new(api: &'a dyn MacOsApi) -> Self {
        Self { api }
    }

    unsafe fn list_create(&self) -> Result<FavoritesHandle> {
        let ptr = unsafe {
            self.api.ls_shared_file_list_create(
                kCFAllocatorDefault,
                kLSSharedFileListFavoriteItems,
                std::ptr::null(),
            )
        };
        Ok(FavoritesHandle::try_from(ptr)?)
    }

    unsafe fn copy_snapshot(&self, list: FavoritesHandle) -> Result<Snapshot> {
        let mut seed: u32 = 0;
        let array_ref = unsafe {
            self.api
                .ls_shared_file_list_copy_snapshot(list.into(), &mut seed)
        };
        Ok(Snapshot::try_from(array_ref)?)
    }

    unsafe fn copy_display_name(&self, item: &SnapshotItem) -> Result<DisplayName> {
        let name_ref = unsafe {
            self.api
                .ls_shared_file_list_item_copy_display_name(item.into())
        };
        Ok(DisplayName::try_from(name_ref)?)
    }

    unsafe fn copy_resolved_url(&self, item: &SnapshotItem) -> Result<Url> {
        let url_ref = unsafe {
            self.api.ls_shared_file_list_item_copy_resolved_url(
                item.into(),
                LSSharedFileListResolutionFlags::default(),
                std::ptr::null_mut(),
            )
        };
        Ok(Url::try_from(url_ref)?)
    }

    unsafe fn convert_item(&self, item: SnapshotItem) -> Result<SidebarItem> {
        let display_name = unsafe { self.copy_display_name(&item) }
            .ok()
            .map(|name| name.to_string())
            .and_then(|name| (!name.is_empty()).then_some(name));
        let url = unsafe { self.copy_resolved_url(&item) }?;
        let target = Target(url.to_string());
        Ok(SidebarItem::new(display_name, target))
    }
}

impl FavoritesApi for Favorites<'_> {
    fn list_items(&self) -> Result<Vec<SidebarItem>> {
        unsafe {
            let list = self.list_create()?;
            let snapshot = self.copy_snapshot(list)?;

            snapshot
                .into_iter()
                .map(|item| self.convert_item(item))
                .collect::<Result<Vec<_>>>()
        }
    }
}
