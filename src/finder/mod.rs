mod errors;
mod macos;

pub use errors::Result;
pub use macos::RealMacOsApi;

use core_foundation::{
    base::{CFAllocatorRef, CFTypeRef, kCFAllocatorDefault},
    string::CFStringRef,
};
use core_services::{LSSharedFileListRef, kLSSharedFileListFavoriteItems};

/// Trait for interacting with MacOS APIs.
/// This allows us to mock the MacOS API for testing.
pub trait MacOsApi {
    /// Creates a new shared file list reference.
    ///
    /// # Safety
    ///
    /// This function is unsafe because:
    /// - It interacts with raw C pointers through Core Foundation API
    /// - The caller must ensure the allocator and list_type pointers are valid
    /// - The returned list reference must be properly released
    unsafe fn ls_shared_file_list_create(
        &self,
        allocator: CFAllocatorRef,
        list_type: CFStringRef,
        list_options: CFTypeRef,
    ) -> LSSharedFileListRef;
}

pub struct FinderApi<'a, T: MacOsApi> {
    macos_api: &'a T,
}

impl<'a, T: MacOsApi> FinderApi<'a, T> {
    pub fn new(macos_api: &'a T) -> Self {
        Self { macos_api }
    }

    pub fn get_favorites_list(&self) -> Result<Vec<String>> {
        unsafe {
            let _list = self.macos_api.ls_shared_file_list_create(
                kCFAllocatorDefault,
                kLSSharedFileListFavoriteItems,
                std::ptr::null(),
            );

            Ok(vec![])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use coverage_helper::test;
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
    fn test_get_favorites_list_calls_macos_api() {
        let mock_api = MockMacOsApi::new();
        let api = FinderApi::new(&mock_api);
        let _ = api.get_favorites_list();
        assert!(mock_api.create_called.get());
    }
}
