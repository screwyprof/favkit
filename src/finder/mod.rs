use std::fmt;

pub mod macos;
pub mod macos_impl;
pub mod macos_url;
pub mod repository;
pub mod sidebar;
pub mod sidebar_item;
pub mod target;

use sidebar::Sidebar;
use target::TargetLocation;

pub use macos_impl::SystemMacOsApi;

pub struct Finder {
    sidebar: Sidebar,
}

impl Finder {
    pub fn new(sidebar: Sidebar) -> Self {
        Self { sidebar }
    }

    pub fn sidebar(&self) -> &Sidebar {
        &self.sidebar
    }
}

impl fmt::Display for Finder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Favorites:")?;
        for item in self.sidebar().favorites() {
            let target_location = match item.target().location() {
                TargetLocation::Path(p) => p.display().to_string(),
                TargetLocation::Url(u) => u.to_string(),
            };
            writeln!(f, "- {} ({})", item, target_location)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::finder::{
        sidebar_item::SidebarItem,
        target::{Target, TargetLocation},
    };
    use std::path::PathBuf;

    #[test]
    fn test_finder_provides_access_to_sidebar() {
        // Given
        let favorites = vec![
            SidebarItem::new(Target::Home(TargetLocation::Path(PathBuf::from("/Users/test")))),
            SidebarItem::new(Target::Desktop(TargetLocation::Path(PathBuf::from("/Users/test/Desktop")))),
        ];
        let sidebar = sidebar::Sidebar::new(favorites.clone());
        let finder = Finder::new(sidebar);

        // When
        let result = finder.sidebar().favorites();

        // Then
        assert_eq!(result, favorites.as_slice());
    }
}
