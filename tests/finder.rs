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
fn should_return_error_when_list_handle_is_null() -> Result<()> {
    let mock_api = MockMacOsApi::new().with_list_create(std::ptr::null_mut);
    let finder = Finder::new(mock_api);

    let result = finder.get_favorites_list();

    assert!(matches!(
        result,
        Err(FinderError::AccessError(FavoritesError::NullListHandle))
    ));
    Ok(())
}

#[test]
fn should_return_error_when_snapshot_handle_is_null() -> Result<()> {
    let mock_api = MockMacOsApi::new()
        .with_list_create(|| 1 as LSSharedFileListRef)
        .with_snapshot(|_| std::ptr::null_mut());

    let finder = Finder::new(mock_api);

    let result = finder.get_favorites_list();

    assert!(matches!(
        result,
        Err(FinderError::AccessError(FavoritesError::NullSnapshotHandle))
    ));
    Ok(())
}

#[test]
fn should_get_empty_list_when_no_favorites() -> Result<()> {
    let items: Vec<LSSharedFileListItemRef> = vec![];
    let mock_api = MockMacOsApi::new()
        .with_items(items)
        .with_list_create(|| 1 as LSSharedFileListRef);

    let finder = Finder::new(mock_api);

    let favorites = finder.get_favorites_list()?;
    assert_eq!(favorites, Vec::<SidebarItem>::new());

    Ok(())
}

#[test]
fn should_get_favorite_with_display_name_and_url() -> Result<()> {
    // Create a mock item (using a non-null pointer)
    let item: LSSharedFileListItemRef = 1 as LSSharedFileListItemRef;
    let items = vec![item];

    // Create a display name
    let display_name = "Documents";
    let cf_string = CFString::from_static_string(display_name);

    // Create a URL
    let url = "/Users/user/Documents";
    let cf_path = CFString::new(url);
    let cf_url = CFURL::from_file_system_path(cf_path, kCFURLPOSIXPathStyle, true);

    let mock_api = MockMacOsApi::new()
        .with_items(items)
        .with_list_create(|| 1 as LSSharedFileListRef)
        .with_display_name(move |_| cf_string.as_concrete_TypeRef())
        .with_resolved_url(move |_| cf_url.as_concrete_TypeRef());

    let finder = Finder::new(mock_api);

    let favorites = finder.get_favorites_list()?;
    assert_eq!(favorites, vec![SidebarItem::new(
        Some(display_name.to_string()),
        Target("file:///Users/user/Documents/".to_string()),
    )]);

    Ok(())
}

#[test]
fn should_include_favorite_with_null_display_name() -> Result<()> {
    // Create two mock items (using non-null pointers)
    let item1: LSSharedFileListItemRef = 1 as LSSharedFileListItemRef;
    let item2: LSSharedFileListItemRef = 2 as LSSharedFileListItemRef;
    let items = vec![item1, item2];

    // Create a display name for the first item only
    let display_name = "Documents";
    let cf_string = CFString::from_static_string(display_name);

    // Create URLs for both items
    let url1 = "/Users/user/Documents";
    let url2 = "/Users/user/Downloads";
    let cf_path1 = CFString::new(url1);
    let cf_path2 = CFString::new(url2);
    let cf_url1 = CFURL::from_file_system_path(cf_path1, kCFURLPOSIXPathStyle, true);
    let cf_url2 = CFURL::from_file_system_path(cf_path2, kCFURLPOSIXPathStyle, true);

    let mock_api = MockMacOsApi::new()
        .with_items(items)
        .with_list_create(|| 1 as LSSharedFileListRef)
        .with_display_name(move |item| {
            if item == item1 {
                cf_string.as_concrete_TypeRef()
            } else {
                std::ptr::null_mut()
            }
        })
        .with_resolved_url(move |item| {
            if item == item1 {
                cf_url1.as_concrete_TypeRef()
            } else {
                cf_url2.as_concrete_TypeRef()
            }
        });

    let finder = Finder::new(mock_api);

    let favorites = finder.get_favorites_list()?;
    assert_eq!(favorites, vec![
        SidebarItem::new(
            Some(display_name.to_string()),
            Target("file:///Users/user/Documents/".to_string())
        ),
        SidebarItem::new(None, Target("file:///Users/user/Downloads/".to_string())),
    ]);

    Ok(())
}

#[test]
fn should_handle_airdrop_item() -> Result<()> {
    // Create a mock item (using a non-null pointer)
    let item: LSSharedFileListItemRef = 1 as LSSharedFileListItemRef;
    let items = vec![item];

    // Create a URL for AirDrop
    let url = "nwnode://domain-AirDrop";
    let cf_path = CFString::new(url);
    let cf_url = CFURL::from_file_system_path(cf_path, kCFURLPOSIXPathStyle, false);

    // Create an empty display name (this is what macOS returns for AirDrop)
    let empty_name = CFString::new("");

    let mock_api = MockMacOsApi::new()
        .with_items(items)
        .with_list_create(|| 1 as LSSharedFileListRef)
        .with_display_name(move |_| empty_name.as_concrete_TypeRef())
        .with_resolved_url(move |_| cf_url.as_concrete_TypeRef());

    let finder = Finder::new(mock_api);

    let favorites = finder.get_favorites_list()?;
    assert_eq!(favorites, vec![SidebarItem::new(
        None,
        Target("nwnode://domain-AirDrop".to_string())
    ),]);

    // Also verify the string representation
    assert_eq!(
        format!("{}", favorites[0]),
        "<no name> -> nwnode://domain-AirDrop"
    );

    Ok(())
}
