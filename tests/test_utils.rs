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
        self.verify_init_sequence();
        self.verify_favorite_sequences();
        self.verify_no_extra_calls();
    }

    fn verify_init_sequence(&mut self) {
        self.verify_next_is(MacOsApiCall::GetFavoritesList, "GetFavoritesList must be called first");
        self.verify_next_is(MacOsApiCall::GetFavoritesSnapshot, "GetFavoritesSnapshot must be called second");
    }

    fn verify_favorite_sequences(&mut self) {
        for favorite_idx in 0..self.favorites_count {
            let item_ref = self.make_item_ref(favorite_idx);
            let url_ref = item_ref as CFURLRef;
            
            self.verify_next_is(
                MacOsApiCall::GetItemUrl(item_ref), 
                &format!("GetItemUrl must be called for favorite {}", favorite_idx + 1)
            );
            self.verify_next_is(
                MacOsApiCall::UrlToTarget(url_ref), 
                &format!("UrlToTarget must be called for favorite {}", favorite_idx + 1)
            );
        }
    }

    fn verify_no_extra_calls(&self) {
        assert_eq!(
            self.current_pos,
            self.calls.len(),
            "Expected {} calls, but got {} extra calls",
            self.current_pos,
            self.calls.len() - self.current_pos
        );
    }

    fn verify_next_is(&mut self, expected: MacOsApiCall, message: &str) {
        assert!(
            self.current_pos < self.calls.len(),
            "Expected {}, but no more calls were made",
            message
        );
        
        assert_eq!(
            self.calls[self.current_pos],
            expected,
            "{} at position {}",
            message,
            self.current_pos
        );
        
        self.current_pos += 1;
    }

    fn make_item_ref(&self, favorite_idx: usize) -> LSSharedFileListItemRef {
        ((favorite_idx + 1) as *mut std::ffi::c_void) as LSSharedFileListItemRef
    }
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
