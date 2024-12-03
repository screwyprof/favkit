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
        println!("Step 1: Creating test data");
        let values = vec![1 as *const c_void, 2 as *const c_void];
        println!("Values: {:?}", values);

        // Step 2: Create CFArray
        println!("Step 2: Creating CFArray");
        let array_ref = CFArrayCreate(
            kCFAllocatorDefault,
            values.as_ptr(),
            values.len() as CFIndex,
            ptr::null(),
        );
        println!("Created array: {:?}", array_ref);
        assert!(!array_ref.is_null());

        // Step 3: Wrap it in a safe Rust type
        println!("Step 3: Wrapping array");
        let array = CFArray::<*const c_void>::wrap_under_create_rule(array_ref);
        println!("Wrapped array");

        // Step 4: Test array properties
        println!("Step 4: Testing array");
        assert_eq!(array.len(), 2);
        println!("Array length is correct: {}", array.len());

        // Step 5: Array will be automatically released when dropped
        println!("Step 5: Letting array be dropped");
    }
    println!("Test completed");
}

#[test]
fn test_item_ref_array() {
    unsafe {
        // Step 1: Create some test item refs
        println!("Step 1: Creating test item refs");
        let values = vec![1 as LSSharedFileListItemRef, 2 as LSSharedFileListItemRef];
        println!("Item refs: {:?}", values);

        // Step 2: Create CFArray
        println!("Step 2: Creating CFArray");
        let array_ref = CFArrayCreate(
            kCFAllocatorDefault,
            values.as_ptr() as *const *const c_void,
            values.len() as CFIndex,
            ptr::null(),
        );
        println!("Created array: {:?}", array_ref);
        assert!(!array_ref.is_null());

        // Step 3: Wrap it in a safe Rust type
        println!("Step 3: Wrapping array");
        let array = CFArray::<LSSharedFileListItemRef>::wrap_under_create_rule(array_ref);
        println!("Wrapped array");

        // Step 4: Test array properties
        println!("Step 4: Testing array");
        assert_eq!(array.len(), 2);
        println!("Array length is correct: {}", array.len());

        // Step 5: Array will be automatically released when dropped
        println!("Step 5: Letting array be dropped");
    }
    println!("Test completed");
}

#[test]
fn test_mock_snapshot() {
    unsafe {
        // Step 1: Create a list (simulating create_favorites_list)
        println!("Step 1: Creating list");
        let list = 1 as LSSharedFileListRef;
        println!("Created list: {:?}", list);

        // Step 2: Create and fill a vector of item refs
        println!("Step 2: Creating item refs");
        let values = vec![1 as LSSharedFileListItemRef, 2 as LSSharedFileListItemRef];
        println!("Created item refs: {:?}", values);

        // Step 3: Create the array (simulating copy_snapshot)
        println!("Step 3: Creating array");
        let array_ref = CFArrayCreate(
            kCFAllocatorDefault,
            values.as_ptr() as *const *const c_void,
            values.len() as CFIndex,
            ptr::null(),
        );
        println!("Created array: {:?}", array_ref);
        assert!(!array_ref.is_null());

        // Step 4: Wrap the array (simulating what SidebarApi does)
        println!("Step 4: Wrapping array");
        let array = CFArray::<LSSharedFileListItemRef>::wrap_under_create_rule(array_ref);
        println!("Array length: {}", array.len());
        assert_eq!(array.len(), 2);

        // Step 5: Let array drop naturally (no manual release)
        println!("Step 5: Test completed, letting array drop");
    }
}
