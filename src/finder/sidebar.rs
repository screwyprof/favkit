use super::sidebar_item::SidebarItem;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Sidebar {
    favorites: Vec<SidebarItem>,
}

#[allow(dead_code)]
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
    use super::super::target::Target;
    use std::path::PathBuf;

    #[test]
    fn test_sidebar_returns_favorites() {
        let favorites = vec![SidebarItem::new(Target::Home(PathBuf::from("/Users/happygopher")))];
        let sidebar = Sidebar::new(favorites);
        
        assert_eq!(
            sidebar.favorites()[0].target(),
            &Target::Home(PathBuf::from("/Users/happygopher"))
        );
    }
}
