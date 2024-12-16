use crate::finder::{Result, SidebarItem};

pub trait FavoritesApi {
    fn list_items(&self) -> Result<Vec<SidebarItem>>;
}
