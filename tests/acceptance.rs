use core_foundation::{array::CFArray, url::CFURL, base::TCFType};
use core_services::{LSSharedFileListRef, LSSharedFileListItemRef};
use favkit::{
    errors::{Result, FinderError},
    finder::{
        Finder,
        SidebarRepository,
        Target,
        SidebarItem,
    },
};
use favkit::macos::test_utils::MockMacOsApi;

#[test]
fn loads_home_and_airdrop() -> Result<()> {
    // Given
    let home_url = CFURL::from_path(Target::home().path(), true)
        .ok_or(FinderError::invalid_path(Target::home().path()))?;
    let airdrop_url = CFURL::from_path(Target::airdrop().path(), true)
        .ok_or(FinderError::invalid_path(Target::airdrop().path()))?;

    let item1 = 1 as LSSharedFileListItemRef;
    let item2 = 2 as LSSharedFileListItemRef;
    let items = vec![item1, item2];
    let items_array = CFArray::from_copyable(&items);

    let api = MockMacOsApi::new()
        .with_favorites_list(1 as LSSharedFileListRef)
        .with_favorites_snapshot(items_array)
        .with_item_url(home_url.as_concrete_TypeRef(), item1)
        .with_item_url(airdrop_url.as_concrete_TypeRef(), item2);

    let repo = SidebarRepository::new(api);
    let finder = Finder::new(repo);

    // Then
    let favorites = finder.sidebar().favorites();
    let home_item = SidebarItem::from(Target::home());
    let airdrop_item = SidebarItem::from(Target::airdrop());
    assert!(favorites.iter().any(|item| item == &home_item));
    assert!(favorites.iter().any(|item| item == &airdrop_item));
    Ok(())
}
