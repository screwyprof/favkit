use std::path::PathBuf;
use std::fmt;
use std::convert::TryFrom;

use super::target::Target;

/// A sidebar item represents a target with an optional display name.
#[derive(Debug, Clone, PartialEq)]
pub struct SidebarItem {
    target: Target,
    display_name: Option<String>,
}

impl SidebarItem {
    pub fn new(target: Target) -> Self {
        Self {
            target,
            display_name: None,
        }
    }

    pub fn with_display_name(target: Target, display_name: impl Into<String>) -> Self {
        Self {
            target,
            display_name: Some(display_name.into()),
        }
    }

    pub fn path(&self) -> Option<PathBuf> {
        if self.target.location().is_path() {
            Some(PathBuf::try_from(self.target.location()).unwrap())
        } else {
            None
        }
    }

    pub fn display_name(&self) -> String {
        self.display_name.clone().unwrap_or_else(|| self.target.to_string())
    }

    pub fn target(&self) -> &Target {
        &self.target
    }
}

impl fmt::Display for SidebarItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::finder::target::TargetLocation;

    #[test]
    fn test_sidebar_item_creation() {
        let path = PathBuf::from("/Users/test/Documents/test.txt");
        let location = TargetLocation::try_from(path).unwrap();
        let target = Target::CustomPath(location);

        // Test creation with default display name
        let item = SidebarItem::new(target.clone());
        assert_eq!(item.display_name(), "test.txt");

        // Test creation with custom display name
        let item = SidebarItem::with_display_name(target, "My Test File");
        assert_eq!(item.display_name(), "My Test File");
    }

    #[test]
    fn test_sidebar_item_path() {
        // Test path for file location
        let path = PathBuf::from("/Users/test/Documents/test.txt");
        let location = TargetLocation::try_from(path).unwrap();
        let target = Target::CustomPath(location);
        let item = SidebarItem::new(target);
        
        assert!(item.path().is_some());
        assert_eq!(item.path().unwrap().file_name().unwrap(), "test.txt");

        // Test path for URL location
        let location = TargetLocation::try_from(String::from("nwnode://domain-AirDrop")).unwrap();
        let target = Target::AirDrop(location);
        let item = SidebarItem::new(target);
        assert!(item.path().is_none());
    }

    #[test]
    fn test_sidebar_item_display() {
        // Test display for standard target
        let location = TargetLocation::try_from(String::from("nwnode://domain-AirDrop")).unwrap();
        let target = Target::AirDrop(location);
        let item = SidebarItem::new(target);
        assert_eq!(item.to_string(), "AirDrop");

        // Test display with custom name
        let location = TargetLocation::try_from(String::from("nwnode://domain-AirDrop")).unwrap();
        let target = Target::AirDrop(location);
        let item = SidebarItem::with_display_name(target, "My AirDrop");
        assert_eq!(item.to_string(), "My AirDrop");
    }
}
