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
    use super::*;
    use super::super::target::{Target, TargetLocation};
    use std::path::PathBuf;

    #[test]
    fn test_section_items() {
        // Test Favorites section
        let favorites = vec![
            SidebarItem::new(Target::Url(TargetLocation::Url("nwnode://domain-AirDrop".to_string()))),
            SidebarItem::new(Target::Path(TargetLocation::Path(PathBuf::from("/Applications")))),
            SidebarItem::new(Target::Path(TargetLocation::Path(PathBuf::from("/Users/test/Downloads")))),
        ];
        let section = Section::Favorites(favorites.clone());
        assert_eq!(section.items(), favorites.as_slice());

        // Test Locations section
        let locations = vec![
            SidebarItem::new(Target::Path(TargetLocation::Path(PathBuf::from("/")))),  // Macintosh HD
        ];
        let section = Section::Locations(locations.clone());
        assert_eq!(section.items(), locations.as_slice());
    }
}
