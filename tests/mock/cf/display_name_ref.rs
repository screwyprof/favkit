use std::rc::Rc;

use core_foundation::string::CFStringRef;
use core_services::OpaqueLSSharedFileListItemRef;
use favkit::system::favorites::DisplayName;

use super::item_index::ItemIndex;

pub struct DisplayNameRef(pub(crate) CFStringRef);

impl From<(&Rc<Vec<DisplayName>>, *mut OpaqueLSSharedFileListItemRef)> for DisplayNameRef {
    fn from(
        (display_names, item): (&Rc<Vec<DisplayName>>, *mut OpaqueLSSharedFileListItemRef),
    ) -> Self {
        let idx = ItemIndex::from(item);
        Self((&display_names.clone()[idx.index]).into())
    }
}

impl From<DisplayNameRef> for CFStringRef {
    fn from(display_name_ref: DisplayNameRef) -> Self {
        display_name_ref.0
    }
}
