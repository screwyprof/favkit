use core_foundation::{
    array::{CFArray, CFArrayRef},
    base::TCFType,
};
use core_services::LSSharedFileListItemRef;

pub(crate) struct RawSnapshot(CFArrayRef);

impl From<CFArrayRef> for RawSnapshot {
    fn from(array: CFArrayRef) -> Self {
        Self(array)
    }
}

#[allow(dead_code)]
pub(crate) struct Snapshot(CFArray<LSSharedFileListItemRef>);

impl From<RawSnapshot> for Option<Snapshot> {
    fn from(snapshot: RawSnapshot) -> Self {
        (!snapshot.0.is_null())
            .then(|| unsafe { Snapshot(CFArray::wrap_under_get_rule(snapshot.0)) })
    }
}
