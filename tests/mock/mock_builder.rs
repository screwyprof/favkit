use std::rc::Rc;

use core_foundation::{
    array::{CFArray, CFArrayRef},
    base::{CFAllocatorRef, CFTypeRef, TCFType},
    error::CFErrorRef,
    string::{CFString, CFStringRef},
    url::{CFURL, CFURLRef, kCFURLPOSIXPathStyle},
};
use core_services::{LSSharedFileListItemRef, LSSharedFileListRef};

use super::{favorites::FavoriteItem, mac_os_api::MockMacOsApi};

// States
pub struct Initial;
pub struct WithNullFavorites;
pub struct WithNullSnapshot;
pub struct WithFavorites {
    items: Vec<FavoriteItem>,
}

pub struct MockBuilder<State> {
    state: State,
}

// Initial state transitions
impl MockBuilder<Initial> {
    pub fn new() -> Self {
        Self { state: Initial }
    }

    pub fn with_null_favorites(self) -> MockBuilder<WithNullFavorites> {
        MockBuilder {
            state: WithNullFavorites,
        }
    }

    pub fn with_null_snapshot(self) -> MockBuilder<WithNullSnapshot> {
        MockBuilder {
            state: WithNullSnapshot,
        }
    }

    pub fn with_favorites(self, items: Vec<FavoriteItem>) -> MockBuilder<WithFavorites> {
        MockBuilder {
            state: WithFavorites { items },
        }
    }
}

// Terminal state builders
impl MockBuilder<WithNullFavorites> {
    fn create_null_list(_: CFAllocatorRef, _: CFStringRef, _: CFTypeRef) -> LSSharedFileListRef {
        std::ptr::null_mut()
    }

    pub fn build(self) -> MockMacOsApi {
        let mut mock = MockMacOsApi::new();
        mock.expect_ls_shared_file_list_create()
            .returning_st(Self::create_null_list);
        mock
    }
}

impl MockBuilder<WithNullSnapshot> {
    fn create_list(_: CFAllocatorRef, _: CFStringRef, _: CFTypeRef) -> LSSharedFileListRef {
        1 as *mut _
    }

    fn create_null_snapshot(_: LSSharedFileListRef, _: *mut u32) -> CFArrayRef {
        std::ptr::null()
    }

    pub fn build(self) -> MockMacOsApi {
        let mut mock = MockMacOsApi::new();
        mock.expect_ls_shared_file_list_create()
            .returning_st(Self::create_list);
        mock.expect_ls_shared_file_list_copy_snapshot()
            .returning_st(Self::create_null_snapshot);
        mock
    }
}

impl MockBuilder<WithFavorites> {
    fn create_list(_: CFAllocatorRef, _: CFStringRef, _: CFTypeRef) -> LSSharedFileListRef {
        1 as *mut _
    }

    fn create_snapshot_items(count: usize) -> Vec<LSSharedFileListItemRef> {
        (1..=count)
            .map(|i| (i as i32) as LSSharedFileListItemRef)
            .collect()
    }

    fn create_display_name(name: Option<&str>) -> CFString {
        let name = name.unwrap_or_default();
        CFString::new(name)
    }

    fn create_display_names(items: &[FavoriteItem]) -> Vec<CFString> {
        items
            .iter()
            .map(|item| Self::create_display_name(item.name.as_deref()))
            .collect()
    }

    fn create_url(path: &str) -> CFURL {
        let is_dir = path.ends_with('/');
        let file_path = CFString::new(path);
        CFURL::from_file_system_path(file_path, kCFURLPOSIXPathStyle, is_dir)
    }

    fn create_urls(items: &[FavoriteItem]) -> Vec<CFURL> {
        items
            .iter()
            .map(|item| Self::create_url(&item.path))
            .collect()
    }

    fn get_item_index(item: LSSharedFileListItemRef) -> usize {
        ((item as i32) - 1) as usize
    }

    fn create_snapshot_closure(
        array: Rc<CFArray<LSSharedFileListItemRef>>,
    ) -> impl Fn(LSSharedFileListRef, *mut u32) -> CFArrayRef {
        move |_, _| array.as_concrete_TypeRef()
    }

    fn create_display_name_closure(
        strings: Rc<Vec<CFString>>,
    ) -> impl Fn(LSSharedFileListItemRef) -> CFStringRef {
        move |item| {
            let idx = Self::get_item_index(item);
            strings[idx].as_concrete_TypeRef()
        }
    }

    fn create_url_closure(
        urls: Rc<Vec<CFURL>>,
    ) -> impl FnMut(LSSharedFileListItemRef, u32, *mut CFErrorRef) -> CFURLRef {
        move |item, _, _| {
            let idx = Self::get_item_index(item);
            urls[idx].as_concrete_TypeRef()
        }
    }

    pub fn build(self) -> MockMacOsApi {
        let mut mock = MockMacOsApi::new();

        // Configure list creation
        mock.expect_ls_shared_file_list_create()
            .returning_st(Self::create_list);

        // Create snapshot items and keep them alive
        let snapshot_items = Self::create_snapshot_items(self.state.items.len());
        let array = Rc::new(CFArray::from_copyable(&snapshot_items));
        mock.expect_ls_shared_file_list_copy_snapshot()
            .returning_st(Self::create_snapshot_closure(array));

        // Configure display names
        let strings = Rc::new(Self::create_display_names(&self.state.items));
        mock.expect_ls_shared_file_list_item_copy_display_name()
            .returning_st(Self::create_display_name_closure(strings));

        // Configure URLs
        let urls = Rc::new(Self::create_urls(&self.state.items));
        mock.expect_ls_shared_file_list_item_copy_resolved_url()
            .returning_st(Self::create_url_closure(urls));

        mock
    }
}
