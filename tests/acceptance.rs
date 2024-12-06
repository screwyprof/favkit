use favkit::{
    errors::Result,
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
    let api = MockMacOsApi::with_favorites(vec![Target::home(), Target::airdrop()]);
    let repo = SidebarRepository::new(api);
    let finder = Finder::new(repo);

    // When
    let favorites = finder.sidebar().favorites();

    // Then
    let home_item = SidebarItem::from(Target::home());
    let airdrop_item = SidebarItem::from(Target::airdrop());
    assert!(favorites.iter().any(|item| item == &home_item));
    assert!(favorites.iter().any(|item| item == &airdrop_item));

    Ok(())
}
