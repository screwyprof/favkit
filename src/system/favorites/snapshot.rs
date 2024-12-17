use crate::system::core_foundation::CFRef;
use core_foundation::{
    array::{CFArray, CFArrayRef},
    base::CFRange,
};
use core_services::LSSharedFileListItemRef;

use super::item::SnapshotItem;

pub(crate) struct Snapshot(CFRef<CFArray<LSSharedFileListItemRef>>);

impl Snapshot {
    pub(crate) fn from_ref(array_ref: CFArrayRef) -> Option<Self> {
        CFRef::from_ref(array_ref).map(Self)
    }

    fn len(&self) -> usize {
        usize::try_from(self.0.0.len()).unwrap_or(0)
    }

    fn get_item(&self, index: usize) -> Option<SnapshotItem> {
        let cf_index = isize::try_from(index).unwrap_or(0);
        let range = CFRange::init(cf_index, 1);
        let mut values = CFArray::get_values(&self.0.0, range);
        values
            .pop()
            .map(|ptr| ptr as LSSharedFileListItemRef)
            .map(SnapshotItem::from)
    }
}

impl<'a> IntoIterator for &'a Snapshot {
    type Item = SnapshotItem;
    type IntoIter = SnapshotIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        SnapshotIterator {
            snapshot: self,
            index: 0,
        }
    }
}

pub(crate) struct SnapshotIterator<'a> {
    snapshot: &'a Snapshot,
    index: usize,
}

impl Iterator for SnapshotIterator<'_> {
    type Item = SnapshotItem;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.snapshot.len() {
            return None;
        }

        let item = self.snapshot.get_item(self.index);
        self.index += 1;
        item
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.snapshot.len().saturating_sub(self.index);
        (remaining, Some(remaining))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core_foundation::{array::CFArrayRef, base::TCFType};
    use core_services::LSSharedFileListItemRef;

    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn should_return_none_for_null_snapshot() -> Result<()> {
        let ptr: CFArrayRef = std::ptr::null();
        assert!(Snapshot::from_ref(ptr).is_none());
        Ok(())
    }

    #[test]
    fn should_wrap_valid_snapshot() -> Result<()> {
        let item: LSSharedFileListItemRef = 1 as LSSharedFileListItemRef;
        let array = CFArray::from_copyable(&[item]);
        let ptr = array.as_concrete_TypeRef();
        let snapshot = Snapshot::from_ref(ptr).ok_or("Failed to create Snapshot")?;
        assert_eq!(snapshot.len(), 1);
        Ok(())
    }

    #[test]
    fn iterator_should_return_none_when_exhausted() -> Result<()> {
        let item: LSSharedFileListItemRef = 1 as LSSharedFileListItemRef;
        let array = CFArray::from_copyable(&[item]);
        let snapshot =
            Snapshot::from_ref(array.as_concrete_TypeRef()).ok_or("Failed to create Snapshot")?;
        let mut iter = (&snapshot).into_iter();
        assert!(iter.next().is_some());
        assert!(iter.next().is_none());
        Ok(())
    }

    #[test]
    fn iterator_should_report_correct_size_hint() -> Result<()> {
        let item: LSSharedFileListItemRef = 1 as LSSharedFileListItemRef;
        let array = CFArray::from_copyable(&[item]);
        let snapshot =
            Snapshot::from_ref(array.as_concrete_TypeRef()).ok_or("Failed to create Snapshot")?;
        let mut iter = (&snapshot).into_iter();

        assert_eq!(iter.size_hint(), (1, Some(1)));
        iter.next();
        assert_eq!(iter.size_hint(), (0, Some(0)));
        Ok(())
    }
}
