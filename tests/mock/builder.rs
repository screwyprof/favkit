use std::{ops::Deref, rc::Rc};

use core_foundation::{
    array::{CFArray, CFArrayRef},
    base::TCFType,
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

    pub struct Snapshot(Vec<LSSharedFileListItemRef>);

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

fn create_item_closure<T: Clone, R>(
    items: Rc<Vec<T>>,
    f: impl Fn(&T) -> R + 'static,
) -> impl FnMut(LSSharedFileListItemRef) -> R {
    move |item| {
        let idx = ItemIndex::from(item);
        f(&items[*idx])
    }
}

// States
pub struct Initial;
pub struct WithNullFavorites;
pub struct WithNullSnapshot;
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

impl Builder<WithFavorites> {
    fn convert_items<T, F>(&self, f: F) -> Vec<T>
    where
        F: Fn(&FavoriteItem) -> T,
    {
        self.state.items.iter().map(f).collect()
    }

    fn configure_item_expectation<T, R, F, G>(
        &self,
        mock: &mut MockMacOsApi,
        convert_fn: F,
        expect_fn: G,
    ) where
        T: Clone + 'static,
        R: 'static,
        F: Fn(&FavoriteItem) -> T,
        G: for<'a> FnOnce(
            &'a mut MockMacOsApi,
            Box<dyn FnMut(LSSharedFileListItemRef) -> R + 'static>,
        ),
        T: Into<R>,
    {
        let items = Rc::new(self.convert_items(convert_fn));
        let closure = Box::new(create_item_closure(items, |t| t.clone().into()));
        expect_fn(mock, closure);
    }

    pub fn build(self) -> MockMacOsApi {
        let mut mock = MockMacOsApi::new();

        // Configure list creation
        mock.expect_ls_shared_file_list_create()
            .returning_st(|_, _, _| List::default().into());

        // Create snapshot items and keep them alive
        let snapshot_items: Vec<_> = Snapshot::from(self.state.items.len()).into();
        let array = CFArray::from_copyable(&snapshot_items);
        mock.expect_ls_shared_file_list_copy_snapshot()
            .returning_st(move |_, _| array.as_concrete_TypeRef());

        // Configure display names
        self.configure_item_expectation(
            &mut mock,
            |item| DisplayName::from(&item.name),
            |mock, closure| {
                mock.expect_ls_shared_file_list_item_copy_display_name()
                    .returning_st(closure);
            },
        );

        // Configure URLs
        self.configure_item_expectation(
            &mut mock,
            |item| Url::from(&item.path),
            |mock, mut closure| {
                mock.expect_ls_shared_file_list_item_copy_resolved_url()
                    .returning_st(move |item, _, _| closure(item));
            },
        );

        mock
    }
}
