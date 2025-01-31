pub mod favorites;

mod display_name;
mod errors;
mod sidebar;

pub use display_name::DisplayName;
pub use errors::{FinderError, Result};
use favorites::FavoritesApi;
pub use sidebar::{SidebarItem, Target};

use crate::system::{RealMacOsApi, api::MacOsApi, favorites::Favorites};

pub struct Finder {
    favorites: Favorites,
}

impl Finder {
    /// Creates a new Finder with the provided MacOS API implementation.
    pub fn new(api: impl MacOsApi + 'static) -> Self {
        Self {
            favorites: Favorites::new(api),
        }
    }

    pub fn get_favorites_list(&self) -> Result<Vec<SidebarItem>> {
        self.favorites.list_items()
    }
}

impl Default for Finder {
    fn default() -> Self {
        Self::new(RealMacOsApi::new())
    }
}
