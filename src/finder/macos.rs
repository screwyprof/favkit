use core_foundation::{array::CFArray, string::CFStringRef, url::CFURLRef};
use core_services::{LSSharedFileListItemRef, LSSharedFileListRef};

/// Trait for interacting with MacOS APIs
/// This allows us to mock the MacOS API for testing
pub trait MacOsApi {
    /// Get the favorites list
    /// 
    /// # Safety
    /// 
    /// This function is unsafe because it calls into Core Foundation APIs
    unsafe fn get_favorites_list(&self) -> LSSharedFileListRef;

    /// Get a snapshot of the favorites list
    /// 
    /// # Safety
    /// 
    /// This function is unsafe because it calls into Core Foundation APIs
    unsafe fn get_favorites_snapshot(
        &self,
        list: LSSharedFileListRef,
        seed: &mut u32,
    ) -> CFArray<LSSharedFileListItemRef>;

    /// Get the display name of a list item
    /// 
    /// # Safety
    /// 
    /// This function is unsafe because it calls into Core Foundation APIs
    unsafe fn get_item_display_name(&self, item: LSSharedFileListItemRef) -> CFStringRef;

    /// Get the URL of a list item
    /// 
    /// # Safety
    /// 
    /// This function is unsafe because it calls into Core Foundation APIs
    unsafe fn get_item_url(&self, item: LSSharedFileListItemRef) -> CFURLRef;
}
