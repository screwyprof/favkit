use anyhow::Result;
use favkit::{Finder, Repository};

#[test]
fn test_end_to_end_shows_favorites() -> Result<()> {
    // Given
    let api = Box::new(favkit::finder::macos_impl::SystemMacOsApi::new());
    let repository = Repository::new(api);
    let sidebar = repository.load();
    let finder = Finder::new(sidebar);

    // When
    let favorites = finder.sidebar().favorites();

    // Then
    // Just verify we can load favorites without errors
    // We don't want to make assumptions about what's in the user's Finder
    assert!(!favorites.is_empty());
    Ok(())
}
