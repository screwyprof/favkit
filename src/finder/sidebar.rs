use super::sidebar_item::SidebarItem;

#[derive(Debug, Clone)]
pub struct Sidebar {
    favorites: Vec<SidebarItem>,
}

impl Sidebar {
    pub fn new(favorites: Vec<SidebarItem>) -> Self {
        Self { favorites }
    }

    pub fn favorites(&self) -> &[SidebarItem] {
        &self.favorites
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::target::{Target, TargetLocation};
    use std::path::PathBuf;

    #[test]
    fn test_sidebar_new() {
        let favorites = vec![SidebarItem::new(Target::Home(TargetLocation::Path(
            PathBuf::from("/Users/happygopher")
        )))];
        let sidebar = Sidebar::new(favorites);

        assert_eq!(
            sidebar.favorites()[0].target(),
            &Target::Home(TargetLocation::Path(PathBuf::from("/Users/happygopher")))
        );
    }
}
