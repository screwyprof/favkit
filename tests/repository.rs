use std::path::PathBuf;

use favkit::{
    finder::{
        repository::Repository,
        sidebar::{item::SidebarItem, target::Target},
    },
};

use crate::test_utils::{ApiCall, ApiCallRecorder};

mod test_utils;

#[test]
fn test_get_favorites() {
    let api = ApiCallRecorder::new();
    let repository = Repository::new(Box::new(api.clone()));

    let _ = repository.load();

    let expected_calls = vec![
        ApiCall::CreateFavoritesList,
        ApiCall::GetFavoritesSnapshot,
    ];
    assert_eq!(api.get_calls(), expected_calls);
}

#[test]
fn test_get_favorites_with_items() {
    let items = [
        SidebarItem::new(Target::Documents(PathBuf::from("/Users/current/Documents")), "Documents"),
        SidebarItem::new(Target::Downloads(PathBuf::from("/Users/current/Downloads")), "Downloads"),
    ];

    println!("Test items: {:?}", items);

    let api = ApiCallRecorder::with_items(
        items
            .iter()
            .map(|item: &SidebarItem| (
                format!("file://{}", item.target()),
                item.display_name().to_string(),
            ))
            .collect(),
    );
    let repository = Repository::new(Box::new(api.clone()));

    let favorites = repository.load().unwrap();
    println!("Loaded favorites: {:?}", favorites);

    assert_eq!(favorites.len(), 2);
    assert_eq!(favorites[0].display_name(), "Documents");
    assert_eq!(favorites[1].display_name(), "Downloads");

    let expected_calls = vec![
        ApiCall::CreateFavoritesList,
        ApiCall::GetFavoritesSnapshot,
        ApiCall::GetItemDisplayName(0),
        ApiCall::GetItemUrl(0),
        ApiCall::GetItemDisplayName(1),
        ApiCall::GetItemUrl(1),
    ];
    let actual_calls = api.get_calls();
    println!("Expected calls: {:?}", expected_calls);
    println!("Actual calls: {:?}", actual_calls);
    assert_eq!(actual_calls, expected_calls);
}

#[test]
fn test_get_favorites_with_null_names() {
    let items = [
        SidebarItem::new(Target::Documents(PathBuf::from("/Users/current/Documents")), "Documents"),
        SidebarItem::new(Target::Downloads(PathBuf::from("/Users/current/Downloads")), "Downloads"),
    ];

    let api = ApiCallRecorder::with_items_without_names(
        items
            .iter()
            .map(|item: &SidebarItem| (
                format!("file://{}", item.target()),
                item.display_name().to_string(),
            ))
            .collect(),
        vec![1],
    );
    let repository = Repository::new(Box::new(api.clone()));

    let favorites = repository.load().unwrap();

    assert_eq!(favorites.len(), 1);
    assert_eq!(favorites[0].display_name(), "Documents");

    let expected_calls = vec![
        ApiCall::CreateFavoritesList,
        ApiCall::GetFavoritesSnapshot,
        ApiCall::GetItemDisplayName(0),
        ApiCall::GetItemUrl(0),
        ApiCall::GetItemDisplayName(1),
    ];
    assert_eq!(api.get_calls(), expected_calls);
}

#[test]
fn test_get_favorites_with_multiple_items() {
    let items = [
        SidebarItem::new(Target::Documents(PathBuf::from("/Users/current/Documents")), "Documents"),
        SidebarItem::new(Target::Downloads(PathBuf::from("/Users/current/Downloads")), "Downloads"),
        SidebarItem::new(Target::Home(PathBuf::from("/Users/current")), "Home"),
    ];

    let api = ApiCallRecorder::with_items(
        items
            .iter()
            .map(|item: &SidebarItem| (
                format!("file://{}", item.target()),
                item.display_name().to_string(),
            ))
            .collect(),
    );
    let repository = Repository::new(Box::new(api.clone()));

    let favorites = repository.load().unwrap();

    assert_eq!(favorites.len(), 3);
    assert_eq!(favorites[0].display_name(), "Documents");
    assert_eq!(favorites[1].display_name(), "Downloads");
    assert_eq!(favorites[2].display_name(), "Home");

    let expected_calls = vec![
        ApiCall::CreateFavoritesList,
        ApiCall::GetFavoritesSnapshot,
        ApiCall::GetItemDisplayName(0),
        ApiCall::GetItemUrl(0),
        ApiCall::GetItemDisplayName(1),
        ApiCall::GetItemUrl(1),
        ApiCall::GetItemDisplayName(2),
        ApiCall::GetItemUrl(2),
    ];
    assert_eq!(api.get_calls(), expected_calls);
}

#[test]
fn test_get_favorites_with_invalid_path() {
    let items = [
        SidebarItem::new(Target::UserPath(PathBuf::from("/invalid/path")), "Invalid Path"),
    ];

    let api = ApiCallRecorder::with_items(
        items
            .iter()
            .map(|item: &SidebarItem| (
                format!("file://{}", item.target()),
                item.display_name().to_string(),
            ))
            .collect(),
    );
    let repository = Repository::new(Box::new(api.clone()));

    let favorites = repository.load().unwrap();

    assert_eq!(favorites.len(), 0);

    let expected_calls = vec![
        ApiCall::CreateFavoritesList,
        ApiCall::GetFavoritesSnapshot,
        ApiCall::GetItemDisplayName(0),
        ApiCall::GetItemUrl(0),
    ];
    assert_eq!(api.get_calls(), expected_calls);
}

#[test]
fn test_get_favorites_with_unsupported_url() {
    let items = [
        SidebarItem::new(Target::UserPath(PathBuf::from("/some/invalid/path")), "Unsupported URL"),
    ];

    let api = ApiCallRecorder::with_items(
        items
            .iter()
            .map(|item: &SidebarItem| (
                format!("file://{}", item.target()),
                item.display_name().to_string(),
            ))
            .collect(),
    );
    let repository = Repository::new(Box::new(api.clone()));

    let favorites = repository.load().unwrap();

    assert_eq!(favorites.len(), 0);

    let expected_calls = vec![
        ApiCall::CreateFavoritesList,
        ApiCall::GetFavoritesSnapshot,
        ApiCall::GetItemDisplayName(0),
        ApiCall::GetItemUrl(0),
    ];
    assert_eq!(api.get_calls(), expected_calls);
}
