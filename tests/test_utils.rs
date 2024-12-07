use std::{cell::RefCell, ptr, rc::Rc};

use core_foundation::{array::CFArray, string::CFStringRef, url::CFURLRef};
use core_services::{LSSharedFileListItemRef, LSSharedFileListRef};
use favkit::{finder::{macos::MacOsApi, macos_url}, Target};

#[derive(Debug, Clone, PartialEq)]
pub enum MacOsApiCall {
    FavoritesList,
    FavoritesSnapshot,
    ItemUrl(LSSharedFileListItemRef),
}

#[derive(Clone)]
pub struct MockMacOsApi {
    favorites: Vec<Target>,
    calls: Rc<RefCell<Vec<MacOsApiCall>>>,
}

impl MockMacOsApi {
    pub fn with_favorites(favorites: Vec<Target>) -> Self {
        Self {
            favorites,
            calls: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn calls(&self) -> Vec<MacOsApiCall> {
        self.calls.borrow().clone()
    }

    pub fn verify_expected_calls(&self) {
        let calls = self.calls();
        let mut verifier = CallVerifier::new(&calls, self.favorites.len());
        verifier.verify();
    }

    fn make_item_ref(&self, index: usize) -> LSSharedFileListItemRef {
        (index + 1) as LSSharedFileListItemRef
    }
}

impl MacOsApi for MockMacOsApi {
    unsafe fn get_favorites_list(&self) -> LSSharedFileListRef {
        self.calls.borrow_mut().push(MacOsApiCall::FavoritesList);
        1 as LSSharedFileListRef
    }

    unsafe fn get_favorites_snapshot(
        &self,
        _list: LSSharedFileListRef,
        _seed: &mut u32,
    ) -> CFArray<LSSharedFileListItemRef> {
        self.calls.borrow_mut().push(MacOsApiCall::FavoritesSnapshot);
        // Create item refs for our items in the exact order they were provided
        let values: Vec<LSSharedFileListItemRef> = (0..self.favorites.len())
            .map(|i| self.make_item_ref(i))
            .collect();
        CFArray::from_copyable(&values)
    }

    unsafe fn get_item_display_name(&self, _item: LSSharedFileListItemRef) -> CFStringRef {
        ptr::null_mut()
    }

    unsafe fn get_item_url(&self, item: LSSharedFileListItemRef) -> CFURLRef {
        self.calls.borrow_mut().push(MacOsApiCall::ItemUrl(item));
        let index = item as usize - 1;
        if let Some(target) = self.favorites.get(index) {
            macos_url::target_to_url(target)
        } else {
            ptr::null()
        }
    }
}

struct CallVerifier<'a> {
    calls: &'a [MacOsApiCall],
    favorites_count: usize,
    current_pos: usize,
}

impl<'a> CallVerifier<'a> {
    fn new(calls: &'a [MacOsApiCall], favorites_count: usize) -> Self {
        Self {
            calls,
            favorites_count,
            current_pos: 0,
        }
    }

    fn verify(&mut self) {
        // First, verify the initialization sequence
        self.verify_next_is(MacOsApiCall::FavoritesList, "FavoritesList must be called first");
        self.verify_next_is(MacOsApiCall::FavoritesSnapshot, "FavoritesSnapshot must be called second");

        // Then verify that get_item_url is called for each favorite in order
        for i in 0..self.favorites_count {
            let expected_item_ref = ((i + 1) as *mut std::ffi::c_void) as LSSharedFileListItemRef;
            self.verify_next_is(
                MacOsApiCall::ItemUrl(expected_item_ref),
                &format!("ItemUrl must be called for favorite {} with correct item ref", i + 1)
            );
        }

        // Verify no extra calls were made
        assert_eq!(
            self.current_pos,
            self.calls.len(),
            "Expected {} calls but got {}",
            self.current_pos,
            self.calls.len()
        );
    }

    fn verify_next_is(&mut self, expected: MacOsApiCall, message: &str) {
        assert!(
            self.current_pos < self.calls.len(),
            "Expected more calls but got none: {}",
            message
        );
        
        assert_eq!(
            self.calls[self.current_pos], expected,
            "Unexpected call sequence: {}",
            message
        );
        
        self.current_pos += 1;
    }
}
