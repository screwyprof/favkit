#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
use coverage_helper::test;

use core_foundation::{
    array::{CFArray, CFArrayRef},
    base::{CFAllocatorRef, CFTypeRef, TCFType},
    string::{CFString, CFStringRef},
    url::{CFURL, CFURLRef, kCFURLPOSIXPathStyle},
};
use core_services::{LSSharedFileListItemRef, LSSharedFileListRef, OpaqueLSSharedFileListItemRef};
use favkit::{
    Finder,
    finder::{FinderError, Result, SidebarItem, Target},
    system::{
        api::MacOsApi,
        favorites::{DisplayName, FavoritesError, Snapshot, Url},
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
            .map(|(name, url)| FavoriteItem::new(*name, url))
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
    display_name: DisplayName,
    url: Url,
}

impl FavoriteItem {
    fn new(display_name: Option<&str>, url: &str) -> Self {
        let display_name = {
            let name = display_name.unwrap_or_default();
            let cf_string = CFString::new(name);
            DisplayName::try_from(cf_string.as_concrete_TypeRef()).unwrap()
        };

        let url = {
            let is_dir = url.ends_with('/');
            let file_path = CFString::new(url);
            let url_cf = CFURL::from_file_system_path(file_path, kCFURLPOSIXPathStyle, is_dir);
            Url::try_from(url_cf.as_concrete_TypeRef()).unwrap()
        };

        Self { display_name, url }
    }
}

struct MockMacOsApi {
    display_names: Rc<Vec<DisplayName>>,
    urls: Rc<Vec<Url>>,
    snapshot: Option<Snapshot>,
    list_create_fn: ListCreateFn,
    snapshot_fn: SnapshotFn,
    display_name_fn: DisplayNameFn,
    resolved_url_fn: ResolvedUrlFn,
}

impl Default for MockMacOsApi {
    fn default() -> Self {
        Self {
            display_names: Rc::new(Vec::new()),
            urls: Rc::new(Vec::new()),
            snapshot: None,
            list_create_fn: Box::new(std::ptr::null_mut),
            snapshot_fn: Box::new(|_| std::ptr::null_mut()),
            display_name_fn: Box::new(|_| std::ptr::null_mut()),
            resolved_url_fn: Box::new(|_| std::ptr::null_mut()),
        }
    }
}

impl MockMacOsApi {
    fn new() -> Self {
        Self {
            display_names: Rc::new(Vec::new()),
            urls: Rc::new(Vec::new()),
            snapshot: None,
            list_create_fn: Box::new(std::ptr::null_mut),
            snapshot_fn: Box::new(|_| std::ptr::null()),
            display_name_fn: Box::new(|_| std::ptr::null()),
            resolved_url_fn: Box::new(|_| std::ptr::null()),
        }
    }

    fn with_favorites(mut self, favorites: Vec<FavoriteItem>) -> Self {
        // Create everything in one pass
        let (raw_refs, rest): (Vec<_>, Vec<_>) = favorites
            .into_iter()
            .enumerate()
            .map(|(i, item)| {
                let raw_ref = ((i + 1) as i32) as *mut OpaqueLSSharedFileListItemRef;
                (raw_ref, (item.display_name, item.url))
            })
            .unzip();

        let (display_names, urls) = rest.into_iter().unzip();

        // Store display names and urls
        self.display_names = Rc::new(display_names);
        self.urls = Rc::new(urls);

        // Create snapshot
        let array = CFArray::from_copyable(&raw_refs);
        let array_ref = array.as_concrete_TypeRef();
        self.snapshot = Snapshot::try_from(array_ref).ok();

        // 1. Mock ls_shared_file_list_create
        let raw_list = 1 as LSSharedFileListRef;
        self.list_create_fn = Box::new(move || raw_list);

        // 2. Mock ls_shared_file_list_copy_snapshot
        let snapshot_ref = self.snapshot.as_ref().map(Into::into).unwrap();
        self.snapshot_fn = Box::new(move |_| snapshot_ref);

        // 3. Mock ls_shared_file_list_item_copy_display_name
        let display_names = Rc::clone(&self.display_names);
        self.display_name_fn = Box::new(move |item_ref| {
            let idx = (item_ref as i32 - 1) as usize;
            (&display_names[idx]).into()
        });

        // 4. Mock ls_shared_file_list_item_copy_resolved_url
        let urls = Rc::clone(&self.urls);
        self.resolved_url_fn = Box::new(move |item_ref| {
            let idx = (item_ref as i32 - 1) as usize;
            (&urls[idx]).into()
        });

        self
    }

    fn failing_list() -> Self {
        Self {
            display_names: Rc::new(Vec::new()),
            urls: Rc::new(Vec::new()),
            snapshot: None,
            list_create_fn: Box::new(std::ptr::null_mut),
            snapshot_fn: Box::new(|_| std::ptr::null()),
            display_name_fn: Box::new(|_| std::ptr::null()),
            resolved_url_fn: Box::new(|_| std::ptr::null()),
        }
    }

    fn failing_snapshot() -> Self {
        Self {
            display_names: Rc::new(Vec::new()),
            urls: Rc::new(Vec::new()),
            snapshot: None,
            list_create_fn: Box::new(|| 1 as LSSharedFileListRef),
            snapshot_fn: Box::new(|_| std::ptr::null()),
            display_name_fn: Box::new(|_| std::ptr::null()),
            resolved_url_fn: Box::new(|_| std::ptr::null()),
        }
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
