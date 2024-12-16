#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
use coverage_helper::test;

use core_foundation::{
    array::{CFArray, CFArrayRef},
    base::{CFAllocatorRef, CFTypeRef, TCFType},
    string::CFStringRef,
};
use core_services::{LSSharedFileListItemRef, LSSharedFileListRef};
use favkit::{Favorites, FinderApi, system::api::MacOsApi};
use std::cell::Cell;

struct MockMacOsApi {
    create_called: Cell<bool>,
    copy_snapshot_called: Cell<bool>,
}

impl MockMacOsApi {
    fn new() -> Self {
        Self {
            create_called: Cell::new(false),
            copy_snapshot_called: Cell::new(false),
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
        1 as LSSharedFileListRef
    }

    #[allow(clippy::vec_init_then_push)]
    unsafe fn ls_shared_file_list_copy_snapshot(
        &self,
        _list: LSSharedFileListRef,
        _seed: *mut u32,
    ) -> CFArrayRef {
        self.copy_snapshot_called.set(true);

        let mut refs = Vec::new();
        refs.push(1 as LSSharedFileListItemRef);
        refs.push(2 as LSSharedFileListItemRef);

        let array = CFArray::from_copyable(&refs);
        array.as_concrete_TypeRef()
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
