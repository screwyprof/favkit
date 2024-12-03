mod common;

use common::MockMacOsApi;
use favkit::sidebar::{MacOsLocation, Sidebar, SidebarItem};

#[test]
fn it_retrieves_favorites_from_macos() {
    // Arrange
    let expected_favorites = vec![
        SidebarItem::from(MacOsLocation::Applications),
        SidebarItem::from(MacOsLocation::Downloads),
    ];
    let api = MockMacOsApi::with_favorites(expected_favorites.clone());
    let initial_call_count = api.list_favorites_call_count();
    let sidebar = Sidebar::with_api(api);

    // Assert no API calls during setup
    assert_eq!(
        initial_call_count, 0,
        "No API calls should be made during setup"
    );

    // Act
    let retrieved_favorites = sidebar.favorites().list_items();

    // Assert
    assert_eq!(
        retrieved_favorites, expected_favorites,
        "Retrieved favorites should match expected exactly"
    );
}
