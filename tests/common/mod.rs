#![allow(dead_code)]

use core_foundation::{
    array::{CFArrayCreate, CFArrayRef},
    base::{kCFAllocatorDefault, CFIndex, TCFType},
    string::{CFString, CFStringRef},
    url::{CFURLCreateWithString, CFURLRef},
};
use core_services::{LSSharedFileListItemRef, LSSharedFileListRef};
use favkit::sidebar::{MacOsApi, SidebarItem};
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

struct ApiCallState {
    items: Vec<SidebarItem>,
    item_refs: Mutex<Vec<LSSharedFileListItemRef>>,
    next_ref: Mutex<i64>,
    calls: Mutex<Vec<ApiCall>>,
}

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
    values: Vec<i64>,
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
            values: Vec::new(),
        }
    }
}

impl ApiCallRecorder {
    pub fn with_items(items: Vec<SidebarItem>) -> Self {
        let values: Vec<i64> = (1..=items.len()).map(|i| i as i64).collect();
        Self {
            state: Arc::new(ApiCallState {
                items,
                item_refs: Mutex::new(Vec::new()),
                next_ref: Mutex::new(1),
                calls: Mutex::new(Vec::new()),
            }),
            values,
        }
    }

    pub fn get_test_item(&self, index: usize) -> LSSharedFileListItemRef {
        self.values[index] as LSSharedFileListItemRef
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
        let index = self
            .values
            .iter()
            .position(|&v| v as LSSharedFileListItemRef == item)?;
        Some(&self.state.items[index])
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
        self.state.item_refs.lock().unwrap().clear();
        1 as LSSharedFileListRef
    }

    unsafe fn copy_snapshot(&self, list: LSSharedFileListRef, _seed: &mut u32) -> CFArrayRef {
        self.state
            .calls
            .lock()
            .unwrap()
            .push(ApiCall::CopySnapshot(list));

        let mut refs = self.state.item_refs.lock().unwrap();
        refs.clear();

        // Create item refs for our items
        let mut values = Vec::with_capacity(self.state.items.len());
        for _ in 0..self.state.items.len() {
            let item_ref = self.get_next_ref();
            refs.push(item_ref);
            values.push(item_ref as *const std::ffi::c_void);
        }

        // Create array directly with Core Foundation
        CFArrayCreate(
            kCFAllocatorDefault,
            values.as_ptr(),
            values.len() as CFIndex,
            ptr::null(),
        )
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
            let url_str = format!("file://{}", item.path());
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
