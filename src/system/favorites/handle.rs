use std::ptr::NonNull;

use crate::system::core_foundation::RawRef;
use core_services::OpaqueLSSharedFileListRef;

pub(crate) struct FavoritesHandle(RawRef<OpaqueLSSharedFileListRef>);

impl From<NonNull<OpaqueLSSharedFileListRef>> for FavoritesHandle {
    fn from(ptr: NonNull<OpaqueLSSharedFileListRef>) -> Self {
        Self(RawRef::from(ptr))
    }
}

impl From<FavoritesHandle> for *mut OpaqueLSSharedFileListRef {
    fn from(handle: FavoritesHandle) -> Self {
        let ptr: NonNull<OpaqueLSSharedFileListRef> = handle.0.into();
        ptr.as_ptr()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::finder::{FinderError, Result};

    #[test]
    fn should_return_error_for_null_handle() {
        let handle_ref: *mut OpaqueLSSharedFileListRef = std::ptr::null_mut();
        assert!(RawRef::try_from(handle_ref).is_err());
    }

    #[test]
    fn should_convert_raw_pointer_to_handle() -> Result<()> {
        let mut value = 42;
        let handle_ref = &mut value as *mut _ as *mut OpaqueLSSharedFileListRef;
        let ptr = NonNull::new(handle_ref).ok_or(FinderError::NullListHandle)?;
        let handle = FavoritesHandle::from(ptr);
        let unwrapped: *mut OpaqueLSSharedFileListRef = handle.into();
        assert_eq!(unwrapped, handle_ref);
        Ok(())
    }
}
