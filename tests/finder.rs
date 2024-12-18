#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
use coverage_helper::test;

use core_foundation::{
    array::{CFArray, CFArrayRef},
    base::{CFAllocatorRef, CFTypeRef, TCFType},
    string::{CFString, CFStringRef},
    url::{CFURL, CFURLRef, kCFURLPOSIXPathStyle},
};
use core_services::{LSSharedFileListItemRef, LSSharedFileListRef};
use favkit::{
    Finder,
    finder::{FinderError, Result, SidebarItem, Target},
    system::{api::MacOsApi, favorites::FavoritesError},
};

// Helper functions for creating test data
fn create_mock_item(id: i32) -> LSSharedFileListItemRef {
    id as LSSharedFileListItemRef
}

fn create_cf_string(s: &str) -> CFString {
    if s.is_empty() {
        CFString::new("")
    } else {
        CFString::new(s)
    }
}

fn create_cf_url(path: &str) -> CFURL {
    let cf_path = CFString::new(path);
    let is_file_path = !path.contains("://");
    CFURL::from_file_system_path(cf_path, kCFURLPOSIXPathStyle, is_file_path)
}

/// Test builder for Finder tests
struct FinderTest {
    finder: Finder,
}

impl FinderTest {
    /// Creates a new test with custom mock API
    fn with_mock_api(mock_api: MockMacOsApi) -> Self {
        Self {
            finder: Finder::new(mock_api),
        }
    }

    /// Creates a test with a list of favorite items
    fn with_favorites(items: Vec<LSSharedFileListItemRef>) -> Self {
        let mock_api = MockMacOsApi::new()
            .with_items(items)
            .with_list_create(|| 1 as LSSharedFileListRef);
        Self::with_mock_api(mock_api)
    }

    /// Creates a test with a single favorite item and its metadata
    fn with_favorite(item_id: i32, display_name: Option<&str>, url_path: &str) -> Self {
        let item = create_mock_item(item_id);
        let cf_string = display_name.map(create_cf_string);
        let cf_url = create_cf_url(url_path);

        let mock_api = MockMacOsApi::new()
            .with_items(vec![item])
            .with_list_create(|| 1 as LSSharedFileListRef)
            .with_display_name(move |_| {
                cf_string
                    .as_ref()
                    .map_or(std::ptr::null_mut(), |s| s.as_concrete_TypeRef())
            })
            .with_resolved_url(move |_| cf_url.as_concrete_TypeRef());

        Self::with_mock_api(mock_api)
    }

    /// Lists favorites and returns the result
    fn list_favorites(&self) -> Result<Vec<SidebarItem>> {
        self.finder.get_favorites_list()
    }
}

type ListCreateFn = Box<dyn Fn() -> LSSharedFileListRef>;
type SnapshotFn = Box<dyn Fn(LSSharedFileListRef) -> CFArrayRef>;
type DisplayNameFn = Box<dyn Fn(LSSharedFileListItemRef) -> CFStringRef>;
type ResolvedUrlFn = Box<dyn Fn(LSSharedFileListItemRef) -> CFURLRef>;

struct MockMacOsApi {
    list_create_fn: ListCreateFn,
    snapshot_fn: SnapshotFn,
    display_name_fn: DisplayNameFn,
    resolved_url_fn: ResolvedUrlFn,
    items: Option<CFArray<LSSharedFileListItemRef>>,
}

impl Default for MockMacOsApi {
    fn default() -> Self {
        Self::new()
    }
}

impl MockMacOsApi {
    fn new() -> Self {
        Self {
            list_create_fn: Box::new(|| 1 as LSSharedFileListRef),
            snapshot_fn: Box::new(|_| std::ptr::null_mut()),
            display_name_fn: Box::new(|_| std::ptr::null_mut()),
            resolved_url_fn: Box::new(|_| std::ptr::null_mut()),
            items: None,
        }
    }

    fn with_items(mut self, items: Vec<LSSharedFileListItemRef>) -> Self {
        let array = CFArray::from_copyable(&items);
        self.items = Some(array);
        self
    }

    fn with_list_create<F>(mut self, f: F) -> Self
    where
        F: Fn() -> LSSharedFileListRef + 'static,
    {
        self.list_create_fn = Box::new(f);
        self
    }

    fn with_snapshot<F>(mut self, f: F) -> Self
    where
        F: Fn(LSSharedFileListRef) -> CFArrayRef + 'static,
    {
        self.snapshot_fn = Box::new(f);
        self
    }

    fn with_display_name<F>(mut self, f: F) -> Self
    where
        F: Fn(LSSharedFileListItemRef) -> CFStringRef + 'static,
    {
        self.display_name_fn = Box::new(f);
        self
    }

    fn with_resolved_url<F>(mut self, f: F) -> Self
    where
        F: Fn(LSSharedFileListItemRef) -> CFURLRef + 'static,
    {
        self.resolved_url_fn = Box::new(f);
        self
    }
}

impl MacOsApi for MockMacOsApi {
    unsafe fn ls_shared_file_list_create(
        &self,
        _allocator: CFAllocatorRef,
        _list_type: CFStringRef,
        _list_options: CFTypeRef,
    ) -> LSSharedFileListRef {
        (self.list_create_fn)()
    }

    unsafe fn ls_shared_file_list_copy_snapshot(
        &self,
        list: LSSharedFileListRef,
        _seed: *mut u32,
    ) -> CFArrayRef {
        if let Some(array) = &self.items {
            array.as_concrete_TypeRef()
        } else {
            (self.snapshot_fn)(list)
        }
    }

    unsafe fn ls_shared_file_list_item_copy_display_name(
        &self,
        item: LSSharedFileListItemRef,
    ) -> CFStringRef {
        (self.display_name_fn)(item)
    }

    unsafe fn ls_shared_file_list_item_copy_resolved_url(
        &self,
        item: LSSharedFileListItemRef,
        _flags: core_services::LSSharedFileListResolutionFlags,
        _error: *mut core_foundation::error::CFErrorRef,
    ) -> CFURLRef {
        (self.resolved_url_fn)(item)
    }
}

#[test]
fn should_fail_when_list_handle_is_null() -> Result<()> {
    let result =
        FinderTest::with_mock_api(MockMacOsApi::new().with_list_create(std::ptr::null_mut))
            .list_favorites();

    assert!(matches!(
        result,
        Err(FinderError::AccessError(FavoritesError::NullListHandle))
    ));
    Ok(())
}

#[test]
fn should_fail_when_snapshot_handle_is_null() -> Result<()> {
    let result = FinderTest::with_mock_api(
        MockMacOsApi::new()
            .with_list_create(|| 1 as LSSharedFileListRef)
            .with_snapshot(|_| std::ptr::null_mut()),
    )
    .list_favorites();

    assert!(matches!(
        result,
        Err(FinderError::AccessError(FavoritesError::NullSnapshotHandle))
    ));
    Ok(())
}

#[test]
fn should_return_empty_list_when_no_favorites() -> Result<()> {
    let favorites = FinderTest::with_favorites(vec![]).list_favorites()?;

    assert_eq!(favorites, Vec::<SidebarItem>::new());
    Ok(())
}

#[test]
fn should_return_favorite_with_display_name_and_url() -> Result<()> {
    let favorites = FinderTest::with_favorite(1, Some("Documents"), "/Users/user/Documents")
        .list_favorites()?;

    assert_eq!(favorites, vec![SidebarItem::new(
        Some("Documents".to_string()),
        Target("file:///Users/user/Documents/".to_string()),
    )]);
    Ok(())
}

#[test]
fn should_handle_favorite_with_null_display_name() -> Result<()> {
    let favorites = FinderTest::with_favorite(1, None, "/Users/user/Downloads").list_favorites()?;

    assert_eq!(favorites, vec![SidebarItem::new(
        None,
        Target("file:///Users/user/Downloads/".to_string()),
    )]);
    Ok(())
}

#[test]
fn should_handle_airdrop_item() -> Result<()> {
    let favorites =
        FinderTest::with_favorite(1, Some(""), "nwnode://domain-AirDrop").list_favorites()?;

    assert_eq!(favorites, vec![SidebarItem::new(
        None,
        Target("nwnode://domain-AirDrop".to_string()),
    )]);

    assert_eq!(
        format!("{}", favorites[0]),
        "<no name> -> nwnode://domain-AirDrop"
    );
    Ok(())
}
