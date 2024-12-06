use super::sidebar_item::SidebarItem;
use std::slice::Iter;

#[derive(Debug, Default)]
pub struct Sidebar {
    favorites: Vec<SidebarItem>,
    // locations will be added later
}

#[derive(Debug)]
pub struct Favorites<'a> {
    sidebar_items: &'a [SidebarItem],
}

impl<'a> Favorites<'a> {
    fn new(sidebar_items: &'a [SidebarItem]) -> Self {
        Self { sidebar_items }
    }

    pub fn iter(&self) -> Iter<'_, SidebarItem> {
        self.sidebar_items.iter()
    }
}

impl AsRef<[SidebarItem]> for Favorites<'_> {
    fn as_ref(&self) -> &[SidebarItem] {
        self.sidebar_items
    }
}

impl<'a> IntoIterator for Favorites<'a> {
    type Item = &'a SidebarItem;
    type IntoIter = Iter<'a, SidebarItem>;

    fn into_iter(self) -> Self::IntoIter {
        self.sidebar_items.iter()
    }
}

impl<'a> IntoIterator for &'a Favorites<'a> {
    type Item = &'a SidebarItem;
    type IntoIter = Iter<'a, SidebarItem>;

    fn into_iter(self) -> Self::IntoIter {
        self.sidebar_items.iter()
    }
}

impl Sidebar {
    pub fn new(favorites: Vec<SidebarItem>) -> Self {
        Self { favorites }
    }

    pub fn favorites(&self) -> Favorites {
        Favorites::new(&self.favorites)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::finder::Target;

    #[test]
    fn creates_sidebar_with_home_item() {
        let mut sidebar = Sidebar::default();
        let sidebar_item = SidebarItem::new(Target::home().path()).unwrap();
        sidebar.favorites.push(sidebar_item);
        
        assert_eq!(sidebar.favorites().iter().count(), 1);
    }
}
