use std::ptr::NonNull;

use crate::system::{
    core_foundation::RawRef,
    favorites::errors::{FavoritesError, Result},
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
    type Error = FavoritesError;

    fn try_from(ptr: *mut OpaqueLSSharedFileListItemRef) -> Result<Self> {
        NonNull::new(ptr)
            .map(Self::from)
            .ok_or(FavoritesError::NullListHandle)
    }
}

impl From<&SnapshotItem> for *mut OpaqueLSSharedFileListItemRef {
    fn from(item: &SnapshotItem) -> Self {
        item.0.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SNAPSHOT_ITEM_REF: *mut OpaqueLSSharedFileListItemRef = 42 as *mut _;

    #[test]
    fn should_fail_when_snapshot_item_is_null() {
        // Arrange
        let snapshot_item_ref = std::ptr::null_mut();

        // Act & Assert
        assert!(SnapshotItem::try_from(snapshot_item_ref).is_err());
    }

    #[test]
    fn should_wrap_snapshot_item() -> Result<()> {
        // Arrange
        let snapshot_item_ref = SNAPSHOT_ITEM_REF;

        // Act
        let _snapshot_item = SnapshotItem::try_from(snapshot_item_ref)?;

        // Assert
        Ok(())
    }

    #[test]
    fn should_unwrap_snapshot_item() -> Result<()> {
        // Arrange
        let snapshot_item_ref = SNAPSHOT_ITEM_REF;
        let ptr = NonNull::new(snapshot_item_ref).unwrap();
        let snapshot_item = SnapshotItem::from(ptr);

        // Act
        let unwrapped: *mut OpaqueLSSharedFileListItemRef = (&snapshot_item).into();

        // Assert
        assert_eq!(unwrapped, snapshot_item_ref);
        Ok(())
    }
}
