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
use mockall::predicate::*;

mockall::mock! {
    pub MacOsApi {}

    impl favkit::system::api::MacOsApi for MacOsApi {
        unsafe fn ls_shared_file_list_create(
            &self,
            allocator: CFAllocatorRef,
            list_type: CFStringRef,
            list_options: CFTypeRef,
        ) -> LSSharedFileListRef;

        unsafe fn ls_shared_file_list_copy_snapshot(
            &self,
            list: LSSharedFileListRef,
            seed: *mut u32,
        ) -> CFArrayRef;

        unsafe fn ls_shared_file_list_item_copy_display_name(
            &self,
            item: LSSharedFileListItemRef,
        ) -> CFStringRef;


        unsafe fn ls_shared_file_list_item_copy_resolved_url(
            &self,
            item: LSSharedFileListItemRef,
            flags: LSSharedFileListResolutionFlags,
            error: *mut CFErrorRef,
        ) -> CFURLRef;
    }
}
