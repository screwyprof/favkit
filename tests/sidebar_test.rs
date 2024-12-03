use favkit::sidebar::{MacOsApi, MacOsPath, Sidebar};
use std::path::Path;

struct MockMacOsApi {
    favorites: Vec<(String, MacOsPath)>,
}

impl MockMacOsApi {
    fn new() -> Self {
        Self {
            favorites: Vec::new(),
        }
    }

    fn with_favorites<P: AsRef<Path>>(mut self, favorites: Vec<(&str, P)>) -> Self {
        self.favorites = favorites
            .into_iter()
            .map(|(name, path)| (name.to_string(), MacOsPath::new(path)))
            .collect();
        self
    }
}

impl MacOsApi for MockMacOsApi {
    fn list_favorite_items(&self) -> Vec<(String, MacOsPath)> {
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
        ("Applications", Path::new("/Applications")),
        ("Downloads", Path::new("~/Downloads")),
    ]);
    let sidebar = Sidebar::with_api(mock_api);

    let favorites = sidebar.favorites().list_items();

    assert_eq!(favorites.len(), 2);
    assert_eq!(favorites[0].name, "Applications");
    assert_eq!(favorites[0].path.as_path(), Path::new("/Applications"));
    assert_eq!(favorites[1].name, "Downloads");
    assert_eq!(favorites[1].path.as_path(), Path::new("~/Downloads"));
}
