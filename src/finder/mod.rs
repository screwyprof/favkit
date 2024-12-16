mod errors;

use crate::favorites::FavoritesApi;
pub use errors::{FinderError, ListErrorKind, Result};

pub struct FinderApi<'a> {
    favorites: &'a dyn FavoritesApi,
}

impl<'a> FinderApi<'a> {
    pub fn new(favorites: &'a dyn FavoritesApi) -> Self {
        Self { favorites }
    }

    pub fn get_favorites_list(&self) -> Result<Vec<String>> {
        self.favorites.list_items()
    }
}
