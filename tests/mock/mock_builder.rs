use super::{
    cf_favorites::CFFavorites, display_name_ref::DisplayNameRef, favorites::Favorites,
    favorites_ref::FavoritesRef, mac_os_api::MockMacOsApi, snapshot::SnapshotRef, url_ref::UrlRef,
};

// States
pub struct Initial;
pub struct WithNullFavorites;
pub struct WithFavorites {
    favorites_ref: FavoritesRef,
}
pub struct WithNullSnapshot {
    favorites_ref: FavoritesRef,
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
                favorites_ref: FavoritesRef::new(),
            },
        }
    }
}

// WithFavorites state transitions
impl MockBuilder<WithFavorites> {
    pub fn with_null_snapshot(self) -> MockBuilder<WithNullSnapshot> {
        MockBuilder {
            state: WithNullSnapshot {
                favorites_ref: self.state.favorites_ref,
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
            .returning_st(move |_, _, _| FavoritesRef::null().into());
        mock
    }
}

impl MockBuilder<WithNullSnapshot> {
    pub fn build(self) -> MockMacOsApi {
        let mut mock = MockMacOsApi::new();

        mock.expect_ls_shared_file_list_create()
            .returning_st(move |_, _, _| self.state.favorites_ref.into());
        mock.expect_ls_shared_file_list_copy_snapshot()
            .returning_st(move |_, _| SnapshotRef::null().into());

        mock
    }
}

impl MockBuilder<WithItems> {
    pub fn build(self) -> MockMacOsApi {
        let mut mock = MockMacOsApi::new();
        let cf_favorites = CFFavorites::try_from(&self.state.favorites).unwrap();

        mock.expect_ls_shared_file_list_create()
            .returning_st(|_, _, _| FavoritesRef::new().into());

        mock.expect_ls_shared_file_list_copy_snapshot()
            .returning_st(move |_, _| SnapshotRef::from(&cf_favorites.snapshot).into());

        mock.expect_ls_shared_file_list_item_copy_display_name()
            .returning_st(move |item| {
                DisplayNameRef::from((&cf_favorites.display_names, item)).into()
            });

        mock.expect_ls_shared_file_list_item_copy_resolved_url()
            .returning_st(move |item, _, _| UrlRef::from((&cf_favorites.urls, item)).into());

        mock
    }
}
