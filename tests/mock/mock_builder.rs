use core_foundation::{
    array::CFArrayRef,
    base::{CFAllocatorRef, CFTypeRef},
    error::CFErrorRef,
    string::CFStringRef,
    url::CFURLRef,
};
use core_services::{LSSharedFileListItemRef, LSSharedFileListRef};

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
    state: State,
}

// Initial state transitions
impl MockBuilder<Initial> {
    pub fn new() -> Self {
        Self { state: Initial }
    }

    pub fn with_null_favorites(self) -> MockBuilder<WithNullFavorites> {
        MockBuilder {
            state: WithNullFavorites,
        }
    }

    pub fn with_favorites(self) -> MockBuilder<WithFavorites> {
        MockBuilder {
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
            state: WithNullSnapshot {
                handle: self.state.handle,
            },
        }
    }

    pub fn with_items(self, favorites: Favorites) -> MockBuilder<WithItems> {
        MockBuilder {
            state: WithItems { favorites },
        }
    }
}

// Terminal state builders
impl MockBuilder<WithNullFavorites> {
    pub fn build(self) -> MockMacOsApi {
        let mut mock = MockMacOsApi::new();
        mock.expect_ls_shared_file_list_create()
            .returning_st(move |_, _, _| Handle::null().into());
        mock
    }
}

impl MockBuilder<WithNullSnapshot> {
    pub fn build(self) -> MockMacOsApi {
        let mut mock = MockMacOsApi::new();
        let handle = self.state.handle;

        mock.expect_ls_shared_file_list_create()
            .returning_st(move |_, _, _| handle.into());
        mock.expect_ls_shared_file_list_copy_snapshot()
            .returning_st(move |_, _| std::ptr::null_mut());

        mock
    }
}

impl MockBuilder<WithItems> {
    fn create_handle(_: CFAllocatorRef, _: CFStringRef, _: CFTypeRef) -> LSSharedFileListRef {
        Handle::new().into()
    }

    fn get_snapshot(
        cf_favorites: CFFavorites,
    ) -> impl Fn(LSSharedFileListRef, *mut u32) -> CFArrayRef {
        move |_, _| cf_favorites.get_snapshot()
    }

    fn get_display_name(
        cf_favorites: CFFavorites,
    ) -> impl Fn(LSSharedFileListItemRef) -> CFStringRef {
        move |item| cf_favorites.get_display_name(item)
    }

    fn get_url(
        cf_favorites: CFFavorites,
    ) -> impl Fn(LSSharedFileListItemRef, u32, *mut CFErrorRef) -> CFURLRef {
        move |item, _, _| cf_favorites.get_url(item)
    }

    pub fn build(self) -> MockMacOsApi {
        let mut mock = MockMacOsApi::new();
        let cf_favorites = CFFavorites::try_from(&self.state.favorites).unwrap();

        mock.expect_ls_shared_file_list_create()
            .returning_st(Self::create_handle);

        mock.expect_ls_shared_file_list_copy_snapshot()
            .returning_st(Self::get_snapshot(cf_favorites.clone()));

        mock.expect_ls_shared_file_list_item_copy_display_name()
            .returning_st(Self::get_display_name(cf_favorites.clone()));

        mock.expect_ls_shared_file_list_item_copy_resolved_url()
            .returning_st(Self::get_url(cf_favorites.clone()));

        mock
    }
}
