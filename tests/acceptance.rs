mod common;

use common::ApiCallRecorder;
use favkit::sidebar::{Sidebar, SidebarItem};

#[test]
fn test_list_favorites() {
    // Given
    let items = vec![SidebarItem::applications(), SidebarItem::downloads()];
    let recorder = ApiCallRecorder::with_items(items.clone());
    let sidebar = Sidebar::with_api(recorder.clone());

    // When
    let result = sidebar.list_items().expect("Failed to list items");

    // Then
    assert_eq!(result, items);
}
