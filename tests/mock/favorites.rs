#[derive(Debug)]
pub struct FavoriteItem {
    pub(crate) name: Option<String>,
    pub(crate) path: String,
}

#[derive(Debug, Default)]
pub struct Favorites {
    items: Vec<FavoriteItem>,
}

impl Favorites {
    pub fn new(items: Vec<FavoriteItem>) -> Self {
        Self { items }
    }

    pub fn items(&self) -> &[FavoriteItem] {
        &self.items
    }
}

/// Builder for creating test data
#[derive(Default)]
pub struct FavoritesBuilder {
    items: Vec<(Option<&'static str>, &'static str)>,
}

impl FavoritesBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_item(mut self, name: Option<&'static str>, path: &'static str) -> Self {
        self.items.push((name, path));
        self
    }

    pub fn build(self) -> Favorites {
        let items = self
            .items
            .into_iter()
            .map(|(name, path)| FavoriteItem {
                name: name.map(String::from),
                path: path.to_string(),
            })
            .collect();
        Favorites::new(items)
    }
}
