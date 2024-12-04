mod macos_api;
mod path;
mod sidebar_api;

use core_foundation::string::CFString;
use std::convert::TryFrom;

use crate::error::{Error, Result};

// Re-export all public types
pub use self::{
    macos_api::{MacOsApi, RealMacOsApi},
    path::{CFURLWrapper, MacOsLocation, MacOsPath},
    sidebar_api::SidebarApi,
};

pub struct Sidebar<T: MacOsApi = RealMacOsApi> {
    api: SidebarApi<T>,
}

impl<T: MacOsApi> Sidebar<T> {
    pub fn with_api(api: T) -> Self {
        Self {
            api: SidebarApi::new(api),
        }
    }

    pub fn favorites(&self) -> FavoritesSection<'_, T> {
        FavoritesSection { api: &self.api }
    }

    pub fn list_items(&self) -> Vec<SidebarItem> {
        self.favorites().list_items()
    }
}

impl Default for Sidebar {
    fn default() -> Self {
        Self::with_api(RealMacOsApi)
    }
}

impl Sidebar {
    pub fn new() -> Self {
        Self::default()
    }
}

pub struct FavoritesSection<'a, T: MacOsApi> {
    api: &'a SidebarApi<T>,
}

impl<T: MacOsApi> FavoritesSection<'_, T> {
    pub fn list_items(&self) -> Vec<SidebarItem> {
        self.api.list_favorite_items()
    }

    pub fn iter(&self) -> impl Iterator<Item = SidebarItem> + '_ {
        self.list_items().into_iter()
    }
}

impl<T: MacOsApi> IntoIterator for &FavoritesSection<'_, T> {
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

impl TryFrom<(CFURLWrapper<'_>, Option<CFString>)> for SidebarItem {
    type Error = Error;

    fn try_from((url_wrapper, name): (CFURLWrapper<'_>, Option<CFString>)) -> Result<Self> {
        let path = MacOsPath::try_from(url_wrapper)?;

        Ok(match &path {
            MacOsPath::Location(location) => match location {
                MacOsLocation::AirDrop => Self::airdrop(),
                MacOsLocation::Applications => Self::applications(),
                MacOsLocation::Desktop => Self::desktop(),
                MacOsLocation::Documents => Self::documents(),
                MacOsLocation::Downloads => Self::downloads(),
                MacOsLocation::Home => Self::home(),
                MacOsLocation::Recents => Self::recents(),
                MacOsLocation::UserApplications => Self::new("Applications", path),
            },
            MacOsPath::Custom(_) => match name {
                Some(name) => Self::new(name.to_string(), path),
                None => Self::new(path.name(), path),
            },
        })
    }
}

impl SidebarItem {
    pub fn new(name: impl Into<String>, path: impl Into<MacOsPath>) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
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

    pub fn airdrop() -> Self {
        Self::location(MacOsLocation::AirDrop)
    }

    pub fn recents() -> Self {
        Self::location(MacOsLocation::Recents)
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
