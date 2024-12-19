use core_foundation::{
    array::{CFArray, CFArrayRef},
    base::TCFType,
    base::{CFAllocatorRef, CFTypeRef},
    error::CFErrorRef,
    string::CFStringRef,
    url::CFURLRef,
};
use core_services::{
    LSSharedFileListItemRef, LSSharedFileListRef, LSSharedFileListResolutionFlags,
    OpaqueLSSharedFileListItemRef,
};
use favkit::system::favorites::{DisplayName, Snapshot, Url};
use std::rc::Rc;

use super::favorites::Favorites;

/// Index type for safer array access
#[derive(Debug, Clone, Copy)]
pub struct ItemIndex(pub(crate) usize);

impl From<LSSharedFileListItemRef> for ItemIndex {
    fn from(raw: LSSharedFileListItemRef) -> Self {
        Self((raw as i32 - 1) as usize)
    }
}

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

/// State markers
pub struct Uninitialized;
pub struct WithList;
pub struct WithNullList;
pub struct WithNullSnapshot;

/// Builder for creating mock API implementations
pub struct MockMacOsApiBuilder<State = Uninitialized> {
    list_create_fn: Option<CreateListFn>,
    snapshot_fn: Option<GetSnapshotFn>,
    display_name_fn: Option<GetDisplayNameFn>,
    resolved_url_fn: Option<GetUrlFn>,
    _state: std::marker::PhantomData<State>,
}

impl Default for MockMacOsApiBuilder<Uninitialized> {
    fn default() -> Self {
        let raw_list = 1 as ListHandle;
        let empty_snapshot =
            CFArray::from_copyable(&Vec::<*mut OpaqueLSSharedFileListItemRef>::new());
        let snapshot = Rc::new(Some(
            Snapshot::try_from(empty_snapshot.as_concrete_TypeRef()).unwrap(),
        ));

        Self {
            list_create_fn: Some(Box::new(move || raw_list)),
            snapshot_fn: Some(Box::new(move |_| {
                let snapshot = snapshot.as_ref().as_ref().unwrap();
                snapshot.into()
            })),
            display_name_fn: None,
            resolved_url_fn: None,
            _state: std::marker::PhantomData,
        }
    }
}

impl MockMacOsApiBuilder<Uninitialized> {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_display_name(
        display_names: &[DisplayName],
        item_ref: LSSharedFileListItemRef,
    ) -> CFStringRef {
        let idx: ItemIndex = item_ref.into();
        (&display_names[idx.0]).into()
    }

    fn get_url(urls: &[Url], item_ref: LSSharedFileListItemRef) -> CFURLRef {
        let idx: ItemIndex = item_ref.into();
        (&urls[idx.0]).into()
    }

    pub fn with_favorites(self, favorites: Favorites) -> MockMacOsApiBuilder<WithList> {
        let raw_list = 1 as ListHandle;
        let snapshot = Rc::clone(&favorites.snapshot);
        let display_names = Rc::clone(&favorites.display_names);
        let urls = Rc::clone(&favorites.urls);

        MockMacOsApiBuilder {
            list_create_fn: Some(Box::new(move || raw_list)),
            snapshot_fn: Some(Box::new(move |_| {
                let snapshot = snapshot.as_ref().as_ref().unwrap();
                snapshot.into()
            })),
            display_name_fn: Some(Box::new(move |item_ref| {
                Self::get_display_name(&display_names, item_ref)
            })),
            resolved_url_fn: Some(Box::new(move |item_ref| Self::get_url(&urls, item_ref))),
            _state: std::marker::PhantomData,
        }
    }

    pub fn with_null_list(self) -> MockMacOsApiBuilder<WithNullList> {
        MockMacOsApiBuilder {
            list_create_fn: Some(Box::new(std::ptr::null_mut)),
            snapshot_fn: None,
            display_name_fn: None,
            resolved_url_fn: None,
            _state: std::marker::PhantomData,
        }
    }

    pub fn with_null_snapshot(self) -> MockMacOsApiBuilder<WithNullSnapshot> {
        let raw_list = 1 as ListHandle;
        MockMacOsApiBuilder {
            list_create_fn: Some(Box::new(move || raw_list)),
            snapshot_fn: Some(Box::new(|_| std::ptr::null())),
            display_name_fn: None,
            resolved_url_fn: None,
            _state: std::marker::PhantomData,
        }
    }
}

// Implement build() for each final state
impl<State> MockMacOsApiBuilder<State> {
    pub fn build(self) -> MockMacOsApi {
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

impl favkit::system::MacOsApi for MockMacOsApi {
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
        _flags: LSSharedFileListResolutionFlags,
        _error: *mut CFErrorRef,
    ) -> CFURLRef {
        (self.resolved_url_fn)(item)
    }
}
