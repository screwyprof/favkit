use favkit::sidebar::{MacOsApi, MacOsLocation, MacOsPath, Sidebar};

#[test]
fn it_converts_from_str() {
    let applications: MacOsLocation = "/Applications".into();
    let downloads: MacOsLocation = "~/Downloads".into();
    let custom: MacOsLocation = "/some/custom/path".into();

    assert_eq!(applications, MacOsLocation::Applications);
    assert_eq!(downloads, MacOsLocation::Downloads);
    assert!(matches!(custom, MacOsLocation::Custom(_)));

    let applications: MacOsPath = "/Applications".into();
    let downloads: MacOsPath = "~/Downloads".into();
    let custom: MacOsPath = "/some/custom/path".into();

    assert_eq!(applications.location(), &MacOsLocation::Applications);
    assert_eq!(downloads.location(), &MacOsLocation::Downloads);
    assert!(matches!(custom.location(), MacOsLocation::Custom(_)));
}

struct MockMacOsApi {
    favorites: Vec<(String, MacOsPath)>,
}

impl MockMacOsApi {
    fn new() -> Self {
        Self {
            favorites: Vec::new(),
        }
    }

    fn with_favorites(mut self, favorites: Vec<(&str, MacOsLocation)>) -> Self {
        self.favorites = favorites
            .into_iter()
            .map(|(name, location)| (name.to_string(), location.into()))
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
        ("Applications", "/Applications".into()),
        ("Downloads", "~/Downloads".into()),
    ]);
    let sidebar = Sidebar::with_api(mock_api);

    let favorites = sidebar.favorites().list_items();

    assert_eq!(favorites.len(), 2);
    assert_eq!(favorites[0].name, "Applications");
    assert_eq!(favorites[0].path.location(), &MacOsLocation::Applications);
    assert_eq!(favorites[1].name, "Downloads");
    assert_eq!(favorites[1].path.location(), &MacOsLocation::Downloads);
}
