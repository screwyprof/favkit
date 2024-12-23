use std::{cell::RefCell, ffi::c_void, rc::Rc};

use core_foundation::{
    array::{CFArray, CFArrayRef},
    base::{CFAllocatorRef, TCFType},
    error::CFErrorRef,
    string::CFStringRef,
    url::CFURLRef,
};
use core_services::{
    LSSharedFileListItemRef, LSSharedFileListRef, LSSharedFileListResolutionFlags,
    OpaqueLSSharedFileListItemRef,
};
use favkit::system::{
    MacOsApi,
    favorites::{DisplayName, Snapshot, Url},
};

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
type SnapshotHandle = CFArrayRef;
type DisplayNameHandle = CFStringRef;
type UrlHandle = CFURLRef;

/// Function types for mocking API behavior
pub mod handlers {
    use super::*;

    pub type CreateListFn = Box<dyn Fn() -> ListHandle>;
    pub type GetSnapshotFn = Box<dyn Fn(ListHandle) -> SnapshotHandle>;
    pub type GetDisplayNameFn = Box<dyn Fn(LSSharedFileListItemRef) -> DisplayNameHandle>;
    pub type GetUrlFn = Box<dyn Fn(LSSharedFileListItemRef) -> UrlHandle>;
}
use handlers::*;

#[derive(Default)]
pub struct Expectations {
    pub list_create_calls: usize,
    pub snapshot_calls: usize,
    pub display_name_calls: usize,
    pub resolved_url_calls: usize,
}

/// Mock implementation of MacOsApi for testing
pub struct MockMacOsApi {
    list_create_fn: CreateListFn,
    snapshot_fn: GetSnapshotFn,
    display_name_fn: GetDisplayNameFn,
    resolved_url_fn: GetUrlFn,
    call_tracker: Rc<CallTracker>,
}

impl MockMacOsApi {
    pub fn call_tracker(&self) -> Rc<CallTracker> {
        Rc::clone(&self.call_tracker)
    }

    pub fn new() -> Self {
        let raw_list = 1 as ListHandle;
        let empty_snapshot =
            CFArray::from_copyable(&Vec::<*mut OpaqueLSSharedFileListItemRef>::new());
        let snapshot = Rc::new(Some(
            Snapshot::try_from(empty_snapshot.as_concrete_TypeRef()).unwrap(),
        ));

        let expectations = Expectations {
            list_create_calls: 1,
            snapshot_calls: 1,
            display_name_calls: 0,
            resolved_url_calls: 0,
        };

        Self {
            list_create_fn: Box::new(move || raw_list),
            snapshot_fn: Box::new(move |_| {
                let snapshot = snapshot.as_ref().as_ref().unwrap();
                snapshot.into()
            }),
            display_name_fn: Box::new(|_| std::ptr::null_mut()),
            resolved_url_fn: Box::new(|_| std::ptr::null_mut()),
            call_tracker: Rc::new(CallTracker::new(expectations)),
        }
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

    pub fn with_favorites(favorites: Favorites) -> Self {
        let raw_list = 1 as ListHandle;
        let snapshot = Rc::clone(&favorites.snapshot);
        let display_names = Rc::clone(&favorites.display_names);
        let urls = Rc::clone(&favorites.urls);
        let expectations = Self::calculate_expectations(Some(&favorites));

        Self {
            list_create_fn: Box::new(move || raw_list),
            snapshot_fn: Box::new(move |_| {
                let snapshot = snapshot.as_ref().as_ref().unwrap();
                snapshot.into()
            }),
            display_name_fn: Box::new(move |item_ref| {
                Self::get_display_name(&display_names, item_ref)
            }),
            resolved_url_fn: Box::new(move |item_ref| Self::get_url(&urls, item_ref)),
            call_tracker: Rc::new(CallTracker::new(expectations)),
        }
    }

    pub fn with_null_list() -> Self {
        let expectations = Expectations {
            list_create_calls: 1,
            ..Default::default()
        };

        Self {
            list_create_fn: Box::new(std::ptr::null_mut),
            snapshot_fn: Box::new(|_| std::ptr::null()),
            display_name_fn: Box::new(|_| std::ptr::null_mut()),
            resolved_url_fn: Box::new(|_| std::ptr::null_mut()),
            call_tracker: Rc::new(CallTracker::new(expectations)),
        }
    }

    pub fn with_null_snapshot() -> Self {
        let raw_list = 1 as ListHandle;
        let expectations = Expectations {
            list_create_calls: 1,
            snapshot_calls: 1,
            ..Default::default()
        };

        Self {
            list_create_fn: Box::new(move || raw_list),
            snapshot_fn: Box::new(|_| std::ptr::null()),
            display_name_fn: Box::new(|_| std::ptr::null_mut()),
            resolved_url_fn: Box::new(|_| std::ptr::null_mut()),
            call_tracker: Rc::new(CallTracker::new(expectations)),
        }
    }

    fn calculate_expectations(favorites: Option<&Favorites>) -> Expectations {
        match favorites {
            Some(f) => {
                let item_count = f.display_names.len();
                Expectations {
                    list_create_calls: 1,
                    snapshot_calls: 1,
                    display_name_calls: item_count,
                    resolved_url_calls: item_count,
                }
            }
            None => Expectations {
                list_create_calls: 1,
                snapshot_calls: 1,
                display_name_calls: 0,
                resolved_url_calls: 0,
            },
        }
    }
}

impl MacOsApi for MockMacOsApi {
    unsafe fn ls_shared_file_list_create(
        &self,
        _allocator: CFAllocatorRef,
        _list_type: CFStringRef,
        _options: *const c_void,
    ) -> ListHandle {
        self.call_tracker.track_list_create();
        (self.list_create_fn)()
    }

    unsafe fn ls_shared_file_list_copy_snapshot(
        &self,
        list: ListHandle,
        _seed: *mut u32,
    ) -> SnapshotHandle {
        self.call_tracker.track_snapshot(list);
        (self.snapshot_fn)(list)
    }

    unsafe fn ls_shared_file_list_item_copy_display_name(
        &self,
        item: LSSharedFileListItemRef,
    ) -> DisplayNameHandle {
        self.call_tracker.track_display_name(item);
        (self.display_name_fn)(item)
    }

    unsafe fn ls_shared_file_list_item_copy_resolved_url(
        &self,
        item: LSSharedFileListItemRef,
        _flags: LSSharedFileListResolutionFlags,
        _error: *mut CFErrorRef,
    ) -> UrlHandle {
        self.call_tracker.track_resolved_url(item);
        (self.resolved_url_fn)(item)
    }
}

#[derive(Default)]
pub struct CallTracker {
    list_create_calls: RefCell<Vec<()>>,
    snapshot_calls: RefCell<Vec<LSSharedFileListRef>>,
    display_name_calls: RefCell<Vec<LSSharedFileListItemRef>>,
    resolved_url_calls: RefCell<Vec<LSSharedFileListItemRef>>,
    expectations: Expectations,
}

impl CallTracker {
    pub fn new(expectations: Expectations) -> Self {
        Self {
            expectations,
            ..Default::default()
        }
    }

    pub fn list_create_called(&self) -> usize {
        self.list_create_calls.borrow().len()
    }

    pub fn snapshot_calls(&self) -> Vec<LSSharedFileListRef> {
        self.snapshot_calls.borrow().clone()
    }

    pub fn display_name_calls(&self) -> Vec<LSSharedFileListItemRef> {
        self.display_name_calls.borrow().clone()
    }

    pub fn resolved_url_calls(&self) -> Vec<LSSharedFileListItemRef> {
        self.resolved_url_calls.borrow().clone()
    }

    pub(crate) fn track_list_create(&self) {
        self.list_create_calls.borrow_mut().push(());
    }

    pub(crate) fn track_snapshot(&self, list: LSSharedFileListRef) {
        self.snapshot_calls.borrow_mut().push(list);
    }

    pub(crate) fn track_display_name(&self, item: LSSharedFileListItemRef) {
        self.display_name_calls.borrow_mut().push(item);
    }

    pub(crate) fn track_resolved_url(&self, item: LSSharedFileListItemRef) {
        self.resolved_url_calls.borrow_mut().push(item);
    }

    pub fn verify(&self) {
        assert_eq!(
            self.list_create_called(),
            self.expectations.list_create_calls,
            "Expected list_create to be called {} times",
            self.expectations.list_create_calls
        );
        assert_eq!(
            self.snapshot_calls().len(),
            self.expectations.snapshot_calls,
            "Expected copy_snapshot to be called {} times",
            self.expectations.snapshot_calls
        );
        assert_eq!(
            self.display_name_calls().len(),
            self.expectations.display_name_calls,
            "Expected copy_display_name to be called {} times",
            self.expectations.display_name_calls
        );
        assert_eq!(
            self.resolved_url_calls().len(),
            self.expectations.resolved_url_calls,
            "Expected copy_resolved_url to be called {} times",
            self.expectations.resolved_url_calls
        );
    }
}
