use favkit::{Repository, Target};
use test_utils::{MacOsApiCall, MockMacOsApi};

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

    // Verify macOS API calls
    let calls = mock_api.calls();
    assert_eq!(calls[0], MacOsApiCall::GetFavoritesList);
    assert_eq!(calls[1], MacOsApiCall::GetFavoritesSnapshot);
    
    // For each favorite, we should have a GetItemUrl and UrlToTarget call
    for i in 1..=expected_favorites.len() {
        let item_ref = (i as *mut std::ffi::c_void) as core_services::LSSharedFileListItemRef;
        let url_ref = item_ref as core_foundation::url::CFURLRef;
        
        let expected_get_url = MacOsApiCall::GetItemUrl(item_ref);
        let expected_to_target = MacOsApiCall::UrlToTarget(url_ref);
        
        assert!(calls.contains(&expected_get_url), "Missing GetItemUrl call for item {}", i);
        assert!(calls.contains(&expected_to_target), "Missing UrlToTarget call for item {}", i);
    }
}
