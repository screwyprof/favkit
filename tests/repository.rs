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
        Target::AirDrop("nwnode://domain-AirDrop".to_string()),
        Target::CustomPath(home_dir.join("Projects")),
        Target::Documents(home_dir.join("Documents")),
    ];

    let mock_api = MockMacOsApi::with_favorites(expected_favorites.clone());
    let repository = Repository::new(Box::new(mock_api.clone()));

    // When
    let sidebar = repository.load();

    // Then
    let loaded_favorites: Vec<_> = sidebar
        .favorites()
        .iter()
        .map(|item| item.target().clone())
        .collect();

    assert_eq!(loaded_favorites, expected_favorites);
    mock_api.verify_expected_calls();
}
