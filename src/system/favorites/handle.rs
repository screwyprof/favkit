use core_services::{LSSharedFileListRef, OpaqueLSSharedFileListRef};

use crate::system::core_foundation::{Raw, Safe};

pub(crate) type RawFavoritesHandle = Raw<*mut OpaqueLSSharedFileListRef>;
pub(crate) type FavoritesHandle = Safe<LSSharedFileListRef>;

impl From<FavoritesHandle> for LSSharedFileListRef {
    fn from(handle: FavoritesHandle) -> Self {
        handle.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn should_return_none_for_null_handle() -> Result<()> {
        let handle_ref = std::ptr::null_mut();
        let raw = Raw::from(handle_ref);
        assert!(Option::<FavoritesHandle>::from(raw).is_none());
        Ok(())
    }

    #[test]
    fn should_wrap_valid_handle() -> Result<()> {
        let handle_ref = 1 as *mut OpaqueLSSharedFileListRef;
        let raw = Raw::from(handle_ref);
        let handle =
            Option::<FavoritesHandle>::from(raw).ok_or("Failed to create FavoritesHandle")?;
        assert_eq!(LSSharedFileListRef::from(handle), handle_ref);
        Ok(())
    }
}
