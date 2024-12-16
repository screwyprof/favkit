use crate::finder::Result;

pub trait FavoritesApi {
    fn list_items(&self) -> Result<Vec<String>>;
}
