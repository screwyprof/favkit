mod common;

use common::MockMacOsApi;
use favkit::sidebar::{MacOsApi, MacOsLocation};

#[test]
fn it_lists_favorite_items() {
    let favorites = vec![
        ("Applications".to_string(), "/Applications".into()),
        ("Downloads".to_string(), "~/Downloads".into()),
    ];
    let api = MockMacOsApi::with_favorites(favorites);
    let items = api.list_favorite_items();

    // Test that we get the expected number of items
    assert_eq!(items.len(), 2, "Should have 2 favorite items");

    // Test that we correctly parse system paths
    let has_applications = items
        .iter()
        .any(|(_, path)| path.location() == &MacOsLocation::Applications);
    assert!(has_applications, "Should find Applications");

    let has_downloads = items
        .iter()
        .any(|(_, path)| path.location() == &MacOsLocation::Downloads);
    assert!(has_downloads, "Should find Downloads");
}

#[test]
fn it_handles_special_paths() {
    let favorites = vec![
        ("Downloads".to_string(), "~/Downloads".into()),
        ("Documents".to_string(), "~/Documents".into()),
        ("Custom Folder".to_string(), "/Users/test/Custom".into()),
    ];
    let api = MockMacOsApi::with_favorites(favorites);
    let items = api.list_favorite_items();

    // Test that we correctly handle home directory paths
    let home_paths: Vec<_> = items
        .iter()
        .filter(|(_, path)| {
            matches!(
                path.location(),
                MacOsLocation::Downloads | MacOsLocation::Documents
            )
        })
        .collect();

    assert_eq!(home_paths.len(), 2, "Should find 2 home directory paths");

    // Test custom paths
    let custom_paths: Vec<_> = items
        .iter()
        .filter(|(_, path)| matches!(path.location(), MacOsLocation::Custom(_)))
        .collect();

    assert_eq!(custom_paths.len(), 1, "Should find 1 custom path");
    assert_eq!(custom_paths[0].0, "Custom Folder");
}
