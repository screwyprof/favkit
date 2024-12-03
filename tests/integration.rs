mod common;

use common::MockMacOsApi;
use favkit::sidebar::Sidebar;

#[test]
fn it_retrieves_favorites_from_macos() {
    // Set up SUT
    let api = MockMacOsApi::with_favorites(vec![
        ("Applications".to_string(), "/Applications".into()),
        ("Downloads".to_string(), "~/Downloads".into()),
    ]);
    let sidebar = Sidebar::with_api(&api);

    // Verify no API calls made yet
    assert_eq!(api.list_favorites_call_count(), 0);

    // Perform action
    let items = sidebar.favorites().list_items();

    // Verify expected result
    assert_eq!(items.len(), 2);
    assert_eq!(items[0].name, "Applications");
    assert_eq!(items[1].name, "Downloads");

    // Verify API was called exactly once
    assert_eq!(api.list_favorites_call_count(), 1);
}

#[test]
fn it_handles_empty_favorites_list() {
    // Set up SUT
    let api = MockMacOsApi::with_favorites(vec![]);
    let sidebar = Sidebar::with_api(&api);

    // Verify no API calls made yet
    assert_eq!(api.list_favorites_call_count(), 0);

    // Perform action
    let items = sidebar.favorites().list_items();

    // Verify expected result
    assert!(items.is_empty());

    // Verify API was called exactly once
    assert_eq!(api.list_favorites_call_count(), 1);
}
