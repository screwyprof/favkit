use core_foundation::{
    array::CFArray,
    string::CFStringRef,
    url::CFURLRef,
};
use core_services::{LSSharedFileListItemRef, LSSharedFileListRef};

/// MacOS API for interacting with the Finder sidebar.
///
/// This trait abstracts the Core Foundation API calls required to read and write
/// sidebar items. All methods are unsafe because they deal with raw Core Foundation
/// types that require manual memory management.
pub trait MacOsApi {
    /// Creates a reference to the system's favorites list.
    ///
    /// # Safety
    /// This function is unsafe because it interacts with Core Foundation APIs that require manual memory management.
    /// The caller must ensure that the returned LSSharedFileListRef is properly released when no longer needed.
    unsafe fn get_favorites_list(&self) -> LSSharedFileListRef;

    /// Gets a snapshot of the current state of the favorites list.
    ///
    /// # Safety
    /// This function is unsafe because it interacts with Core Foundation APIs that require manual memory management.
    /// The caller must ensure that:
    /// - The list parameter is a valid LSSharedFileListRef
    /// - The returned CFArray is properly released when no longer needed
    unsafe fn get_favorites_snapshot(
        &self,
        list: LSSharedFileListRef,
        seed: &mut u32,
    ) -> CFArray<LSSharedFileListItemRef>;

    /// Gets the display name of a favorites list item.
    ///
    /// # Safety
    /// This function is unsafe because it interacts with Core Foundation APIs that require manual memory management.
    /// The caller must ensure that:
    /// - The item parameter is a valid LSSharedFileListItemRef
    /// - The returned CFStringRef is properly released when no longer needed
    unsafe fn get_item_display_name(&self, item: LSSharedFileListItemRef) -> CFStringRef;

    /// Gets the resolved URL of a favorites list item.
    ///
    /// # Safety
    /// This function is unsafe because it interacts with Core Foundation APIs that require manual memory management.
    /// The caller must ensure that:
    /// - The item parameter is a valid LSSharedFileListItemRef
    /// - The returned CFURLRef is properly released when no longer needed
    unsafe fn get_item_url(&self, item: LSSharedFileListItemRef) -> CFURLRef;
}

#[cfg(any(test, feature = "test-utils"))]
pub mod test_utils {
    use super::*;
    use std::cell::RefCell;
    use std::collections::HashMap;

    pub struct MockMacOsApi {
        favorites_list: Option<LSSharedFileListRef>,
        favorites_snapshot: RefCell<Option<CFArray<LSSharedFileListItemRef>>>,
        item_urls: RefCell<HashMap<LSSharedFileListItemRef, CFURLRef>>,
    }

    impl Default for MockMacOsApi {
        fn default() -> Self {
            Self::new()
        }
    }

    impl MockMacOsApi {
        pub fn new() -> Self {
            Self {
                favorites_list: None,
                favorites_snapshot: RefCell::new(None),
                item_urls: RefCell::new(HashMap::new()),
            }
        }

        pub fn with_favorites_list(mut self, list: LSSharedFileListRef) -> Self {
            self.favorites_list = Some(list);
            self
        }

        pub fn with_favorites_snapshot(self, snapshot: CFArray<LSSharedFileListItemRef>) -> Self {
            *self.favorites_snapshot.borrow_mut() = Some(snapshot);
            self
        }

        pub fn with_item_url(self, url: CFURLRef, item_ref: LSSharedFileListItemRef) -> Self {
            self.item_urls.borrow_mut().insert(item_ref, url);
            self
        }
    }

    impl MacOsApi for MockMacOsApi {
        unsafe fn get_favorites_list(&self) -> LSSharedFileListRef {
            self.favorites_list.unwrap_or(std::ptr::null_mut())
        }

        unsafe fn get_favorites_snapshot(
            &self,
            _list: LSSharedFileListRef,
            _seed: &mut u32,
        ) -> CFArray<LSSharedFileListItemRef> {
            println!("Mock: get_favorites_snapshot called");
            match self.favorites_snapshot.borrow().as_ref() {
                Some(array) => {
                    let values: Vec<LSSharedFileListItemRef> = array
                        .get_all_values()
                        .into_iter()
                        .map(|ptr| ptr as LSSharedFileListItemRef)
                        .collect();
                    println!("Mock: returning snapshot with {} values", values.len());
                    CFArray::from_copyable(&values)
                }
                None => {
                    println!("Mock: returning empty snapshot");
                    CFArray::from_copyable(&[])
                }
            }
        }

        unsafe fn get_item_display_name(&self, _item: LSSharedFileListItemRef) -> CFStringRef {
            std::ptr::null()
        }

        unsafe fn get_item_url(&self, item: LSSharedFileListItemRef) -> CFURLRef {
            println!("Mock: get_item_url called for item {:?}", item);
            let url = self.item_urls.borrow().get(&item).copied().unwrap_or(std::ptr::null_mut());
            println!("Mock: returning url {:?}", url);
            url
        }
    }
}
