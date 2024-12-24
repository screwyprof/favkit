use core_services::OpaqueLSSharedFileListItemRef;

#[derive(Debug)]
pub(crate) struct ItemIndex {
    pub(crate) index: usize,
}

impl From<*mut OpaqueLSSharedFileListItemRef> for ItemIndex {
    fn from(raw: *mut OpaqueLSSharedFileListItemRef) -> Self {
        Self {
            index: (raw as i32 - 1) as usize,
        }
    }
}
