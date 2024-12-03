use core_foundation::{
    array::CFArray,
    base::{CFRelease, CFType, TCFType},
    string::CFString,
    url::{CFURLGetString, CFURL},
};
use core_services::{
    kLSSharedFileListFavoriteItems, LSSharedFileListCopySnapshot, LSSharedFileListCreate,
    LSSharedFileListItemCopyDisplayName, LSSharedFileListItemCopyResolvedURL,
    LSSharedFileListItemRef,
};
use std::ffi::c_void;

use crate::sidebar::MacOsPath;

pub trait MacOsApi {
    fn list_favorite_items(&self) -> Vec<(String, MacOsPath)>; // (name, path)
}

#[derive(Default)]
pub struct RealMacOsApi;

impl RealMacOsApi {
    pub fn new() -> Self {
        Self
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

            let result = {
                let mut seed = 0;
                let items = LSSharedFileListCopySnapshot(favorites_list, &mut seed);
                let items = CFArray::<CFType>::wrap_under_create_rule(items);

                let mut result = Vec::new();

                for item in items.iter() {
                    let item_ref = item.as_concrete_TypeRef() as LSSharedFileListItemRef;
                    let name_ref = LSSharedFileListItemCopyDisplayName(item_ref);
                    let url_ref =
                        LSSharedFileListItemCopyResolvedURL(item_ref, 0, std::ptr::null_mut());

                    if !name_ref.is_null() && !url_ref.is_null() {
                        let name = CFString::wrap_under_create_rule(name_ref);
                        let url = CFURL::wrap_under_create_rule(url_ref);
                        let path = CFString::wrap_under_create_rule(CFURLGetString(
                            url.as_concrete_TypeRef(),
                        ));

                        result.push((name.to_string(), MacOsPath::new(path.to_string())));
                    }
                }

                result
            };

            CFRelease(favorites_list as *mut c_void);
            result
        }
    }
}
