use core_foundation::{
    array::{CFArray, CFArrayRef},
    base::{CFRange, TCFType},
};
use core_services::{LSSharedFileListItemRef, OpaqueLSSharedFileListItemRef};

use super::item::{RawSnapshotItem, SnapshotItem};

pub(crate) struct RawSnapshot(CFArrayRef);

impl From<CFArrayRef> for RawSnapshot {
    fn from(array: CFArrayRef) -> Self {
        Self(array)
    }
}

pub(crate) struct Snapshot(CFArray<LSSharedFileListItemRef>);

impl Snapshot {
    fn len(&self) -> usize {
        self.0.len().try_into().unwrap_or(0)
    }

    fn get_item(&self, index: usize) -> Option<SnapshotItem> {
        let cf_index = index.try_into().unwrap_or(0);
        let range = CFRange::init(cf_index, 1);
        let mut values = CFArray::get_values(&self.0, range);
        values
            .pop()
            .map(|ptr| ptr as *mut OpaqueLSSharedFileListItemRef)
            .map(RawSnapshotItem::from)
            .and_then(Option::from)
    }
}

pub(crate) struct SnapshotIter<'a> {
    snapshot: &'a Snapshot,
    index: usize,
}

impl Iterator for SnapshotIter<'_> {
    type Item = SnapshotItem;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.snapshot.len() {
            return None;
        }

        let item = self.snapshot.get_item(self.index);
        self.index += 1;
        item
    }
}

impl<'a> IntoIterator for &'a Snapshot {
    type Item = SnapshotItem;
    type IntoIter = SnapshotIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        SnapshotIter {
            snapshot: self,
            index: 0,
        }
    }
}

impl From<RawSnapshot> for Option<Snapshot> {
    fn from(snapshot: RawSnapshot) -> Self {
        (!snapshot.0.is_null())
            .then(|| unsafe { Snapshot(CFArray::wrap_under_get_rule(snapshot.0)) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn should_return_none_for_null_snapshot() -> Result<()> {
        let ptr: CFArrayRef = std::ptr::null();
        let raw = RawSnapshot::from(ptr);
        assert!(Option::<Snapshot>::from(raw).is_none());
        Ok(())
    }

    #[test]
    fn should_wrap_valid_snapshot() -> Result<()> {
        let item: LSSharedFileListItemRef = 1 as LSSharedFileListItemRef;
        let array = CFArray::from_copyable(&[item]);
        let ptr = array.as_concrete_TypeRef();
        let raw = RawSnapshot::from(ptr);
        let snapshot = Option::<Snapshot>::from(raw).ok_or("Failed to create Snapshot")?;
        assert_eq!(snapshot.len(), 1);
        Ok(())
    }

    #[test]
    fn iterator_should_return_none_when_exhausted() -> Result<()> {
        let item: LSSharedFileListItemRef = 1 as LSSharedFileListItemRef;
        let array = CFArray::from_copyable(&[item]);
        let snapshot = Snapshot(array);
        let mut iter = snapshot.into_iter();
        assert!(iter.next().is_some());
        assert!(iter.next().is_none());
        Ok(())
    }
}
