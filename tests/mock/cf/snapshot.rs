use core_foundation::{array::CFArray, base::TCFType};
use core_services::OpaqueLSSharedFileListItemRef;
use favkit::system::favorites::{Snapshot as SystemSnapshot, errors::Result};

pub struct Snapshot(usize);

impl From<usize> for Snapshot {
    fn from(items_count: usize) -> Self {
        Self(items_count)
    }
}

impl TryFrom<Snapshot> for SystemSnapshot {
    type Error = favkit::system::favorites::errors::FavoritesError;

    fn try_from(snapshot: Snapshot) -> Result<Self> {
        let snapshot_items: Vec<_> = (1..=snapshot.0)
            .map(|i| (i as i32) as *mut OpaqueLSSharedFileListItemRef)
            .collect();
        let array = CFArray::from_copyable(&snapshot_items);
        Self::try_from(array.as_concrete_TypeRef())
    }
}
