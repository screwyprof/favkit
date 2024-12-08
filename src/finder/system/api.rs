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
///
/// This type provides a safe wrapper around the raw `LSSharedFileListItemRef` pointer,
/// ensuring proper lifetime management and type safety.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SidebarItemRef(pub(crate) LSSharedFileListItemRef);

impl SidebarItemRef {
    /// Creates a new `SidebarItemRef` from a raw `LSSharedFileListItemRef`.
    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    /// - `item_ref` is a valid `LSSharedFileListItemRef`
    /// - The reference remains valid for the lifetime of the returned `SidebarItemRef`
    pub unsafe fn new(item_ref: LSSharedFileListItemRef) -> Self {
        Self(item_ref)
    }

    /// Gets the underlying `LSSharedFileListItemRef`.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the reference is used safely and according to the
    /// macOS API requirements.
    pub unsafe fn as_raw(&self) -> LSSharedFileListItemRef {
        self.0
    }
}

/// A collection of sidebar items that safely wraps a `CFArray<LSSharedFileListItemRef>`.
///
/// This type provides safe access to the underlying array and implements iteration
/// over the contained items.
#[derive(Debug)]
pub struct SidebarItemArray(CFArray<LSSharedFileListItemRef>);

impl SidebarItemArray {
    /// Creates a new `SidebarItemArray` from a `CFArray<LSSharedFileListItemRef>`.
    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    /// - The array contains valid `LSSharedFileListItemRef` items
    /// - The array remains valid for the lifetime of the returned `SidebarItemArray`
    pub unsafe fn new(array: CFArray<LSSharedFileListItemRef>) -> Self {
        Self(array)
    }

    /// Returns an iterator over the sidebar items in the array.
    ///
    /// The iterator yields `SidebarItemRef` instances that provide safe access to the
    /// underlying macOS sidebar items.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use favkit::finder::system::api::SidebarItemArray;
    /// # let array: SidebarItemArray = unimplemented!();
    /// for item in array.iter() {
    ///     // Process each sidebar item
    /// }
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = SidebarItemRef> + '_ {
        let len = self.0.len().try_into().unwrap_or(0);
        (0..len).map(|i| {
            // SAFETY: We trust that Core Foundation provides valid item references
            // and maintains them for the lifetime of the array
            unsafe {
                let ptr = self.0.as_concrete_TypeRef();
                let item_ref = CFArrayGetValueAtIndex(ptr, i.try_into().unwrap_or(0)) as LSSharedFileListItemRef;
                SidebarItemRef::new(item_ref)
            }
        })
    }
}

/// The macOS API for interacting with the Finder sidebar.
///
/// This trait provides a safe interface to the unsafe macOS APIs for working with
/// the Finder sidebar. Implementations must ensure they maintain the safety
/// requirements specified in each method.
pub trait MacOsApi {
    /// Gets the favorites list.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it calls into macOS APIs that:
    /// - Allocate and manage raw pointers
    /// - Require manual memory management
    ///
    /// The caller must ensure that the returned `LSSharedFileListRef` is properly
    /// released when no longer needed.
    ///
    /// # Errors
    ///
    /// Returns `FinderError` if:
    /// - Failed to create the favorites list
    unsafe fn get_favorites_list(&self) -> Result<LSSharedFileListRef, FinderError>;

    /// Gets a snapshot of the favorites list.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it calls into macOS APIs that:
    /// - Allocate and manage raw pointers
    /// - Require manual memory management
    ///
    /// The caller must ensure that:
    /// - `list` is a valid `LSSharedFileListRef`
    /// - The returned `SidebarItemArray` is properly released when no longer needed
    ///
    /// # Errors
    ///
    /// Returns `FinderError` if:
    /// - Failed to create the snapshot
    /// - The snapshot array is invalid
    unsafe fn get_favorites_snapshot(
        &self,
        list: LSSharedFileListRef,
        seed: &mut u32,
    ) -> Result<SidebarItemArray, FinderError>;

    /// Gets the display name of a favorites list item.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it calls into macOS APIs that:
    /// - Allocate and manage raw pointers
    /// - Require manual memory management
    ///
    /// # Returns
    ///
    /// Returns `None` if the item's display name could not be retrieved or is invalid.
    unsafe fn get_item_display_name(&self, item: SidebarItemRef) -> Option<String>;

    /// Gets the URL of a favorites list item.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it calls into macOS APIs that:
    /// - Allocate and manage raw pointers
    /// - Require manual memory management
    ///
    /// # Returns
    ///
    /// Returns `None` if the item's URL could not be retrieved or is invalid.
    unsafe fn get_item_url(&self, item: SidebarItemRef) -> Option<MacOsUrl>;
}

/// Implements `MacOsApi` for `Box<T>` where `T: MacOsApi`.
///
/// This allows using boxed implementations of `MacOsApi` where the trait is needed,
/// enabling dynamic dispatch.
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

/// The real implementation of `MacOsApi` that interacts with the macOS system APIs.
#[derive(Debug, Clone, Copy, Default)]
pub struct RealMacOsApi;

impl RealMacOsApi {
    /// Creates a new instance of `RealMacOsApi`.
    pub fn new() -> Self {
        Self
    }
}

impl MacOsApi for RealMacOsApi {
    unsafe fn get_favorites_list(&self) -> Result<LSSharedFileListRef, FinderError> {
        let list_ref = LSSharedFileListCreate(
            kCFAllocatorDefault,
            kLSSharedFileListFavoriteItems,
            ptr::null(),
        );

        (!list_ref.is_null()).then_some(list_ref).ok_or(FinderError::FavoritesError {
            kind: FavoritesErrorKind::FailedToGetList,
        })
    }

    unsafe fn get_favorites_snapshot(
        &self,
        list: LSSharedFileListRef,
        seed: &mut u32,
    ) -> Result<SidebarItemArray, FinderError> {
        let array_ref = LSSharedFileListCopySnapshot(list, seed);
        let array = CFArray::wrap_under_create_rule(array_ref);

        (!array.as_concrete_TypeRef().is_null())
            .then(|| SidebarItemArray::new(array))
            .ok_or(FinderError::FavoritesError {
                kind: FavoritesErrorKind::FailedToGetSnapshot,
            })
    }

    unsafe fn get_item_display_name(&self, item: SidebarItemRef) -> Option<String> {
        let name_ref = LSSharedFileListItemCopyDisplayName(item.as_raw());
        (!name_ref.is_null())
            .then(|| CFString::wrap_under_create_rule(name_ref))
            .map(|name| name.to_string())
    }

    unsafe fn get_item_url(&self, item: SidebarItemRef) -> Option<MacOsUrl> {
        let url_ref = LSSharedFileListItemCopyResolvedURL(item.as_raw(), 0, ptr::null_mut());
        (!url_ref.is_null()).then(|| MacOsUrl::from(url_ref))
    }
}
