use core_services::{LSSharedFileListRef, OpaqueLSSharedFileListRef};

pub(crate) struct RawFavoritesList(*mut OpaqueLSSharedFileListRef);

impl From<*mut OpaqueLSSharedFileListRef> for RawFavoritesList {
    fn from(list: *mut OpaqueLSSharedFileListRef) -> Self {
        Self(list)
    }
}

pub(crate) struct FavoritesList(pub(crate) LSSharedFileListRef);

impl From<RawFavoritesList> for Option<FavoritesList> {
    fn from(list: RawFavoritesList) -> Self {
        (!list.0.is_null()).then_some(FavoritesList(list.0))
    }
}
