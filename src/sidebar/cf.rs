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
    /// Creates a new shared file list.
    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    /// - `list_type` is a valid CFStringRef pointing to a valid list type
    /// - The returned pointer is properly managed and released
    unsafe fn create_list(&self, list_type: CFStringRef) -> Option<LSSharedFileListRef>;

    /// Gets a snapshot of the shared file list items.
    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    /// - `list` is a valid LSSharedFileListRef
    /// - The list has not been deallocated
    unsafe fn copy_snapshot(&self, list: LSSharedFileListRef) -> Option<CFArray<CFType>>;

    /// Gets the display name of a shared file list item.
    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    /// - `item` is a valid LSSharedFileListItemRef
    /// - The item has not been deallocated
    unsafe fn copy_display_name(&self, item: LSSharedFileListItemRef) -> Option<CFString>;

    /// Gets the resolved URL of a shared file list item.
    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    /// - `item` is a valid LSSharedFileListItemRef
    /// - The item has not been deallocated
    unsafe fn copy_resolved_url(&self, item: LSSharedFileListItemRef) -> Option<CFURL>;

    /// Inserts a new item into the shared file list.
    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    /// - `list` is a valid LSSharedFileListRef
    /// - The list has not been deallocated
    /// - `url` is a valid CFURL
    unsafe fn insert_item(&self, list: LSSharedFileListRef, url: &CFURL);

    /// Removes an item from the shared file list.
    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    /// - `list` is a valid LSSharedFileListRef
    /// - The list has not been deallocated
    /// - `item` is a valid LSSharedFileListItemRef from the same list
    unsafe fn remove_item(&self, list: LSSharedFileListRef, item: LSSharedFileListItemRef);
}

/// Default implementation that calls actual Core Services APIs
#[derive(Debug, Default, Clone)]
pub struct DefaultCoreServices;

impl CoreServicesOperations for DefaultCoreServices {
    unsafe fn create_list(&self, list_type: CFStringRef) -> Option<LSSharedFileListRef> {
        let list = LSSharedFileListCreate(std::ptr::null(), list_type, std::ptr::null());
        if list.is_null() {
            None
        } else {
            Some(list)
        }
    }

    unsafe fn copy_snapshot(&self, list: LSSharedFileListRef) -> Option<CFArray<CFType>> {
        let mut seed: u32 = 0;
        let items_ptr = core_services::LSSharedFileListCopySnapshot(list, &mut seed);
        if items_ptr.is_null() {
            None
        } else {
            Some(CFArray::wrap_under_create_rule(items_ptr.cast()))
        }
    }

    unsafe fn copy_display_name(&self, item: LSSharedFileListItemRef) -> Option<CFString> {
        let name_ref = LSSharedFileListItemCopyDisplayName(item);
        if name_ref.is_null() {
            None
        } else {
            Some(CFString::wrap_under_create_rule(name_ref))
        }
    }

    unsafe fn copy_resolved_url(&self, item: LSSharedFileListItemRef) -> Option<CFURL> {
        let url_ref = LSSharedFileListItemCopyResolvedURL(item, 0, std::ptr::null_mut());
        if url_ref.is_null() {
            None
        } else {
            Some(CFURL::wrap_under_create_rule(url_ref))
        }
    }

    unsafe fn insert_item(&self, list: LSSharedFileListRef, url: &CFURL) {
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

    unsafe fn remove_item(&self, list: LSSharedFileListRef, item: LSSharedFileListItemRef) {
        LSSharedFileListItemRemove(list, item);
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
        unsafe {
            self.core_services
                .copy_display_name(self.item_ref)
                .map(|cf_name| cf_name.to_string())
        }
    }

    pub fn resolved_url(&self) -> Option<CFURL> {
        unsafe { self.core_services.copy_resolved_url(self.item_ref) }
    }
}
