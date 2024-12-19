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

struct FavoriteBuilder {
    id: i32,
    display_name: Option<String>,
    url: String,
}

impl FavoriteBuilder {
    fn new(id: i32) -> Self {
        Self {
            id,
            display_name: None,
            url: "file:///".to_string(),
        }
    }

    fn with_display_name(mut self, name: Option<impl Into<String>>) -> Self {
        self.display_name = name.map(|n| n.into());
        self
    }

    fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = url.into();
        self
    }

    fn build(self) -> (i32, Option<String>, String) {
        (self.id, self.display_name, self.url)
    }
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

    /// Creates a test with a list of favorite items
    fn with_favorites(items: Vec<i32>) -> Self {
        let mock_api = MockMacOsApi::new().with_items(items);
        Self::new(mock_api)
    }

    /// Creates a test with a single favorite item and its metadata
    fn with_favorite(display_name: Option<&str>, url: &str) -> Self {
        let builder = FavoriteBuilder::new(1)
            .with_url(url)
            .with_display_name(display_name);
        let (item_id, display_name, url) = builder.build();

        let mock_api = MockMacOsApi::new()
            .with_items(vec![item_id])
            .with_display_name(display_name.as_deref())
            .with_url(&url);

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

struct MockMacOsApi {
    list_create_fn: ListCreateFn,
    snapshot_fn: SnapshotFn,
    display_name_fn: DisplayNameFn,
    resolved_url_fn: ResolvedUrlFn,
    items: Option<CFArray<LSSharedFileListItemRef>>,
    display_name: Option<DisplayName>,
    url: Option<Url>,
}

impl Default for MockMacOsApi {
    fn default() -> Self {
        Self {
            list_create_fn: Box::new(std::ptr::null_mut),
            snapshot_fn: Box::new(|_| std::ptr::null_mut()),
            display_name_fn: Box::new(|_| std::ptr::null_mut()),
            resolved_url_fn: Box::new(|_| std::ptr::null_mut()),
            items: None,
            display_name: None,
            url: None,
        }
    }
}

impl MockMacOsApi {
    fn new() -> Self {
        Self::default()
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

    fn with_items(mut self, ids: Vec<i32>) -> Self {
        let items = ids
            .into_iter()
            .map(Self::create_mock_item)
            .collect::<Vec<_>>();

        // Set up items
        let array = CFArray::from_copyable(&items);
        self.items = Some(array);

        // Set up list creation
        self.list_create_fn = Box::new(|| 1 as LSSharedFileListRef);

        // Set up snapshot to return our items
        let items_ref = self.items.as_ref().unwrap().as_concrete_TypeRef();
        self.snapshot_fn = Box::new(move |_| items_ref);

        self
    }

    fn create_mock_item(id: i32) -> LSSharedFileListItemRef {
        id as LSSharedFileListItemRef
    }

    fn with_display_name(mut self, name: Option<&str>) -> Self {
        let display_name_cf = CFString::new(name.unwrap_or_default());
        let display_name_cf_ref = display_name_cf.as_concrete_TypeRef();
        let display_name = DisplayName::try_from(display_name_cf_ref).unwrap();

        self.display_name = Some(display_name);
        let display_name_ref = self.display_name.as_ref().unwrap().into();
        self.display_name_fn = Box::new(move |_| display_name_ref);
        self
    }

    fn with_url(mut self, url: &str) -> Self {
        let is_dir = url.ends_with('/');
        let file_path = CFString::new(url);

        let url_cf = CFURL::from_file_system_path(file_path, kCFURLPOSIXPathStyle, is_dir);
        let url_cf_ref = url_cf.as_concrete_TypeRef();
        let url = Url::try_from(url_cf_ref).unwrap();

        self.url = Some(url);
        let url_ref = self.url.as_ref().unwrap().into();
        self.resolved_url_fn = Box::new(move |_| url_ref);
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
