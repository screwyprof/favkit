use core_foundation::{array::CFArray, base::TCFType, string::CFString, url::CFURLRef};
use core_services::{
    kLSSharedFileListFavoriteItems, LSSharedFileListCopySnapshot, LSSharedFileListCreate,
    LSSharedFileListItemCopyDisplayName, LSSharedFileListItemCopyResolvedURL,
    LSSharedFileListItemRef, LSSharedFileListRef,
};
use std::ptr;
use crate::errors::{FinderError, FavoritesErrorKind};

pub trait MacOsApi {
    /// Creates a reference to the system's favorites list.
    ///
    /// # Safety
    /// This function is unsafe because it interacts with Core Foundation APIs that require manual memory management.
    /// The caller must ensure that the returned LSSharedFileListRef is properly released when no longer needed.
    unsafe fn get_favorites_list(&self) -> Result<LSSharedFileListRef, FinderError>;

    /// Gets a snapshot of the current state of the favorites list.
    ///
    /// # Safety
    /// This function is unsafe because it interacts with Core Foundation APIs that require manual memory management.
    /// The caller must ensure that:
    /// - The list parameter is a valid LSSharedFileListRef
    /// - The returned CFArray is properly released when no longer needed
    unsafe fn get_favorites_snapshot(
        &self,
        list: LSSharedFileListRef,
        seed: &mut u32,
    ) -> Result<CFArray<LSSharedFileListItemRef>, FinderError>;

    /// Gets the display name of a favorites list item.
    ///
    /// # Safety
    /// The caller must ensure that:
    /// - The item parameter is a valid LSSharedFileListItemRef
    unsafe fn get_item_display_name(&self, item: LSSharedFileListItemRef) -> Option<String>;

    /// Gets the resolved URL of a favorites list item.
    ///
    /// # Safety
    /// The caller must ensure that:
    /// - The item parameter is a valid LSSharedFileListItemRef
    /// - The returned CFURLRef is properly released when no longer needed
    unsafe fn get_item_url(&self, item: LSSharedFileListItemRef) -> CFURLRef;
}

pub struct RealMacOsApi;

impl RealMacOsApi {
    pub fn new() -> Self {
        Self
    }
}

impl Default for RealMacOsApi {
    fn default() -> Self {
        Self::new()
    }
}

impl MacOsApi for RealMacOsApi {
    unsafe fn get_favorites_list(&self) -> Result<LSSharedFileListRef, FinderError> {
        let list = LSSharedFileListCreate(ptr::null(), kLSSharedFileListFavoriteItems, ptr::null());
        if list.is_null() {
            return Err(FinderError::FavoritesError {
                kind: FavoritesErrorKind::FailedToGetList,
            });
        }
        Ok(list)
    }

    unsafe fn get_favorites_snapshot(
        &self,
        list: LSSharedFileListRef,
        seed: &mut u32,
    ) -> Result<CFArray<LSSharedFileListItemRef>, FinderError> {
        let array_ref = LSSharedFileListCopySnapshot(list, seed);
        let array = CFArray::wrap_under_create_rule(array_ref);
        if array.as_concrete_TypeRef().is_null() {
            return Err(FinderError::FavoritesError {
                kind: FavoritesErrorKind::FailedToGetSnapshot,
            });
        }
        Ok(array)
    }

    unsafe fn get_item_display_name(&self, item: LSSharedFileListItemRef) -> Option<String> {
        let name = LSSharedFileListItemCopyDisplayName(item);
        (!name.is_null())
            .then_some(name)
            .map(|ptr| unsafe { CFString::wrap_under_create_rule(ptr) })
            .map(|cf_str| cf_str.to_string())
    }

    unsafe fn get_item_url(&self, item: LSSharedFileListItemRef) -> CFURLRef {
        LSSharedFileListItemCopyResolvedURL(item, 0, ptr::null_mut())
    }
}
