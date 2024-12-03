mod common;

use common::MockMacOsApi;
use favkit::sidebar::{MacOsLocation, Sidebar, SidebarItem};

#[test]
fn browsing_finder_favorites() {
    // Set up a typical macOS Finder sidebar
    let favorites = vec![
        SidebarItem::from(MacOsLocation::Applications),
        SidebarItem::from(MacOsLocation::Downloads),
        SidebarItem::from(MacOsLocation::Documents),
    ];
    let api = MockMacOsApi::with_favorites(favorites);
    let sidebar = Sidebar::with_api(&api);

    // When user lists favorites
    let items = sidebar.favorites().list_items();

    // Then they should see standard macOS locations
    assert!(
        items
            .iter()
            .any(|item| item.path.location() == &MacOsLocation::Applications),
        "Applications should be visible in Favorites"
    );
    assert!(
        items
            .iter()
            .any(|item| item.path.location() == &MacOsLocation::Downloads),
        "Downloads should be visible in Favorites"
    );
}

#[test]
fn viewing_custom_favorites() {
    // Given a Finder sidebar with custom folders
    let favorites = vec![
        SidebarItem::new(
            "Work Projects".to_string(),
            MacOsLocation::Custom("/Users/me/Work".into()).into(),
        ),
        SidebarItem::new(
            "Photos 2023".to_string(),
            MacOsLocation::Custom("~/Pictures/2023".into()).into(),
        ),
        SidebarItem::new(
            "Games".to_string(),
            MacOsLocation::Custom("/Applications/Games".into()).into(),
        ),
    ];
    let api = MockMacOsApi::with_favorites(favorites);
    let sidebar = Sidebar::with_api(&api);

    // When user lists favorites
    let items = sidebar.favorites().list_items();

    // Then they should see their custom folders with correct paths
    let work_project = items
        .iter()
        .find(|item| item.name == "Work Projects")
        .unwrap();
    assert!(matches!(
        work_project.path.location(),
        MacOsLocation::Custom(_)
    ));

    let photos = items
        .iter()
        .find(|item| item.name == "Photos 2023")
        .unwrap();
    assert!(matches!(photos.path.location(), MacOsLocation::Custom(_)));

    let games = items.iter().find(|item| item.name == "Games").unwrap();
    assert!(matches!(games.path.location(), MacOsLocation::Custom(_)));
}
