use core_services::LSSharedFileListRef;

use crate::system::core_foundation::LSSharedFileListHandle;

pub(crate) type FavoritesHandle = LSSharedFileListHandle;

impl From<FavoritesHandle> for LSSharedFileListRef {
    fn from(handle: FavoritesHandle) -> Self {
        handle.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core_services::OpaqueLSSharedFileListRef;

    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn should_return_none_for_null_handle() -> Result<()> {
        let handle_ref = std::ptr::null_mut();
        assert!(FavoritesHandle::from_ref(handle_ref).is_none());
        Ok(())
    }

    #[test]
    fn should_wrap_valid_handle() -> Result<()> {
        let handle_ref = 1 as *mut OpaqueLSSharedFileListRef;
        let handle =
            FavoritesHandle::from_ref(handle_ref).ok_or("Failed to create FavoritesHandle")?;
        assert_eq!(LSSharedFileListRef::from(handle), handle_ref);
        Ok(())
    }
}
