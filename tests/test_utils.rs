use core_foundation::{
    array::{CFArray, CFArrayCreate},
    base::{kCFAllocatorDefault, CFIndex, TCFType},
};
use core_services::{LSSharedFileListItemRef, LSSharedFileListRef};
use favkit::{MacOsApi, SidebarItem, Target};
use favkit::errors::FinderError;
use favkit::finder::system::url::MacOsUrl;
use favkit::finder::system::api::{SidebarItemRef, SidebarItemArray};
use std::{
    ffi::c_void,
    fmt::Debug,
    ptr,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone, PartialEq)]
pub enum ApiCall {
    CreateFavoritesList,
    GetFavoritesSnapshot(LSSharedFileListRef),
    GetItemDisplayName(SidebarItemRef),
    GetItemUrl(SidebarItemRef),
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

    pub fn verify_calls(&self, expected_calls: &[ApiCall]) {
        assert_eq!(
            &*self.state.calls.lock().unwrap(),
            expected_calls,
            "API calls did not match expected"
        );
    }

    fn get_item_by_ref(&self, item: SidebarItemRef) -> Option<&SidebarItem> {
        // SAFETY: This is test code and we ensure the reference is valid
        let index = (unsafe { item.as_raw() } as i64 - 1) as usize;
        self.state.items.get(index)
    }

    fn should_return_null_name(&self, item: SidebarItemRef) -> bool {
        // SAFETY: This is test code and we ensure the reference is valid
        let index = (unsafe { item.as_raw() } as i64 - 1) as usize;
        self.state.items_without_names.contains(&index)
    }
}

impl MacOsApi for ApiCallRecorder {
    unsafe fn get_favorites_list(&self) -> Result<LSSharedFileListRef, FinderError> {
        self.state
            .calls
            .lock()
            .unwrap()
            .push(ApiCall::CreateFavoritesList);

        Ok(1 as LSSharedFileListRef)
    }

    unsafe fn get_favorites_snapshot(
        &self,
        list: LSSharedFileListRef,
        _seed: &mut u32,
    ) -> Result<SidebarItemArray, FinderError> {
        println!("MOCK: get_favorites_snapshot called with list: {:?}", list);
        self.state
            .calls
            .lock()
            .unwrap()
            .push(ApiCall::GetFavoritesSnapshot(list));

        let items_len = self.state.items.len();
        println!("MOCK: Creating array with {} items", items_len);

        let item_refs: Vec<LSSharedFileListItemRef> = (1..=items_len)
            .map(|i| i as LSSharedFileListItemRef)
            .collect();

        let array_ref = unsafe {
            CFArrayCreate(
                kCFAllocatorDefault,
                item_refs.as_ptr() as *const *const c_void,
                item_refs.len() as CFIndex,
                ptr::null(),
            )
        };

        let array = CFArray::wrap_under_create_rule(array_ref);
        Ok(unsafe { SidebarItemArray::new(array) })
    }

    unsafe fn get_item_display_name(&self, item: SidebarItemRef) -> Option<String> {
        println!("MOCK: get_item_display_name called with item_ref: {:?}", item);
        self.state
            .calls
            .lock()
            .unwrap()
            .push(ApiCall::GetItemDisplayName(item));

        if self.should_return_null_name(item) {
            println!("MOCK: Returning null name for item");
            None
        } else if let Some(item) = self.get_item_by_ref(item) {
            println!("MOCK: Found item: {:?}", item);
            // Return None for AirDrop items to simulate macOS behavior
            if matches!(item.target(), Target::AirDrop(_)) {
                println!("MOCK: AirDrop item, returning None for display name");
                None
            } else {
                println!("MOCK: Returning display name: {}", item.display_name());
                Some(item.display_name().to_string())
            }
        } else {
            println!("MOCK: Item not found, returning null");
            None
        }
    }

    unsafe fn get_item_url(&self, item: SidebarItemRef) -> Option<MacOsUrl> {
        println!("MOCK: get_item_url called with item_ref: {:?}", item);
        self.state
            .calls
            .lock()
            .unwrap()
            .push(ApiCall::GetItemUrl(item));

        if let Some(item) = self.get_item_by_ref(item) {
            println!("MOCK: Found item: {:?}", item);
            match item.target() {
                Target::AirDrop(url) => Some(MacOsUrl::try_from(url.as_str()).unwrap()),
                _ => {
                    let url_str = String::from(item.target());
                    MacOsUrl::try_from(url_str.as_str()).ok()
                }
            }
        } else {
            println!("MOCK: Item not found, returning None");
            None
        }
    }
}
