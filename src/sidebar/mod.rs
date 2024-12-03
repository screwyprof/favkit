mod macos_api;

pub use self::macos_api::{MacOsApi, RealMacOsApi};

pub struct Sidebar<A: MacOsApi> {
    api: A,
}

impl<A: MacOsApi> Sidebar<A> {
    pub fn with_api(api: A) -> Self {
        Self { api }
    }

    pub fn favorites(&self) -> FavoritesSection<'_, A> {
        FavoritesSection { api: &self.api }
    }
}

impl Default for Sidebar<RealMacOsApi> {
    fn default() -> Self {
        Self::new()
    }
}

impl Sidebar<RealMacOsApi> {
    pub fn new() -> Self {
        Self::with_api(RealMacOsApi::new())
    }
}

pub struct FavoritesSection<'a, A> {
    api: &'a A,
}

impl<A: MacOsApi> FavoritesSection<'_, A> {
    pub fn list_items(&self) -> Vec<SidebarItem> {
        self.api
            .list_favorite_items()
            .into_iter()
            .map(|(name, _path)| SidebarItem { name })
            .collect()
    }
}

#[derive(Debug, PartialEq)]
pub struct SidebarItem {
    pub name: String,
}
