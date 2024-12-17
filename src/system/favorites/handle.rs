use std::ptr::NonNull;

use crate::{
    finder::{FinderError, Result},
    system::core_foundation::RawRef,
};
use core_services::OpaqueLSSharedFileListRef;

pub(crate) struct FavoritesHandle(RawRef<OpaqueLSSharedFileListRef>);

impl TryFrom<*mut OpaqueLSSharedFileListRef> for FavoritesHandle {
    type Error = FinderError;

    fn try_from(ptr: *mut OpaqueLSSharedFileListRef) -> Result<Self> {
        NonNull::new(ptr)
            .map(|ptr| Self(RawRef::new(ptr)))
            .ok_or(FinderError::NullListHandle)
    }
}

impl From<FavoritesHandle> for *mut OpaqueLSSharedFileListRef {
    fn from(handle: FavoritesHandle) -> Self {
        handle.0.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_error_for_null_handle() {
        let handle_ref = std::ptr::null_mut();
        assert!(FavoritesHandle::try_from(handle_ref).is_err());
    }

    #[test]
    fn should_convert_raw_pointer_to_handle() -> Result<()> {
        const FAVORITES_REF: *mut OpaqueLSSharedFileListRef = 42 as *mut _;
        let handle = FavoritesHandle::try_from(FAVORITES_REF)?;
        let unwrapped: *mut OpaqueLSSharedFileListRef = handle.into();
        assert_eq!(unwrapped, FAVORITES_REF);
        Ok(())
    }
}
