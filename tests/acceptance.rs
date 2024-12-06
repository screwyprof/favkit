use favkit::{
    errors::Result,
    finder::{
        Finder,
        SidebarRepository,
        Target,
    },
};
use favkit::macos::test_utils::MockMacOsApi;

#[test]
fn it_shows_favorites() -> Result<()> {
    // Given
    let expected = vec![Target::home(), Target::airdrop()];
    
    let api = MockMacOsApi::with_favorites(expected.clone());
    let repo = SidebarRepository::new(api);
    let finder = Finder::new(repo);

    // When
    let favorites = finder.sidebar().favorites();

    // Then
    assert_eq!(favorites, expected);
    Ok(())
}
