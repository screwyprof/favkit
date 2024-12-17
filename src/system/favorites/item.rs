use std::ptr::NonNull;

use crate::system::core_foundation::RawRef;
use core_services::OpaqueLSSharedFileListItemRef;

#[derive(Clone)]
pub(crate) struct SnapshotItem(RawRef<OpaqueLSSharedFileListItemRef>);

impl From<NonNull<OpaqueLSSharedFileListItemRef>> for SnapshotItem {
    fn from(ptr: NonNull<OpaqueLSSharedFileListItemRef>) -> Self {
        Self(RawRef::from(ptr))
    }
}

impl From<SnapshotItem> for *mut OpaqueLSSharedFileListItemRef {
    fn from(item: SnapshotItem) -> Self {
        let ptr: NonNull<OpaqueLSSharedFileListItemRef> = item.0.into();
        ptr.as_ptr()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::finder::{FinderError, Result};

    #[test]
    fn should_return_error_for_null_item() {
        let item_ref: *mut OpaqueLSSharedFileListItemRef = std::ptr::null_mut();
        assert!(RawRef::try_from(item_ref).is_err());
    }

    #[test]
    fn should_convert_raw_pointer_to_item() -> Result<()> {
        let mut value = 42;
        let item_ref = &mut value as *mut _ as *mut OpaqueLSSharedFileListItemRef;
        let ptr = NonNull::new(item_ref).ok_or(FinderError::NullListHandle)?;
        let item = SnapshotItem::from(ptr);
        let unwrapped: *mut OpaqueLSSharedFileListItemRef = item.into();
        assert_eq!(unwrapped, item_ref);
        Ok(())
    }
}
