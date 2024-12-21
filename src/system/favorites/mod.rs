mod display_name;
mod errors;
mod handle;
mod snapshot;
mod snapshot_item;
mod url;
mod url_mapper;

use core_foundation::base::kCFAllocatorDefault;
use core_services::{LSSharedFileListResolutionFlags, kLSSharedFileListFavoriteItems};
pub use display_name::DisplayName;
pub use errors::FavoritesError;
pub use handle::FavoritesHandle;
pub use snapshot::Snapshot;
pub use snapshot_item::SnapshotItem;
pub use url::Url;
pub use url_mapper::TargetUrl;

use crate::{
    finder::{Result, SidebarItem, Target, favorites::FavoritesApi},
    system::api::MacOsApi,
};

pub struct Favorites {
    api: Box<dyn MacOsApi>,
}

impl Favorites {
    pub fn new(api: impl MacOsApi + 'static) -> Self {
        Self { api: Box::new(api) }
    }

    unsafe fn list_create(&self) -> errors::Result<FavoritesHandle> {
        let ptr = unsafe {
            self.api.ls_shared_file_list_create(
                kCFAllocatorDefault,
                kLSSharedFileListFavoriteItems,
                std::ptr::null(),
            )
        };
        FavoritesHandle::try_from(ptr)
    }

    unsafe fn copy_snapshot(&self, list: FavoritesHandle) -> errors::Result<Snapshot> {
        let mut seed: u32 = 0;
        let array_ref = unsafe {
            self.api
                .ls_shared_file_list_copy_snapshot(list.into(), &mut seed)
        };
        Snapshot::try_from(array_ref)
    }

    unsafe fn copy_display_name(&self, item: &SnapshotItem) -> errors::Result<DisplayName> {
        let name_ref = unsafe {
            self.api
                .ls_shared_file_list_item_copy_display_name(item.into())
        };
        DisplayName::try_from(name_ref)
    }

    unsafe fn copy_resolved_url(&self, item: &SnapshotItem) -> errors::Result<Url> {
        let url_ref = unsafe {
            self.api.ls_shared_file_list_item_copy_resolved_url(
                item.into(),
                LSSharedFileListResolutionFlags::default(),
                std::ptr::null_mut(),
            )
        };
        Url::try_from(url_ref)
    }

    unsafe fn convert_item(&self, item: SnapshotItem) -> Result<SidebarItem> {
        let url = unsafe { self.copy_resolved_url(&item) }?;
        let name = unsafe { self.copy_display_name(&item) }?;
        let target = Target::from(TargetUrl(url, name));
        Ok(SidebarItem::new(target))
    }
}

impl FavoritesApi for Favorites {
    fn list_items(&self) -> Result<Vec<SidebarItem>> {
        unsafe {
            let list = self.list_create()?;
            let snapshot = self.copy_snapshot(list)?;

            snapshot
                .into_iter()
                .map(|item| self.convert_item(item))
                .collect()
        }
    }
}
