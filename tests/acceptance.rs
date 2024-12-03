mod common;

use common::MockMacOsApi;
use favkit::sidebar::{Sidebar, SidebarItem};

#[test]
fn browsing_finder_favorites() {
    // Given a Finder sidebar with both standard and custom locations
    let favorites = vec![
        // Standard locations
        SidebarItem::applications(),
        SidebarItem::downloads(),
        SidebarItem::documents(),
        // Custom folders
        SidebarItem::new("Work Projects", "/Users/me/Work"),
        SidebarItem::new("Photos 2023", "~/Pictures/2023"),
        SidebarItem::new("Games", "/Applications/Games"),
    ];
    let api = MockMacOsApi::with_favorites(favorites);
    let sidebar = Sidebar::with_api(&api);

    // When listing favorites
    let items: Vec<_> = sidebar.favorites().iter().collect();

    // Then both standard and custom locations are present
    assert_eq!(items.len(), 6);

    // Standard locations
    assert!(items.iter().any(|item| item.name() == "Applications"));
    assert!(items.iter().any(|item| item.name() == "Downloads"));
    assert!(items.iter().any(|item| item.name() == "Documents"));

    // Custom folders
    assert!(items.iter().any(|item| item.name() == "Work Projects"));
    assert!(items.iter().any(|item| item.name() == "Photos 2023"));
    assert!(items.iter().any(|item| item.name() == "Games"));
}

#[test]
fn creating_favorites_with_typed_paths() {
    // Given a Finder sidebar with items created using different API patterns
    let favorites = vec![
        // Pattern 1: Standard locations
        SidebarItem::applications(),
        SidebarItem::downloads(),
        // Pattern 2: Custom paths
        SidebarItem::new("Projects", "/Users/me/Projects"),
        SidebarItem::new("Development", "/Users/me/Development"),
    ];
    let api = MockMacOsApi::with_favorites(favorites);
    let sidebar = Sidebar::with_api(&api);

    // When listing favorites
    let items: Vec<_> = sidebar.favorites().iter().collect();

    // Then all items are present
    assert_eq!(items.len(), 4);
    assert!(items.iter().any(|item| item.name() == "Applications"));
    assert!(items.iter().any(|item| item.name() == "Downloads"));
    assert!(items.iter().any(|item| item.name() == "Projects"));
    assert!(items.iter().any(|item| item.name() == "Development"));
}
