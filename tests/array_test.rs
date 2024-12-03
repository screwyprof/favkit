use core_foundation::{
    array::{CFArray, CFArrayCreate},
    base::{kCFAllocatorDefault, CFIndex, TCFType},
};
use core_services::{LSSharedFileListItemRef, LSSharedFileListRef};
use std::{ffi::c_void, ptr};

#[test]
fn test_basic_cfarray() {
    unsafe {
        // Step 1: Create some test data
        let values = [1 as *const c_void, 2 as *const c_void];

        // Step 2: Create CFArray
        let array_ref = CFArrayCreate(
            kCFAllocatorDefault,
            values.as_ptr(),
            values.len() as CFIndex,
            ptr::null(),
        );
        assert!(!array_ref.is_null());

        // Step 3: Wrap it in a safe Rust type
        let array = CFArray::<*const c_void>::wrap_under_create_rule(array_ref);

        // Step 4: Test array properties
        assert_eq!(array.len(), 2);

        // Step 5: Array will be automatically released when dropped
    }
}

#[test]
fn test_item_ref_array() {
    unsafe {
        // Step 1: Create some test item refs
        let values = [1 as LSSharedFileListItemRef, 2 as LSSharedFileListItemRef];

        // Step 2: Create CFArray
        let array_ref = CFArrayCreate(
            kCFAllocatorDefault,
            values.as_ptr() as *const *const c_void,
            values.len() as CFIndex,
            ptr::null(),
        );
        assert!(!array_ref.is_null());

        // Step 3: Wrap it in a safe Rust type
        let array = CFArray::<LSSharedFileListItemRef>::wrap_under_create_rule(array_ref);

        // Step 4: Test array properties
        assert_eq!(array.len(), 2);

        // Step 5: Array will be automatically released when dropped
    }
}

#[test]
fn test_mock_snapshot() {
    unsafe {
        // Step 1: Create a list (simulating create_favorites_list)
        let _list = 1 as LSSharedFileListRef;

        // Step 2: Create and fill a vector of item refs
        let values = [1 as LSSharedFileListItemRef, 2 as LSSharedFileListItemRef];

        // Step 3: Create the array (simulating copy_snapshot)
        let array_ref = CFArrayCreate(
            kCFAllocatorDefault,
            values.as_ptr() as *const *const c_void,
            values.len() as CFIndex,
            ptr::null(),
        );
        assert!(!array_ref.is_null());

        // Step 4: Wrap the array (simulating what SidebarApi does)
        let array = CFArray::<LSSharedFileListItemRef>::wrap_under_create_rule(array_ref);
        assert_eq!(array.len(), 2);

        // Step 5: Let array drop naturally (no manual release)
    }
}
