use crate::system::{
    core_foundation::RawRef,
    favorites::errors::{FavoritesError, Result},
};
use core_services::OpaqueLSSharedFileListRef;

pub struct FavoritesHandle(RawRef<OpaqueLSSharedFileListRef>);

impl TryFrom<*mut OpaqueLSSharedFileListRef> for FavoritesHandle {
    type Error = FavoritesError;

    fn try_from(ptr: *mut OpaqueLSSharedFileListRef) -> Result<Self> {
        RawRef::try_from(ptr)
            .map(Self)
            .map_err(|_| FavoritesError::NullListHandle)
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
    use std::ptr::NonNull;
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
