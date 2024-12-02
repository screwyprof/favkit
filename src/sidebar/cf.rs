use crate::sidebar::error::{Result, SidebarError};
use crate::sidebar::SidebarUrl;
use core_foundation::{
    array::CFArray,
    base::{CFRelease, CFRetain, CFType, TCFType},
    string::{CFString, CFStringRef},
    url::{CFURLGetString, CFURL},
};
use core_services::{
    kLSSharedFileListFavoriteItems, kLSSharedFileListFavoriteVolumes, LSSharedFileListCreate,
    LSSharedFileListInsertItemURL, LSSharedFileListItemCopyDisplayName,
    LSSharedFileListItemCopyResolvedURL, LSSharedFileListItemRef, LSSharedFileListItemRemove,
    LSSharedFileListRef,
};
use std::path::Path;

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

    fn create_favorites_list(&self) -> Result<LSSharedFileListRef> {
        unsafe {
            self.create_list(kLSSharedFileListFavoriteItems)
                .ok_or_else(|| SidebarError::CreateList("Failed to create favorites list".into()))
        }
    }

    fn create_volumes_list(&self) -> Result<LSSharedFileListRef> {
        unsafe {
            self.create_list(kLSSharedFileListFavoriteVolumes)
                .ok_or_else(|| SidebarError::CreateList("Failed to create volumes list".into()))
        }
    }
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
            CFRetain(list as *mut _);
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

impl DefaultCoreServices {
    pub fn create_favorites_list(&self) -> Result<LSSharedFileListRef> {
        unsafe {
            self.create_list(kLSSharedFileListFavoriteItems)
                .ok_or_else(|| SidebarError::CreateList("Failed to create favorites list".into()))
        }
    }

    pub fn create_volumes_list(&self) -> Result<LSSharedFileListRef> {
        unsafe {
            self.create_list(kLSSharedFileListFavoriteVolumes)
                .ok_or_else(|| SidebarError::CreateList("Failed to create volumes list".into()))
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

    pub fn from_cf_type(
        item: &CFType,
        core_services: &'a dyn CoreServicesOperations,
    ) -> Option<Self> {
        let item_ref = item.as_CFTypeRef() as *const _ as *mut _;
        unsafe {
            CFRetain(item_ref as *mut _);
        }
        Some(Self {
            item_ref,
            core_services,
        })
    }

    pub fn display_name(&self) -> Option<String> {
        unsafe {
            let name = self.core_services.copy_display_name(self.item_ref)?;
            Some(name.to_string())
        }
    }

    pub fn resolved_url(&self) -> Option<CFURL> {
        unsafe { self.core_services.copy_resolved_url(self.item_ref) }
    }

    pub fn item_ref(&self) -> LSSharedFileListItemRef {
        self.item_ref
    }

    /// Checks if the item matches the target path and removes it if it does.
    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    /// - `list` is a valid LSSharedFileListRef
    /// - The list has not been deallocated
    /// - The item_ref in self is valid and from the same list
    pub unsafe fn get_url_and_remove_if_matches(
        &self,
        list: LSSharedFileListRef,
        target_path: &Path,
    ) -> Result<bool> {
        if let Some(url) = self.core_services.copy_resolved_url(self.item_ref) {
            if let Some(item_path) = url.to_path() {
                if item_path == target_path {
                    self.core_services.remove_item(list, self.item_ref);
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    /// Gets all items from the list.
    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    /// - `list` is a valid LSSharedFileListRef
    /// - The list has not been deallocated
    pub unsafe fn get_items(
        list: LSSharedFileListRef,
        core_services: &'a dyn CoreServicesOperations,
    ) -> Result<Vec<Self>> {
        let items = core_services
            .copy_snapshot(list)
            .ok_or_else(|| SidebarError::Snapshot("Failed to get items snapshot".into()))?;

        Ok(items
            .iter()
            .filter_map(|item| Self::from_cf_type(&item, core_services))
            .collect())
    }

    /// Adds a URL to the list.
    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    /// - `list` is a valid LSSharedFileListRef
    /// - The list has not been deallocated
    /// - `url` is a valid CFURL
    pub unsafe fn add_to_list(
        list: LSSharedFileListRef,
        url: CFURL,
        core_services: &dyn CoreServicesOperations,
    ) -> Result<()> {
        core_services.insert_item(list, &url);
        Ok(())
    }

    pub fn parse_url(&self) -> Result<SidebarUrl> {
        let url = unsafe { self.core_services.copy_resolved_url(self.item_ref) }
            .ok_or_else(|| SidebarError::CoreFoundation("Failed to get URL".into()))?;

        let url_string = unsafe {
            let str_ref = CFURLGetString(url.as_concrete_TypeRef());
            CFString::wrap_under_get_rule(str_ref).to_string()
        };

        match url_string {
            s if s.starts_with("nwnode://") && s.contains("domain-AirDrop") => {
                Ok(SidebarUrl::AirDrop)
            }
            s if s.starts_with("com-apple-sfl://") && s.contains("IsRemoteDisc") => {
                Ok(SidebarUrl::RemoteDisc)
            }
            s if s.starts_with("file://") => {
                if let Some(path) = url.to_path() {
                    Ok(SidebarUrl::File(path))
                } else {
                    Ok(SidebarUrl::NotFound)
                }
            }
            s => Ok(SidebarUrl::SystemUrl(s)),
        }
    }
}

impl Drop for CFItem<'_> {
    fn drop(&mut self) {
        unsafe {
            CFRelease(self.item_ref as *mut _);
        }
    }
}

/// Safe wrapper around Core Foundation list operations
pub struct CFList<'a> {
    list: LSSharedFileListRef,
    core_services: &'a dyn CoreServicesOperations,
}

impl std::fmt::Debug for CFList<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CFList")
            .field("list", &(self.list as *const _))
            .field("core_services", &"<dyn CoreServicesOperations>")
            .finish()
    }
}

impl<'a> CFList<'a> {
    pub fn new_favorites(core_services: &'a dyn CoreServicesOperations) -> Result<Self> {
        let list = core_services.create_favorites_list()?;
        Ok(Self {
            list,
            core_services,
        })
    }

    pub fn new_volumes(core_services: &'a dyn CoreServicesOperations) -> Result<Self> {
        let list = core_services.create_volumes_list()?;
        Ok(Self {
            list,
            core_services,
        })
    }

    pub fn get_items(&self) -> Result<Vec<CFItem>> {
        unsafe { CFItem::get_items(self.list, self.core_services) }
    }

    pub fn add_url(&self, url: CFURL) -> Result<()> {
        unsafe { CFItem::add_to_list(self.list, url, self.core_services) }
    }

    pub fn remove_item(&self, item: &CFItem) -> Result<()> {
        unsafe {
            self.core_services.remove_item(self.list, item.item_ref());
            Ok(())
        }
    }
}
