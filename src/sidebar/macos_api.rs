use core_foundation::{
    array::CFArray,
    base::{CFType, ItemRef, TCFType},
    string::CFString,
    url::{CFURLGetString, CFURL},
};
use core_services::{
    kLSSharedFileListFavoriteItems, LSSharedFileListCopySnapshot, LSSharedFileListCreate,
    LSSharedFileListItemCopyDisplayName, LSSharedFileListItemCopyResolvedURL,
    LSSharedFileListItemRef,
};

use crate::sidebar::MacOsPath;

pub trait MacOsApi {
    fn list_favorite_items(&self) -> Vec<(String, MacOsPath)>;
}

#[derive(Default)]
pub struct RealMacOsApi;

impl RealMacOsApi {
    pub fn new() -> Self {
        Self
    }

    unsafe fn convert_list_item(item: ItemRef<'_, CFType>) -> Option<(String, MacOsPath)> {
        let item_ref = item.as_concrete_TypeRef() as LSSharedFileListItemRef;

        // Get item name
        let name_ref = LSSharedFileListItemCopyDisplayName(item_ref);
        if name_ref.is_null() {
            return None;
        }
        let name = CFString::wrap_under_create_rule(name_ref);

        // Get item URL
        let url_ref = LSSharedFileListItemCopyResolvedURL(item_ref, 0, std::ptr::null_mut());
        if url_ref.is_null() {
            return None;
        }
        let url = CFURL::wrap_under_create_rule(url_ref);
        let path_str = CFString::wrap_under_get_rule(CFURLGetString(url.as_concrete_TypeRef()));

        Some((name.to_string(), path_str.to_string().into()))
    }
}

impl MacOsApi for RealMacOsApi {
    fn list_favorite_items(&self) -> Vec<(String, MacOsPath)> {
        unsafe {
            let favorites_list = LSSharedFileListCreate(
                std::ptr::null(),
                kLSSharedFileListFavoriteItems,
                std::ptr::null(),
            );

            if favorites_list.is_null() {
                return vec![];
            }

            let mut seed = 0;
            let items_ref = LSSharedFileListCopySnapshot(favorites_list, &mut seed);
            let items = CFArray::<CFType>::wrap_under_create_rule(items_ref);

            items
                .iter()
                .filter_map(|item| Self::convert_list_item(item))
                .collect()
        }
    }
}
