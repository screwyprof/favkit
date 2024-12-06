use core_foundation::{
    array::CFArray,
    base::TCFType,
    string::CFStringRef,
    url::CFURLRef,
};
use core_services::{
    kLSSharedFileListFavoriteItems, LSSharedFileListCopySnapshot, LSSharedFileListCreate,
    LSSharedFileListItemCopyDisplayName, LSSharedFileListItemCopyResolvedURL,
    LSSharedFileListItemRef, LSSharedFileListRef,
};
use std::ptr;

#[derive(Default)]
pub struct SystemMacOsApi;

impl SystemMacOsApi {
    #[cfg(test)]
    pub fn new() -> Self {
        Self
    }
}

impl super::macos::MacOsApi for SystemMacOsApi {
    unsafe fn get_favorites_list(&self) -> LSSharedFileListRef {
        LSSharedFileListCreate(ptr::null(), kLSSharedFileListFavoriteItems, ptr::null())
    }

    unsafe fn get_favorites_snapshot(
        &self,
        list: LSSharedFileListRef,
        seed: &mut u32,
    ) -> CFArray<LSSharedFileListItemRef> {
        let array_ref = LSSharedFileListCopySnapshot(list, seed);
        CFArray::wrap_under_create_rule(array_ref)
    }

    unsafe fn get_item_display_name(&self, item: LSSharedFileListItemRef) -> CFStringRef {
        LSSharedFileListItemCopyDisplayName(item)
    }

    unsafe fn get_item_url(&self, item: LSSharedFileListItemRef) -> CFURLRef {
        LSSharedFileListItemCopyResolvedURL(item, 0, ptr::null_mut())
    }     
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn system_api_can_be_created() {
        let _api = SystemMacOsApi::new();
    }
}
