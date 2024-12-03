mod common;

use common::MockMacOsApi;
use favkit::sidebar::{Sidebar, SidebarItem};

#[test]
fn browsing_finder_favorites() {
    // Given a Finder sidebar with both standard and custom locations
    let favorites = vec![
        // Standard locations with well-known paths
        SidebarItem::applications(),
        SidebarItem::downloads(),
        SidebarItem::documents(),
        // Custom folders with explicit paths
        SidebarItem::new("Work Projects", "/Users/me/Work"),
        SidebarItem::new("Photos 2023", "~/Pictures/2023"),
        SidebarItem::new("Games", "/Applications/Games"),
    ];

    let api = MockMacOsApi::with_favorites(favorites.clone());
    let sidebar = Sidebar::with_api(api);

    // When listing favorites
    let items: Vec<_> = sidebar.favorites().iter().collect();

    // Then all items match exactly (both names and paths)
    assert_eq!(items, favorites);
}
