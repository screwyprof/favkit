use favkit::{Repository, Target};
use test_utils::MockMacOsApi;

mod test_utils;

#[test]
fn test_repository_loads_favorites_from_macos_api() {
    // Given
    let home_dir = dirs::home_dir().unwrap();
    let expected_favorites = vec![
        Target::Home(home_dir.clone()),
        Target::Desktop(home_dir.join("Desktop")),
    ];

    let mock_api = MockMacOsApi::with_favorites(expected_favorites.clone(), home_dir);
    let repository = Repository::new(Box::new(mock_api));

    // When
    let sidebar = repository.load();

    // Then
    let loaded_favorites: Vec<_> = sidebar
        .favorites()
        .iter()
        .map(|item| item.target().clone())
        .collect();

    assert_eq!(loaded_favorites, expected_favorites);
}
