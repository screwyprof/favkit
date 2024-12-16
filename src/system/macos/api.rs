use core_foundation::{
    array::CFArrayRef,
    base::{CFAllocatorRef, CFTypeRef},
    string::CFStringRef,
};
use core_services::LSSharedFileListRef;

use crate::system::api::MacOsApi;

#[derive(Default)]
pub struct RealMacOsApi;

impl RealMacOsApi {
    pub fn new() -> Self {
        Self
    }
}

impl MacOsApi for RealMacOsApi {
    unsafe fn ls_shared_file_list_create(
        &self,
        allocator: CFAllocatorRef,
        list_type: CFStringRef,
        list_options: CFTypeRef,
    ) -> LSSharedFileListRef {
        // SAFETY: We're calling an unsafe Core Foundation API.
        // The safety requirements are enforced by the caller.
        unsafe { core_services::LSSharedFileListCreate(allocator, list_type, list_options) }
    }

    unsafe fn ls_shared_file_list_copy_snapshot(
        &self,
        list: LSSharedFileListRef,
        seed: *mut u32,
    ) -> CFArrayRef {
        // SAFETY: We're calling an unsafe Core Foundation API.
        // The safety requirements are enforced by the caller.
        unsafe { core_services::LSSharedFileListCopySnapshot(list, seed) }
    }
}
