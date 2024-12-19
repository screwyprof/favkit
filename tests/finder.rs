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
    finder::{FinderError, Result, SidebarItem},
    system::{
        api::MacOsApi,
        favorites::{DisplayName, FavoritesError, Url},
    },
};

mod test_data {
    pub const DOCUMENTS_NAME: &str = "Documents";
    pub const DOCUMENTS_PATH: &str = "/Users/user/Documents/";
    pub const AIRDROP_URL: &str = "nwnode://domain-AirDrop";
}

/// Test builder for Finder tests
struct FinderTest {
    finder: Finder,
}

impl FinderTest {
    /// Creates a new test with custom mock API
    fn new(mock_api: MockMacOsApi) -> Self {
        Self {
            finder: Finder::new(mock_api),
        }
    }

    /// Creates a test with a single favorite item and its metadata
    fn with_favorite(display_name: Option<&str>, url: &str) -> Self {
        let mock_item = MockItem::new(1, display_name, url);
        let mock_api = MockMacOsApi::new().with_items(vec![mock_item]);
        Self::new(mock_api)
    }

    /// Creates a test with multiple favorite items
    fn with_favorites(items: Vec<(Option<&str>, &str)>) -> Self {
        let mock_items = items
            .into_iter()
            .enumerate()
            .map(|(id, (name, url))| MockItem::new(id as i32 + 1, name, url))
            .collect();
        let mock_api = MockMacOsApi::new().with_items(mock_items);
        Self::new(mock_api)
    }

    /// Lists favorites and returns the result
    fn list_favorites(&self) -> Result<Vec<SidebarItem>> {
        self.finder.get_favorites_list()
    }

    fn assert_has_favorite(&self, display_name: Option<&str>, url: &str) -> Result<()> {
        let favorites = self.list_favorites()?;
        assert!(favorites.iter().any(|item| {
            item.display_name().as_ref() == display_name.map(String::from).as_ref()
                && item.target().as_str() == url
        }));
        Ok(())
    }

    fn assert_is_empty(&self) -> Result<()> {
        let favorites = self.list_favorites()?;
        assert!(favorites.is_empty());
        Ok(())
    }
}

type ListCreateFn = Box<dyn Fn() -> LSSharedFileListRef>;
type SnapshotFn = Box<dyn Fn(LSSharedFileListRef) -> CFArrayRef>;
type DisplayNameFn = Box<dyn Fn(LSSharedFileListItemRef) -> CFStringRef>;
type ResolvedUrlFn = Box<dyn Fn(LSSharedFileListItemRef) -> CFURLRef>;

struct MockItem {
    id: i32,
    display_name: Option<DisplayName>,
    url: Url,
}

impl MockItem {
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
    items: Option<CFArray<LSSharedFileListItemRef>>,
    mock_items: Vec<MockItem>,
    display_names: Vec<CFString>,
    urls: Vec<CFURL>,
}

impl Default for MockMacOsApi {
    fn default() -> Self {
        Self {
            list_create_fn: Box::new(std::ptr::null_mut),
            snapshot_fn: Box::new(|_| std::ptr::null_mut()),
            display_name_fn: Box::new(|_| std::ptr::null_mut()),
            resolved_url_fn: Box::new(|_| std::ptr::null_mut()),
            items: None,
            mock_items: Vec::new(),
            display_names: Vec::new(),
            urls: Vec::new(),
        }
    }
}

impl MockMacOsApi {
    fn new() -> Self {
        Self::default()
    }

    fn with_items(mut self, mock_items: Vec<MockItem>) -> Self {
        let ids: Vec<_> = mock_items.iter().map(|item| item.id).collect();
        let items = ids
            .into_iter()
            .map(Self::create_mock_item)
            .collect::<Vec<_>>();

        // Set up items and snapshot
        let array = CFArray::from_copyable(&items);
        let items_ref = array.as_concrete_TypeRef();
        self.items = Some(array);
        self.snapshot_fn = Box::new(move |_| items_ref);

        // Set up list creation
        self.list_create_fn = Box::new(|| 1 as LSSharedFileListRef);

        // Store mock items for display name and URL lookups
        self.mock_items = mock_items;

        // Set up display name function
        let mut display_names = Vec::new();
        let mock_items = self
            .mock_items
            .iter()
            .map(|item| {
                let name_str = item
                    .display_name
                    .as_ref()
                    .map(|name| name.to_string())
                    .unwrap_or_default();
                let cf_string = CFString::new(&name_str);
                display_names.push(cf_string.clone());
                (item.id, cf_string.as_concrete_TypeRef())
            })
            .collect::<Vec<_>>();

        self.display_names = display_names;
        self.display_name_fn = Box::new(move |item_ref| {
            let id = item_ref as i32;
            mock_items
                .iter()
                .find(|(item_id, _)| *item_id == id)
                .map(|(_, name_ref)| *name_ref)
                .unwrap_or_else(|| CFString::new("").as_concrete_TypeRef())
        });

        // Set up URL function
        let mut urls = Vec::new();
        let mock_items = self
            .mock_items
            .iter()
            .map(|item| {
                let url_str = item.url.to_string();
                let is_dir = url_str.ends_with('/');
                let file_path = CFString::new(&url_str);
                let url = CFURL::from_file_system_path(file_path, kCFURLPOSIXPathStyle, is_dir);
                urls.push(url.clone());
                (item.id, url.as_concrete_TypeRef())
            })
            .collect::<Vec<_>>();

        self.urls = urls;
        self.resolved_url_fn = Box::new(move |item_ref| {
            let id = item_ref as i32;
            mock_items
                .iter()
                .find(|(item_id, _)| *item_id == id)
                .map(|(_, url_ref)| *url_ref)
                .unwrap_or_else(std::ptr::null)
        });

        self
    }

    fn create_mock_item(id: i32) -> LSSharedFileListItemRef {
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
    let result = FinderTest::new(MockMacOsApi::failing_list()).list_favorites();

    assert!(matches!(
        result,
        Err(FinderError::AccessError(FavoritesError::NullListHandle))
    ));
    Ok(())
}

#[test]
fn should_fail_when_snapshot_handle_is_null() -> Result<()> {
    let result = FinderTest::new(MockMacOsApi::failing_snapshot()).list_favorites();

    assert!(matches!(
        result,
        Err(FinderError::AccessError(FavoritesError::NullSnapshotHandle))
    ));
    Ok(())
}

#[test]
fn should_return_empty_list_when_no_favorites() -> Result<()> {
    FinderTest::with_favorites(vec![]).assert_is_empty()
}

#[test]
fn should_return_favorite_with_display_name_and_url() -> Result<()> {
    FinderTest::with_favorite(Some(test_data::DOCUMENTS_NAME), test_data::DOCUMENTS_PATH)
        .assert_has_favorite(
            Some(test_data::DOCUMENTS_NAME),
            &format!("file://{}", test_data::DOCUMENTS_PATH),
        )
}

#[test]
fn should_handle_airdrop_item() -> Result<()> {
    let finder = FinderTest::with_favorite(None, test_data::AIRDROP_URL);
    finder.assert_has_favorite(None, test_data::AIRDROP_URL)?;

    let favorites = finder.list_favorites()?;
    assert_eq!(
        format!("{}", favorites[0]),
        format!("<no name> -> {}", test_data::AIRDROP_URL)
    );
    Ok(())
}

#[test]
fn should_handle_multiple_favorites() -> Result<()> {
    let finder = FinderTest::with_favorites(vec![
        (None, test_data::AIRDROP_URL),
        (Some("Applications"), "/Applications/"),
        (Some("Downloads"), "/Users/user/Downloads/"),
    ]);

    finder.assert_has_favorite(None, test_data::AIRDROP_URL)?;
    finder.assert_has_favorite(Some("Applications"), "file:///Applications/")?;
    finder.assert_has_favorite(Some("Downloads"), "file:///Users/user/Downloads/")?;
    Ok(())
}
