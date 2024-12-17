mod display_name;
mod errors;
mod sidebar;

use crate::favorites::FavoritesApi;
pub use display_name::DisplayName;
pub use errors::{FinderError, Result};
pub use sidebar::{SidebarItem, Target};

pub struct FinderApi<'a> {
    favorites: &'a dyn FavoritesApi,
}

impl<'a> FinderApi<'a> {
    pub fn new(favorites: &'a dyn FavoritesApi) -> Self {
        Self { favorites }
    }

    pub fn get_favorites_list(&self) -> Result<Vec<SidebarItem>> {
        self.favorites.list_items()
    }
}
