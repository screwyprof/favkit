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
    system::{
        api::MacOsApi,
        favorites::{DisplayName, FavoritesError, Url},
    },
};
use std::rc::Rc;

mod test_data {
    pub const DOCUMENTS_NAME: &str = "Documents";
    pub const DOCUMENTS_PATH: &str = "/Users/user/Documents/";
    pub const AIRDROP_URL: &str = "nwnode://domain-AirDrop";
}

/// Test builder for mock API
struct MockBuilder {
    items: Vec<(Option<&'static str>, &'static str)>,
}

impl MockBuilder {
    fn new() -> Self {
        Self { items: Vec::new() }
    }

    fn with_items(mut self, items: Vec<(Option<&'static str>, &'static str)>) -> Self {
        self.items = items;
        self
    }

    fn build(&self) -> MockMacOsApi {
        let favorites = self
            .items
            .iter()
            .enumerate()
            .map(|(id, (name, url))| FavoriteItem::new(id as i32 + 1, *name, url))
            .collect();
        MockMacOsApi::new().with_favorites(favorites)
    }
}

type ListCreateFn = Box<dyn Fn() -> LSSharedFileListRef>;
type SnapshotFn = Box<dyn Fn(LSSharedFileListRef) -> CFArrayRef>;
type DisplayNameFn = Box<dyn Fn(LSSharedFileListItemRef) -> CFStringRef>;
type ResolvedUrlFn = Box<dyn Fn(LSSharedFileListItemRef) -> CFURLRef>;

/// Represents a favorite item with its Core Foundation data
struct FavoriteItem {
    id: i32,
    display_name: Option<DisplayName>,
    url: Url,
}

impl FavoriteItem {
    fn new(id: i32, display_name: Option<&str>, url: &str) -> Self {
        let display_name = display_name.map(|name| {
            let cf_string = CFString::new(name);
            DisplayName::try_from(cf_string.as_concrete_TypeRef()).unwrap()
        });

        let is_dir = url.ends_with('/');
        let file_path = CFString::new(url);
        let url_cf = CFURL::from_file_system_path(file_path, kCFURLPOSIXPathStyle, is_dir);
        let url = Url::try_from(url_cf.as_concrete_TypeRef()).unwrap();

        Self {
            id,
            display_name,
            url,
        }
    }
}

struct MockMacOsApi {
    list_create_fn: ListCreateFn,
    snapshot_fn: SnapshotFn,
    display_name_fn: DisplayNameFn,
    resolved_url_fn: ResolvedUrlFn,
    snapshot_array: Option<CFArray<LSSharedFileListItemRef>>,
    display_names: Vec<Option<DisplayName>>,
    urls: Vec<Url>,
}

impl Default for MockMacOsApi {
    fn default() -> Self {
        Self {
            list_create_fn: Box::new(std::ptr::null_mut),
            snapshot_fn: Box::new(|_| std::ptr::null_mut()),
            display_name_fn: Box::new(|_| std::ptr::null_mut()),
            resolved_url_fn: Box::new(|_| std::ptr::null_mut()),
            snapshot_array: None,
            display_names: Vec::new(),
            urls: Vec::new(),
        }
    }
}

impl MockMacOsApi {
    fn new() -> Self {
        Self::default()
    }

    fn with_favorites(mut self, favorites: Vec<FavoriteItem>) -> Self {
        let mut item_refs = Vec::new();
        let mut display_names = Vec::new();
        let mut urls = Vec::new();
        let mut lookup_data = Vec::new();

        // First, store all domain objects
        for item in favorites {
            item_refs.push(Self::to_list_item_ref(item.id));
            display_names.push(item.display_name);
            urls.push(item.url);
        }

        // Then build lookup data using references to stored objects
        for (i, id) in item_refs.iter().enumerate() {
            lookup_data.push((
                *id as i32,
                display_names[i]
                    .as_ref()
                    .map(|name| name.into())
                    .unwrap_or_else(|| CFString::new("").as_concrete_TypeRef()),
                (&urls[i]).into(),
            ));
        }

        let array = CFArray::from_copyable(&item_refs);
        let items_ref = array.as_concrete_TypeRef();

        self.snapshot_array = Some(array);
        self.snapshot_fn = Box::new(move |_| items_ref);
        self.list_create_fn = Box::new(|| 1 as LSSharedFileListRef);
        self.display_names = display_names;
        self.urls = urls;

        let lookup_data = Rc::new(lookup_data);
        let lookup_data_clone = Rc::clone(&lookup_data);

        self.display_name_fn =
            Box::new(move |item_ref| Self::lookup_display_name_ref(&lookup_data, item_ref as i32));

        self.resolved_url_fn =
            Box::new(move |item_ref| Self::lookup_url_ref(&lookup_data_clone, item_ref as i32));

        self
    }

    fn to_list_item_ref(id: i32) -> LSSharedFileListItemRef {
        id as LSSharedFileListItemRef
    }

    fn failing_list() -> Self {
        Self {
            list_create_fn: Box::new(std::ptr::null_mut),
            ..Default::default()
        }
    }

    fn failing_snapshot() -> Self {
        Self {
            list_create_fn: Box::new(|| 1 as LSSharedFileListRef),
            snapshot_fn: Box::new(|_| std::ptr::null_mut()),
            ..Default::default()
        }
    }

    fn lookup_display_name_ref(
        lookup_data: &[(i32, CFStringRef, CFURLRef)],
        id: i32,
    ) -> CFStringRef {
        lookup_data
            .iter()
            .find(|(item_id, _, _)| *item_id == id)
            .map(|(_, name_ref, _)| *name_ref)
            .unwrap_or_else(|| CFString::new("").as_concrete_TypeRef())
    }

    fn lookup_url_ref(lookup_data: &[(i32, CFStringRef, CFURLRef)], id: i32) -> CFURLRef {
        lookup_data
            .iter()
            .find(|(item_id, _, _)| *item_id == id)
            .map(|(_, _, url_ref)| *url_ref)
            .unwrap_or_else(std::ptr::null)
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
        (self.snapshot_fn)(list)
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
    // Arrange
    let expected_error = Err(FinderError::AccessError(FavoritesError::NullListHandle));
    let mock_api = MockMacOsApi::failing_list();
    let finder = Finder::new(mock_api);

    // Act
    let result = finder.get_favorites_list();

    // Assert
    assert_eq!(result, expected_error);
    Ok(())
}

#[test]
fn should_fail_when_snapshot_handle_is_null() -> Result<()> {
    // Arrange
    let expected_error = Err(FinderError::AccessError(FavoritesError::NullSnapshotHandle));
    let mock_api = MockMacOsApi::failing_snapshot();
    let finder = Finder::new(mock_api);

    // Act
    let result = finder.get_favorites_list();

    // Assert
    assert_eq!(result, expected_error);
    Ok(())
}

#[test]
fn should_return_empty_list_when_no_favorites() -> Result<()> {
    // Arrange
    let expected_result: Vec<SidebarItem> = vec![];
    let mock_api = MockBuilder::new().with_items(vec![]).build();
    let finder = Finder::new(mock_api);

    // Act
    let result = finder.get_favorites_list()?;

    // Assert
    assert_eq!(result, expected_result);
    Ok(())
}

#[test]
fn should_return_favorite_with_display_name_and_url() -> Result<()> {
    // Arrange
    let expected_result = vec![SidebarItem::new(
        Some(test_data::DOCUMENTS_NAME.to_string()),
        Target(format!("file://{}", test_data::DOCUMENTS_PATH)),
    )];
    let mock_api = MockBuilder::new()
        .with_items(vec![(
            Some(test_data::DOCUMENTS_NAME),
            test_data::DOCUMENTS_PATH,
        )])
        .build();
    let finder = Finder::new(mock_api);

    // Act
    let result = finder.get_favorites_list()?;

    // Assert
    assert_eq!(result, expected_result);
    Ok(())
}

#[test]
fn should_handle_airdrop_item() -> Result<()> {
    // Arrange
    let expected_result = vec![SidebarItem::new(
        None,
        Target(test_data::AIRDROP_URL.to_string()),
    )];
    let mock_api = MockBuilder::new()
        .with_items(vec![(None, test_data::AIRDROP_URL)])
        .build();
    let finder = Finder::new(mock_api);

    // Act
    let result = finder.get_favorites_list()?;

    // Assert
    assert_eq!(result, expected_result);
    Ok(())
}

#[test]
fn should_handle_multiple_favorites() -> Result<()> {
    // Arrange
    let expected_result = vec![
        SidebarItem::new(None, Target(test_data::AIRDROP_URL.to_string())),
        SidebarItem::new(
            Some("Applications".to_string()),
            Target("file:///Applications/".to_string()),
        ),
        SidebarItem::new(
            Some("Downloads".to_string()),
            Target("file:///Users/user/Downloads/".to_string()),
        ),
    ];
    let mock_api = MockBuilder::new()
        .with_items(vec![
            (None, test_data::AIRDROP_URL),
            (Some("Applications"), "/Applications/"),
            (Some("Downloads"), "/Users/user/Downloads/"),
        ])
        .build();
    let finder = Finder::new(mock_api);

    // Act
    let result = finder.get_favorites_list()?;

    // Assert
    assert_eq!(result, expected_result);
    Ok(())
}
