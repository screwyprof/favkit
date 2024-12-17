use std::ptr::NonNull;

use crate::{
    finder::{FinderError, Result},
    system::core_foundation::RawRef,
};
use core_services::OpaqueLSSharedFileListItemRef;

#[derive(Clone)]
pub(crate) struct SnapshotItem(RawRef<OpaqueLSSharedFileListItemRef>);

impl From<NonNull<OpaqueLSSharedFileListItemRef>> for SnapshotItem {
    fn from(ptr: NonNull<OpaqueLSSharedFileListItemRef>) -> Self {
        Self(RawRef::new(ptr))
    }
}

impl TryFrom<*mut OpaqueLSSharedFileListItemRef> for SnapshotItem {
    type Error = FinderError;

    fn try_from(ptr: *mut OpaqueLSSharedFileListItemRef) -> Result<Self> {
        NonNull::new(ptr)
            .map(Self::from)
            .ok_or(FinderError::NullListHandle)
    }
}

impl From<SnapshotItem> for *mut OpaqueLSSharedFileListItemRef {
    fn from(item: SnapshotItem) -> Self {
        item.0.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_ITEM_PTR: *mut OpaqueLSSharedFileListItemRef = 42 as *mut _;

    #[test]
    fn should_return_error_for_null_item() {
        let item_ref = std::ptr::null_mut();
        assert!(SnapshotItem::try_from(item_ref).is_err());
    }

    #[test]
    fn should_convert_raw_pointer_to_item() -> Result<()> {
        let item = SnapshotItem::try_from(TEST_ITEM_PTR)?;
        let unwrapped: *mut OpaqueLSSharedFileListItemRef = item.into();
        assert_eq!(unwrapped, TEST_ITEM_PTR);
        Ok(())
    }
}
