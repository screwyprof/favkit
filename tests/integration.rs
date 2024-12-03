mod common;

use common::{ApiCall, ApiCallRecorder};
use core_services::{LSSharedFileListItemRef, LSSharedFileListRef};
use favkit::sidebar::{Sidebar, SidebarItem};

#[test]
fn it_lists_favorite_items() {
    // Given
    let recorder = ApiCallRecorder::with_items(vec![
        SidebarItem::applications(),
        SidebarItem::downloads(),
        SidebarItem::new("Projects", "/Users/happygopher/Projects"),
    ]);
    let sidebar = Sidebar::with_api(recorder.clone());

    // When
    let items = sidebar.favorites().list_items();

    // Then
    assert_eq!(
        items,
        vec![
            SidebarItem::applications(),
            SidebarItem::downloads(),
            SidebarItem::new("Projects", "/Users/happygopher/Projects"),
        ]
    );

    // And verify API calls
    recorder.verify_calls(&[
        ApiCall::CreateFavoritesList,
        ApiCall::CopySnapshot(1 as LSSharedFileListRef),
        ApiCall::CopyDisplayName(1 as LSSharedFileListItemRef),
        ApiCall::CopyResolvedUrl(1 as LSSharedFileListItemRef),
        ApiCall::CopyDisplayName(2 as LSSharedFileListItemRef),
        ApiCall::CopyResolvedUrl(2 as LSSharedFileListItemRef),
        ApiCall::CopyDisplayName(3 as LSSharedFileListItemRef),
        ApiCall::CopyResolvedUrl(3 as LSSharedFileListItemRef),
    ]);
}
