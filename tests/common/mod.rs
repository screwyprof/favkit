#![allow(dead_code)]

use core_foundation::{
    array::CFArrayCreate,
    base::{kCFAllocatorDefault, CFIndex, TCFType},
    string::{CFString, CFStringRef},
    url::{CFURLCreateWithString, CFURLRef},
};
use core_services::{CFArray, LSSharedFileListItemRef, LSSharedFileListRef};
use favkit::sidebar::{MacOsApi, SidebarItem};
use std::{
    ffi::c_void,
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

struct ApiCallState {
    items: Vec<SidebarItem>,
    next_ref: Mutex<i64>,
    calls: Mutex<Vec<ApiCall>>,
}

unsafe impl Send for ApiCallState {}
unsafe impl Sync for ApiCallState {}

impl Clone for ApiCallState {
    fn clone(&self) -> Self {
        Self {
            items: self.items.clone(),
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
                next_ref: Mutex::new(1),
                calls: Mutex::new(Vec::new()),
            }),
        }
    }
}

impl ApiCallRecorder {
    pub fn with_items(items: Vec<SidebarItem>) -> Self {
        Self {
            state: Arc::new(ApiCallState {
                items,
                next_ref: Mutex::new(1),
                calls: Mutex::new(Vec::new()),
            }),
        }
    }

    pub fn get_test_item(&self, index: usize) -> LSSharedFileListItemRef {
        (index + 1) as LSSharedFileListItemRef
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

    fn get_item_by_ref(&self, item: LSSharedFileListItemRef) -> Option<&SidebarItem> {
        let index = (item as i64 - 1) as usize;
        self.state.items.get(index)
    }
}

impl MacOsApi for ApiCallRecorder {
    unsafe fn create_favorites_list(&self) -> LSSharedFileListRef {
        self.state
            .calls
            .lock()
            .unwrap()
            .push(ApiCall::CreateFavoritesList);
        *self.state.next_ref.lock().unwrap() = 1;
        1 as LSSharedFileListRef
    }

    unsafe fn copy_snapshot(
        &self,
        list: LSSharedFileListRef,
        _seed: &mut u32,
    ) -> CFArray<LSSharedFileListItemRef> {
        self.state
            .calls
            .lock()
            .unwrap()
            .push(ApiCall::CopySnapshot(list));

        // Create item refs for our items in the exact order they were provided
        let values: Vec<LSSharedFileListItemRef> = (0..self.state.items.len())
            .map(|i| self.get_test_item(i))
            .collect();

        // Create array and wrap it
        let array_ref = CFArrayCreate(
            kCFAllocatorDefault,
            values.as_ptr() as *const *const c_void,
            values.len() as CFIndex,
            ptr::null(),
        );
        CFArray::wrap_under_create_rule(array_ref)
    }

    unsafe fn copy_display_name(&self, item: LSSharedFileListItemRef) -> CFStringRef {
        self.state
            .calls
            .lock()
            .unwrap()
            .push(ApiCall::CopyDisplayName(item));

        if let Some(item) = self.get_item_by_ref(item) {
            CFString::new(item.name()).as_concrete_TypeRef()
        } else {
            ptr::null()
        }
    }

    unsafe fn copy_resolved_url(&self, item: LSSharedFileListItemRef) -> CFURLRef {
        self.state
            .calls
            .lock()
            .unwrap()
            .push(ApiCall::CopyResolvedUrl(item));

        if let Some(item) = self.get_item_by_ref(item) {
            let url_str = item.path().url();
            let cf_str = CFString::new(&url_str);
            CFURLCreateWithString(
                kCFAllocatorDefault,
                cf_str.as_concrete_TypeRef(),
                ptr::null(),
            )
        } else {
            ptr::null()
        }
    }
}
