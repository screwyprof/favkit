use std::{cell::RefCell, ptr, rc::Rc};

use core_foundation::{array::CFArray, string::CFStringRef, url::CFURLRef};
use core_services::{LSSharedFileListItemRef, LSSharedFileListRef};
use favkit::{finder::macos::MacOsApi, Target};

#[derive(Debug, Clone, PartialEq)]
pub enum MacOsApiCall {
    GetFavoritesList,
    GetFavoritesSnapshot,
    GetItemUrl(LSSharedFileListItemRef),
    UrlToTarget(CFURLRef),
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
        
        // Verify initial calls
        assert_eq!(calls[0], MacOsApiCall::GetFavoritesList, "First call should be GetFavoritesList");
        assert_eq!(calls[1], MacOsApiCall::GetFavoritesSnapshot, "Second call should be GetFavoritesSnapshot");
        
        // For each favorite, verify the pair of GetItemUrl and UrlToTarget calls
        let mut expected_calls = Vec::new();
        for i in 1..=self.favorites.len() {
            let item_ref = (i as *mut std::ffi::c_void) as LSSharedFileListItemRef;
            let url_ref = item_ref as CFURLRef;
            
            expected_calls.push(MacOsApiCall::GetItemUrl(item_ref));
            expected_calls.push(MacOsApiCall::UrlToTarget(url_ref));
        }

        // Verify all expected calls are present
        for expected_call in expected_calls {
            assert!(
                calls.contains(&expected_call),
                "Missing expected call: {:?}",
                expected_call
            );
        }

        // Verify no unexpected calls
        assert_eq!(
            calls.len(),
            2 + self.favorites.len() * 2,
            "Unexpected number of calls"
        );
    }
}

impl MacOsApi for MockMacOsApi {
    unsafe fn get_favorites_list(&self) -> LSSharedFileListRef {
        self.calls.borrow_mut().push(MacOsApiCall::GetFavoritesList);
        1 as LSSharedFileListRef
    }

    unsafe fn get_favorites_snapshot(
        &self,
        _list: LSSharedFileListRef,
        _seed: &mut u32,
    ) -> CFArray<LSSharedFileListItemRef> {
        self.calls.borrow_mut().push(MacOsApiCall::GetFavoritesSnapshot);
        CFArray::from_copyable(&(1..=self.favorites.len())
            .map(|i| i as *mut std::ffi::c_void as LSSharedFileListItemRef)
            .collect::<Vec<_>>())
    }

    unsafe fn get_item_display_name(&self, _item: LSSharedFileListItemRef) -> CFStringRef {
        ptr::null_mut()
    }

    unsafe fn get_item_url(&self, item: LSSharedFileListItemRef) -> CFURLRef {
        self.calls.borrow_mut().push(MacOsApiCall::GetItemUrl(item));
        item as CFURLRef
    }

    unsafe fn url_to_target(&self, url: CFURLRef) -> Target {
        self.calls.borrow_mut().push(MacOsApiCall::UrlToTarget(url));
        let index = (url as usize) - 1;
        self.favorites[index].clone()
    }
}
