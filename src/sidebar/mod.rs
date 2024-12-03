mod macos_api;
mod path;

pub use self::macos_api::{MacOsApi, RawMacOsApi, RealMacOsApi};
pub use self::path::{MacOsLocation, MacOsPath};

pub struct Sidebar<T: RawMacOsApi = RealMacOsApi> {
    api: MacOsApi<T>,
}

impl<T: RawMacOsApi> Sidebar<T> {
    pub fn with_api(raw_api: T) -> Self {
        Self {
            api: MacOsApi::new(raw_api),
        }
    }

    pub fn favorites(&self) -> FavoritesSection<'_, T> {
        FavoritesSection { api: &self.api }
    }

    pub fn list_items(&self) -> Vec<SidebarItem> {
        self.favorites().list_items()
    }
}

impl Default for Sidebar<RealMacOsApi> {
    fn default() -> Self {
        Self::with_api(RealMacOsApi)
    }
}

impl Sidebar<RealMacOsApi> {
    pub fn new() -> Self {
        Self::default()
    }
}

pub struct FavoritesSection<'a, T: RawMacOsApi> {
    api: &'a MacOsApi<T>,
}

impl<T: RawMacOsApi> FavoritesSection<'_, T> {
    pub fn list_items(&self) -> Vec<SidebarItem> {
        self.api
            .list_favorite_items()
            .into_iter()
            .map(|(name, path)| SidebarItem::new(name, path))
            .collect()
    }

    pub fn iter(&self) -> impl Iterator<Item = SidebarItem> + '_ {
        self.api
            .list_favorite_items()
            .into_iter()
            .map(|(name, path)| SidebarItem::new(name, path))
    }
}

impl<T: RawMacOsApi> IntoIterator for &FavoritesSection<'_, T> {
    type Item = SidebarItem;
    type IntoIter = std::vec::IntoIter<SidebarItem>;

    fn into_iter(self) -> Self::IntoIter {
        self.list_items().into_iter()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SidebarItem {
    name: String,
    path: MacOsPath,
}

impl SidebarItem {
    pub fn new(name: impl Into<String>, path: impl AsRef<std::path::Path>) -> Self {
        Self {
            name: name.into(),
            path: MacOsPath::from(path.as_ref()),
        }
    }

    pub fn applications() -> Self {
        Self::location(MacOsLocation::Applications)
    }

    pub fn downloads() -> Self {
        Self::location(MacOsLocation::Downloads)
    }

    pub fn documents() -> Self {
        Self::location(MacOsLocation::Documents)
    }

    pub fn desktop() -> Self {
        Self::location(MacOsLocation::Desktop)
    }

    pub fn home() -> Self {
        Self::location(MacOsLocation::Home)
    }

    fn location(location: MacOsLocation) -> Self {
        Self {
            name: location.name().to_string(),
            path: location.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> &MacOsPath {
        &self.path
    }
}

impl std::fmt::Display for SidebarItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.path)
    }
}
