use std::{ops::Deref, rc::Rc};

use core_foundation::{
    array::{CFArray, CFArrayRef},
    base::{CFTypeRef, TCFType},
    error::CFErrorRef,
    string::{CFString, CFStringRef},
    url::{CFURL, CFURLRef, kCFURLPOSIXPathStyle},
};
use core_services::{LSSharedFileListItemRef, LSSharedFileListRef};

use super::{favorites::FavoriteItem, mac_os_api::MockMacOsApi};

mod cf {
    use std::convert::AsRef;

    use super::*;

    #[derive(Clone)]
    #[repr(transparent)]
    pub struct DisplayName(CFString);

    impl From<&Option<String>> for DisplayName {
        fn from(name: &Option<String>) -> Self {
            Self(CFString::new(name.as_deref().unwrap_or_default()))
        }
    }

    impl From<DisplayName> for CFStringRef {
        fn from(name: DisplayName) -> Self {
            name.0.as_concrete_TypeRef()
        }
    }

    #[derive(Clone)]
    #[repr(transparent)]
    pub struct Url(CFURL);

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

    impl From<Url> for CFURLRef {
        fn from(url: Url) -> Self {
            url.0.as_concrete_TypeRef()
        }
    }

    #[derive(Clone)]
    pub struct Snapshot(Rc<CFArray>);

    impl From<usize> for Snapshot {
        fn from(count: usize) -> Self {
            let items: Vec<CFTypeRef> = (1..=count)
                .map(|i| (i as i32) as LSSharedFileListItemRef)
                .map(|item| item as CFTypeRef)
                .collect();
            Self(Rc::new(CFArray::from_copyable(&items)))
        }
    }

    impl From<Snapshot> for CFArrayRef {
        fn from(snapshot: Snapshot) -> Self {
            snapshot.0.as_concrete_TypeRef()
        }
    }

    pub struct List(LSSharedFileListRef);

    impl Default for List {
        fn default() -> Self {
            Self(1 as *mut _)
        }
    }

    impl List {
        pub fn null() -> Self {
            Self(std::ptr::null_mut())
        }
    }

    impl From<List> for LSSharedFileListRef {
        fn from(list: List) -> Self {
            list.0
        }
    }

    pub struct NullSnapshot(CFArrayRef);

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
}

use cf::*;

struct ItemIndex(usize);

impl From<LSSharedFileListItemRef> for ItemIndex {
    fn from(item: LSSharedFileListItemRef) -> Self {
        Self(((item as i32) - 1) as usize)
    }
}

impl Deref for ItemIndex {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn create_display_name_closure(
    items: Rc<Vec<DisplayName>>,
) -> impl FnMut(LSSharedFileListItemRef) -> CFStringRef {
    move |item| {
        let idx = ItemIndex::from(item);
        items[*idx].clone().into()
    }
}

fn create_url_closure(
    items: Rc<Vec<Url>>,
) -> impl FnMut(LSSharedFileListItemRef, u32, *mut CFErrorRef) -> CFURLRef {
    move |item, _, _| {
        let idx = ItemIndex::from(item);
        items[*idx].clone().into()
    }
}

fn create_snapshot_closure(
    snapshot: Snapshot,
) -> impl FnMut(LSSharedFileListRef, *mut u32) -> CFArrayRef {
    move |_, _| snapshot.clone().into()
}

// States
pub struct Initial;
pub struct WithNullFavorites;
pub struct WithNullSnapshot;
pub struct WithEmptyFavorites;
pub struct WithFavorites {
    items: Vec<FavoriteItem>,
}

pub struct Builder<State> {
    state: State,
}

// Initial state transitions
impl Builder<Initial> {
    pub fn new() -> Self {
        Self { state: Initial }
    }

    pub fn with_null_favorites(self) -> Builder<WithNullFavorites> {
        Builder {
            state: WithNullFavorites,
        }
    }

    pub fn with_null_snapshot(self) -> Builder<WithNullSnapshot> {
        Builder {
            state: WithNullSnapshot,
        }
    }

    pub fn with_empty_favorites(self) -> Builder<WithEmptyFavorites> {
        Builder {
            state: WithEmptyFavorites,
        }
    }

    pub fn with_favorites(self, items: Vec<FavoriteItem>) -> Builder<WithFavorites> {
        Builder {
            state: WithFavorites { items },
        }
    }
}

// Terminal state builders
impl Builder<WithNullFavorites> {
    pub fn build(self) -> MockMacOsApi {
        let mut mock = MockMacOsApi::new();
        mock.expect_ls_shared_file_list_create()
            .returning_st(|_, _, _| List::null().into());
        mock
    }
}

impl Builder<WithNullSnapshot> {
    pub fn build(self) -> MockMacOsApi {
        let mut mock = MockMacOsApi::new();
        mock.expect_ls_shared_file_list_create()
            .returning_st(|_, _, _| List::default().into());
        mock.expect_ls_shared_file_list_copy_snapshot()
            .returning_st(|_, _| NullSnapshot::default().into());
        mock
    }
}

impl Builder<WithEmptyFavorites> {
    pub fn build(self) -> MockMacOsApi {
        let mut mock = MockMacOsApi::new();

        // Configure list creation
        mock.expect_ls_shared_file_list_create()
            .returning_st(|_, _, _| List::default().into());

        // Configure snapshots
        let snapshot = Snapshot::from(0);
        mock.expect_ls_shared_file_list_copy_snapshot()
            .returning_st(create_snapshot_closure(snapshot));

        mock
    }
}

impl Builder<WithFavorites> {
    fn convert_items<T, F>(&self, f: F) -> Vec<T>
    where
        F: Fn(&FavoriteItem) -> T,
    {
        self.state.items.iter().map(f).collect()
    }

    pub fn build(self) -> MockMacOsApi {
        let mut mock = MockMacOsApi::new();

        // Configure list creation
        mock.expect_ls_shared_file_list_create()
            .returning_st(|_, _, _| List::default().into());

        // Configure snapshots
        let snapshot = Snapshot::from(self.state.items.len());
        mock.expect_ls_shared_file_list_copy_snapshot()
            .returning_st(create_snapshot_closure(snapshot));

        // Configure display names
        let items = Rc::new(self.convert_items(|item| DisplayName::from(&item.name)));
        let closure = create_display_name_closure(items);
        mock.expect_ls_shared_file_list_item_copy_display_name()
            .returning_st(closure);

        // Configure URLs
        let items = Rc::new(self.convert_items(|item| Url::from(&item.path)));
        let closure = create_url_closure(items);
        mock.expect_ls_shared_file_list_item_copy_resolved_url()
            .returning_st(closure);

        mock
    }
}
