use core_foundation::{
    array::CFArrayRef,
    base::{CFAllocatorRef, CFTypeRef},
    string::CFStringRef,
};
use core_services::{LSSharedFileListItemRef, LSSharedFileListRef};

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
        unsafe { core_services::LSSharedFileListCreate(allocator, list_type, list_options) }
    }

    unsafe fn ls_shared_file_list_copy_snapshot(
        &self,
        list: LSSharedFileListRef,
        seed: *mut u32,
    ) -> CFArrayRef {
        unsafe { core_services::LSSharedFileListCopySnapshot(list, seed) }
    }

    unsafe fn ls_shared_file_list_item_copy_display_name(
        &self,
        item: LSSharedFileListItemRef,
    ) -> CFStringRef {
        unsafe { core_services::LSSharedFileListItemCopyDisplayName(item) }
    }
}
