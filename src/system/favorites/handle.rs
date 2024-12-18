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

    const FAVORITES_REF: *mut OpaqueLSSharedFileListRef = 42 as *mut _;

    #[test]
    fn should_fail_when_handle_is_null() {
        // Arrange
        let favorites_ref = std::ptr::null_mut();

        // Act & Assert
        assert!(FavoritesHandle::try_from(favorites_ref).is_err());
    }

    #[test]
    fn should_wrap_handle() -> Result<()> {
        // Arrange
        let favorites_ref = FAVORITES_REF;

        // Act & Assert
        let _handle = FavoritesHandle::try_from(favorites_ref)?;
        Ok(())
    }

    #[test]
    fn should_unwrap_handle() {
        // Arrange
        let favorites_ref = FAVORITES_REF;
        let ptr = NonNull::new(favorites_ref).unwrap();
        let handle = FavoritesHandle(RawRef::new(ptr));

        // Act
        let unwrapped: *mut OpaqueLSSharedFileListRef = handle.into();

        // Assert
        assert_eq!(unwrapped, favorites_ref);
    }
}
