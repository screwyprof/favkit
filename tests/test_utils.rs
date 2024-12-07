use core_foundation::{
    array::CFArray,
    string::CFStringRef,
    url::CFURLRef,
};
use core_services::{LSSharedFileListItemRef, LSSharedFileListRef};
use favkit::finder::{macos::MacOsApi, target::Target};
use std::{cell::RefCell, path::PathBuf, ptr, rc::Rc};

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
    items: Vec<LSSharedFileListItemRef>,
    calls: Rc<RefCell<Vec<MacOsApiCall>>>,
}

impl Default for MockMacOsApi {
    fn default() -> Self {
        Self::new()
    }
}

impl MockMacOsApi {
    pub fn new() -> Self {
        Self { 
            favorites: Vec::new(),
            items: Vec::new(),
            calls: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn with_favorites(favorites: Vec<Target>, _home_dir: PathBuf) -> Self {
        // Create mock LSSharedFileListItemRef for each favorite
        // Start indices from 1 to match the expected order
        let items: Vec<LSSharedFileListItemRef> = (1..=favorites.len())
            .map(|i| (i as *mut std::ffi::c_void) as LSSharedFileListItemRef)
            .collect();
        
        Self { 
            favorites, 
            items,
            calls: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn calls(&self) -> Vec<MacOsApiCall> {
        self.calls.borrow().clone()
    }
}

impl MacOsApi for MockMacOsApi {
    unsafe fn get_favorites_list(&self) -> LSSharedFileListRef {
        self.calls.borrow_mut().push(MacOsApiCall::GetFavoritesList);
        // Return a non-null pointer to indicate a valid list
        1 as LSSharedFileListRef
    }

    unsafe fn get_favorites_snapshot(
        &self,
        _list: LSSharedFileListRef,
        _seed: &mut u32,
    ) -> CFArray<LSSharedFileListItemRef> {
        self.calls.borrow_mut().push(MacOsApiCall::GetFavoritesSnapshot);
        CFArray::from_copyable(&self.items)
    }

    unsafe fn get_item_display_name(&self, _item: LSSharedFileListItemRef) -> CFStringRef {
        ptr::null_mut()
    }

    unsafe fn get_item_url(&self, item: LSSharedFileListItemRef) -> CFURLRef {
        self.calls.borrow_mut().push(MacOsApiCall::GetItemUrl(item));
        item as *const _ as CFURLRef
    }

    unsafe fn url_to_target(&self, url: CFURLRef) -> Target {
        self.calls.borrow_mut().push(MacOsApiCall::UrlToTarget(url));
        
        if url.is_null() {
            return Target::Home(dirs::home_dir().unwrap_or_default());
        }

        // Find the item in our items vector
        let item = url as LSSharedFileListItemRef;
        let index = self.items.iter().position(|&i| i == item).unwrap_or(0);
        
        // Return the corresponding favorite
        self.favorites.get(index).cloned().unwrap_or_else(|| {
            Target::CustomPath(PathBuf::from("/unknown"))
        })
    }
}
