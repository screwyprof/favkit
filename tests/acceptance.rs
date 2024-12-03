mod common;

use common::MockMacOsApi;
use favkit::sidebar::{MacOsLocation, MacOsPath, Sidebar};

#[test]
fn browsing_finder_favorites() {
    // Set up a typical macOS Finder sidebar
    let favorites = vec![
        ("Applications".to_string(), "/Applications".into()),
        ("Downloads".to_string(), "~/Downloads".into()),
        ("Documents".to_string(), "~/Documents".into()),
    ];
    let api = MockMacOsApi::with_favorites(favorites);
    let sidebar = Sidebar::with_api(api);

    // When user lists favorites
    let items = sidebar.favorites().list_items();

    // Then they should see standard macOS locations
    assert!(
        items.iter().any(|item| item.name == "Applications"),
        "Applications should be visible in Favorites"
    );
    assert!(
        items.iter().any(|item| item.name == "Downloads"),
        "Downloads should be visible in Favorites"
    );
}

#[test]
fn working_with_macos_paths() {
    // When user works with different types of paths
    let applications: MacOsPath = "/Applications".into();
    let downloads: MacOsPath = "~/Downloads".into();
    let custom: MacOsPath = "/Users/custom/path".into();

    // Then paths should be correctly categorized
    assert!(matches!(
        applications.location(),
        MacOsLocation::Applications
    ));
    assert!(matches!(downloads.location(), MacOsLocation::Downloads));
    assert!(matches!(custom.location(), MacOsLocation::Custom(_)));

    // And paths should be comparable
    let apps_from_location = MacOsPath::from(MacOsLocation::Applications);
    assert_eq!(applications, apps_from_location);
    assert_ne!(applications, downloads);
}
