use favkit::sidebar::{MacOsApi, Sidebar};

struct MockMacOsApi {
    favorites: Vec<(String, String)>,
}

impl MockMacOsApi {
    fn new() -> Self {
        Self {
            favorites: Vec::new(),
        }
    }

    fn with_favorites(mut self, favorites: Vec<(&str, &str)>) -> Self {
        self.favorites = favorites
            .into_iter()
            .map(|(name, path)| (name.to_string(), path.to_string()))
            .collect();
        self
    }
}

impl MacOsApi for MockMacOsApi {
    fn list_favorite_items(&self) -> Vec<(String, String)> {
        self.favorites.clone()
    }
}

#[test]
fn it_returns_empty_favorites_by_default() {
    let sidebar = Sidebar::with_api(MockMacOsApi::new());
    let favorites = sidebar.favorites().list_items();

    assert!(favorites.is_empty());
}

#[test]
fn it_lists_favorite_items() {
    let mock_api = MockMacOsApi::new().with_favorites(vec![
        ("Applications", "/Applications"),
        ("Downloads", "~/Downloads"),
    ]);
    let sidebar = Sidebar::with_api(mock_api);

    let favorites = sidebar.favorites().list_items();

    assert_eq!(favorites.len(), 2);
    assert_eq!(favorites[0].name, "Applications");
    assert_eq!(favorites[1].name, "Downloads");
}
