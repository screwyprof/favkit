use core_foundation::{
    array::CFArray,
    base::{CFType, TCFType},
    string::{CFString, CFStringRef},
    url::CFURL,
};
use core_services::{LSSharedFileListItemRef, LSSharedFileListRef};
use favkit::sidebar::{cf::CoreServicesOperations, FavoriteItem, Sidebar, SpecialLocation};

use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, PartialEq)]
enum MockOperation {
    CreateList,
    CopySnapshot,
    CopyDisplayName,
    CopyResolvedUrl,
    InsertItem,
    RemoveItem,
}

#[derive(Clone)]
struct MockCoreServices {
    operations: Arc<Mutex<Vec<MockOperation>>>,
    return_items: Arc<Mutex<bool>>,
}

impl Default for MockCoreServices {
    fn default() -> Self {
        Self {
            operations: Arc::new(Mutex::new(Vec::new())),
            return_items: Arc::new(Mutex::new(true)),
        }
    }
}

impl MockCoreServices {
    fn record_operation(&self, op: MockOperation) {
        self.operations.lock().unwrap().push(op);
    }

    fn operations(&self) -> Vec<MockOperation> {
        self.operations.lock().unwrap().clone()
    }

    fn clear_operations(&self) {
        self.operations.lock().unwrap().clear();
    }

    fn assert_operations_sequence(&self, expected_ops: &[MockOperation]) {
        let ops = self.operations();
        assert_eq!(
            ops.len(),
            expected_ops.len(),
            "Unexpected number of operations"
        );

        for (actual, expected) in ops.iter().zip(expected_ops.iter()) {
            assert_eq!(actual, expected, "Operation mismatch");
        }
    }

    fn set_return_items(&self, value: bool) {
        *self.return_items.lock().unwrap() = value;
    }
}

impl CoreServicesOperations for MockCoreServices {
    unsafe fn create_list(&self, _list_type: CFStringRef) -> Option<LSSharedFileListRef> {
        self.record_operation(MockOperation::CreateList);
        Some(42 as *mut _)
    }

    unsafe fn copy_snapshot(&self, _list: LSSharedFileListRef) -> Option<CFArray<CFType>> {
        self.record_operation(MockOperation::CopySnapshot);
        if *self.return_items.lock().unwrap() {
            let mock_item = CFString::from_static_string("mock_id");
            Some(CFArray::from_CFTypes(&[mock_item.as_CFType()]))
        } else {
            Some(CFArray::from_CFTypes(&[]))
        }
    }

    unsafe fn copy_display_name(&self, _item: LSSharedFileListItemRef) -> Option<CFString> {
        self.record_operation(MockOperation::CopyDisplayName);
        Some(CFString::from_static_string("Test Item"))
    }

    unsafe fn copy_resolved_url(&self, _item: LSSharedFileListItemRef) -> Option<CFURL> {
        self.record_operation(MockOperation::CopyResolvedUrl);
        Some(CFURL::from_path(std::path::Path::new("/test/path"), true).unwrap())
    }

    unsafe fn insert_item(&self, _list: LSSharedFileListRef, _url: &CFURL) {
        self.record_operation(MockOperation::InsertItem);
    }

    unsafe fn remove_item(&self, _list: LSSharedFileListRef, _item: LSSharedFileListItemRef) {
        self.record_operation(MockOperation::RemoveItem);
    }
}

#[test]
fn test_list_items() {
    let mock = MockCoreServices::default();
    mock.clear_operations();
    let sidebar = Sidebar::new_with_core_services(Box::new(mock.clone()));

    let favorites = sidebar.favorites();
    mock.clear_operations();

    let items = favorites.list_items().unwrap();

    // Assert
    let expected_ops = vec![
        MockOperation::CopySnapshot,
        MockOperation::CopyResolvedUrl,
        MockOperation::CopyDisplayName,
    ];
    mock.assert_operations_sequence(&expected_ops);
    assert!(!items.is_empty());
}

#[test]
fn test_add_favorite() {
    // Arrange
    let mock = MockCoreServices::default();
    let sidebar = Sidebar::new_with_core_services(Box::new(mock.clone()));
    mock.clear_operations();

    // Act
    sidebar
        .favorites()
        .add_item(std::env::temp_dir().to_str().unwrap())
        .unwrap();

    // Assert
    let expected_ops = vec![MockOperation::CreateList, MockOperation::InsertItem];
    mock.assert_operations_sequence(&expected_ops);
}

#[test]
fn test_remove_item() {
    // Arrange
    let mock = MockCoreServices::default();
    mock.clear_operations();
    let sidebar = Sidebar::new_with_core_services(Box::new(mock.clone()));
    let favorites = sidebar.favorites();
    mock.clear_operations();
    mock.set_return_items(false);

    // Act
    let result = favorites.remove_item("/nonexistent/path");

    // Assert
    assert!(result.is_err());
    let expected_ops = vec![MockOperation::CopySnapshot];
    mock.assert_operations_sequence(&expected_ops);
}

#[test]
fn test_add_special_location() {
    // Arrange
    let mock = MockCoreServices::default();
    let sidebar = Sidebar::new_with_core_services(Box::new(mock.clone()));
    mock.clear_operations();

    // Act
    let result = sidebar
        .favorites()
        .add_special_location(SpecialLocation::AirDrop);

    // Assert
    assert!(result.is_ok());
    let expected_ops = vec![MockOperation::CreateList, MockOperation::InsertItem];
    mock.assert_operations_sequence(&expected_ops);
}

#[test]
fn test_favorite_item_paths() {
    for item in [
        FavoriteItem::Documents,
        FavoriteItem::Downloads,
        FavoriteItem::Desktop,
        FavoriteItem::Home,
        FavoriteItem::Applications,
        FavoriteItem::Movies,
        FavoriteItem::Music,
        FavoriteItem::Pictures,
    ] {
        let path = item.path();
        assert!(!path.as_os_str().is_empty());
        assert!(!item.display_name().is_empty());
    }
}

#[test]
fn test_special_location_display_names() {
    assert_eq!(SpecialLocation::AirDrop.display_name(), "AirDrop");
    assert_eq!(SpecialLocation::RemoteDisc.display_name(), "Remote Disc");
    assert_eq!(SpecialLocation::RecentsFolder.display_name(), "Recents");
    assert_eq!(SpecialLocation::AllMyFiles.display_name(), "All My Files");
    assert_eq!(SpecialLocation::NetworkFolder.display_name(), "Network");
    assert_eq!(SpecialLocation::ICloudDrive.display_name(), "iCloud Drive");
}
