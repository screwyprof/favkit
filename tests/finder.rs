use core_foundation::{
    array::{CFArray, CFArrayRef},
    base::{CFAllocatorRef, CFTypeRef, TCFType},
    string::{CFString, CFStringRef},
    url::{CFURL, CFURLRef, kCFURLPOSIXPathStyle},
};
use core_services::{LSSharedFileListItemRef, LSSharedFileListRef, OpaqueLSSharedFileListItemRef};
use favkit::{
    finder::{Finder, FinderError, Result, SidebarItem, Target},
    system::{
        MacOsApi,
        favorites::{DisplayName, FavoritesError, Snapshot, Url},
    },
};
use std::rc::Rc;

mod constants {
    pub const DOCUMENTS_NAME: &str = "Documents";
    pub const DOCUMENTS_PATH: &str = "/Users/user/Documents/";
    pub const AIRDROP_URL: &str = "nwnode://domain-AirDrop";
}

/// Index type for safer array access
#[derive(Debug, Clone, Copy)]
struct ItemIndex(usize);

impl From<LSSharedFileListItemRef> for ItemIndex {
    fn from(raw: LSSharedFileListItemRef) -> Self {
        Self((raw as i32 - 1) as usize)
    }
}

mod favorites {
    use super::*;

    /// Builder for creating test data
    #[derive(Default)]
    pub struct FavoritesBuilder {
        items: Vec<(Option<&'static str>, &'static str)>,
    }

    impl FavoritesBuilder {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn add_item(mut self, name: Option<&'static str>, url: &'static str) -> Self {
            self.items.push((name, url));
            self
        }

        pub fn build(self) -> Favorites {
            let items = self
                .items
                .into_iter()
                .map(|(name, url)| FavoriteItem::new(name, url))
                .collect();
            Favorites::new(items)
        }
    }

    /// Represents a favorite item with its Core Foundation data
    #[derive(Debug)]
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

    /// Collection of favorite items with their snapshot
    #[derive(Debug)]
    pub struct Favorites {
        pub(crate) snapshot: Rc<Option<Snapshot>>,
        pub(crate) display_names: Rc<Vec<DisplayName>>,
        pub(crate) urls: Rc<Vec<Url>>,
    }

    impl Default for Favorites {
        fn default() -> Self {
            Self {
                snapshot: Rc::new(None),
                display_names: Rc::new(Vec::new()),
                urls: Rc::new(Vec::new()),
            }
        }
    }

    impl Favorites {
        fn new(items: Vec<FavoriteItem>) -> Self {
            let snapshot = {
                let snapshot_items: Vec<_> = (1..=items.len())
                    .map(|i| (i as i32) as *mut OpaqueLSSharedFileListItemRef)
                    .collect();
                let array = CFArray::from_copyable(&snapshot_items);
                Rc::new(Some(
                    Snapshot::try_from(array.as_concrete_TypeRef()).unwrap(),
                ))
            };

            let display_names =
                Rc::new(items.iter().map(|item| item.display_name.clone()).collect());
            let urls = Rc::new(items.iter().map(|item| item.url.clone()).collect());

            Self {
                snapshot,
                display_names,
                urls,
            }
        }
    }
}

mod mock_api {
    use super::*;
    use favorites::Favorites;

    type ListHandle = LSSharedFileListRef;
    type SnapshotArray = CFArrayRef;

    /// Function types for mocking API behavior
    pub mod handlers {
        use super::*;

        pub type CreateListFn = Box<dyn Fn() -> ListHandle>;
        pub type GetSnapshotFn = Box<dyn Fn(ListHandle) -> SnapshotArray>;
        pub type GetDisplayNameFn = Box<dyn Fn(LSSharedFileListItemRef) -> CFStringRef>;
        pub type GetUrlFn = Box<dyn Fn(LSSharedFileListItemRef) -> CFURLRef>;
    }
    use handlers::*;

    /// Builder for creating mock API implementations
    pub struct MockMacOsApiBuilder {
        favorites: Option<Favorites>,
        list_create_fn: Option<CreateListFn>,
        snapshot_fn: Option<GetSnapshotFn>,
        display_name_fn: Option<GetDisplayNameFn>,
        resolved_url_fn: Option<GetUrlFn>,
    }

    impl Default for MockMacOsApiBuilder {
        fn default() -> Self {
            let raw_list = 1 as ListHandle;
            let empty_snapshot =
                CFArray::from_copyable(&Vec::<*mut OpaqueLSSharedFileListItemRef>::new());
            let snapshot = Rc::new(Some(
                Snapshot::try_from(empty_snapshot.as_concrete_TypeRef()).unwrap(),
            ));

            Self {
                favorites: None,
                list_create_fn: Some(Box::new(move || raw_list)),
                snapshot_fn: Some(Box::new(move |_| {
                    let snapshot = snapshot.as_ref().as_ref().unwrap();
                    snapshot.into()
                })),
                display_name_fn: None,
                resolved_url_fn: None,
            }
        }
    }

    impl MockMacOsApiBuilder {
        pub fn new() -> Self {
            Self::default()
        }

        fn with_list_handle(mut self, handle: ListHandle) -> Self {
            self.list_create_fn = Some(Box::new(move || handle));
            self
        }

        pub fn with_favorites(mut self, favorites: Favorites) -> Self {
            let raw_list = 1 as ListHandle;
            self = self.with_list_handle(raw_list);

            let snapshot = Rc::clone(&favorites.snapshot);
            self.snapshot_fn = Some(Box::new(move |_| {
                let snapshot = snapshot.as_ref().as_ref().unwrap();
                snapshot.into()
            }));

            let display_names = Rc::clone(&favorites.display_names);
            self.display_name_fn = Some(Box::new(move |item_ref| {
                let idx: ItemIndex = item_ref.into();
                (&display_names[idx.0]).into()
            }));

            let urls = Rc::clone(&favorites.urls);
            self.resolved_url_fn = Some(Box::new(move |item_ref| {
                let idx: ItemIndex = item_ref.into();
                (&urls[idx.0]).into()
            }));

            self.favorites = Some(favorites);
            self
        }

        pub fn with_null_list(self) -> Self {
            self.with_list_handle(std::ptr::null_mut())
        }

        pub fn with_null_snapshot(mut self) -> Self {
            let raw_list = 1 as ListHandle;
            self = self.with_list_handle(raw_list);
            self.snapshot_fn = Some(Box::new(|_| std::ptr::null()));
            self
        }

        pub fn build(self) -> MockMacOsApi {
            let _favorites = self.favorites.unwrap_or_default();

            MockMacOsApi {
                list_create_fn: self
                    .list_create_fn
                    .unwrap_or_else(|| Box::new(std::ptr::null_mut)),
                snapshot_fn: self
                    .snapshot_fn
                    .unwrap_or_else(|| Box::new(|_| std::ptr::null())),
                display_name_fn: self
                    .display_name_fn
                    .unwrap_or_else(|| Box::new(|_| std::ptr::null_mut())),
                resolved_url_fn: self
                    .resolved_url_fn
                    .unwrap_or_else(|| Box::new(|_| std::ptr::null_mut())),
            }
        }
    }

    /// Mock implementation of MacOsApi for testing
    pub struct MockMacOsApi {
        list_create_fn: CreateListFn,
        snapshot_fn: GetSnapshotFn,
        display_name_fn: GetDisplayNameFn,
        resolved_url_fn: GetUrlFn,
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
            _list: LSSharedFileListRef,
            _seed: *mut u32,
        ) -> CFArrayRef {
            (self.snapshot_fn)(_list)
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
}

use favorites::FavoritesBuilder;
use mock_api::MockMacOsApiBuilder;

#[test]
fn should_fail_when_list_handle_is_null() -> Result<()> {
    // Arrange
    let expected_error = Err(FinderError::AccessError(FavoritesError::NullListHandle));
    let mock_api = MockMacOsApiBuilder::new().with_null_list().build();
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
    let mock_api = MockMacOsApiBuilder::new().with_null_snapshot().build();
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
    let mock_api = MockMacOsApiBuilder::new().build();
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
        Some(constants::DOCUMENTS_NAME.to_string()),
        Target(format!("file://{}", constants::DOCUMENTS_PATH)),
    )];
    let favorites = FavoritesBuilder::new()
        .add_item(Some(constants::DOCUMENTS_NAME), constants::DOCUMENTS_PATH)
        .build();
    let mock_api = MockMacOsApiBuilder::new().with_favorites(favorites).build();
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
        Target(constants::AIRDROP_URL.to_string()),
    )];
    let favorites = FavoritesBuilder::new()
        .add_item(None, constants::AIRDROP_URL)
        .build();
    let mock_api = MockMacOsApiBuilder::new().with_favorites(favorites).build();
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
        SidebarItem::new(None, Target(constants::AIRDROP_URL.to_string())),
        SidebarItem::new(
            Some("Applications".to_string()),
            Target("file:///Applications/".to_string()),
        ),
        SidebarItem::new(
            Some("Downloads".to_string()),
            Target("file:///Users/user/Downloads/".to_string()),
        ),
    ];
    let favorites = FavoritesBuilder::new()
        .add_item(None, constants::AIRDROP_URL)
        .add_item(Some("Applications"), "/Applications/")
        .add_item(Some("Downloads"), "/Users/user/Downloads/")
        .build();
    let mock_api = MockMacOsApiBuilder::new().with_favorites(favorites).build();
    let finder = Finder::new(mock_api);

    // Act
    let result = finder.get_favorites_list()?;

    // Assert
    assert_eq!(result, expected_result);
    Ok(())
}
