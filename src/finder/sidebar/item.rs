use std::path::Path;
use super::Target;

/// A sidebar item represents a target with a display name.
#[derive(Debug, Clone, PartialEq)]
pub struct SidebarItem {
    target: Target,
    display_name: String,
}

impl SidebarItem {
    pub fn new(target: Target, display_name: impl Into<String>) -> Self {
        Self {
            target,
            display_name: display_name.into(),
        }
    }

    pub fn target(&self) -> &Target {
        &self.target
    }

    pub fn display_name(&self) -> &str {
        &self.display_name
    }
}

impl std::fmt::Display for SidebarItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sidebar_item_creation() {
        let test_cases = vec![
            (
                Target::Documents(std::path::PathBuf::from("/Users/test/Documents")),
                "Documents",
            ),
            (
                Target::Downloads(std::path::PathBuf::from("/Users/test/Downloads")),
                "Downloads",
            ),
            (
                Target::AirDrop("airdrop://".to_string()),
                "AirDrop",
            ),
        ];

        for (target, display_name) in test_cases {
            let item = SidebarItem::new(target.clone(), display_name);
            assert_eq!(item.target(), &target);
            assert_eq!(item.display_name(), display_name);
        }
    }
}
