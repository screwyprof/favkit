#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
use coverage_helper::test;

use core_foundation::{
    array::{CFArray, CFArrayRef},
    base::{CFAllocatorRef, CFTypeRef, TCFType},
    string::CFStringRef,
};
use core_services::{LSSharedFileListItemRef, LSSharedFileListRef};
use favkit::{
    Favorites, FinderApi,
    finder::{FinderError, Result},
    system::api::MacOsApi,
};

type ListCreateFn = Box<dyn Fn() -> LSSharedFileListRef>;
type SnapshotFn = Box<dyn Fn(LSSharedFileListRef) -> CFArrayRef>;

struct MockMacOsApi {
    list_create_fn: ListCreateFn,
    snapshot_fn: SnapshotFn,
    items: Option<CFArray<LSSharedFileListItemRef>>,
}

impl Default for MockMacOsApi {
    fn default() -> Self {
        Self::new()
    }
}

impl MockMacOsApi {
    fn new() -> Self {
        Self {
            list_create_fn: Box::new(|| 1 as LSSharedFileListRef),
            snapshot_fn: Box::new(|_| std::ptr::null_mut()),
            items: None,
        }
    }

    fn with_items(mut self, items: Vec<LSSharedFileListItemRef>) -> Self {
        let array = CFArray::from_copyable(&items);
        self.items = Some(array);
        self
    }

    fn with_list_create<F>(mut self, f: F) -> Self
    where
        F: Fn() -> LSSharedFileListRef + 'static,
    {
        self.list_create_fn = Box::new(f);
        self
    }

    fn with_snapshot<F>(mut self, f: F) -> Self
    where
        F: Fn(LSSharedFileListRef) -> CFArrayRef + 'static,
    {
        self.snapshot_fn = Box::new(f);
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
}

#[test]
fn should_return_error_when_list_handle_is_null() -> Result<()> {
    let mock_api = MockMacOsApi::new().with_list_create(std::ptr::null_mut);

    let favorites = Favorites::new(&mock_api);
    let finder = FinderApi::new(&favorites);

    let result = finder.get_favorites_list();

    assert!(matches!(result, Err(FinderError::NullListHandle)));
    Ok(())
}

#[test]
fn should_return_error_when_snapshot_handle_is_null() -> Result<()> {
    let mock_api = MockMacOsApi::new()
        .with_list_create(|| 1 as LSSharedFileListRef)
        .with_snapshot(|_| std::ptr::null_mut());

    let favorites = Favorites::new(&mock_api);
    let finder = FinderApi::new(&favorites);

    let result = finder.get_favorites_list();

    assert!(matches!(result, Err(FinderError::NullSnapshotHandle)));
    Ok(())
}

#[test]
fn should_get_empty_list_when_no_favorites() -> Result<()> {
    let items: Vec<LSSharedFileListItemRef> = vec![];
    let mock_api = MockMacOsApi::new()
        .with_items(items)
        .with_list_create(|| 1 as LSSharedFileListRef);

    let favorites = Favorites::new(&mock_api);
    let finder = FinderApi::new(&favorites);

    let favorites = finder.get_favorites_list()?;
    assert_eq!(favorites, Vec::<String>::new());

    Ok(())
}
