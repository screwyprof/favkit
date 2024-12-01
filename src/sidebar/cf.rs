use core_foundation::{
    array::CFArray,
    base::{CFType, TCFType},
    string::{CFString, CFStringRef},
    url::CFURL,
};
use core_services::{
    LSSharedFileListCreate, LSSharedFileListInsertItemURL, LSSharedFileListItemCopyDisplayName,
    LSSharedFileListItemCopyResolvedURL, LSSharedFileListItemRef, LSSharedFileListItemRemove,
    LSSharedFileListRef,
};

/// Trait for Core Services operations
pub trait CoreServicesOperations {
    fn create_list(&self, list_type: CFStringRef) -> Option<LSSharedFileListRef>;
    fn copy_snapshot(&self, list: LSSharedFileListRef) -> Option<CFArray<CFType>>;
    fn copy_display_name(&self, item: LSSharedFileListItemRef) -> Option<CFString>;
    fn copy_resolved_url(&self, item: LSSharedFileListItemRef) -> Option<CFURL>;
    fn insert_item(&self, list: LSSharedFileListRef, url: &CFURL);
    fn remove_item(&self, list: LSSharedFileListRef, item: LSSharedFileListItemRef);
}

/// Default implementation that calls actual Core Services APIs
#[derive(Debug, Default, Clone)]
pub struct DefaultCoreServices;

impl CoreServicesOperations for DefaultCoreServices {
    fn create_list(&self, list_type: CFStringRef) -> Option<LSSharedFileListRef> {
        unsafe {
            let list = LSSharedFileListCreate(std::ptr::null(), list_type, std::ptr::null());
            if list.is_null() {
                None
            } else {
                Some(list)
            }
        }
    }

    fn copy_snapshot(&self, list: LSSharedFileListRef) -> Option<CFArray<CFType>> {
        unsafe {
            let mut seed: u32 = 0;
            let items_ptr = core_services::LSSharedFileListCopySnapshot(list, &mut seed);
            if items_ptr.is_null() {
                None
            } else {
                Some(CFArray::wrap_under_create_rule(items_ptr.cast()))
            }
        }
    }

    fn copy_display_name(&self, item: LSSharedFileListItemRef) -> Option<CFString> {
        unsafe {
            let name_ref = LSSharedFileListItemCopyDisplayName(item);
            if name_ref.is_null() {
                None
            } else {
                Some(CFString::wrap_under_create_rule(name_ref))
            }
        }
    }

    fn copy_resolved_url(&self, item: LSSharedFileListItemRef) -> Option<CFURL> {
        unsafe {
            let url_ref = LSSharedFileListItemCopyResolvedURL(item, 0, std::ptr::null_mut());
            if url_ref.is_null() {
                None
            } else {
                Some(CFURL::wrap_under_create_rule(url_ref))
            }
        }
    }

    fn insert_item(&self, list: LSSharedFileListRef, url: &CFURL) {
        unsafe {
            LSSharedFileListInsertItemURL(
                list,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                url.as_concrete_TypeRef(),
                std::ptr::null(),
                std::ptr::null_mut(),
            );
        }
    }

    fn remove_item(&self, list: LSSharedFileListRef, item: LSSharedFileListItemRef) {
        unsafe {
            LSSharedFileListItemRemove(list, item);
        }
    }
}

/// Safe wrapper for Core Foundation operations
pub struct CFItem<'a> {
    item_ref: LSSharedFileListItemRef,
    core_services: &'a dyn CoreServicesOperations,
}

impl std::fmt::Debug for CFItem<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CFItem")
            .field("item_ref", &(self.item_ref as *const _))
            .field("core_services", &"<dyn CoreServicesOperations>")
            .finish()
    }
}

impl<'a> CFItem<'a> {
    pub fn new(
        item_ref: LSSharedFileListItemRef,
        core_services: &'a dyn CoreServicesOperations,
    ) -> Self {
        Self {
            item_ref,
            core_services,
        }
    }

    pub fn display_name(&self) -> Option<String> {
        self.core_services
            .copy_display_name(self.item_ref)
            .map(|cf_name| cf_name.to_string())
    }

    pub fn resolved_url(&self) -> Option<CFURL> {
        self.core_services.copy_resolved_url(self.item_ref)
    }
}
