mod errors;

use crate::favorites::FavoritesApi;
pub use errors::{FinderError, Result};

pub struct FinderApi<'a> {
    favorites: &'a dyn FavoritesApi,
}

impl<'a> FinderApi<'a> {
    pub fn new(favorites: &'a dyn FavoritesApi) -> Self {
        Self { favorites }
    }

    pub fn get_favorites_list(&self) -> Result<Vec<Option<String>>> {
        self.favorites.list_items()
    }
}
