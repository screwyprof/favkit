use super::sidebar_item::SidebarItem;
use std::slice::Iter;

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

    pub fn items(&self) -> &[SidebarItem] {
        self.items
    }

    pub fn iter(&self) -> Iter<'_, SidebarItem> {
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
        assert!(sidebar.favorites().items().is_empty());
    }

    #[test]
    fn adds_item_to_favorites() {
        let mut sidebar = Sidebar::default();
        let item = SidebarItem::new(Target::home().path()).unwrap();
        sidebar.favorites = vec![item];
        
        let favorites = sidebar.favorites();
        assert_eq!(favorites.items().len(), 1);
        assert_eq!(favorites.items()[0].label(), "Home");
    }
}
