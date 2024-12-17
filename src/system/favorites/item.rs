use core_services::LSSharedFileListItemRef;

use crate::system::core_foundation::RawRef;

pub(crate) type SnapshotItem = RawRef<core_services::OpaqueLSSharedFileListItemRef>;

impl From<SnapshotItem> for LSSharedFileListItemRef {
    fn from(item: SnapshotItem) -> Self {
        item.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core_services::OpaqueLSSharedFileListItemRef;

    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn should_return_none_for_null_item() -> Result<()> {
        let item_ref = std::ptr::null_mut();
        assert!(SnapshotItem::from_ref(item_ref).is_none());
        Ok(())
    }

    #[test]
    fn should_wrap_valid_item() -> Result<()> {
        let item_ref = 1 as *mut OpaqueLSSharedFileListItemRef;
        let item = SnapshotItem::from_ref(item_ref).ok_or("Failed to create SnapshotItem")?;
        assert_eq!(LSSharedFileListItemRef::from(item), item_ref);
        Ok(())
    }
}
