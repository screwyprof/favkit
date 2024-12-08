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
            Target::AirDrop("nwnode://domain-AirDrop".to_string()),
            "AirDrop",
        ),
        SidebarItem::new(
            Target::Documents(dirs::document_dir().unwrap()),
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
            Target::AirDrop("nwnode://domain-AirDrop".to_string()),
            "AirDrop",
        ),
        SidebarItem::new(
            Target::Documents(dirs::document_dir().unwrap()),
            "Documents",
        ),
        SidebarItem::new(
            Target::Downloads(dirs::download_dir().unwrap()),
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
    let items = vec![SidebarItem::new(
        Target::UserPath(PathBuf::from("/invalid/path")),
        "Invalid Path",
    )];

    let api = ApiCallRecorder::with_items(items.clone());
    let repository = Repository::new(Box::new(api));

    let sidebar = repository.load().unwrap();
    assert_eq!(sidebar, items);
}

#[test]
fn test_load_favorites_with_unsupported_url() {
    let items = vec![SidebarItem::new(
        Target::UserPath(PathBuf::from("/some/invalid/path")),
        "Unsupported URL",
    )];

    let api = ApiCallRecorder::with_items(items.clone());
    let repository = Repository::new(Box::new(api));

    let sidebar = repository.load().unwrap();
    assert_eq!(sidebar, items);
}

#[test]
fn test_load_favorites_with_null_display_names() {
    let items = vec![
        SidebarItem::new(
            Target::AirDrop("nwnode://domain-AirDrop".to_string()),
            "AirDrop",
        ),
        SidebarItem::new(
            Target::Documents(dirs::document_dir().unwrap()),
            "Documents",
        ),
        SidebarItem::new(
            Target::Downloads(dirs::download_dir().unwrap()),
            "Downloads",
        ),
    ];

    // Only AirDrop should have null display name - macOS provides names for others
    let null_name_indices = vec![0];
    let api = ApiCallRecorder::with_items_and_null_names(items.clone(), null_name_indices);
    let repository = Repository::new(Box::new(api));

    let sidebar = repository.load().unwrap();
    assert_eq!(sidebar, items);
}
