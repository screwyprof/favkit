pub mod repository;
pub mod sidebar;
pub mod system;

pub use repository::Repository;
pub use sidebar::{item::SidebarItem, Target};
pub use system::{MacOsApi, RealMacOsApi};

/// A type alias for a Repository that uses the real macOS API
pub type RealRepository = Repository<RealMacOsApi>;

impl RealRepository {
    pub fn new_with_real_api() -> Self {
        Self::new(RealMacOsApi::new())
    }
}

pub struct Finder {
    sidebar: Vec<SidebarItem>,
}

impl Finder {
    pub fn new(sidebar: Vec<SidebarItem>) -> Self {
        Self { sidebar }
    }

    pub fn sidebar(&self) -> &[SidebarItem] {
        &self.sidebar
    }
}

impl std::fmt::Display for Finder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Finder")
    }
}
