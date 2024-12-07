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
    fn test_finder_integration() {
        // Given
        let favorites = vec![
            SidebarItem::new(Target::Home(PathBuf::from("/Users/test"))),
            SidebarItem::new(Target::Desktop(PathBuf::from("/Users/test/Desktop"))),
        ];
        let sidebar = sidebar::Sidebar::new(favorites);
        let finder = Finder::new(sidebar);

        // When
        let result = finder.sidebar().favorites();

        // Then
        assert_eq!(result.len(), 2);
        assert!(matches!(result[0].target(), Target::Home(_)));
        assert!(matches!(result[1].target(), Target::Desktop(_)));
    }
}
