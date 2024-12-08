use super::item::SidebarItem;

#[derive(Debug, Clone)]
pub enum Section {
    Favorites(Vec<SidebarItem>),
    Locations(Vec<SidebarItem>),
}

impl Section {
    pub fn items(&self) -> &[SidebarItem] {
        match self {
            Section::Favorites(items) => items,
            Section::Locations(items) => items,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::target::Target;
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_section_items() {
        // Test Favorites section
        let favorites = vec![
            SidebarItem::new(
                Target::AirDrop("nwnode://domain-AirDrop".to_string()),
                "AirDrop",
            ),
            SidebarItem::new(
                Target::Applications(PathBuf::from("/Applications")),
                "Applications",
            ),
            SidebarItem::new(
                Target::Downloads(PathBuf::from("/Users/test/Downloads")),
                "Downloads",
            ),
        ];
        let section = Section::Favorites(favorites.clone());
        assert_eq!(section.items(), favorites.as_slice());

        // Test Locations section
        let locations = vec![
            SidebarItem::new(Target::Home(PathBuf::from("/")), "Macintosh HD"), // Macintosh HD
        ];
        let section = Section::Locations(locations.clone());
        assert_eq!(section.items(), locations.as_slice());
    }
}
