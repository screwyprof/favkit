use core_foundation::{
    array::CFArrayRef,
    base::{CFAllocatorRef, CFTypeRef},
    string::CFStringRef,
};
use core_services::LSSharedFileListRef;

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
}
