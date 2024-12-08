use core_foundation::{
    array::{CFArray, CFArrayGetValueAtIndex},
    base::{TCFType, kCFAllocatorDefault},
    string::CFString,
};
use core_services::{LSSharedFileListItemRef, LSSharedFileListRef, kLSSharedFileListFavoriteItems, LSSharedFileListCopySnapshot, LSSharedFileListCreate, LSSharedFileListItemCopyDisplayName, LSSharedFileListItemCopyResolvedURL};
use std::ptr;

use crate::errors::{FinderError, FavoritesErrorKind};
use crate::finder::system::url::MacOsUrl;

/// A reference to a sidebar item in the system's finder
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SidebarItemRef(pub(crate) LSSharedFileListItemRef);

impl SidebarItemRef {
    /// Create a new SidebarItemRef from a raw LSSharedFileListItemRef
    /// 
    /// # Safety
    /// The caller must ensure that the item_ref is a valid LSSharedFileListItemRef
    pub unsafe fn new(item_ref: LSSharedFileListItemRef) -> Self {
        Self(item_ref)
    }

    /// Get the underlying LSSharedFileListItemRef
    /// 
    /// # Safety
    /// The caller must ensure that the reference is used safely
    pub unsafe fn as_raw(&self) -> LSSharedFileListItemRef {
        self.0
    }
}

/// A collection of sidebar items
#[derive(Debug)]
pub struct SidebarItemArray(CFArray<LSSharedFileListItemRef>);

impl SidebarItemArray {
    /// Create a new SidebarItemArray from a CFArray<LSSharedFileListItemRef>
    /// 
    /// # Safety
    /// The caller must ensure that the array contains valid LSSharedFileListItemRef items
    pub unsafe fn new(array: CFArray<LSSharedFileListItemRef>) -> Self {
        Self(array)
    }

    /// Get the number of items in the array
    pub fn len(&self) -> usize {
        self.0.len().try_into().unwrap()
    }

    /// Check if the array is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get an iterator over the items in the array
    pub fn iter(&self) -> impl Iterator<Item = SidebarItemRef> + '_ {
        (0..self.len()).map(|i| {
            // SAFETY: We trust that Core Foundation provides valid item references
            unsafe {
                let ptr = self.0.as_concrete_TypeRef();
                let item_ref = CFArrayGetValueAtIndex(ptr, i.try_into().unwrap()) as LSSharedFileListItemRef;
                SidebarItemRef::new(item_ref)
            }
        })
    }
}

/// The macOS API for interacting with the Finder sidebar
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
    /// - The returned SidebarItemArray is properly released when no longer needed
    unsafe fn get_favorites_snapshot(
        &self,
        list: LSSharedFileListRef,
        seed: &mut u32,
    ) -> Result<SidebarItemArray, FinderError>;

    /// Gets the display name of a favorites list item.
    ///
    /// # Safety
    /// The caller must ensure that:
    /// - The item parameter is a valid LSSharedFileListItemRef
    unsafe fn get_item_display_name(&self, item: SidebarItemRef) -> Option<String>;

    /// Gets the URL of a favorites list item.
    ///
    /// # Safety
    /// The caller must ensure that:
    /// - The item parameter is a valid LSSharedFileListItemRef
    unsafe fn get_item_url(&self, item: SidebarItemRef) -> Option<MacOsUrl>;
}

impl<T: MacOsApi> MacOsApi for Box<T> {
    unsafe fn get_favorites_list(&self) -> Result<LSSharedFileListRef, FinderError> {
        (**self).get_favorites_list()
    }

    unsafe fn get_favorites_snapshot(
        &self,
        list: LSSharedFileListRef,
        seed: &mut u32,
    ) -> Result<SidebarItemArray, FinderError> {
        (**self).get_favorites_snapshot(list, seed)
    }

    unsafe fn get_item_display_name(&self, item: SidebarItemRef) -> Option<String> {
        (**self).get_item_display_name(item)
    }

    unsafe fn get_item_url(&self, item: SidebarItemRef) -> Option<MacOsUrl> {
        (**self).get_item_url(item)
    }
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
        let list_ref = LSSharedFileListCreate(
            kCFAllocatorDefault,
            kLSSharedFileListFavoriteItems,
            ptr::null(),
        );

        if list_ref.is_null() {
            return Err(FinderError::FavoritesError {
                kind: FavoritesErrorKind::FailedToGetList,
            });
        }

        Ok(list_ref)
    }

    unsafe fn get_favorites_snapshot(
        &self,
        list: LSSharedFileListRef,
        seed: &mut u32,
    ) -> Result<SidebarItemArray, FinderError> {
        let array_ref = LSSharedFileListCopySnapshot(list, seed);
        let array = CFArray::wrap_under_create_rule(array_ref);
        if array.as_concrete_TypeRef().is_null() {
            return Err(FinderError::FavoritesError {
                kind: FavoritesErrorKind::FailedToGetSnapshot,
            });
        }
        Ok(SidebarItemArray::new(array))
    }

    unsafe fn get_item_display_name(&self, item: SidebarItemRef) -> Option<String> {
        let name_ref = LSSharedFileListItemCopyDisplayName(item.as_raw());
        if name_ref.is_null() {
            return None;
        }

        let name = CFString::wrap_under_create_rule(name_ref);
        Some(name.to_string())
    }

    unsafe fn get_item_url(&self, item: SidebarItemRef) -> Option<MacOsUrl> {
        let url_ref = LSSharedFileListItemCopyResolvedURL(item.as_raw(), 0, ptr::null_mut());
        if url_ref.is_null() {
            return None;
        }

        Some(MacOsUrl::from(url_ref))
    }
}
