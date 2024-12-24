use super::{
    cf_favorites::{CFFavorites, ItemIndex},
    favorites::Favorites,
    handle::Handle,
    mac_os_api::MockMacOsApi,
};

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
    pub fn build(self) -> MockMacOsApi {
        let mut mock = MockMacOsApi::new();
        let cf_favorites = CFFavorites::try_from(&self.state.favorites).unwrap();

        mock.expect_ls_shared_file_list_create()
            .returning_st(|_, _, _| Handle::new().into());

        mock.expect_ls_shared_file_list_copy_snapshot()
            .returning_st({
                let snapshot = cf_favorites.snapshot.clone();
                move |_, _| {
                    snapshot
                        .as_ref()
                        .as_ref()
                        .expect("Snapshot must exist")
                        .into()
                }
            });

        mock.expect_ls_shared_file_list_item_copy_display_name()
            .returning_st({
                let display_names = cf_favorites.display_names.clone();
                move |item| {
                    let idx = ItemIndex::from(item);
                    (&display_names[idx.index]).into()
                }
            });

        mock.expect_ls_shared_file_list_item_copy_resolved_url()
            .returning_st({
                let urls = cf_favorites.urls.clone();
                move |item, _, _| {
                    let idx = ItemIndex::from(item);
                    (&urls[idx.index]).into()
                }
            });

        mock
    }
}
