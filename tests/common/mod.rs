#![allow(dead_code)]

use core_foundation::{
    array::{CFArray, CFArrayRef},
    base::TCFType,
    string::{CFString, CFStringRef},
    url::{CFURLRef, CFURL},
};
use core_services::{LSSharedFileListItemRef, LSSharedFileListRef};
use favkit::sidebar::RawMacOsApi;
use std::{
    ptr,
    sync::{Arc, Mutex},
};

#[derive(Debug, PartialEq, Clone)]
pub enum ApiCall {
    CreateFavoritesList,
    CopySnapshot(LSSharedFileListRef),
    CopyDisplayName(LSSharedFileListItemRef),
    CopyResolvedUrl(LSSharedFileListItemRef),
}

// All fields are Send + Sync:
// - Vec<String> is Send + Sync
// - Mutex<T> is Send + Sync when T is Send
// - LSSharedFileListItemRef (raw pointer) is Send + Sync
// - ApiCall is Send + Sync (derives Clone which requires Send + Sync)
struct ApiCallState {
    items: Vec<(String, String)>,
    item_refs: Mutex<Vec<LSSharedFileListItemRef>>,
    next_ref: Mutex<usize>,
    calls: Mutex<Vec<ApiCall>>,
}

// Implement Send and Sync explicitly to confirm thread safety
unsafe impl Send for ApiCallState {}
unsafe impl Sync for ApiCallState {}

impl Clone for ApiCallState {
    fn clone(&self) -> Self {
        Self {
            items: self.items.clone(),
            item_refs: Mutex::new(self.item_refs.lock().unwrap().clone()),
            next_ref: Mutex::new(*self.next_ref.lock().unwrap()),
            calls: Mutex::new(self.calls.lock().unwrap().clone()),
        }
    }
}

#[derive(Clone)]
pub struct ApiCallRecorder {
    state: Arc<ApiCallState>,
}

impl Default for ApiCallRecorder {
    fn default() -> Self {
        Self {
            state: Arc::new(ApiCallState {
                items: Vec::new(),
                item_refs: Mutex::new(Vec::new()),
                next_ref: Mutex::new(1),
                calls: Mutex::new(Vec::new()),
            }),
        }
    }
}

impl ApiCallRecorder {
    pub fn with_items(items: Vec<(String, String)>) -> Self {
        Self {
            state: Arc::new(ApiCallState {
                items,
                item_refs: Mutex::new(Vec::new()),
                next_ref: Mutex::new(1),
                calls: Mutex::new(Vec::new()),
            }),
        }
    }

    fn get_next_ref(&self) -> LSSharedFileListItemRef {
        let mut next_ref = self.state.next_ref.lock().unwrap();
        let current = *next_ref;
        *next_ref += 1;
        current as LSSharedFileListItemRef
    }

    pub fn verify_calls(&self, expected_calls: &[ApiCall]) {
        let calls = self.state.calls.lock().unwrap();
        assert_eq!(
            &*calls, expected_calls,
            "API calls don't match expected calls"
        );
    }
}

impl RawMacOsApi for ApiCallRecorder {
    unsafe fn create_favorites_list(&self) -> LSSharedFileListRef {
        self.state
            .calls
            .lock()
            .unwrap()
            .push(ApiCall::CreateFavoritesList);
        *self.state.next_ref.lock().unwrap() = 1;
        self.state.item_refs.lock().unwrap().clear();
        1 as LSSharedFileListRef
    }

    unsafe fn copy_snapshot(&self, list: LSSharedFileListRef, _seed: &mut u32) -> CFArrayRef {
        self.state
            .calls
            .lock()
            .unwrap()
            .push(ApiCall::CopySnapshot(list));
        let mut item_refs = Vec::with_capacity(self.state.items.len());
        let mut refs = self.state.item_refs.lock().unwrap();

        for _ in 0..self.state.items.len() {
            let item_ref = self.get_next_ref();
            refs.push(item_ref);
            item_refs.push(item_ref as *const std::ffi::c_void);
        }

        // Create a CFArray that will be retained by the caller
        let array = CFArray::from_copyable(&item_refs);
        let array_ref = array.as_concrete_TypeRef();
        std::mem::forget(array); // Don't release the array since it will be retained by the caller
        array_ref
    }

    unsafe fn copy_display_name(&self, item: LSSharedFileListItemRef) -> CFStringRef {
        self.state
            .calls
            .lock()
            .unwrap()
            .push(ApiCall::CopyDisplayName(item));
        let refs = self.state.item_refs.lock().unwrap();
        if let Some(index) = refs.iter().position(|&r| r == item) {
            if index < self.state.items.len() {
                let string = CFString::new(&self.state.items[index].0);
                let string_ref = string.as_concrete_TypeRef();
                std::mem::forget(string); // Don't release the string since it will be retained by the caller
                return string_ref;
            }
        }
        ptr::null()
    }

    unsafe fn copy_resolved_url(&self, item: LSSharedFileListItemRef) -> CFURLRef {
        self.state
            .calls
            .lock()
            .unwrap()
            .push(ApiCall::CopyResolvedUrl(item));
        let refs = self.state.item_refs.lock().unwrap();
        if let Some(index) = refs.iter().position(|&r| r == item) {
            if index < self.state.items.len() {
                let url = CFURL::from_path(&self.state.items[index].1, false)
                    .expect("Failed to create CFURL");
                let url_ref = url.as_concrete_TypeRef();
                std::mem::forget(url); // Don't release the URL since it will be retained by the caller
                return url_ref;
            }
        }
        ptr::null()
    }
}
