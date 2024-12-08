use std::path::PathBuf;

use favkit::{Repository, SidebarItem, Target};

use crate::test_utils::{ApiCall, ApiCallRecorder};

mod test_utils;

#[test]
fn test_load_empty_favorites() {
    let api = ApiCallRecorder::default();
    let repository = Repository::new(Box::new(api.clone()));

    let sidebar = repository.load().unwrap();

    assert_eq!(sidebar.len(), 0);
    api.verify_calls(&[
        ApiCall::CreateFavoritesList,
        ApiCall::GetFavoritesSnapshot(1 as _),
    ]);
}

#[test]
fn test_load_favorites() {
    let items = vec![
        SidebarItem::new(
            Target::AirDrop("airdrop://".to_string()),
            "AirDrop",
        ),
        SidebarItem::new(
            Target::Documents(PathBuf::from("/Users/test/Documents")),
            "Documents",
        ),
    ];

    let api = ApiCallRecorder::with_items(items.clone());
    let repository = Repository::new(Box::new(api));

    let sidebar = repository.load().unwrap();
    assert_eq!(sidebar, items);
}

#[test]
fn test_load_favorites_with_multiple_items() {
    let items = vec![
        SidebarItem::new(
            Target::AirDrop("airdrop://".to_string()),
            "AirDrop",
        ),
        SidebarItem::new(
            Target::Documents(PathBuf::from("/Users/test/Documents")),
            "Documents",
        ),
        SidebarItem::new(
            Target::Downloads(PathBuf::from("/Users/test/Downloads")),
            "Downloads",
        ),
    ];

    let api = ApiCallRecorder::with_items(items.clone());
    let repository = Repository::new(Box::new(api));

    let sidebar = repository.load().unwrap();
    assert_eq!(sidebar, items);
}

#[test]
fn test_load_favorites_with_invalid_path() {
    let items = vec![
        SidebarItem::new(
            Target::Documents(PathBuf::from("/invalid/path")),
            "Documents",
        ),
    ];

    let api = ApiCallRecorder::with_items(items);
    let repository = Repository::new(Box::new(api));

    let sidebar = repository.load().unwrap();
    assert_eq!(sidebar.len(), 1);
}

#[test]
fn test_load_favorites_with_unsupported_url() {
    let items = vec![
        SidebarItem::new(
            Target::Documents(PathBuf::from("unsupported://path")),
            "Documents",
        ),
    ];

    let api = ApiCallRecorder::with_items(items);
    let repository = Repository::new(Box::new(api));

    let sidebar = repository.load().unwrap();
    assert_eq!(sidebar.len(), 1);
}
