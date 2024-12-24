use std::rc::Rc;

use core_foundation::{
    array::{CFArray, CFArrayRef},
    base::TCFType,
    error::CFErrorRef,
    string::{CFString, CFStringRef},
    url::{CFURL, CFURLRef, kCFURLPOSIXPathStyle},
};
use core_services::{LSSharedFileListItemRef, LSSharedFileListRef};

use super::{favorites::FavoriteItem, mac_os_api::MockMacOsApi};

// Core Foundation wrappers
#[repr(transparent)]
struct DisplayName(CFString);

impl From<&Option<String>> for DisplayName {
    fn from(name: &Option<String>) -> Self {
        Self(CFString::new(name.as_deref().unwrap_or_default()))
    }
}

impl From<DisplayName> for CFString {
    fn from(name: DisplayName) -> Self {
        name.0
    }
}

#[repr(transparent)]
struct Url(CFURL);

impl<T: AsRef<str>> From<T> for Url {
    fn from(path: T) -> Self {
        let is_dir = path.as_ref().ends_with('/');
        let file_path = CFString::new(path.as_ref());
        Self(CFURL::from_file_system_path(
            file_path,
            kCFURLPOSIXPathStyle,
            is_dir,
        ))
    }
}

impl From<Url> for CFURL {
    fn from(url: Url) -> Self {
        url.0
    }
}

struct Snapshot(Vec<LSSharedFileListItemRef>);

impl From<usize> for Snapshot {
    fn from(count: usize) -> Self {
        Self(
            (1..=count)
                .map(|i| (i as i32) as LSSharedFileListItemRef)
                .collect(),
        )
    }
}

impl From<Snapshot> for Vec<LSSharedFileListItemRef> {
    fn from(snapshot: Snapshot) -> Self {
        snapshot.0
    }
}

struct List(LSSharedFileListRef);

impl Default for List {
    fn default() -> Self {
        Self(1 as *mut _)
    }
}

impl List {
    fn null() -> Self {
        Self(std::ptr::null_mut())
    }
}

impl From<List> for LSSharedFileListRef {
    fn from(list: List) -> Self {
        list.0
    }
}

struct NullSnapshot(CFArrayRef);

impl Default for NullSnapshot {
    fn default() -> Self {
        Self(std::ptr::null())
    }
}

impl From<NullSnapshot> for CFArrayRef {
    fn from(snapshot: NullSnapshot) -> Self {
        snapshot.0
    }
}

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
    pub fn build(self) -> MockMacOsApi {
        let mut mock = MockMacOsApi::new();
        mock.expect_ls_shared_file_list_create()
            .returning_st(|_, _, _| List::null().into());
        mock
    }
}

impl MockBuilder<WithNullSnapshot> {
    pub fn build(self) -> MockMacOsApi {
        let mut mock = MockMacOsApi::new();
        mock.expect_ls_shared_file_list_create()
            .returning_st(|_, _, _| List::default().into());
        mock.expect_ls_shared_file_list_copy_snapshot()
            .returning_st(|_, _| NullSnapshot::default().into());
        mock
    }
}

impl MockBuilder<WithFavorites> {
    fn get_item_index(item: LSSharedFileListItemRef) -> usize {
        ((item as i32) - 1) as usize
    }

    fn convert_items<T, F>(&self, f: F) -> Rc<Vec<T>>
    where
        F: Fn(&FavoriteItem) -> T,
    {
        Rc::new(self.state.items.iter().map(f).collect())
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
            .returning_st(|_, _, _| List::default().into());

        // Create snapshot items and keep them alive
        let snapshot_items: Vec<_> = Snapshot::from(self.state.items.len()).into();
        let array = Rc::new(CFArray::from_copyable(&snapshot_items));
        mock.expect_ls_shared_file_list_copy_snapshot()
            .returning_st(Self::create_snapshot_closure(array));

        // Configure display names
        let strings = self.convert_items(|item| CFString::from(DisplayName::from(&item.name)));
        mock.expect_ls_shared_file_list_item_copy_display_name()
            .returning_st(Self::create_display_name_closure(strings));

        // Configure URLs
        let urls = self.convert_items(|item| CFURL::from(Url::from(&item.path)));
        mock.expect_ls_shared_file_list_item_copy_resolved_url()
            .returning_st(Self::create_url_closure(urls));

        mock
    }
}
