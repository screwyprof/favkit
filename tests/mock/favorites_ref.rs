use core_services::LSSharedFileListRef;

#[derive(Clone, Copy)]
pub(crate) struct FavoritesRef(LSSharedFileListRef);

impl Default for FavoritesRef {
    fn default() -> Self {
        Self(1 as LSSharedFileListRef)
    }
}

impl FavoritesRef {
    pub(crate) fn null() -> Self {
        Self(std::ptr::null_mut())
    }

    pub(crate) fn new() -> Self {
        Self::default()
    }
}

impl From<FavoritesRef> for LSSharedFileListRef {
    fn from(handle: FavoritesRef) -> Self {
        handle.0
    }
}
