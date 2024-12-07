use favkit::finder::{repository::Repository, target::{Target, TargetLocation}, sidebar_item::SidebarItem};
use favkit::errors::FinderError;

use crate::test_utils::{ApiCall, ApiCallRecorder};

mod test_utils;

#[test]
fn test_load_empty_favorites() {
    // Given
    let api = ApiCallRecorder::default();
    let repository = Repository::new(Box::new(api.clone()));

    // When
    let actual_items = repository.load().unwrap();

    // Then
    assert_eq!(actual_items.favorites().len(), 0);
    api.verify_calls(&[
        ApiCall::CreateFavoritesList,
        ApiCall::GetFavoritesSnapshot(1 as _),
    ]);
}

#[test]
fn test_load_favorites() {
    // Given
    let target = Target::Home(TargetLocation::Path("/Users/test/Documents".into()));
    let expected_items = vec![
        SidebarItem::with_display_name(target.clone(), "Test Home".to_string()),
    ];

    let api = ApiCallRecorder::with_items(expected_items.clone());
    let repository = Repository::new(Box::new(api.clone()));

    // When
    let actual_items = repository.load().unwrap();

    // Then
    assert_eq!(actual_items.favorites(), &expected_items);
    api.verify_calls(&[
        ApiCall::CreateFavoritesList,
        ApiCall::GetFavoritesSnapshot(1 as _),
        ApiCall::GetItemUrl(api.get_test_item(0)),
        ApiCall::GetItemDisplayName(api.get_test_item(0)),
    ]);
}

#[test]
fn test_load_multiple_favorites() {
    // Given
    let target1 = Target::Home(TargetLocation::Path("/Users/test/Documents".into()));
    let target2 = Target::Downloads(TargetLocation::Path("/Users/test/Downloads".into()));
    let expected_items = vec![
        SidebarItem::with_display_name(target1.clone(), "Documents".to_string()),
        SidebarItem::with_display_name(target2.clone(), "Downloads".to_string()),
    ];

    let api = ApiCallRecorder::with_items(expected_items.clone());
    let repository = Repository::new(Box::new(api.clone()));

    // When
    let actual_items = repository.load().unwrap();

    // Then
    assert_eq!(actual_items.favorites(), &expected_items);
    api.verify_calls(&[
        ApiCall::CreateFavoritesList,
        ApiCall::GetFavoritesSnapshot(1 as _),
        ApiCall::GetItemUrl(api.get_test_item(0)),
        ApiCall::GetItemDisplayName(api.get_test_item(0)),
        ApiCall::GetItemUrl(api.get_test_item(1)),
        ApiCall::GetItemDisplayName(api.get_test_item(1)),
    ]);
}

#[test]
fn test_load_favorites_with_invalid_url() {
    // Given
    let invalid_target = Target::Home(TargetLocation::Path("/invalid/path".into()));
    let api = ApiCallRecorder::with_items(vec![
        SidebarItem::with_display_name(invalid_target, "Invalid".to_string()),
    ]);
    let repository = Repository::new(Box::new(api.clone()));

    // When
    let result = repository.load();

    // Then
    assert!(matches!(result, Err(FinderError::InvalidPath { .. })));
    api.verify_calls(&[
        ApiCall::CreateFavoritesList,
        ApiCall::GetFavoritesSnapshot(1 as _),
        ApiCall::GetItemUrl(api.get_test_item(0)),
    ]);
}

#[test]
fn test_load_favorites_with_unsupported_url() {
    // Given
    let unsupported_target = Target::Home(TargetLocation::Url("unsupported://path".into()));
    let api = ApiCallRecorder::with_items(vec![
        SidebarItem::with_display_name(unsupported_target, "Unsupported".to_string()),
    ]);
    let repository = Repository::new(Box::new(api.clone()));

    // When
    let result = repository.load();

    // Then
    assert!(matches!(result, Err(FinderError::UnsupportedTarget(_))));
    api.verify_calls(&[
        ApiCall::CreateFavoritesList,
        ApiCall::GetFavoritesSnapshot(1 as _),
        ApiCall::GetItemUrl(api.get_test_item(0)),
    ]);
}
