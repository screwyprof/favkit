pub mod repository;
pub mod system;
pub mod sidebar;

pub use repository::Repository;
pub use system::{MacOsApi, RealMacOsApi};
pub use sidebar::{Target, item::SidebarItem};

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

impl Repository {
    pub fn new_with_real_api() -> Self {
        Self::new(Box::new(RealMacOsApi::new()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
