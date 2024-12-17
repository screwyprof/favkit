use crate::system::core_foundation::RawRef;
use core_services::OpaqueLSSharedFileListItemRef;

pub(crate) type SnapshotItem = RawRef<OpaqueLSSharedFileListItemRef>;

#[cfg(test)]
mod tests {
    use super::*;
    use core_services::LSSharedFileListItemRef;

    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn should_convert_raw_pointer_to_item() -> Result<()> {
        let item_ref = 1 as LSSharedFileListItemRef;
        let item = SnapshotItem::from(item_ref);
        assert_eq!(LSSharedFileListItemRef::from(item), item_ref);
        Ok(())
    }

    #[test]
    fn should_convert_item_back_to_raw_pointer() -> Result<()> {
        let item_ref = 1 as LSSharedFileListItemRef;
        let item = SnapshotItem::from(item_ref);
        let unwrapped: LSSharedFileListItemRef = item.into();
        assert_eq!(unwrapped, item_ref);
        Ok(())
    }
}
