mod cf;
mod error;
mod finder;
mod url;

use core_foundation::string::CFStringRef;
use core_services::{kLSSharedFileListFavoriteItems, kLSSharedFileListFavoriteVolumes};
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;

pub use self::error::{Result, SidebarError};
use self::finder::FinderSidebar;

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
    unsafe fn list_type(&self) -> CFStringRef {
        match self {
            Self::Favorites => kLSSharedFileListFavoriteItems,
            Self::Locations => kLSSharedFileListFavoriteVolumes,
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
        unsafe { FinderSidebar::new(section.list_type()) }.map(Self)
    }

    pub fn favorites() -> Result<Self> {
        Self::new(SidebarSection::Favorites)
    }

    pub fn locations() -> Result<Self> {
        Self::new(SidebarSection::Locations)
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
