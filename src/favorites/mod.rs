use crate::finder::{Result, SidebarItem};

/// Provides access to Finder favorites
pub trait FavoritesApi {
    /// Lists all items in the Finder favorites sidebar
    ///
    /// Returns a vector of sidebar items or an error if favorites cannot be accessed
    #[must_use = "this Result contains Finder favorites or an error that should be handled"]
    fn list_items(&self) -> Result<Vec<SidebarItem>>;
}
