use super::sidebar_item::SidebarItem;

#[derive(Default)]
pub struct Favorites {
    items: Vec<SidebarItem>,
}

impl Favorites {
    pub fn add(&mut self, item: SidebarItem) {
        self.items.push(item);
    }

    pub fn items(&self) -> &[SidebarItem] {
        &self.items
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn adds_item_to_favorites() {
        let mut favorites = Favorites::default();
        let item = SidebarItem::home();
        
        favorites.add(item);
        
        let items = favorites.items();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].label(), "Home");
    }

    #[test]
    fn empty_favorites_has_no_items() {
        let favorites = Favorites::default();
        assert!(favorites.items().is_empty());
    }
}
