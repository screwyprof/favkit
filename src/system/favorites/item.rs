use core_services::{LSSharedFileListItemRef, OpaqueLSSharedFileListItemRef};

pub(crate) struct RawFavoriteItem(*mut OpaqueLSSharedFileListItemRef);

impl From<*mut OpaqueLSSharedFileListItemRef> for RawFavoriteItem {
    fn from(item: *mut OpaqueLSSharedFileListItemRef) -> Self {
        Self(item)
    }
}

pub(crate) struct FavoriteItem(pub(crate) LSSharedFileListItemRef);

impl From<RawFavoriteItem> for Option<FavoriteItem> {
    fn from(item: RawFavoriteItem) -> Self {
        (!item.0.is_null()).then_some(FavoriteItem(item.0))
    }
}

impl From<FavoriteItem> for LSSharedFileListItemRef {
    fn from(item: FavoriteItem) -> Self {
        item.0
    }
}
