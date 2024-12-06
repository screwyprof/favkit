use super::sidebar_item::SidebarItem;
use std::slice::Iter;
use std::ops::Deref;

#[derive(Default, Clone)]
pub struct Sidebar {
    favorites: Vec<SidebarItem>,
}

pub struct Items<'a> {
    items: &'a [SidebarItem],
}

impl<'a> Items<'a> {
    fn new(items: &'a [SidebarItem]) -> Self {
        Self { items }
    }
}

impl Deref for Items<'_> {
    type Target = [SidebarItem];

    fn deref(&self) -> &Self::Target {
        self.items
    }
}

impl<'a> IntoIterator for &'a Items<'a> {
    type Item = &'a SidebarItem;
    type IntoIter = Iter<'a, SidebarItem>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

impl<'a> IntoIterator for Items<'a> {
    type Item = &'a SidebarItem;
    type IntoIter = Iter<'a, SidebarItem>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

impl Sidebar {
    pub fn new(favorites: Vec<SidebarItem>) -> Self {
        Self { favorites }
    }

    pub fn favorites(&self) -> Items {
        Items::new(&self.favorites)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::finder::target::Target;

    #[test]
    fn default_sidebar_has_empty_favorites() {
        let sidebar = Sidebar::default();
        assert!(sidebar.favorites().is_empty());
    }

    #[test]
    fn adds_item_to_favorites() {
        let mut sidebar = Sidebar::default();
        let item = SidebarItem::new(Target::home().path()).unwrap();
        sidebar.favorites = vec![item];
        
        let favorites = sidebar.favorites();
        // Test slice operations
        assert_eq!(favorites.len(), 1);
        assert_eq!(&favorites[0].label(), "Home");

        // Test reference iteration
        for item in &favorites {
            assert_eq!(item.label(), "Home");
        }

        // Test owned iteration
        for item in favorites {
            assert_eq!(item.label(), "Home");
        }
    }
}
