#![allow(dead_code)]

use core_foundation::{
    array::{CFArray, CFArrayCreate},
    base::{kCFAllocatorDefault, CFIndex, TCFType},
    string::{CFString, CFStringRef},
    url::CFURLRef,
};
use core_services::{LSSharedFileListItemRef, LSSharedFileListRef};
use favkit::{MacOsApi, SidebarItem};
use std::{
    ffi::c_void,
    ptr,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone, PartialEq)]
pub enum ApiCall {
    CreateFavoritesList,
    GetFavoritesSnapshot(LSSharedFileListRef),
    GetItemDisplayName(LSSharedFileListItemRef),
    GetItemUrl(LSSharedFileListItemRef),
}

struct ApiCallState {
    items: Vec<SidebarItem>,
    items_without_names: Vec<usize>,
    next_ref: Mutex<i64>,
    calls: Mutex<Vec<ApiCall>>,
}

unsafe impl Send for ApiCallState {}
unsafe impl Sync for ApiCallState {}

impl Clone for ApiCallState {
    fn clone(&self) -> Self {
        Self {
            items: self.items.clone(),
            items_without_names: self.items_without_names.clone(),
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
                items_without_names: Vec::new(),
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
                items_without_names: Vec::new(),
                next_ref: Mutex::new(1),
                calls: Mutex::new(Vec::new()),
            }),
        }
    }

    pub fn with_items_and_null_names(
        items: Vec<SidebarItem>,
        items_without_names: Vec<usize>,
    ) -> Self {
        Self {
            state: Arc::new(ApiCallState {
                items,
                items_without_names,
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

    fn should_return_null_name(&self, item: LSSharedFileListItemRef) -> bool {
        let index = (item as i64 - 1) as usize;
        self.state.items_without_names.contains(&index)
    }
}

impl MacOsApi for ApiCallRecorder {
    unsafe fn get_favorites_list(&self) -> LSSharedFileListRef {
        self.state
            .calls
            .lock()
            .unwrap()
            .push(ApiCall::CreateFavoritesList);
        1 as LSSharedFileListRef
    }

    unsafe fn get_favorites_snapshot(
        &self,
        list: LSSharedFileListRef,
        _seed: &mut u32,
    ) -> CFArray<LSSharedFileListItemRef> {
        println!("MOCK: get_favorites_snapshot called with list: {:?}", list);
        self.state
            .calls
            .lock()
            .unwrap()
            .push(ApiCall::GetFavoritesSnapshot(list));

        // Create array of indices as LSSharedFileListItemRef
        let values: Vec<LSSharedFileListItemRef> = (0..self.state.items.len())
            .map(|i| {
                println!("MOCK: Creating item ref for index {}", i);
                (i + 1) as LSSharedFileListItemRef
            })
            .collect();
        println!("MOCK: Created {} item refs", values.len());

        // Create array and wrap it
        let array = CFArrayCreate(
            kCFAllocatorDefault,
            values.as_ptr() as *const *const c_void,
            values.len() as CFIndex,
            ptr::null(),
        );
        println!("MOCK: Created CFArray: {:?}", array);
        CFArray::wrap_under_create_rule(array)
    }

    unsafe fn get_item_display_name(&self, item_ref: LSSharedFileListItemRef) -> CFStringRef {
        println!(
            "MOCK: get_item_display_name called with item_ref: {:?}",
            item_ref
        );
        self.state
            .calls
            .lock()
            .unwrap()
            .push(ApiCall::GetItemDisplayName(item_ref));

        let index = (item_ref as usize) - 1;
        println!("MOCK: Looking up item at index {}", index);

        if let Some(item) = self.state.items.get(index) {
            println!("MOCK: Found item: {:?}", item);
            if self.should_return_null_name(item_ref) {
                println!("MOCK: Returning null name for item");
                ptr::null()
            } else {
                println!("MOCK: Returning display name: {}", item.display_name());
                let cfstr = CFString::new(item.display_name());
                let ptr = cfstr.as_concrete_TypeRef();
                std::mem::forget(cfstr); // Don't drop the CFString since we're returning its pointer
                ptr
            }
        } else {
            println!("MOCK: Item not found, returning null");
            ptr::null()
        }
    }

    unsafe fn get_item_url(&self, item_ref: LSSharedFileListItemRef) -> CFURLRef {
        println!("MOCK: get_item_url called with item_ref: {:?}", item_ref);
        self.state
            .calls
            .lock()
            .unwrap()
            .push(ApiCall::GetItemUrl(item_ref));

        let index = (item_ref as usize) - 1;
        println!("MOCK: Looking up item at index {}", index);

        if let Some(item) = self.state.items.get(index) {
            println!("MOCK: Found item: {:?}", item);
            CFURLRef::from(item.target())
        } else {
            println!("MOCK: Item not found, returning null");
            ptr::null()
        }
    }
}
