mod macos_api;
mod path;
mod sidebar_api;

use core_foundation::string::CFString;
use std::convert::TryFrom;
use std::path::PathBuf;

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

    pub fn list_items(&self) -> Result<Vec<SidebarItem>> {
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
    pub fn list_items(&self) -> Result<Vec<SidebarItem>> {
        self.api.list_favorite_items()
    }

    pub fn iter(&self) -> impl Iterator<Item = SidebarItem> + '_ {
        self.list_items().unwrap_or_default().into_iter()
    }
}

impl<T: MacOsApi> IntoIterator for &FavoritesSection<'_, T> {
    type Item = SidebarItem;
    type IntoIter = std::vec::IntoIter<SidebarItem>;

    fn into_iter(self) -> Self::IntoIter {
        self.list_items().unwrap_or_default().into_iter()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SidebarItem {
    name: String,
    path: MacOsPath,
}

impl SidebarItem {
    /// Creates a new SidebarItem builder.
    pub fn builder() -> SidebarItemBuilder {
        SidebarItemBuilder::new()
    }

    /// Creates a new SidebarItem with the given name and path.
    pub fn new(name: impl Into<String>, path: impl Into<MacOsPath>) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
        }
    }

    /// Gets the name of the item.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Gets the path of the item.
    pub fn path(&self) -> &MacOsPath {
        &self.path
    }

    /// Creates a new Applications item.
    pub fn applications() -> Self {
        Self::location(MacOsLocation::Applications)
    }

    /// Creates a new Downloads item.
    pub fn downloads() -> Self {
        Self::location(MacOsLocation::Downloads)
    }

    /// Creates a new Desktop item.
    pub fn desktop() -> Self {
        Self::location(MacOsLocation::Desktop)
    }

    /// Creates a new Documents item.
    pub fn documents() -> Self {
        Self::location(MacOsLocation::Documents)
    }

    /// Creates a new Home item.
    pub fn home() -> Self {
        Self::location(MacOsLocation::Home)
    }

    /// Creates a new AirDrop item.
    pub fn airdrop() -> Self {
        Self::location(MacOsLocation::AirDrop)
    }

    /// Creates a new Recents item.
    pub fn recents() -> Self {
        Self::location(MacOsLocation::Recents)
    }

    /// Creates a new item from a MacOsLocation.
    fn location(location: MacOsLocation) -> Self {
        let name = location.name().to_string();
        Self::new(name, location)
    }
}

impl std::fmt::Display for SidebarItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.path)
    }
}

/// A builder for creating SidebarItems.
#[derive(Debug, Default)]
pub struct SidebarItemBuilder {
    name: Option<String>,
    path: Option<MacOsPath>,
}

impl SidebarItemBuilder {
    /// Creates a new SidebarItemBuilder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the name of the item.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Sets the path of the item.
    pub fn path(mut self, path: impl Into<MacOsPath>) -> Self {
        self.path = Some(path.into());
        self
    }

    /// Sets the location of the item.
    pub fn location(mut self, location: MacOsLocation) -> Self {
        self.name = Some(location.name().to_string());
        self.path = Some(location.into());
        self
    }

    /// Builds the SidebarItem.
    pub fn build(self) -> Result<SidebarItem> {
        let path = self.path.ok_or_else(|| Error::InvalidPath {
            path: PathBuf::from("<no path>"),
        })?;

        let name = self.name.unwrap_or_else(|| path.name());

        Ok(SidebarItem { name, path })
    }
}

impl TryFrom<(CFURLWrapper<'_>, Option<CFString>)> for SidebarItem {
    type Error = Error;

    fn try_from((url_wrapper, name): (CFURLWrapper<'_>, Option<CFString>)) -> Result<Self> {
        let path = MacOsPath::try_from(url_wrapper)?;

        let builder = Self::builder().path(path);
        let builder = if let Some(name) = name {
            builder.name(name.to_string())
        } else {
            builder
        };

        builder.build()
    }
}
