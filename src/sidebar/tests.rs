use crate::sidebar::{
    cf::{CoreServicesImpl, CoreServicesOperations},
    FavoriteItem, Sidebar, SidebarOperations, SidebarSection, SpecialLocation,
};
use core_foundation::{
    array::CFArray,
    base::CFType,
    string::{CFString, CFStringRef},
    url::CFURL,
};
use core_services::{
    LSSharedFileListItemRef, LSSharedFileListRef, OpaqueLSSharedFileListRef, TCFType,
};
use std::cell::RefCell;
use std::path::PathBuf;
use std::ptr::NonNull;
use std::rc::Rc;
use std::str::FromStr;
use std::sync::atomic::{AtomicUsize, Ordering};

static MOCK_LIST_COUNTER: AtomicUsize = AtomicUsize::new(1);

#[derive(Clone)]
pub struct MockCoreServices {
    operations_log: Rc<RefCell<Vec<String>>>,
    test_items: Rc<RefCell<Vec<(PathBuf, String)>>>,
}

impl Default for MockCoreServices {
    fn default() -> Self {
        Self {
            operations_log: Rc::new(RefCell::new(Vec::new())),
            test_items: Rc::new(RefCell::new(Vec::new())),
        }
    }
}

impl MockCoreServices {
    pub fn log_operation(&self, operation: &str) {
        self.operations_log.borrow_mut().push(operation.to_string());
    }

    pub fn add_test_item(&self, path: impl Into<PathBuf>, name: impl Into<String>) {
        self.test_items
            .borrow_mut()
            .push((path.into(), name.into()));
    }

    pub fn operations(&self) -> Vec<String> {
        self.operations_log.borrow().clone()
    }
}

impl CoreServicesOperations for MockCoreServices {
    fn create_list(&self, _list_type: CFStringRef) -> Option<LSSharedFileListRef> {
        self.log_operation("create_list");
        let ptr = MOCK_LIST_COUNTER.fetch_add(1, Ordering::SeqCst);
        Some(NonNull::new(ptr as *mut OpaqueLSSharedFileListRef)?.as_ptr())
    }

    fn copy_snapshot(&self, _list: LSSharedFileListRef) -> Option<CFArray<CFType>> {
        self.log_operation("copy_snapshot");
        let mock_item = CFString::new("Test Item").as_CFType();
        Some(CFArray::from_CFTypes(&[mock_item]))
    }

    fn copy_display_name(&self, _item: LSSharedFileListItemRef) -> Option<CFString> {
        self.log_operation("copy_display_name");
        Some(CFString::new("Test Item"))
    }

    fn copy_resolved_url(&self, _item: LSSharedFileListItemRef) -> Option<CFURL> {
        self.log_operation("copy_resolved_url");
        let path = std::env::temp_dir();
        CFURL::from_path(&path, true)
    }

    fn insert_item(&self, _list: LSSharedFileListRef, url: &CFURL) {
        if let Some(path) = url.to_path() {
            self.log_operation(&format!("insert_item: {}", path.display()));
        } else {
            self.log_operation("insert_item: <invalid path>");
        }
    }

    fn remove_item(&self, _list: LSSharedFileListRef, _item: LSSharedFileListItemRef) {
        self.log_operation("remove_item");
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_add_favorite() {
        let mock = MockCoreServices::default();
        let sidebar = Sidebar::with_core_services(
            SidebarSection::Favorites,
            CoreServicesImpl::Mock(mock.clone()),
        )
        .unwrap();

        let test_path = std::env::temp_dir();
        let path_str = test_path
            .to_str()
            .expect("Failed to convert path to string");

        sidebar.add_item(path_str).unwrap();

        let ops = mock.operations();
        assert!(ops.contains(&"create_list".to_string()));
        assert!(ops.iter().any(|op| op.starts_with("insert_item")));
    }

    #[test]
    fn test_list_items() {
        let mock = MockCoreServices::default();
        mock.add_test_item(std::env::temp_dir(), "Test Item");

        let sidebar = Sidebar::with_core_services(
            SidebarSection::Favorites,
            CoreServicesImpl::Mock(mock.clone()),
        )
        .unwrap();
        let items = sidebar.list_items().unwrap();

        let ops = mock.operations();
        assert!(ops.contains(&"copy_snapshot".to_string()));
        assert!(!items.is_empty(), "Should return at least one item");
    }

    #[test]
    fn test_remove_item() {
        let mock = MockCoreServices::default();
        let sidebar = Sidebar::with_core_services(
            SidebarSection::Favorites,
            CoreServicesImpl::Mock(mock.clone()),
        )
        .unwrap();

        let result = sidebar.remove_item("/nonexistent/path");
        assert!(result.is_err());

        let ops = mock.operations();
        assert_eq!(ops.len(), 1);
        assert!(ops.contains(&"create_list".to_string()));
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
        let mock = MockCoreServices::default();
        let sidebar = Sidebar::with_core_services(
            SidebarSection::Favorites,
            CoreServicesImpl::Mock(mock.clone()),
        )
        .unwrap();

        let result = sidebar.add_location(SpecialLocation::AirDrop);
        assert!(result.is_err());
    }
}
