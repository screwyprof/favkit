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
    pub fn iter(&self) -> impl Iterator<Item = SidebarItem> {
        self.api
            .list_favorite_items()
            .into_iter()
            .map(|(name, path)| SidebarItem { name, path })
    }

    pub fn list_items(&self) -> Vec<SidebarItem> {
        self.iter().collect()
    }
}

impl<A: MacOsApi> IntoIterator for &FavoritesSection<'_, A> {
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
    // Standard locations
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

    // Custom location
    pub fn new(name: impl Into<String>, path: impl AsRef<std::path::Path>) -> Self {
        Self {
            name: name.into(),
            path: MacOsPath::from(path.as_ref()),
        }
    }

    // Getters
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> &MacOsPath {
        &self.path
    }

    // Private helper
    fn location(location: MacOsLocation) -> Self {
        let name = match &location {
            MacOsLocation::Applications => "Applications",
            MacOsLocation::UserApplications => "Applications",
            MacOsLocation::Downloads => "Downloads",
            MacOsLocation::Documents => "Documents",
            MacOsLocation::Desktop => "Desktop",
            MacOsLocation::Home => "Home",
            MacOsLocation::Custom(path) => path.to_str().unwrap_or("Unknown"),
        }
        .to_string();

        Self {
            name,
            path: location.into(),
        }
    }

    // For testing purposes
    #[cfg(any(test, feature = "test-utils"))]
    pub fn into_parts(self) -> (String, MacOsPath) {
        (self.name, self.path)
    }
}

impl std::fmt::Display for SidebarItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.path)
    }
}

// Keep for backward compatibility
impl From<MacOsLocation> for SidebarItem {
    fn from(location: MacOsLocation) -> Self {
        Self::location(location)
    }
}
