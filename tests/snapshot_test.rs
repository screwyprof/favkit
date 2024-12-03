mod common;

use common::ApiCallRecorder;
use core_foundation::{array::CFArray, base::TCFType, string::CFString, url::CFURL};
use core_services::LSSharedFileListItemRef;
use favkit::{sidebar::SidebarItem, MacOsApi};

#[test]
fn test_copy_snapshot_in_isolation() {
    println!("Starting copy_snapshot test");
    let recorder =
        ApiCallRecorder::with_items(vec![SidebarItem::applications(), SidebarItem::downloads()]);

    unsafe {
        // Step 1: Create list
        println!("Step 1: Creating list");
        let list = recorder.create_favorites_list();
        assert!(!list.is_null(), "favorites list should not be null");

        // Step 2: Get snapshot
        println!("Step 2: Getting snapshot");
        let mut seed = 0;
        let array_ref = recorder.copy_snapshot(list, &mut seed);
        assert!(!array_ref.is_null(), "snapshot should not be null");
        println!("Got array ref: {:?}", array_ref);

        // Step 3: Wrap array
        println!("Step 3: Wrapping array");
        let array = CFArray::<LSSharedFileListItemRef>::wrap_under_create_rule(array_ref);
        println!("Array length: {}", array.len());
        assert_eq!(array.len(), 2);

        // Step 4: Let array drop naturally
        println!("Step 4: Test completed, letting array drop");
    }
    println!("Test completed successfully");
}

#[test]
fn test_copy_display_name_in_isolation() {
    println!("Starting copy_display_name test");
    let recorder =
        ApiCallRecorder::with_items(vec![SidebarItem::applications(), SidebarItem::downloads()]);

    unsafe {
        // Step 1: Get first item
        println!("Step 1: Getting first item");
        let item = recorder.get_test_item(0);
        assert!(!item.is_null(), "item should not be null");

        // Step 2: Get display name
        println!("Step 2: Getting display name");
        let name_ref = recorder.copy_display_name(item);
        assert!(!name_ref.is_null(), "display name should not be null");
        println!("Got name ref: {:?}", name_ref);

        // Step 3: Convert to string
        println!("Step 3: Converting to string");
        let name = CFString::wrap_under_create_rule(name_ref);
        let string = name.to_string();
        println!("Display name: {}", string);
        assert_eq!(string, "Applications");

        // Step 4: Let string drop naturally
        println!("Step 4: Test completed, letting string drop");
    }
    println!("Test completed successfully");
}

#[test]
fn test_copy_resolved_url_in_isolation() {
    println!("Starting copy_resolved_url test");
    let recorder =
        ApiCallRecorder::with_items(vec![SidebarItem::applications(), SidebarItem::downloads()]);

    unsafe {
        // Step 1: Get first item
        println!("Step 1: Getting first item");
        let item = recorder.get_test_item(0);
        assert!(!item.is_null(), "item should not be null");

        // Step 2: Get resolved URL
        println!("Step 2: Getting resolved URL");
        let url_ref = recorder.copy_resolved_url(item);
        assert!(!url_ref.is_null(), "resolved URL should not be null");
        println!("Got URL ref: {:?}", url_ref);

        // Step 3: Convert to string
        println!("Step 3: Converting to string");
        let url = CFURL::wrap_under_create_rule(url_ref);
        let cf_string = url.get_string();
        let string = cf_string.to_string();
        println!("Resolved URL: {}", string);
        assert_eq!(string, "file:///Applications");

        // Step 4: Let URL drop naturally
        println!("Step 4: Test completed, letting URL drop");
    }
    println!("Test completed successfully");
}
