use core_foundation::{
    array::CFArray,
    base::TCFType,
    string::CFString,
    url::{CFURL, kCFURLPOSIXPathStyle},
};
use core_services::OpaqueLSSharedFileListItemRef;
use favkit::system::favorites::{DisplayName, Snapshot, Url};
use std::rc::Rc;

/// Builder for creating test data
#[derive(Default)]
pub struct FavoritesBuilder {
    items: Vec<(Option<&'static str>, &'static str)>,
}

impl FavoritesBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_item(mut self, name: Option<&'static str>, url: &'static str) -> Self {
        self.items.push((name, url));
        self
    }

    pub fn build(self) -> Favorites {
        let items = self
            .items
            .into_iter()
            .map(|(name, url)| FavoriteItem::new(name, url))
            .collect();
        Favorites::new(items)
    }
}

/// Represents a favorite item with its Core Foundation data
#[derive(Debug)]
struct FavoriteItem {
    display_name: DisplayName,
    url: Url,
}

impl FavoriteItem {
    fn new(display_name: Option<&str>, url: &str) -> Self {
        let display_name = {
            let name = display_name.unwrap_or_default();
            let cf_string = CFString::new(name);
            DisplayName::try_from(cf_string.as_concrete_TypeRef()).unwrap()
        };

        let url = {
            let is_dir = url.ends_with('/');
            let file_path = CFString::new(url);
            let url_cf = CFURL::from_file_system_path(file_path, kCFURLPOSIXPathStyle, is_dir);
            Url::try_from(url_cf.as_concrete_TypeRef()).unwrap()
        };

        Self { display_name, url }
    }
}

/// Collection of favorite items with their snapshot
#[derive(Debug)]
pub struct Favorites {
    pub(crate) snapshot: Rc<Option<Snapshot>>,
    pub(crate) display_names: Rc<Vec<DisplayName>>,
    pub(crate) urls: Rc<Vec<Url>>,
}

impl Default for Favorites {
    fn default() -> Self {
        Self {
            snapshot: Rc::new(None),
            display_names: Rc::new(Vec::new()),
            urls: Rc::new(Vec::new()),
        }
    }
}

impl Favorites {
    fn new(items: Vec<FavoriteItem>) -> Self {
        let snapshot = {
            let snapshot_items: Vec<_> = (1..=items.len())
                .map(|i| (i as i32) as *mut OpaqueLSSharedFileListItemRef)
                .collect();
            let array = CFArray::from_copyable(&snapshot_items);
            Rc::new(Some(
                Snapshot::try_from(array.as_concrete_TypeRef()).unwrap(),
            ))
        };

        let display_names = Rc::new(items.iter().map(|item| item.display_name.clone()).collect());
        let urls = Rc::new(items.iter().map(|item| item.url.clone()).collect());

        Self {
            snapshot,
            display_names,
            urls,
        }
    }
}
