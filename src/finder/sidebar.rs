use super::favorites::Favorites;

#[derive(Default, Clone)]
pub struct Sidebar {
    favorites: Favorites,
}

impl Sidebar {
    pub fn favorites(&self) -> &Favorites {
        &self.favorites
    }

    pub fn favorites_mut(&mut self) -> &mut Favorites {
        &mut self.favorites
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use super::super::sidebar_item::SidebarItem;
    
    #[test]
    fn default_sidebar_has_empty_favorites() {
        let sidebar = Sidebar::default();
        assert!(sidebar.favorites().items().is_empty());
    }

    #[test]
    fn adds_item_to_favorites() {
        let mut sidebar = Sidebar::default();
        sidebar.favorites_mut().add(SidebarItem::home());
        
        let items = sidebar.favorites().items();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].label(), "Home");
    }
}
