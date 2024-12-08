use std::path::Path;
use super::Target;

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

    pub fn path(&self) -> Option<&Path> {
        match &self.target {
            Target::UserPath(path) => Some(path.as_path()),
            _ => None
        }
    }

    pub fn target(&self) -> &Target {
        &self.target
    }

    pub fn display_name(&self) -> String {
        self.display_name.clone().unwrap_or_else(|| self.target.to_string())
    }
}

impl std::fmt::Display for SidebarItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sidebar_item_creation() {
        // Test creation with default display name
        let target = Target::UserPath(Path::new("/Users/test/Documents/test.txt").to_path_buf());
        let item = SidebarItem::new(target);
        assert_eq!(item.display_name(), "test.txt");

        // Test creation with custom display name
        let target = Target::UserPath(Path::new("/Users/test/Documents/test.txt").to_path_buf());
        let item = SidebarItem::with_display_name(target, "My Test File");
        assert_eq!(item.display_name(), "My Test File");
    }

    #[test]
    fn test_sidebar_item_special() {
        // Test AirDrop
        let item = SidebarItem::new(Target::AirDrop);
        assert_eq!(item.display_name(), "AirDrop");
        assert!(item.path().is_none());

        // Test Documents with custom name
        let item = SidebarItem::with_display_name(Target::Documents, "My Documents");
        assert_eq!(item.display_name(), "My Documents");
        assert!(item.path().is_none());
    }

    #[test]
    fn test_sidebar_item_path() {
        // Test user path
        let path = Path::new("/some/user/path");
        let target = Target::UserPath(path.to_path_buf());
        let item = SidebarItem::new(target);
        assert_eq!(item.path().unwrap(), path);

        // Test special location has no path
        let item = SidebarItem::new(Target::Documents);
        assert!(item.path().is_none());
    }
}
