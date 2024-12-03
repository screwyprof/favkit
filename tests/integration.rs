mod common;

use common::MockMacOsApi;
use favkit::sidebar::{MacOsLocation, Sidebar, SidebarItem};

#[test]
fn it_retrieves_favorites_from_macos() {
    // Set up SUT
    let api = MockMacOsApi::with_favorites(vec![
        SidebarItem::from(MacOsLocation::Applications),
        SidebarItem::from(MacOsLocation::Downloads),
    ]);
    let call_count = api.list_favorites_call_count();
    let sidebar = Sidebar::with_api(api);

    // Verify no API calls made yet
    assert_eq!(call_count, 0);

    // Perform action
    let items = sidebar.favorites().list_items();

    // Verify expected result
    assert_eq!(items.len(), 2);
    assert_eq!(items[0].name(), "Applications");
    assert_eq!(items[1].name(), "Downloads");
}

#[test]
fn it_handles_empty_favorites_list() {
    // Set up SUT
    let api = MockMacOsApi::with_favorites(vec![]);
    let call_count = api.list_favorites_call_count();
    let sidebar = Sidebar::with_api(api);

    // Verify no API calls made yet
    assert_eq!(call_count, 0);

    // Perform action
    let items = sidebar.favorites().list_items();

    // Verify expected result
    assert!(items.is_empty());
}
