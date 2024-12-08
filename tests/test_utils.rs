use std::sync::{Arc, Mutex};

use core_foundation::array::CFArray;
use core_services::LSSharedFileListItemRef;

use favkit::{
    errors::FinderError,
    finder::system::{
        api::{FavoritesList, MacOsApi, SidebarItemArray, SidebarItemRef},
        url::MacOsUrl,
    },
};

#[derive(Debug, PartialEq, Clone)]
pub enum ApiCall {
    CreateFavoritesList,
    GetFavoritesSnapshot,
    GetItemDisplayName(usize),
    GetItemUrl(usize),
}

#[derive(Clone)]
pub struct ApiCallRecorder {
    calls: Arc<Mutex<Vec<ApiCall>>>,
    items: Arc<Mutex<Vec<(String, String)>>>, // (url, display_name)
    null_names: Arc<Mutex<Vec<usize>>>,
}

impl Default for ApiCallRecorder {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiCallRecorder {
    pub fn new() -> Self {
        Self {
            calls: Arc::new(Mutex::new(Vec::new())),
            items: Arc::new(Mutex::new(Vec::new())),
            null_names: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn with_items(items: Vec<(String, String)>) -> Self {
        Self {
            calls: Arc::new(Mutex::new(Vec::new())),
            items: Arc::new(Mutex::new(items)),
            null_names: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn with_items_without_names(items: Vec<(String, String)>, null_names: Vec<usize>) -> Self {
        Self {
            calls: Arc::new(Mutex::new(Vec::new())),
            items: Arc::new(Mutex::new(items)),
            null_names: Arc::new(Mutex::new(null_names)),
        }
    }

    pub fn get_calls(&self) -> Vec<ApiCall> {
        self.calls.lock().unwrap().clone()
    }
}

impl MacOsApi for ApiCallRecorder {
    unsafe fn get_favorites_list(&self) -> Result<FavoritesList, FinderError> {
        self.calls.lock().unwrap().push(ApiCall::CreateFavoritesList);
        Ok(FavoritesList::new(1 as _))
    }

    unsafe fn get_favorites_snapshot(
        &self,
        _list: &FavoritesList,
        _seed: &mut u32,
    ) -> Result<SidebarItemArray, FinderError> {
        self.calls.lock().unwrap().push(ApiCall::GetFavoritesSnapshot);
        let items = self.items.lock().unwrap();
        println!("Mock items: {:?}", *items);
        let mut refs = Vec::new();
        for i in 0..items.len() {
            refs.push(i as LSSharedFileListItemRef);
        }
        let array = CFArray::from_copyable(&refs);
        Ok(SidebarItemArray::new(array))
    }

    unsafe fn get_item_display_name(&self, item: SidebarItemRef) -> Option<String> {
        let id = item.as_raw() as usize;
        println!("Getting display name for item {}", id);
        self.calls
            .lock()
            .unwrap()
            .push(ApiCall::GetItemDisplayName(id));

        let null_names = self.null_names.lock().unwrap();
        println!("Items with null names: {:?}", *null_names);
        if null_names.contains(&id) {
            println!("Item {} has null name", id);
            None
        } else {
            let items = self.items.lock().unwrap();
            items.get(id).map(|(_, name)| name.clone())
        }
    }

    unsafe fn get_item_url(&self, item: SidebarItemRef) -> Option<MacOsUrl> {
        let id = item.as_raw() as usize;
        println!("Getting URL for item {}", id);
        self.calls.lock().unwrap().push(ApiCall::GetItemUrl(id));

        let items = self.items.lock().unwrap();
        println!("Available URLs: {:?}", *items);
        items.get(id).and_then(|(url, _)| {
            println!("URL for item {}: {}", id, url);
            // Reject invalid paths by returning None
            if url.contains("/invalid/path") {
                None
            } else {
                MacOsUrl::try_from(url.as_str()).ok()
            }
        })
    }
}
