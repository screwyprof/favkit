use std::rc::Rc;

use core_foundation::{
    array::CFArray,
    base::TCFType,
    string::CFString,
    url::{CFURL, kCFURLPOSIXPathStyle},
};
use core_services::{LSSharedFileListItemRef, OpaqueLSSharedFileListItemRef};
use favkit::system::favorites::{
    DisplayName, Snapshot, Url,
    errors::{FavoritesError, Result},
};

use super::favorites::Favorites;

// Type-safe index for Core Foundation items
#[derive(Debug)]
pub(crate) struct ItemIndex {
    pub(crate) index: usize,
}

impl From<LSSharedFileListItemRef> for ItemIndex {
    fn from(raw: LSSharedFileListItemRef) -> Self {
        Self {
            index: (raw as i32 - 1) as usize,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CFFavorites {
    pub(crate) snapshot: Rc<Option<Snapshot>>,
    pub(crate) display_names: Rc<Vec<DisplayName>>,
    pub(crate) urls: Rc<Vec<Url>>,
}

impl CFFavorites {
    fn create_display_name(name: Option<&str>) -> Result<DisplayName> {
        let name = name.unwrap_or_default();
        let cf_string = CFString::new(name);
        DisplayName::try_from(cf_string.as_concrete_TypeRef())
    }

    fn create_url(path: &str) -> Result<Url> {
        let is_dir = path.ends_with('/');
        let file_path = CFString::new(path);
        let url_cf = CFURL::from_file_system_path(file_path, kCFURLPOSIXPathStyle, is_dir);
        Url::try_from(url_cf.as_concrete_TypeRef())
    }

    fn create_snapshot(items_count: usize) -> Result<Snapshot> {
        let snapshot_items: Vec<_> = (1..=items_count)
            .map(|i| (i as i32) as *mut OpaqueLSSharedFileListItemRef)
            .collect();
        let array = CFArray::from_copyable(&snapshot_items);
        Snapshot::try_from(array.as_concrete_TypeRef())
    }
}

impl TryFrom<&Favorites> for CFFavorites {
    type Error = FavoritesError;

    fn try_from(favorites: &Favorites) -> Result<Self> {
        let items = favorites.items();

        let snapshot = Rc::new(Some(Self::create_snapshot(items.len())?));
        let display_names = Rc::new(
            items
                .iter()
                .map(|item| Self::create_display_name(item.name.as_deref()))
                .collect::<Result<Vec<_>>>()?,
        );
        let urls = Rc::new(
            items
                .iter()
                .map(|item| Self::create_url(&item.path))
                .collect::<Result<Vec<_>>>()?,
        );

        Ok(Self {
            snapshot,
            display_names,
            urls,
        })
    }
}
