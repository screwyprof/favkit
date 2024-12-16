#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

use coverage_helper::test;

use core_foundation::{
    base::{CFAllocatorRef, CFTypeRef},
    string::CFStringRef,
};
use core_services::LSSharedFileListRef;
use favkit::{FinderApi, system::api::MacOsApi};
use std::cell::Cell;

struct MockMacOsApi {
    create_called: Cell<bool>,
}

impl MockMacOsApi {
    fn new() -> Self {
        Self {
            create_called: Cell::new(false),
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
        std::ptr::null_mut()
    }
}

#[test]
fn should_call_macos_api_when_getting_favorites() {
    let mock_api = MockMacOsApi::new();
    let api = FinderApi::new(&mock_api);
    let _ = api.get_favorites_list();
    assert!(mock_api.create_called.get());
}
