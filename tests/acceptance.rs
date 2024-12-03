mod common;

use common::ApiCallRecorder;
use favkit::sidebar::{Sidebar, SidebarItem};

#[test]
fn it_lists_favorite_items() {
    println!("Starting test");

    // Given
    let items = vec![
        SidebarItem::applications(),
        SidebarItem::downloads(),
        SidebarItem::new("Projects", "/Users/happygopher/Projects"),
    ];
    println!("Created test items");

    let recorder = ApiCallRecorder::with_items(items.clone());
    println!("Created recorder");

    let sidebar = Sidebar::with_api(recorder.clone());
    println!("Created sidebar");

    // When
    println!("About to list items");
    let result = sidebar.favorites().list_items();
    println!("Listed items");

    // Then
    assert_eq!(result, items);
}
