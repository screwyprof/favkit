use core_services::LSSharedFileListRef;

#[derive(Clone, Copy)]
pub(crate) struct Handle(LSSharedFileListRef);

impl Default for Handle {
    fn default() -> Self {
        Self::new()
    }
}

impl Handle {
    pub(crate) fn null() -> Self {
        Self(std::ptr::null_mut())
    }

    pub(crate) fn new() -> Self {
        Self(1 as LSSharedFileListRef)
    }
}

impl From<Handle> for LSSharedFileListRef {
    fn from(handle: Handle) -> Self {
        handle.0
    }
}
