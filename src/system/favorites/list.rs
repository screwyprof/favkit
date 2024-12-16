use core_services::{LSSharedFileListRef, OpaqueLSSharedFileListRef};

use crate::system::core_foundation::{Raw, Safe};

pub(crate) type RawFavoritesList = Raw<*mut OpaqueLSSharedFileListRef>;
pub(crate) type FavoritesList = Safe<LSSharedFileListRef>;

impl From<FavoritesList> for LSSharedFileListRef {
    fn from(list: FavoritesList) -> Self {
        list.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn should_return_none_for_null_list() -> Result<()> {
        let list_ref = std::ptr::null_mut();
        let raw = Raw::from(list_ref);
        assert!(Option::<FavoritesList>::from(raw).is_none());
        Ok(())
    }

    #[test]
    fn should_wrap_valid_list() -> Result<()> {
        let list_ref = 1 as *mut OpaqueLSSharedFileListRef;
        let raw = Raw::from(list_ref);
        let list = Option::<FavoritesList>::from(raw).ok_or("Failed to create FavoritesList")?;
        assert_eq!(LSSharedFileListRef::from(list), list_ref);
        Ok(())
    }
}
