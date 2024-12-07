pub mod macos;
pub mod macos_impl;
pub mod repository;
pub mod sidebar;
pub mod sidebar_item;
pub mod target;


/// The Finder represents a macOS Finder window.
/// It provides access to the sidebar items and allows modifying them.
pub struct Finder {
    sidebar: sidebar::Sidebar,
}

impl Finder {
    pub fn new(sidebar: sidebar::Sidebar) -> Self {
        Self { sidebar }
    }

    pub fn sidebar(&self) -> &sidebar::Sidebar {
        &self.sidebar
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::finder::{
        sidebar_item::SidebarItem,
        target::Target,
    };
    use std::path::PathBuf;

    #[test]
    fn test_finder_provides_access_to_sidebar() {
        // Given
        let favorites = vec![
            SidebarItem::new(Target::Home(PathBuf::from("/Users/test"))),
            SidebarItem::new(Target::Desktop(PathBuf::from("/Users/test/Desktop"))),
        ];
        let sidebar = sidebar::Sidebar::new(favorites.clone());
        let finder = Finder::new(sidebar);

        // When
        let result = finder.sidebar().favorites();

        // Then
        assert_eq!(result, favorites.as_slice());
    }
}
