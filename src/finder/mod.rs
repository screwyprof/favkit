mod favorites;

pub use favorites::Favorites;

use core_services::LSSharedFileListRef;

pub trait FavoritesApi {
    fn create_favorites_list(&self) -> LSSharedFileListRef;
}

pub struct FinderApi<'a, F: FavoritesApi = Favorites> {
    favorites: &'a F,
}

impl<'a, F: FavoritesApi> FinderApi<'a, F> {
    pub fn new(favorites: &'a F) -> Self {
        Self { favorites }
    }

    pub fn get_favorites_list(&self) -> Vec<String> {
        let _list_ref = self.favorites.create_favorites_list();
        
        // TODO: Convert LSSharedFileListRef to Vec<String>
        vec![String::from("/Applications")]
    }
}

impl Default for FinderApi<'_, Favorites> {
    fn default() -> Self {
        static DEFAULT: Favorites = Favorites;
        Self::new(&DEFAULT)
    }
}

impl<F: FavoritesApi> FavoritesApi for FinderApi<'_, F> {
    fn create_favorites_list(&self) -> LSSharedFileListRef {
        self.favorites.create_favorites_list()
    }
}
