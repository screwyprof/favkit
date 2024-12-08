use core_foundation::{
    base::TCFType,
    string::CFString,
    array::CFArray,
};

use core_services::{
    kLSSharedFileListFavoriteItems, LSSharedFileListCopySnapshot, LSSharedFileListCreate,
    LSSharedFileListItemCopyDisplayName, LSSharedFileListItemCopyResolvedURL,
    LSSharedFileListItemRef, LSSharedFileListRef,
};
use std::ptr;

use crate::{
    errors::{FinderError, FavoritesErrorKind},
    finder::system::url::MacOsUrl,
};

pub trait MacOsApi {
    /// Gets the favorites list.
    ///
    /// # Safety
    /// The caller must ensure that:
    /// - The returned LSSharedFileListRef is properly released when no longer needed
    unsafe fn get_favorites_list(&self) -> Result<LSSharedFileListRef, FinderError>;

    /// Gets a snapshot of the favorites list.
    ///
    /// # Safety
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

    /// Get the URL of a sidebar item
    /// 
    /// # Safety
    /// The caller must ensure that:
    /// - `item` is a valid pointer to a `LSSharedFileListItemRef`
    /// - The item reference remains valid for the duration of this call
    unsafe fn get_item_url(&self, item: LSSharedFileListItemRef) -> Option<MacOsUrl>;
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
        let ptr = LSSharedFileListItemCopyDisplayName(item);
        (!ptr.is_null())
            .then_some(ptr)
            .map(|p| unsafe { CFString::wrap_under_create_rule(p) })
            .map(|cf_str| cf_str.to_string())
    }

    /// Get the URL of a sidebar item
    /// 
    /// # Safety
    /// The caller must ensure that:
    /// - `item` is a valid pointer to a `LSSharedFileListItemRef`
    /// - The item reference remains valid for the duration of this call
    unsafe fn get_item_url(&self, item: LSSharedFileListItemRef) -> Option<MacOsUrl> {
        let url = LSSharedFileListItemCopyResolvedURL(item, 0, ptr::null_mut());
        (!url.is_null()).then(|| MacOsUrl::from(url))
    }
}
