use std::rc::Rc;

use favkit::system::favorites::{
    DisplayName as SystemDisplayName, Snapshot as SystemSnapshot, Url as SystemUrl,
    errors::{FavoritesError, Result},
};

use crate::mock::{
    cf::{display_name::DisplayName, snapshot::Snapshot, url::Url},
    favorites::Favorites,
};

#[derive(Debug, Clone)]
pub struct CFFavorites {
    pub(crate) snapshot: Rc<Option<SystemSnapshot>>,
    pub(crate) display_names: Rc<Vec<SystemDisplayName>>,
    pub(crate) urls: Rc<Vec<SystemUrl>>,
}

impl TryFrom<&Favorites> for CFFavorites {
    type Error = FavoritesError;

    fn try_from(favorites: &Favorites) -> Result<Self> {
        let items = favorites.items();

        let snapshot = Rc::new(Some(SystemSnapshot::try_from(Snapshot::from(items.len()))?));
        let display_names = Rc::new(
            items
                .iter()
                .map(|item| SystemDisplayName::try_from(DisplayName::from(&item.name)))
                .collect::<Result<Vec<_>>>()?,
        );
        let urls = Rc::new(
            items
                .iter()
                .map(|item| SystemUrl::try_from(Url::from(&item.path)))
                .collect::<Result<Vec<_>>>()?,
        );

        Ok(Self {
            snapshot,
            display_names,
            urls,
        })
    }
}
