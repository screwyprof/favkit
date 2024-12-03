mod common;

use common::ApiCallRecorder;
use favkit::sidebar::{Sidebar, SidebarItem};

#[test]
fn it_lists_favorite_items() {
    // Given
    let items = vec![
        SidebarItem::applications(),
        SidebarItem::downloads(),
        SidebarItem::new("Projects", "/Users/happygopher/Projects"),
    ];
    let recorder = ApiCallRecorder::with_items(items.clone());
    let sidebar = Sidebar::with_api(recorder.clone());

    // When
    let result = sidebar.favorites().list_items();

    // Then
    assert_eq!(result, items);
}
