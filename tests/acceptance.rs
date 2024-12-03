mod common;

use common::MockMacOsApi;
use favkit::sidebar::{MacOsLocation, MacOsPath, Sidebar, SidebarItem};

#[test]
fn browsing_finder_favorites() {
    // Given a Finder sidebar with both standard and custom locations
    let favorites = vec![
        // Standard locations
        SidebarItem::from(MacOsLocation::Applications),
        SidebarItem::from(MacOsLocation::Downloads),
        SidebarItem::from(MacOsLocation::Documents),
        // Custom folders
        ("Work Projects", "/Users/me/Work").into(),
        ("Photos 2023", "~/Pictures/2023").into(),
        ("Games", "/Applications/Games").into(),
    ];
    let api = MockMacOsApi::with_favorites(favorites);
    let sidebar = Sidebar::with_api(&api);

    // When listing favorites
    let items = sidebar.favorites().list_items();

    // Then both standard and custom locations are present
    assert_eq!(items.len(), 6);

    // Standard locations
    assert!(items.iter().any(|item| item.name == "Applications"));
    assert!(items.iter().any(|item| item.name == "Downloads"));
    assert!(items.iter().any(|item| item.name == "Documents"));

    // Custom folders
    assert!(items.iter().any(|item| item.name == "Work Projects"));
    assert!(items.iter().any(|item| item.name == "Photos 2023"));
    assert!(items.iter().any(|item| item.name == "Games"));
}

#[test]
fn creating_favorites_with_typed_paths() {
    // Given a Finder sidebar with items created using different API patterns
    let favorites = vec![
        // Pattern 1: Using MacOsLocation enum directly for well-known locations
        SidebarItem::from(MacOsLocation::Applications),
        SidebarItem::from(MacOsLocation::Downloads),
        // Pattern 2: Using MacOsLocation::Custom for custom paths
        SidebarItem::new(
            "Projects",
            MacOsLocation::Custom("/Users/me/Projects".into()),
        ),
        // Pattern 3: Converting string path to MacOsPath first
        SidebarItem::new("Development", MacOsPath::from("/Users/me/Development")),
    ];
    let api = MockMacOsApi::with_favorites(favorites);
    let sidebar = Sidebar::with_api(&api);

    // When listing favorites
    let items = sidebar.favorites().list_items();

    // Then all items are present
    assert_eq!(items.len(), 4);
    assert!(items.iter().any(|item| item.name == "Applications"));
    assert!(items.iter().any(|item| item.name == "Downloads"));
    assert!(items.iter().any(|item| item.name == "Projects"));
    assert!(items.iter().any(|item| item.name == "Development"));
}
