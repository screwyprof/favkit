use core_foundation::{string::CFStringRef, url::CFURLRef};
use core_services::{LSSharedFileListItemRef, LSSharedFileListRef};
use favkit::system::favorites::{DisplayName, Url};

use super::{
    favorites::{CFFavorites, Favorites},
    mac_os_api::MockMacOsApi,
};

// Type-safe wrappers
#[derive(Clone, Copy)]
struct Handle(LSSharedFileListRef);

impl Handle {
    fn null() -> Self {
        Self(std::ptr::null_mut())
    }

    fn new() -> Self {
        Self(1 as LSSharedFileListRef)
    }
}

impl From<Handle> for LSSharedFileListRef {
    fn from(handle: Handle) -> Self {
        handle.0
    }
}

struct ItemIndex(usize);

impl ItemIndex {
    fn from_raw(raw: LSSharedFileListItemRef) -> Self {
        Self((raw as i32 - 1) as usize)
    }
}

// States
pub struct Initial;
pub struct WithNullFavorites;
pub struct WithFavorites {
    handle: Handle,
}
pub struct WithNullSnapshot {
    handle: Handle,
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
                handle: Handle::new(),
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
            .returning_st(move |_, _, _| Handle::null().into());
        mock
    }
}

impl MockBuilder<WithNullSnapshot> {
    pub fn build(self) -> MockMacOsApi {
        let mut mock = self.mock;
        let handle = self.state.handle;

        mock.expect_ls_shared_file_list_create()
            .returning_st(move |_, _, _| handle.into());
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
        let idx = ItemIndex::from_raw(item_ref);
        (&display_names[idx.0]).into()
    }

    fn get_url(urls: &[Url], item_ref: LSSharedFileListItemRef) -> CFURLRef {
        let idx = ItemIndex::from_raw(item_ref);
        (&urls[idx.0]).into()
    }

    pub fn build(self) -> MockMacOsApi {
        let mut mock = self.mock;
        let cf_favorites = CFFavorites::try_from(&self.state.favorites).unwrap();

        mock.expect_ls_shared_file_list_create()
            .returning_st(move |_, _, _| Handle::new().into());

        mock.expect_ls_shared_file_list_copy_snapshot()
            .returning_st(move |_, _| {
                let snapshot = cf_favorites.snapshot.clone();
                let snapshot = snapshot.as_ref().as_ref().unwrap();
                snapshot.into()
            });

        let display_names = cf_favorites.display_names.clone();
        mock.expect_ls_shared_file_list_item_copy_display_name()
            .returning_st(move |item| Self::get_display_name(&display_names, item));

        let urls = cf_favorites.urls.clone();
        mock.expect_ls_shared_file_list_item_copy_resolved_url()
            .returning_st(move |item, _, _| Self::get_url(&urls, item));

        mock
    }
}
