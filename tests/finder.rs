#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
use coverage_helper::test;

use core_foundation::{
    array::{CFArray, CFArrayRef},
    base::{CFAllocatorRef, CFTypeRef, TCFType},
    string::CFStringRef,
};
use core_services::{LSSharedFileListItemRef, LSSharedFileListRef};
use favkit::{Favorites, FinderApi, finder::FinderError, system::api::MacOsApi};
use std::cell::Cell;
use std::rc::Rc;

struct MockMacOsApi {
    create_called: Cell<bool>,
    copy_snapshot_called: Cell<bool>,
    return_null_list: Cell<bool>,
    return_null_snapshot: Cell<bool>,
    items: Rc<Vec<LSSharedFileListItemRef>>,
    array: Cell<Option<CFArray<LSSharedFileListItemRef>>>,
}

impl MockMacOsApi {
    fn new() -> Self {
        Self {
            create_called: Cell::new(false),
            copy_snapshot_called: Cell::new(false),
            return_null_list: Cell::new(false),
            return_null_snapshot: Cell::new(false),
            items: Rc::new(Vec::new()),
            array: Cell::new(None),
        }
    }
}

impl MacOsApi for MockMacOsApi {
    unsafe fn ls_shared_file_list_create(
        &self,
        _allocator: CFAllocatorRef,
        _list_type: CFStringRef,
        _list_options: CFTypeRef,
    ) -> LSSharedFileListRef {
        self.create_called.set(true);
        if self.return_null_list.get() {
            std::ptr::null_mut()
        } else {
            1 as LSSharedFileListRef
        }
    }

    unsafe fn ls_shared_file_list_copy_snapshot(
        &self,
        _list: LSSharedFileListRef,
        _seed: *mut u32,
    ) -> CFArrayRef {
        self.copy_snapshot_called.set(true);
        if self.return_null_snapshot.get() {
            std::ptr::null_mut()
        } else {
            let array = CFArray::from_copyable(&self.items);
            let ptr = array.as_concrete_TypeRef();
            self.array.set(Some(array)); // a hack to retain the array
            ptr
        }
    }
}

#[test]
fn should_call_macos_api_when_getting_list() {
    let mock_api = MockMacOsApi::new();
    let favorites = Favorites::new(&mock_api);
    let finder = FinderApi::new(&favorites);

    let _ = finder.get_favorites_list();

    assert!(mock_api.create_called.get());
    assert!(mock_api.copy_snapshot_called.get());
}

#[test]
fn should_return_error_when_list_handle_is_null() {
    let mock_api = MockMacOsApi::new();
    mock_api.return_null_list.set(true);
    let favorites = Favorites::new(&mock_api);
    let finder = FinderApi::new(&favorites);

    let result = finder.get_favorites_list();

    assert!(matches!(result, Err(FinderError::NullListHandle)));
}

#[test]
fn should_return_error_when_snapshot_handle_is_null() {
    let mock_api = MockMacOsApi::new();
    mock_api.return_null_snapshot.set(true);
    let favorites = Favorites::new(&mock_api);
    let finder = FinderApi::new(&favorites);

    let result = finder.get_favorites_list();

    assert!(matches!(result, Err(FinderError::NullSnapshotHandle)));
}
