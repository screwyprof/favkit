mod common;

use common::{ApiCall, ApiCallRecorder};
use core_services::{LSSharedFileListItemRef, LSSharedFileListRef};
use favkit::sidebar::{Sidebar, SidebarItem};

#[test]
fn test_list_items() {
    // Given
    let expected = vec![SidebarItem::applications(), SidebarItem::downloads()];
    let recorder = ApiCallRecorder::with_items(expected.clone());
    let sidebar = Sidebar::with_api(recorder.clone());

    // When
    let items = sidebar.list_items().expect("Failed to list items");

    // Then
    assert_eq!(expected, items);

    // And verify API calls - one pair of calls (display name + URL) for each item
    recorder.verify_calls(&[
        ApiCall::CreateFavoritesList,
        ApiCall::GetFavoritesSnapshot(1 as LSSharedFileListRef),
        ApiCall::GetItemDisplayName(1 as LSSharedFileListItemRef),
        ApiCall::GetItemUrl(1 as LSSharedFileListItemRef),
        ApiCall::GetItemDisplayName(2 as LSSharedFileListItemRef),
        ApiCall::GetItemUrl(2 as LSSharedFileListItemRef),
    ]);
}
