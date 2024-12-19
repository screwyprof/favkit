use std::fmt;

use super::DisplayName;

#[derive(Debug, PartialEq)]
pub struct Target(pub String);

#[derive(Debug, PartialEq)]
pub struct SidebarItem {
    display_name: DisplayName,
    target: Target,
}

impl SidebarItem {
    pub fn new(display_name: DisplayName, target: Target) -> Self {
        Self {
            display_name,
            target,
        }
    }
}

impl fmt::Display for SidebarItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.display_name {
            Some(name) => write!(f, "{} -> {}", name, self.target.0),
            None => write!(f, "<no name> -> {}", self.target.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_sidebar_item_with_display_name() {
        let item = SidebarItem::new(
            Some("Documents".to_string()),
            Target("file:///Users/user/Documents".to_string()),
        );
        assert_eq!(
            format!("{}", item),
            "Documents -> file:///Users/user/Documents"
        );
    }

    #[test]
    fn should_create_sidebar_item_with_null_display_name() {
        let item = SidebarItem::new(None, Target("nwnode://domain-AirDrop".to_string()));
        assert_eq!(format!("{}", item), "<no name> -> nwnode://domain-AirDrop");
    }
}
