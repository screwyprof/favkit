use core_foundation::{
    array::{CFArray, CFArrayRef},
    base::{CFIndex, CFRange, TCFType},
};
use core_services::{LSSharedFileListItemRef, OpaqueLSSharedFileListItemRef};

use super::item::{FavoriteItem, RawFavoriteItem};

pub(crate) struct RawSnapshot(CFArrayRef);

impl From<CFArrayRef> for RawSnapshot {
    fn from(array: CFArrayRef) -> Self {
        Self(array)
    }
}

pub(crate) struct Snapshot(CFArray<LSSharedFileListItemRef>);

pub(crate) struct SnapshotIter<'a> {
    snapshot: &'a Snapshot,
    index: usize,
}

impl Iterator for SnapshotIter<'_> {
    type Item = FavoriteItem;

    fn next(&mut self) -> Option<Self::Item> {
        let cf_index: CFIndex = self.index.try_into().ok()?;
        if cf_index >= self.snapshot.0.len() {
            return None;
        }

        let range = CFRange::init(cf_index, 1);
        let mut values = CFArray::get_values(&self.snapshot.0, range);
        let item = values
            .pop()
            .map(|ptr| ptr as *mut OpaqueLSSharedFileListItemRef)
            .map(RawFavoriteItem::from)
            .and_then(Option::from);

        self.index += 1;
        item
    }
}

impl<'a> IntoIterator for &'a Snapshot {
    type Item = FavoriteItem;
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
