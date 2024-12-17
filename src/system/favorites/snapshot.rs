use std::{ops::Deref, ptr::NonNull};

use core_foundation::{
    array::{CFArray, CFArrayRef},
    base::CFRange,
};
use core_services::LSSharedFileListItemRef;

use crate::{
    finder::{FinderError, Result},
    system::core_foundation::CFRef,
};

use super::item::SnapshotItem;

pub(crate) struct Snapshot(CFRef<CFArray<LSSharedFileListItemRef>>);

impl TryFrom<CFArrayRef> for Snapshot {
    type Error = FinderError;

    fn try_from(array_ref: CFArrayRef) -> Result<Self> {
        CFRef::try_from_ref(array_ref).map(Self)
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

pub(crate) struct SnapshotIterator {
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
    use super::*;
    use core_foundation::base::TCFType;

    #[test]
    fn should_return_error_for_null_snapshot() {
        let ptr: CFArrayRef = std::ptr::null();
        assert!(CFRef::<CFArray<LSSharedFileListItemRef>>::try_from_ref(ptr).is_err());
    }

    #[test]
    fn should_wrap_valid_snapshot() -> Result<()> {
        let item: LSSharedFileListItemRef = 1 as LSSharedFileListItemRef;
        let array = CFArray::from_copyable(&[item]);
        let ptr = array.as_concrete_TypeRef();
        let snapshot = Snapshot::try_from(ptr)?;
        assert_eq!(snapshot.len(), 1);
        Ok(())
    }

    #[test]
    fn iterator_should_return_none_when_exhausted() -> Result<()> {
        let item: LSSharedFileListItemRef = 1 as LSSharedFileListItemRef;
        let array = CFArray::from_copyable(&[item]);
        let snapshot = Snapshot::try_from(array.as_concrete_TypeRef())?;
        let mut iter = snapshot.into_iter();
        assert!(iter.next().is_some());
        assert!(iter.next().is_none());
        Ok(())
    }

    #[test]
    fn iterator_should_report_correct_size_hint() -> Result<()> {
        let item: LSSharedFileListItemRef = 1 as LSSharedFileListItemRef;
        let array = CFArray::from_copyable(&[item]);
        let snapshot = Snapshot::try_from(array.as_concrete_TypeRef())?;
        let mut iter = snapshot.into_iter();

        assert_eq!(iter.size_hint(), (1, Some(1)));
        iter.next();
        assert_eq!(iter.size_hint(), (0, Some(0)));
        Ok(())
    }
}
