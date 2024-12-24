use std::rc::Rc;

use core_foundation::array::CFArrayRef;
use favkit::system::favorites::Snapshot;

/// Test double for LSSharedFileListCopySnapshot result
#[derive(Clone, Copy)]
pub(crate) struct SnapshotRef(CFArrayRef);

impl SnapshotRef {
    pub(crate) fn null() -> Self {
        Self(std::ptr::null_mut())
    }
}

impl From<&Rc<Option<Snapshot>>> for SnapshotRef {
    fn from(snapshot: &Rc<Option<Snapshot>>) -> Self {
        Self(
            snapshot
                .clone()
                .as_ref()
                .as_ref()
                .expect("Snapshot must exist")
                .into(),
        )
    }
}

impl From<SnapshotRef> for CFArrayRef {
    fn from(handle: SnapshotRef) -> Self {
        handle.0
    }
}
