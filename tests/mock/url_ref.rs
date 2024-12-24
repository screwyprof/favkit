use std::rc::Rc;

use core_foundation::url::CFURLRef;
use core_services::OpaqueLSSharedFileListItemRef;
use favkit::system::favorites::Url;

use super::cf_favorites::ItemIndex;

pub struct UrlRef(pub(crate) CFURLRef);

impl From<(&Rc<Vec<Url>>, *mut OpaqueLSSharedFileListItemRef)> for UrlRef {
    fn from((urls, item): (&Rc<Vec<Url>>, *mut OpaqueLSSharedFileListItemRef)) -> Self {
        let idx = ItemIndex::from(item);
        Self((&urls.clone()[idx.index]).into())
    }
}

impl From<UrlRef> for CFURLRef {
    fn from(url_ref: UrlRef) -> Self {
        url_ref.0
    }
}
