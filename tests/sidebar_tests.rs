use core_foundation::{
    array::CFArray,
    base::{CFType, TCFType},
    string::{CFString, CFStringRef},
    url::CFURL,
};
use core_services::{LSSharedFileListItemRef, LSSharedFileListRef, OpaqueLSSharedFileListRef};
use favkit::sidebar::{
    cf::CoreServicesOperations, FavoriteItem, Sidebar, SidebarOperations, SidebarSection,
    SpecialLocation,
};
use std::cell::RefCell;
use std::ptr::NonNull;
use std::rc::Rc;
use std::str::FromStr;
use std::sync::atomic::{AtomicUsize, Ordering};

static MOCK_LIST_COUNTER: AtomicUsize = AtomicUsize::new(1);

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
    operations: Rc<RefCell<Vec<MockOperation>>>,
}

impl Default for MockCoreServices {
    fn default() -> Self {
        Self {
            operations: Rc::new(RefCell::new(Vec::new())),
        }
    }
}

impl MockCoreServices {
    fn record_operation(&self, op: MockOperation) {
        self.operations.borrow_mut().push(op);
    }

    fn operations(&self) -> Vec<MockOperation> {
        self.operations.borrow().clone()
    }

    fn clear_operations(&self) {
        self.operations.borrow_mut().clear();
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
}

impl CoreServicesOperations for MockCoreServices {
    unsafe fn create_list(&self, _list_type: CFStringRef) -> Option<LSSharedFileListRef> {
        let ptr = MOCK_LIST_COUNTER.fetch_add(1, Ordering::SeqCst);
        let list = NonNull::new(ptr as *mut OpaqueLSSharedFileListRef)?.as_ptr();
        self.record_operation(MockOperation::CreateList);
        Some(list)
    }

    unsafe fn copy_snapshot(&self, _list: LSSharedFileListRef) -> Option<CFArray<CFType>> {
        let items = ["Test Item".to_string()];
        self.record_operation(MockOperation::CopySnapshot);
        let mock_item = CFString::new(&items[0]).as_CFType();
        Some(CFArray::from_CFTypes(&[mock_item]))
    }

    unsafe fn copy_display_name(&self, _item: LSSharedFileListItemRef) -> Option<CFString> {
        let name = "Test Item".to_string();
        self.record_operation(MockOperation::CopyDisplayName);
        Some(CFString::new(&name))
    }

    unsafe fn copy_resolved_url(&self, _item: LSSharedFileListItemRef) -> Option<CFURL> {
        let path = std::env::temp_dir();
        self.record_operation(MockOperation::CopyResolvedUrl);
        CFURL::from_path(&path, true)
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
    // Arrange
    let mock = MockCoreServices::default();
    let sidebar =
        Sidebar::with_core_services(SidebarSection::Favorites, Box::new(mock.clone())).unwrap();
    mock.clear_operations();

    // Act
    let items = sidebar.list_items().unwrap();

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
    let sidebar =
        Sidebar::with_core_services(SidebarSection::Favorites, Box::new(mock.clone())).unwrap();
    mock.clear_operations();

    // Act
    sidebar
        .add_item(std::env::temp_dir().to_str().unwrap())
        .unwrap();

    // Assert
    let expected_ops = vec![MockOperation::InsertItem];
    mock.assert_operations_sequence(&expected_ops);
}

#[test]
fn test_remove_item() {
    // Arrange
    let mock = MockCoreServices::default();
    let sidebar =
        Sidebar::with_core_services(SidebarSection::Favorites, Box::new(mock.clone())).unwrap();
    mock.clear_operations();

    // Act
    let result = sidebar.remove_item("/nonexistent/path");

    // Assert
    assert!(result.is_err());
    assert!(mock.operations().is_empty());
}

#[test]
fn test_sidebar_section_from_str() {
    assert!(matches!(
        SidebarSection::from_str("favorites").unwrap(),
        SidebarSection::Favorites
    ));
    assert!(matches!(
        SidebarSection::from_str("locations").unwrap(),
        SidebarSection::Locations
    ));
    assert!(SidebarSection::from_str("invalid").is_err());
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

#[test]
fn test_add_special_location() {
    // Arrange
    let mock = MockCoreServices::default();
    let sidebar =
        Sidebar::with_core_services(SidebarSection::Favorites, Box::new(mock.clone())).unwrap();
    mock.clear_operations();

    // Act
    let result = sidebar.add_location(SpecialLocation::AirDrop);

    // Assert
    assert!(result.is_err());
}
