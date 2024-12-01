use core_foundation::{base::TCFType, string::CFString, url::CFURL};
use core_services::{
    LSSharedFileListItemCopyDisplayName, LSSharedFileListItemCopyResolvedURL,
    LSSharedFileListItemRef,
};

pub struct CFWrapper;

impl CFWrapper {
    pub unsafe fn get_name(item_ref: LSSharedFileListItemRef) -> Option<String> {
        let name_ref = LSSharedFileListItemCopyDisplayName(item_ref);
        if name_ref.is_null() {
            return None;
        }
        let cf_name = CFString::wrap_under_create_rule(name_ref);
        Some(cf_name.to_string())
    }

    pub unsafe fn get_url(item_ref: LSSharedFileListItemRef) -> Option<CFURL> {
        let url_ref = LSSharedFileListItemCopyResolvedURL(item_ref, 0, std::ptr::null_mut());
        if url_ref.is_null() {
            return None;
        }
        Some(CFURL::wrap_under_create_rule(url_ref))
    }
}
