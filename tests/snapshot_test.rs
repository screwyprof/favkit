mod common;

use common::ApiCallRecorder;
use core_foundation::{base::TCFType, string::CFString, url::CFURL};
use favkit::{sidebar::SidebarItem, MacOsApi};

#[test]
fn test_get_favorites_snapshot_in_isolation() {
    let recorder =
        ApiCallRecorder::with_items(vec![SidebarItem::applications(), SidebarItem::downloads()]);

    unsafe {
        // Step 1: Get list reference
        let list = recorder.get_favorites_list();
        assert!(!list.is_null(), "favorites list should not be null");

        // Step 2: Get snapshot
        let mut seed = 0;
        let array = recorder.get_favorites_snapshot(list, &mut seed);
        assert_eq!(array.len(), 2);

        // Step 3: Let array drop naturally
    }
}

#[test]
fn test_get_item_display_name_in_isolation() {
    let recorder =
        ApiCallRecorder::with_items(vec![SidebarItem::applications(), SidebarItem::downloads()]);

    unsafe {
        // Step 1: Get first item
        let item = recorder.get_test_item(0);
        assert!(!item.is_null(), "item should not be null");

        // Step 2: Get display name
        let name_ref = recorder.get_item_display_name(item);
        assert!(!name_ref.is_null(), "display name should not be null");

        // Step 3: Convert to string
        let name = CFString::wrap_under_create_rule(name_ref);
        let string = name.to_string();
        assert_eq!(string, "Applications");

        // Step 4: Let string drop naturally
    }
}

#[test]
fn test_get_item_url_in_isolation() {
    let recorder =
        ApiCallRecorder::with_items(vec![SidebarItem::applications(), SidebarItem::downloads()]);

    unsafe {
        // Step 1: Get first item
        let item = recorder.get_test_item(0);
        assert!(!item.is_null(), "item should not be null");

        // Step 2: Get resolved URL
        let url_ref = recorder.get_item_url(item);
        assert!(!url_ref.is_null(), "resolved URL should not be null");

        // Step 3: Convert to string
        let url = CFURL::wrap_under_create_rule(url_ref);
        let cf_string = url.get_string();
        let string = cf_string.to_string();
        assert_eq!(string, "file:///Applications");

        // Step 4: Let URL drop naturally
    }
}
