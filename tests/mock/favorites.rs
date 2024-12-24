use std::{convert::TryFrom, rc::Rc};

use core_foundation::{
    array::CFArray,
    base::TCFType,
    string::CFString,
    url::{CFURL, kCFURLPOSIXPathStyle},
};
use core_services::OpaqueLSSharedFileListItemRef;
use favkit::system::favorites::{DisplayName, FavoritesError, Snapshot, Url};

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

impl CFFavorites {
    fn create_display_name(name: Option<&str>) -> Result<DisplayName, FavoritesError> {
        let name = name.unwrap_or_default();
        let cf_string = CFString::new(name);
        DisplayName::try_from(cf_string.as_concrete_TypeRef())
    }

    fn create_url(path: &str) -> Result<Url, FavoritesError> {
        let is_dir = path.ends_with('/');
        let file_path = CFString::new(path);
        let url_cf = CFURL::from_file_system_path(file_path, kCFURLPOSIXPathStyle, is_dir);
        Url::try_from(url_cf.as_concrete_TypeRef())
    }

    fn create_snapshot(items_count: usize) -> Result<Snapshot, FavoritesError> {
        let snapshot_items: Vec<_> = (1..=items_count)
            .map(|i| (i as i32) as *mut OpaqueLSSharedFileListItemRef)
            .collect();
        let array = CFArray::from_copyable(&snapshot_items);
        Snapshot::try_from(array.as_concrete_TypeRef())
    }
}

impl TryFrom<&Favorites> for CFFavorites {
    type Error = FavoritesError;

    fn try_from(favorites: &Favorites) -> Result<Self, Self::Error> {
        let items = favorites.items();

        let snapshot = Rc::new(Some(Self::create_snapshot(items.len())?));
        let display_names = Rc::new(
            items
                .iter()
                .map(|item| Self::create_display_name(item.name.as_deref()))
                .collect::<Result<Vec<_>, _>>()?,
        );
        let urls = Rc::new(
            items
                .iter()
                .map(|item| Self::create_url(&item.path))
                .collect::<Result<Vec<_>, _>>()?,
        );

        Ok(Self {
            snapshot,
            display_names,
            urls,
        })
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
