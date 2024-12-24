#[derive(Clone)]
pub struct FavoriteItem {
    pub name: Option<String>,
    pub path: String,
}

#[derive(Default)]
pub struct FavoritesBuilder {
    items: Vec<FavoriteItem>,
}

impl FavoritesBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_item(mut self, name: Option<&str>, path: &str) -> Self {
        self.items.push(FavoriteItem {
            name: name.map(String::from),
            path: String::from(path),
        });
        self
    }

    pub fn build(self) -> Vec<FavoriteItem> {
        self.items
    }
}
