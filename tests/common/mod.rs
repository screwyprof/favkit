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
        println!(
            "Created values: {:?}",
            values
                .iter()
                .map(|v| format!("0x{:x}", v))
                .collect::<Vec<_>>()
        );
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

    pub fn copy_display_name(&self, item: LSSharedFileListItemRef) -> CFStringRef {
        self.state
            .calls
            .lock()
            .unwrap()
            .push(ApiCall::CopyDisplayName(item));
        println!("copy_display_name called for item: {:?}", item);

        // Find the index of the item in values
        let index = self
            .values
            .iter()
            .position(|&v| v as LSSharedFileListItemRef == item)
            .expect("Item not found in test values");

        // Return a mock string based on the item type
        let item = &self.state.items[index];
        let display_name = match item {
            item if item == &SidebarItem::applications() => "Applications",
            item if item == &SidebarItem::downloads() => "Downloads",
            _ => "Unknown Item", // Catch-all pattern
        };

        CFString::new(display_name).as_concrete_TypeRef()
    }

    pub fn copy_resolved_url(&self, item: LSSharedFileListItemRef) -> CFURLRef {
        self.state
            .calls
            .lock()
            .unwrap()
            .push(ApiCall::CopyResolvedUrl(item));
        println!("copy_resolved_url called for item: {:?}", item);

        // Find the index of the item in values
        let index = self
            .values
            .iter()
            .position(|&v| v as LSSharedFileListItemRef == item)
            .expect("Item not found in test values");

        // Return a mock URL based on the item type
        let item = &self.state.items[index];
        let path = item.path().to_string();
        let url_str = format!("file://{}", path);

        unsafe {
            let cf_str = CFString::new(&url_str);
            let url = CFURLCreateWithString(
                kCFAllocatorDefault,
                cf_str.as_concrete_TypeRef(),
                ptr::null(),
            );
            std::mem::forget(cf_str); // Don't drop the string since we're transferring ownership
            url
        }
    }
}

impl MacOsApi for ApiCallRecorder {
    unsafe fn create_favorites_list(&self) -> LSSharedFileListRef {
        println!("create_favorites_list called");
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
        println!("copy_snapshot: start");
        self.state
            .calls
            .lock()
            .unwrap()
            .push(ApiCall::CopySnapshot(list));

        println!("copy_snapshot: creating item refs");
        let mut refs = self.state.item_refs.lock().unwrap();
        refs.clear();

        // Create item refs for our items
        let mut values = Vec::with_capacity(self.state.items.len());
        for i in 0..self.state.items.len() {
            let item_ref = self.get_next_ref();
            println!("copy_snapshot: created ref {} = {:?}", i, item_ref);
            refs.push(item_ref);
            values.push(item_ref as *const std::ffi::c_void);
        }

        println!("copy_snapshot: creating array from {} refs", values.len());
        // Create array directly with Core Foundation
        let array_ref = CFArrayCreate(
            kCFAllocatorDefault,
            values.as_ptr(),
            values.len() as CFIndex,
            ptr::null(),
        );
        println!("copy_snapshot: created array = {:?}", array_ref);
        array_ref
    }

    unsafe fn copy_display_name(&self, item: LSSharedFileListItemRef) -> CFStringRef {
        println!("copy_display_name: start with item {:?}", item);
        self.state
            .calls
            .lock()
            .unwrap()
            .push(ApiCall::CopyDisplayName(item));

        let refs = self.state.item_refs.lock().unwrap();
        println!("copy_display_name: looking for item in refs: {:?}", refs);
        if let Some(index) = refs.iter().position(|&r| r == item) {
            println!("copy_display_name: found item at index {}", index);
            if index < self.state.items.len() {
                let name = self.state.items[index].name();
                println!("copy_display_name: creating string for name: {}", name);
                let string = CFString::new(name);
                let result = string.as_concrete_TypeRef();
                println!("copy_display_name: returning raw pointer = {:?}", result);
                std::mem::forget(string); // Don't drop the string since we're transferring ownership
                return result;
            }
        }
        println!("copy_display_name: returning null");
        ptr::null()
    }

    unsafe fn copy_resolved_url(&self, item: LSSharedFileListItemRef) -> CFURLRef {
        println!("copy_resolved_url: start with item {:?}", item);
        self.state
            .calls
            .lock()
            .unwrap()
            .push(ApiCall::CopyResolvedUrl(item));

        // Find the index of the item in values
        let index = self
            .values
            .iter()
            .position(|&v| v as LSSharedFileListItemRef == item)
            .expect("Item not found in test values");

        // Return a mock URL based on the item type
        let item = &self.state.items[index];
        let path = item.path().to_string();
        let url_str = format!("file://{}", path);

        let cf_str = CFString::new(&url_str);
        let url = CFURLCreateWithString(
            kCFAllocatorDefault,
            cf_str.as_concrete_TypeRef(),
            ptr::null(),
        );
        std::mem::forget(cf_str); // Don't drop the string since we're transferring ownership
        url
    }
}
