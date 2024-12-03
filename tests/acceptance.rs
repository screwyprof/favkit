mod common;

use common::ApiCallRecorder;
use favkit::sidebar::Sidebar;

#[test]
fn it_lists_favorite_items() {
    // Given
    let recorder = ApiCallRecorder::with_items(vec![
        (
            "Applications".to_string(),
            "file:///Applications".to_string(),
        ),
        (
            "Downloads".to_string(),
            "file:///Users/happygopher/Downloads".to_string(),
        ),
    ]);
    let sidebar = Sidebar::with_api(recorder.clone());

    // When
    let items = sidebar.list_items();

    // Then
    assert_eq!(items.len(), 2);
    assert_eq!(items[0].name(), "Applications");
    assert_eq!(items[1].name(), "Downloads");
}
