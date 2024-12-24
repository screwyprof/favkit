use std::rc::Rc;

use core_foundation::{
    array::CFArray,
    base::TCFType,
    string::CFString,
    url::{CFURL, kCFURLPOSIXPathStyle},
};
use core_services::OpaqueLSSharedFileListItemRef;
use favkit::system::favorites::{DisplayName, Snapshot, Url};

// Domain types
#[derive(Debug)]
pub struct FavoriteItem {
    name: Option<String>,
    path: String,
}

#[derive(Debug, Default)]
pub struct Favorites {
    items: Vec<FavoriteItem>,
}

impl Favorites {
    pub fn new(items: Vec<FavoriteItem>) -> Self {
        Self { items }
    }

    pub fn items(&self) -> &[FavoriteItem] {
        &self.items
    }
}

// Infrastructure types
#[derive(Debug)]
pub struct CFFavorites {
    pub(crate) snapshot: Rc<Option<Snapshot>>,
    pub(crate) display_names: Rc<Vec<DisplayName>>,
    pub(crate) urls: Rc<Vec<Url>>,
}

impl From<&Favorites> for CFFavorites {
    fn from(favorites: &Favorites) -> Self {
        let items = favorites.items();

        // Create snapshot
        let snapshot = {
            let snapshot_items: Vec<_> = (1..=items.len())
                .map(|i| (i as i32) as *mut OpaqueLSSharedFileListItemRef)
                .collect();
            let array = CFArray::from_copyable(&snapshot_items);
            Rc::new(Some(
                Snapshot::try_from(array.as_concrete_TypeRef()).unwrap(),
            ))
        };

        // Create display names
        let display_names = Rc::new(
            items
                .iter()
                .map(|item| {
                    let name = item.name.as_deref().unwrap_or_default();
                    let cf_string = CFString::new(name);
                    DisplayName::try_from(cf_string.as_concrete_TypeRef()).unwrap()
                })
                .collect(),
        );

        // Create URLs
        let urls = Rc::new(
            items
                .iter()
                .map(|item| {
                    let is_dir = item.path.ends_with('/');
                    let file_path = CFString::new(&item.path);
                    let url_cf =
                        CFURL::from_file_system_path(file_path, kCFURLPOSIXPathStyle, is_dir);
                    Url::try_from(url_cf.as_concrete_TypeRef()).unwrap()
                })
                .collect(),
        );

        Self {
            snapshot,
            display_names,
            urls,
        }
    }
}

/// Builder for creating test data
#[derive(Default)]
pub struct FavoritesBuilder {
    items: Vec<(Option<&'static str>, &'static str)>,
}

impl FavoritesBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_item(mut self, name: Option<&'static str>, path: &'static str) -> Self {
        self.items.push((name, path));
        self
    }

    pub fn build(self) -> Favorites {
        let items = self
            .items
            .into_iter()
            .map(|(name, path)| FavoriteItem {
                name: name.map(String::from),
                path: path.to_string(),
            })
            .collect();
        Favorites::new(items)
    }
}
