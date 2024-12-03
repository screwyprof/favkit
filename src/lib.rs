pub struct Sidebar;

impl Default for Sidebar {
    fn default() -> Self {
        Self::new()
    }
}

impl Sidebar {
    pub fn new() -> Self {
        Sidebar
    }

    pub fn favorites(&self) -> FavoritesSection {
        FavoritesSection
    }
}

pub struct FavoritesSection;

impl FavoritesSection {
    pub fn list_items(&self) -> Vec<SidebarItem> {
        vec![]
    }
}

#[derive(Debug, PartialEq)]
pub struct SidebarItem {
    pub name: String,
}
