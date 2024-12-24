use core_foundation::{string::CFStringRef, url::CFURLRef};
use core_services::{LSSharedFileListItemRef, LSSharedFileListRef};
use favkit::system::favorites::{DisplayName, Url};

use super::{favorites::Favorites, mac_os_api::MockMacOsApi};

// States
pub struct Initial;
pub struct WithNullFavorites;
pub struct WithFavorites {
    handle: LSSharedFileListRef,
}
pub struct WithNullSnapshot {
    handle: LSSharedFileListRef,
}
pub struct WithItems {
    favorites: Favorites,
}

// Builder with state
pub struct MockBuilder<State> {
    mock: MockMacOsApi,
    state: State,
}

// Initial state transitions
impl MockBuilder<Initial> {
    pub fn new() -> Self {
        Self {
            mock: MockMacOsApi::new(),
            state: Initial,
        }
    }

    pub fn with_null_favorites(self) -> MockBuilder<WithNullFavorites> {
        MockBuilder {
            mock: self.mock,
            state: WithNullFavorites,
        }
    }

    pub fn with_favorites(self) -> MockBuilder<WithFavorites> {
        MockBuilder {
            mock: self.mock,
            state: WithFavorites {
                handle: 1 as LSSharedFileListRef,
            },
        }
    }
}

// WithFavorites state transitions
impl MockBuilder<WithFavorites> {
    pub fn with_null_snapshot(self) -> MockBuilder<WithNullSnapshot> {
        MockBuilder {
            mock: self.mock,
            state: WithNullSnapshot {
                handle: self.state.handle,
            },
        }
    }

    pub fn with_items(self, favorites: Favorites) -> MockBuilder<WithItems> {
        MockBuilder {
            mock: self.mock,
            state: WithItems { favorites },
        }
    }
}

// Terminal state builders
impl MockBuilder<WithNullFavorites> {
    pub fn build(self) -> MockMacOsApi {
        let mut mock = self.mock;
        mock.expect_ls_shared_file_list_create()
            .returning_st(move |_, _, _| std::ptr::null_mut());
        mock
    }
}

impl MockBuilder<WithNullSnapshot> {
    pub fn build(self) -> MockMacOsApi {
        let mut mock = self.mock;
        let handle = self.state.handle;

        mock.expect_ls_shared_file_list_create()
            .returning_st(move |_, _, _| handle);
        mock.expect_ls_shared_file_list_copy_snapshot()
            .returning_st(move |_, _| std::ptr::null_mut());

        mock
    }
}

impl MockBuilder<WithItems> {
    fn get_display_name(
        display_names: &[DisplayName],
        item_ref: LSSharedFileListItemRef,
    ) -> CFStringRef {
        let idx = (item_ref as i32 - 1) as usize;
        (&display_names[idx]).into()
    }

    fn get_url(urls: &[Url], item_ref: LSSharedFileListItemRef) -> CFURLRef {
        let idx = (item_ref as i32 - 1) as usize;
        (&urls[idx]).into()
    }

    pub fn build(self) -> MockMacOsApi {
        let mut mock = self.mock;
        let favorites = self.state.favorites;

        mock.expect_ls_shared_file_list_create()
            .returning_st(move |_, _, _| 1 as LSSharedFileListRef);

        mock.expect_ls_shared_file_list_copy_snapshot()
            .returning_st(move |_, _| {
                let snapshot = favorites.snapshot.clone();
                let snapshot = snapshot.as_ref().as_ref().unwrap();
                snapshot.into()
            });

        let display_names = favorites.display_names.clone();
        mock.expect_ls_shared_file_list_item_copy_display_name()
            .returning_st(move |item| Self::get_display_name(&display_names, item));

        let urls = favorites.urls.clone();
        mock.expect_ls_shared_file_list_item_copy_resolved_url()
            .returning_st(move |item, _, _| Self::get_url(&urls, item));

        mock
    }
}
