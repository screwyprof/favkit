use core_foundation::{
    array::{CFArray, CFArrayRef},
    base::{CFIndex, CFRange, TCFType},
};
use core_services::LSSharedFileListItemRef;

pub(crate) struct RawSnapshot(CFArrayRef);

impl From<CFArrayRef> for RawSnapshot {
    fn from(array: CFArrayRef) -> Self {
        Self(array)
    }
}

pub(crate) struct Snapshot(CFArray<LSSharedFileListItemRef>);

impl Snapshot {
    pub(crate) fn len(&self) -> usize {
        self.0.len().try_into().unwrap()
    }

    pub(crate) fn get(&self, index: usize) -> Option<LSSharedFileListItemRef> {
        let cf_index: CFIndex = index.try_into().ok()?;
        if cf_index >= self.0.len() {
            return None;
        }

        let range = CFRange::init(cf_index, 1);
        let mut values = CFArray::get_values(&self.0, range);
        values.pop().map(|ptr| ptr as LSSharedFileListItemRef)
    }
}

impl From<RawSnapshot> for Option<Snapshot> {
    fn from(snapshot: RawSnapshot) -> Self {
        (!snapshot.0.is_null())
            .then(|| unsafe { Snapshot(CFArray::wrap_under_get_rule(snapshot.0)) })
    }
}
