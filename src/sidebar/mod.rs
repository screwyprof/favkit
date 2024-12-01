pub mod finder;
pub mod traits;

use core_foundation::string::CFStringRef;
use core_services::{kLSSharedFileListFavoriteItems, kLSSharedFileListFavoriteVolumes};

use finder::FinderSidebar;
pub use traits::SidebarOperations;

#[derive(Debug, Clone, Copy)]
pub enum SidebarSection {
    Favorites,
    Locations,
}

impl SidebarSection {
    unsafe fn list_type(&self) -> CFStringRef {
        match self {
            Self::Favorites => kLSSharedFileListFavoriteItems,
            Self::Locations => kLSSharedFileListFavoriteVolumes,
        }
    }
}

pub struct Sidebar(FinderSidebar);

impl Sidebar {
    pub fn new(section: SidebarSection) -> crate::error::Result<Self> {
        unsafe { FinderSidebar::new(section.list_type()) }.map(Self)
    }
}

// Delegate to the inner implementation
impl SidebarOperations for Sidebar {
    fn list_items(&self) -> crate::error::Result<Vec<crate::types::SidebarItem>> {
        self.0.list_items()
    }

    fn add_item(&self, path: &str) -> crate::error::Result<()> {
        self.0.add_item(path)
    }

    fn remove_item(&self, path: &str) -> crate::error::Result<()> {
        self.0.remove_item(path)
    }
}
