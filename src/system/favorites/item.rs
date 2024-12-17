use core_services::{LSSharedFileListItemRef, OpaqueLSSharedFileListItemRef};

use crate::system::core_foundation::{Raw, Safe};

pub(crate) type RawSnapshotItem = Raw<*mut OpaqueLSSharedFileListItemRef>;
pub(crate) type SnapshotItem = Safe<LSSharedFileListItemRef>;

impl From<SnapshotItem> for LSSharedFileListItemRef {
    fn from(item: SnapshotItem) -> Self {
        item.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn should_return_none_for_null_item() -> Result<()> {
        let item_ref = std::ptr::null_mut();
        let raw = Raw::from(item_ref);
        assert!(Option::<SnapshotItem>::from(raw).is_none());
        Ok(())
    }

    #[test]
    fn should_wrap_valid_item() -> Result<()> {
        let item_ref = 1 as *mut OpaqueLSSharedFileListItemRef;
        let raw = Raw::from(item_ref);
        let item = Option::<SnapshotItem>::from(raw).ok_or("Failed to create SnapshotItem")?;
        assert_eq!(LSSharedFileListItemRef::from(item), item_ref);
        Ok(())
    }
}
