mod cf;
mod error;
mod finder;
#[cfg(test)]
mod tests;
mod url;

use core_foundation::string::CFStringRef;
use core_services::{kLSSharedFileListFavoriteItems, kLSSharedFileListFavoriteVolumes};
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;

use self::cf::CoreServicesOperations;
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
            _ => Err(SidebarError::InvalidSection(s.to_string())),
        }
    }
}

impl SidebarSection {
    fn list_type(&self) -> CFStringRef {
        unsafe {
            match self {
                Self::Favorites => kLSSharedFileListFavoriteItems,
                Self::Locations => kLSSharedFileListFavoriteVolumes,
            }
        }
    }
}

pub trait SidebarOperations {
    fn list_items(&self) -> Result<Vec<SidebarItem>>;
    fn add_item(&self, path: &str) -> Result<()>;
    fn remove_item(&self, path: &str) -> Result<()>;
}

pub struct Sidebar(FinderSidebar);

impl Sidebar {
    pub fn new(section: SidebarSection) -> Result<Self> {
        FinderSidebar::new(section.list_type()).map(Self)
    }

    pub fn with_core_services(
        section: SidebarSection,
        core_services: Box<dyn CoreServicesOperations>,
    ) -> Result<Self> {
        FinderSidebar::with_core_services(section.list_type(), core_services).map(Self)
    }

    pub fn favorites() -> Result<Self> {
        Self::new(SidebarSection::Favorites)
    }

    pub fn locations() -> Result<Self> {
        Self::new(SidebarSection::Locations)
    }

    /// Add a common favorite item to the sidebar
    pub fn add_favorite(&self, item: FavoriteItem) -> Result<()> {
        self.add_item(
            item.path()
                .to_str()
                .ok_or_else(|| SidebarError::InvalidPath(item.path()))?,
        )
    }

    /// Add a special location to the sidebar
    pub fn add_location(&self, _location: SpecialLocation) -> Result<()> {
        // Special locations are handled differently and may require specific URLs
        // This is a placeholder for future implementation
        Err(SidebarError::AddItem(
            "Special locations cannot be added manually".into(),
        ))
    }
}

impl SidebarOperations for Sidebar {
    fn list_items(&self) -> Result<Vec<SidebarItem>> {
        self.0.list_items()
    }

    fn add_item(&self, path: &str) -> Result<()> {
        self.0.add_item(path)
    }

    fn remove_item(&self, path: &str) -> Result<()> {
        self.0.remove_item(path)
    }
}
