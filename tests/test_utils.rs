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
        
        // First call should be GetFavoritesList
        assert_eq!(
            calls.first(),
            Some(&MacOsApiCall::GetFavoritesList),
            "First call must be GetFavoritesList"
        );

        // Second call should be GetFavoritesSnapshot
        assert_eq!(
            calls.get(1),
            Some(&MacOsApiCall::GetFavoritesSnapshot),
            "Second call must be GetFavoritesSnapshot"
        );

        // After that, for each favorite we should have GetItemUrl followed by UrlToTarget
        let mut expected_pairs = Vec::new();
        for i in 1..=self.favorites.len() {
            let item_ref = (i as *mut std::ffi::c_void) as LSSharedFileListItemRef;
            let url_ref = item_ref as CFURLRef;
            expected_pairs.push((
                MacOsApiCall::GetItemUrl(item_ref),
                MacOsApiCall::UrlToTarget(url_ref),
            ));
        }

        // Check that each pair exists in order
        let mut current_pos = 2; // Start after the first two initialization calls
        for (idx, (get_url, to_target)) in expected_pairs.iter().enumerate() {
            let get_url_pos = calls[current_pos..].iter().position(|c| c == get_url).map(|p| p + current_pos);
            let to_target_pos = calls[current_pos..].iter().position(|c| c == to_target).map(|p| p + current_pos);

            assert!(
                get_url_pos.is_some(),
                "Missing GetItemUrl call for item {}",
                idx + 1
            );
            assert!(
                to_target_pos.is_some(),
                "Missing UrlToTarget call for item {}",
                idx + 1
            );

            let get_url_pos = get_url_pos.unwrap();
            let to_target_pos = to_target_pos.unwrap();

            assert!(
                get_url_pos < to_target_pos,
                "GetItemUrl must come before UrlToTarget for item {}",
                idx + 1
            );

            current_pos = to_target_pos + 1;
        }

        // Verify we didn't get any extra calls
        assert_eq!(
            calls.len(),
            2 + self.favorites.len() * 2,
            "Got {} calls, expected {} (2 initialization calls + 2 calls per favorite)",
            calls.len(),
            2 + self.favorites.len() * 2
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
