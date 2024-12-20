use std::{ops::Deref, ptr::NonNull};

use core_foundation::{
    array::{CFArray, CFArrayRef},
    base::CFRange,
};
use core_services::{LSSharedFileListItemRef, TCFType};

use super::snapshot_item::SnapshotItem;
use crate::system::{
    core_foundation::CFRef,
    favorites::errors::{FavoritesError, Result},
};

#[derive(Debug)]
pub struct Snapshot(CFRef<CFArray<LSSharedFileListItemRef>>);

impl TryFrom<CFArrayRef> for Snapshot {
    type Error = FavoritesError;

    fn try_from(array_ref: CFArrayRef) -> Result<Self> {
        CFRef::try_from_ref(array_ref)
            .map(Self)
            .map_err(|_| FavoritesError::NullSnapshotHandle)
    }
}

impl From<&Snapshot> for CFArrayRef {
    fn from(snapshot: &Snapshot) -> Self {
        snapshot.0.as_concrete_TypeRef()
    }
}

impl Deref for Snapshot {
    type Target = CFArray<LSSharedFileListItemRef>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoIterator for Snapshot {
    type Item = SnapshotItem;
    type IntoIter = SnapshotIterator;

    fn into_iter(self) -> Self::IntoIter {
        SnapshotIterator::new(self)
    }
}

pub struct SnapshotIterator {
    values: Vec<LSSharedFileListItemRef>,
}

impl SnapshotIterator {
    fn new(snapshot: Snapshot) -> Self {
        let range = CFRange::init(0, snapshot.len());
        let mut values: Vec<_> = snapshot
            .get_values(range)
            .into_iter()
            .map(|ptr| ptr as LSSharedFileListItemRef)
            .collect();
        values.reverse();
        Self { values }
    }
}

impl Iterator for SnapshotIterator {
    type Item = SnapshotItem;

    fn next(&mut self) -> Option<Self::Item> {
        self.values
            .pop()
            .and_then(|ptr| NonNull::new(ptr as *mut _))
            .map(SnapshotItem::from)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.values.len();
        (len, Some(len))
    }
}

#[cfg(test)]
mod tests {
    use core_foundation::base::TCFType;

    use super::*;

    const ITEM_REF: LSSharedFileListItemRef = 1 as LSSharedFileListItemRef;

    #[test]
    fn should_fail_when_snapshot_is_null() {
        // Arrange
        let snapshot_ref: CFArrayRef = std::ptr::null();

        // Act & Assert
        assert!(Snapshot::try_from(snapshot_ref).is_err());
    }

    #[test]
    fn should_wrap_snapshot() -> Result<()> {
        // Arrange
        let snapshot_array = CFArray::from_copyable(&[ITEM_REF]);
        let snapshot_ref = snapshot_array.as_concrete_TypeRef();

        // Act
        let _snapshot = Snapshot::try_from(snapshot_ref)?;

        // Assert
        Ok(())
    }

    #[test]
    fn should_unwrap_snapshot() -> Result<()> {
        // Arrange
        let snapshot_array = CFArray::from_copyable(&[ITEM_REF]);
        let snapshot_ref = snapshot_array.as_concrete_TypeRef();
        let snapshot = Snapshot::try_from(snapshot_ref)?;

        // Act
        let unwrapped: CFArrayRef = (&snapshot).into();

        // Assert
        assert_eq!(unwrapped, snapshot_ref);
        Ok(())
    }

    #[test]
    fn should_iterate_over_snapshot_items() -> Result<()> {
        // Arrange
        let snapshot_array = CFArray::from_copyable(&[ITEM_REF]);
        let snapshot_ref = snapshot_array.as_concrete_TypeRef();
        let snapshot = Snapshot::try_from(snapshot_ref)?;

        // Act
        let mut iter = snapshot.into_iter();

        // Assert
        assert!(iter.next().is_some());
        assert!(iter.next().is_none());
        Ok(())
    }

    #[test]
    fn should_report_initial_size_hint() -> Result<()> {
        // Arrange
        let snapshot_array = CFArray::from_copyable(&[ITEM_REF]);
        let snapshot = Snapshot::try_from(snapshot_array.as_concrete_TypeRef())?;

        // Act
        let iter = snapshot.into_iter();

        // Assert
        assert_eq!(iter.size_hint(), (1, Some(1)));
        Ok(())
    }

    #[test]
    fn should_update_size_hint_after_iteration() -> Result<()> {
        // Arrange
        let snapshot_array = CFArray::from_copyable(&[ITEM_REF]);
        let snapshot = Snapshot::try_from(snapshot_array.as_concrete_TypeRef())?;
        let mut iter = snapshot.into_iter();

        // Act
        iter.next();

        // Assert
        assert_eq!(iter.size_hint(), (0, Some(0)));
        Ok(())
    }
}
