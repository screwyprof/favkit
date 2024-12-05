use crate::sidebar_item::SidebarItem;

#[derive(Default)]
pub struct Favorites {
    items: Vec<SidebarItem>,
}

impl Favorites {
    pub fn items(&self) -> &[SidebarItem] {
        &self.items
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn default_favorites_has_no_items() {
        let favorites = Favorites::default();
        assert!(favorites.items().is_empty());
    }
}
