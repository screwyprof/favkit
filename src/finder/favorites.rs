use super::sidebar_item::SidebarItem;

#[derive(Default, Clone)]
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

impl<'a> IntoIterator for &'a Favorites {
    type Item = &'a SidebarItem;
    type IntoIter = std::slice::Iter<'a, SidebarItem>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

impl IntoIterator for Favorites {
    type Item = SidebarItem;
    type IntoIter = std::vec::IntoIter<SidebarItem>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
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

    #[test]
    fn iterates_over_favorites() {
        let mut favorites = Favorites::default();
        let item1 = SidebarItem::home();
        let item2 = SidebarItem::airdrop();
        
        favorites.add(item1.clone());
        favorites.add(item2.clone());
        
        let mut iter = favorites.into_iter();
        assert_eq!(iter.next().unwrap(), item1);
        assert_eq!(iter.next().unwrap(), item2);
        assert!(iter.next().is_none());
    }
}
