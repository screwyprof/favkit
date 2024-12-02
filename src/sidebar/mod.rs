pub mod cf;
mod error;
mod finder;

use std::fmt;
use std::path::{Path, PathBuf};
use std::str::FromStr;

pub use self::cf::{CoreServicesOperations, DefaultCoreServices};
pub use self::error::{Result, SidebarError};
use self::finder::FinderSidebar;

/// Common favorite items in the Finder sidebar
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FavoriteItem {
    Applications,
    Desktop,
    Documents,
    Downloads,
    Home,
    Movies,
    Music,
    Pictures,
}

impl FavoriteItem {
    pub fn path(&self) -> PathBuf {
        match self {
            Self::Applications => PathBuf::from("/Applications"),
            Self::Desktop => dirs::desktop_dir().unwrap_or_else(|| PathBuf::from("~/Desktop")),
            Self::Documents => dirs::document_dir().unwrap_or_else(|| PathBuf::from("~/Documents")),
            Self::Downloads => dirs::download_dir().unwrap_or_else(|| PathBuf::from("~/Downloads")),
            Self::Home => dirs::home_dir().unwrap_or_else(|| PathBuf::from("~")),
            Self::Movies => dirs::video_dir().unwrap_or_else(|| PathBuf::from("~/Movies")),
            Self::Music => dirs::audio_dir().unwrap_or_else(|| PathBuf::from("~/Music")),
            Self::Pictures => dirs::picture_dir().unwrap_or_else(|| PathBuf::from("~/Pictures")),
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Applications => "Applications",
            Self::Desktop => "Desktop",
            Self::Documents => "Documents",
            Self::Downloads => "Downloads",
            Self::Home => "Home",
            Self::Movies => "Movies",
            Self::Music => "Music",
            Self::Pictures => "Pictures",
        }
    }
}

/// Special locations in the Finder sidebar
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecialLocation {
    AirDrop,
    RemoteDisc,
    RecentsFolder,
    AllMyFiles,
    NetworkFolder,
    ICloudDrive,
}

impl SpecialLocation {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::AirDrop => "AirDrop",
            Self::RemoteDisc => "Remote Disc",
            Self::RecentsFolder => "Recents",
            Self::AllMyFiles => "All My Files",
            Self::NetworkFolder => "Network",
            Self::ICloudDrive => "iCloud Drive",
        }
    }
}

#[derive(Debug, Clone)]
pub struct SidebarItem {
    pub name: String,
    pub url: SidebarUrl,
}

#[derive(Debug, Clone)]
pub enum SidebarUrl {
    File(PathBuf),
    AirDrop,
    RemoteDisc,
    SystemUrl(String),
    NotFound,
}

impl fmt::Display for SidebarUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::File(path) => write!(f, "file://{}", path.display()),
            Self::AirDrop => write!(f, "nwnode://domain-AirDrop"),
            Self::RemoteDisc => write!(f, "com-apple-sfl://IsRemoteDisc"),
            Self::SystemUrl(url) => write!(f, "{}", url),
            Self::NotFound => write!(f, "NOTFOUND"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SidebarSection {
    Favorites,
    Locations,
}

impl FromStr for SidebarSection {
    type Err = SidebarError;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "favorites" => Ok(Self::Favorites),
            "locations" => Ok(Self::Locations),
            _ => Err(SidebarError::InvalidInput(format!(
                "Invalid section: {}",
                s
            ))),
        }
    }
}

pub trait SidebarOperations {
    fn list_items(&self) -> Result<Vec<SidebarItem>>;
    fn add_item(&self, path: &str) -> Result<()>;
    fn remove_item(&self, path: &str) -> Result<()>;
}

pub struct Sidebar {
    core_services: Box<dyn CoreServicesOperations>,
}

impl Default for Sidebar {
    fn default() -> Self {
        Self {
            core_services: Box::new(DefaultCoreServices),
        }
    }
}

impl Sidebar {
    pub fn new() -> Self {
        Self::default()
    }

    // For testing
    pub fn new_with_core_services(core_services: Box<dyn CoreServicesOperations>) -> Self {
        Self { core_services }
    }

    pub fn favorites(&self) -> FavoritesSidebar<'_> {
        FavoritesSidebar {
            finder: FinderSidebar::new_favorites(self.core_services.as_ref())
                .expect("Failed to create favorites sidebar"),
        }
    }

    pub fn locations(&self) -> LocationsSidebar<'_> {
        LocationsSidebar {
            finder: FinderSidebar::new_volumes(self.core_services.as_ref())
                .expect("Failed to create locations sidebar"),
        }
    }
}

pub struct FavoritesSidebar<'a> {
    finder: FinderSidebar<'a>,
}

impl FavoritesSidebar<'_> {
    pub fn list_items(&self) -> Result<Vec<SidebarItem>> {
        self.finder.list_items()
    }

    pub fn add_item(&self, path: impl AsRef<Path>) -> Result<()> {
        self.finder.add_item(path.as_ref().to_str().ok_or_else(|| {
            SidebarError::InvalidInput(format!("Invalid path: {:?}", path.as_ref()))
        })?)
    }

    pub fn add_favorite(&self, item: FavoriteItem) -> Result<()> {
        self.add_item(item.path())
    }

    pub fn remove_item(&self, name: &str) -> Result<()> {
        let items = self.list_items()?;
        if let Some(item) = items.iter().find(|i| i.name == name) {
            if let SidebarUrl::File(path) = &item.url {
                return self.finder.remove_item(path.to_str().ok_or_else(|| {
                    SidebarError::InvalidInput(format!("Invalid path: {:?}", path))
                })?);
            }
        }
        Err(SidebarError::NotFound(format!("Item not found: {}", name)))
    }

    pub fn add_special_location(&self, location: SpecialLocation) -> Result<()> {
        match location {
            SpecialLocation::AirDrop => self.finder.add_item("nwnode://domain-AirDrop"),
            SpecialLocation::RemoteDisc => self.finder.add_item("com-apple-sfl://IsRemoteDisc"),
            _ => Err(SidebarError::Operation(
                "Unsupported special location".into(),
            )),
        }
    }
}

pub struct LocationsSidebar<'a> {
    finder: FinderSidebar<'a>,
}

impl LocationsSidebar<'_> {
    pub fn list_items(&self) -> Result<Vec<SidebarItem>> {
        self.finder.list_items()
    }
}
