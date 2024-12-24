use core_foundation::{string::CFStringRef, url::CFURLRef};
use core_services::{LSSharedFileListItemRef, LSSharedFileListRef};
use favkit::system::favorites::{DisplayName, Url};

use super::{
    favorites::{FavoriteItem, Favorites},
    mac_os_api::MockMacOsApi,
};

/// Index type for safer array access
#[derive(Debug, Clone, Copy)]
pub struct ItemIndex(usize);

impl From<LSSharedFileListItemRef> for ItemIndex {
    fn from(raw: LSSharedFileListItemRef) -> Self {
        Self((raw as i32 - 1) as usize)
    }
}

pub struct MockBuilder {
    mock: MockMacOsApi,
    handle: Option<LSSharedFileListRef>,
    favorites: Vec<FavoriteItem>,
    should_return_null_snapshot: bool,
}

impl MockBuilder {
    pub fn new() -> Self {
        Self {
            mock: MockMacOsApi::new(),
            handle: Some(1 as LSSharedFileListRef),
            favorites: Vec::new(),
            should_return_null_snapshot: false,
        }
    }

    pub fn with_favorites(mut self, favorites: Favorites) -> Self {
        self.favorites = favorites.into_items();
        self
    }

    fn get_display_name(
        display_names: &[DisplayName],
        item_ref: LSSharedFileListItemRef,
    ) -> CFStringRef {
        let idx: ItemIndex = item_ref.into();
        (&display_names[idx.0]).into()
    }

    fn get_url(urls: &[Url], item_ref: LSSharedFileListItemRef) -> CFURLRef {
        let idx: ItemIndex = item_ref.into();
        (&urls[idx.0]).into()
    }

    pub fn with_null_handle(mut self) -> Self {
        self.handle = None;
        self
    }

    pub fn with_null_snapshot(mut self) -> Self {
        self.should_return_null_snapshot = true;
        self
    }

    pub fn build(self) -> MockMacOsApi {
        // Create and store everything first
        let handle = self.handle.unwrap_or(std::ptr::null_mut());

        // Get mock and set up expectations
        let mut mock = self.mock;

        mock.expect_ls_shared_file_list_create()
            .returning_st(move |_, _, _| handle);

        if self.should_return_null_snapshot {
            mock.expect_ls_shared_file_list_copy_snapshot()
                .returning_st(move |_, _| std::ptr::null_mut());
            return mock;
        }

        // Store everything in
        let favorites = Favorites::new(self.favorites);

        mock.expect_ls_shared_file_list_copy_snapshot()
            .returning_st(move |_, _| {
                let snapshot = favorites.snapshot.clone();
                let snapshot = snapshot.as_ref().as_ref().unwrap();
                snapshot.into()
            });

        if !favorites.display_names.is_empty() {
            let display_names = favorites.display_names.clone();
            mock.expect_ls_shared_file_list_item_copy_display_name()
                .returning_st(move |item| Self::get_display_name(&display_names, item));

            let urls = favorites.urls.clone();
            mock.expect_ls_shared_file_list_item_copy_resolved_url()
                .returning_st(move |item, _, _| Self::get_url(&urls, item));
        }

        mock
    }
}
