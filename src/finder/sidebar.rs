use super::sidebar_item::SidebarItem;
use std::slice::Iter;

pub struct Items<'a> {
    items: &'a [SidebarItem],
}

impl<'a> Items<'a> {
    fn new(items: &'a [SidebarItem]) -> Self {
        Self { items }
    }

    pub fn items(&self) -> &[SidebarItem] {
        self.items
    }

    pub fn iter(&self) -> Iter<'_, SidebarItem> {
        self.items.iter()
    }
}

#[derive(Default, Clone)]
pub struct Sidebar {
    favorites: Vec<SidebarItem>,
}

impl Sidebar {
    pub fn new(favorites: Vec<SidebarItem>) -> Self {
        Self { favorites }
    }

    pub fn default() -> Self {
        Self { favorites: Vec::new() }
    }

    pub fn favorites(&self) -> Items {
        Items::new(&self.favorites)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::finder::sidebar_item::SidebarItem;

    #[test]
    fn default_sidebar_has_empty_favorites() {
        let sidebar = Sidebar::default();
        assert!(sidebar.favorites().iter().next().is_none());
    }

    #[test]
    fn adds_item_to_favorites() {
        let mut sidebar = Sidebar::default();
        let item = SidebarItem::home();
        sidebar.favorites = vec![item];
        
        let favorites = sidebar.favorites();
        assert_eq!(favorites.iter().count(), 1);
        assert_eq!(favorites.iter().next().unwrap().label(), "Home");
    }
}
