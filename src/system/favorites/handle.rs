use crate::system::core_foundation::RawRef;
use core_services::OpaqueLSSharedFileListRef;

pub(crate) type FavoritesHandle = RawRef<OpaqueLSSharedFileListRef>;

#[cfg(test)]
mod tests {
    use super::*;
    use core_services::LSSharedFileListRef;

    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn should_convert_raw_pointer_to_handle() -> Result<()> {
        let handle_ref = 1 as LSSharedFileListRef;
        let handle = FavoritesHandle::from(handle_ref);
        assert_eq!(LSSharedFileListRef::from(handle), handle_ref);
        Ok(())
    }

    #[test]
    fn should_convert_handle_back_to_raw_pointer() -> Result<()> {
        let handle_ref = 1 as LSSharedFileListRef;
        let handle = FavoritesHandle::from(handle_ref);
        let unwrapped: LSSharedFileListRef = handle.into();
        assert_eq!(unwrapped, handle_ref);
        Ok(())
    }
}
