use core_foundation::{
    array::CFArrayRef,
    base::{CFAllocatorRef, CFTypeRef},
    error::CFErrorRef,
    string::CFStringRef,
    url::CFURLRef,
};
use core_services::{
    LSSharedFileListItemRef, LSSharedFileListRef, LSSharedFileListResolutionFlags,
};

/// Trait for interacting with MacOS APIs.
/// This allows us to mock the MacOS API for testing.
pub trait MacOsApi {
    /// Creates a new shared file list reference.
    ///
    /// # Safety
    ///
    /// This function is unsafe because:
    /// - It interacts with raw C pointers through Core Foundation API
    /// - The caller must ensure the allocator and list_type pointers are valid
    /// - The returned list reference must be properly released
    unsafe fn ls_shared_file_list_create(
        &self,
        allocator: CFAllocatorRef,
        list_type: CFStringRef,
        list_options: CFTypeRef,
    ) -> LSSharedFileListRef;

    /// Gets a snapshot of the shared file list.
    ///
    /// # Safety
    ///
    /// This function is unsafe because:
    /// - It interacts with raw C pointers through Core Foundation API
    /// - The caller must ensure the list reference is valid
    /// - The returned array reference must be properly released
    unsafe fn ls_shared_file_list_copy_snapshot(
        &self,
        list: LSSharedFileListRef,
        seed: *mut u32,
    ) -> CFArrayRef;

    /// Gets the display name of a shared file list item.
    ///
    /// # Safety
    ///
    /// This function is unsafe because:
    /// - It interacts with raw C pointers through Core Foundation API
    /// - The caller must ensure the item reference is valid
    /// - The returned string reference must be properly released
    unsafe fn ls_shared_file_list_item_copy_display_name(
        &self,
        item: LSSharedFileListItemRef,
    ) -> CFStringRef;

    /// Gets the resolved URL for a shared file list item.
    ///
    /// # Safety
    ///
    /// This function is unsafe because:
    /// - It interacts with raw C pointers through Core Foundation API
    /// - The caller must ensure the item reference is valid
    /// - The returned URL reference must be properly released
    unsafe fn ls_shared_file_list_item_copy_resolved_url(
        &self,
        item: LSSharedFileListItemRef,
        flags: LSSharedFileListResolutionFlags,
        error: *mut CFErrorRef,
    ) -> CFURLRef;
}
