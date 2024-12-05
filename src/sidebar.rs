use crate::favorites::Favorites;

#[derive(Default)]
pub struct Sidebar {
    favorites: Favorites,
}

impl Sidebar {
    pub fn favorites(&self) -> &Favorites {
        &self.favorites
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn default_sidebar_has_empty_favorites() {
        let sidebar = Sidebar::default();
        assert!(sidebar.favorites().items().is_empty());
    }
}
