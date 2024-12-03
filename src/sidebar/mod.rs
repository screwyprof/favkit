mod macos_api;
mod path;

pub use self::macos_api::{MacOsApi, RealMacOsApi};
pub use self::path::{MacOsLocation, MacOsPath};

pub struct Sidebar<'a, A: MacOsApi> {
    api: &'a A,
}

impl<'a, A: MacOsApi> Sidebar<'a, A> {
    pub fn with_api(api: &'a A) -> Self {
        Self { api }
    }

    pub fn favorites(&self) -> FavoritesSection<'_, A> {
        FavoritesSection { api: self.api }
    }
}

impl Default for Sidebar<'_, RealMacOsApi> {
    fn default() -> Self {
        Self::new()
    }
}

impl Sidebar<'_, RealMacOsApi> {
    pub fn new() -> Self {
        static API: std::sync::OnceLock<RealMacOsApi> = std::sync::OnceLock::new();
        let api = API.get_or_init(RealMacOsApi::new);
        Self::with_api(api)
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
            .map(|(name, path)| SidebarItem { name, path })
            .collect()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SidebarItem {
    pub name: String,
    pub path: MacOsPath,
}
