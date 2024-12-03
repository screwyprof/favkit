mod common;

use crate::common::{ApiCall, ApiCallRecorder};
use core_services::{LSSharedFileListItemRef, LSSharedFileListRef};
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

    // And verify API calls
    recorder.verify_calls(&[
        ApiCall::CreateFavoritesList,
        ApiCall::CopySnapshot(1 as LSSharedFileListRef),
        ApiCall::CopyDisplayName(1 as LSSharedFileListItemRef),
        ApiCall::CopyResolvedUrl(1 as LSSharedFileListItemRef),
        ApiCall::CopyDisplayName(2 as LSSharedFileListItemRef),
        ApiCall::CopyResolvedUrl(2 as LSSharedFileListItemRef),
    ]);
}
