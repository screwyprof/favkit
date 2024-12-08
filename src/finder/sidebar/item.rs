use super::Target;
use crate::finder::system::url::MacOsUrl;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SidebarItemError {
    #[error("Invalid AirDrop item: {0}")]
    InvalidAirDrop(String),
    #[error("Empty display name")]
    EmptyDisplayName,
    #[error("Failed to convert URL to target: {0}")]
    TargetConversion(String),
}

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

/// Creates a sidebar item from a URL and optional display name.
/// For AirDrop URLs, the display name must be either empty or "AirDrop".
/// For other URLs, the display name must not be empty.
impl TryFrom<(&MacOsUrl, Option<String>)> for SidebarItem {
    type Error = SidebarItemError;

    fn try_from((url, display_name): (&MacOsUrl, Option<String>)) -> Result<Self, Self::Error> {
        if url.is_airdrop() {
            // For AirDrop URLs, only allow empty name or "AirDrop"
            if let Some(ref name) = display_name {
                if !name.is_empty() && name != "AirDrop" {
                    return Err(SidebarItemError::InvalidAirDrop(
                        "AirDrop items must have empty name or 'AirDrop'".into(),
                    ));
                }
            }
            return Ok(Self::new(
                Target::AirDrop("nwnode://domain-AirDrop".to_string()),
                "AirDrop",
            ));
        }

        // For non-AirDrop URLs, we need a non-empty display name
        let display_name = display_name
            .filter(|name| !name.is_empty())
            .ok_or(SidebarItemError::EmptyDisplayName)?;

        // Try to convert the URL to a target
        let target = Target::try_from(url)
            .map_err(|e| SidebarItemError::TargetConversion(e.to_string()))?;

        Ok(Self::new(target, display_name))
    }
}

impl std::fmt::Display for SidebarItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.display_name(), self.target())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_sidebar_item_creation() {
        let test_cases = vec![
            (
                Target::Documents(PathBuf::from("/Users/test/Documents")),
                "Documents",
            ),
            (
                Target::Downloads(PathBuf::from("/Users/test/Downloads")),
                "Downloads",
            ),
            (Target::AirDrop("nwnode://domain-AirDrop".to_string()), "AirDrop"),
        ];

        for (target, display_name) in test_cases {
            let item = SidebarItem::new(target.clone(), display_name);
            assert_eq!(item.target(), &target);
            assert_eq!(item.display_name(), display_name);
        }
    }

    #[test]
    fn test_try_from_url() {
        // Valid AirDrop URL with empty name should work
        let url = MacOsUrl::try_from("nwnode://domain-AirDrop").unwrap();
        let item = SidebarItem::try_from((&url, None)).unwrap();
        assert_eq!(item.display_name(), "AirDrop");
        assert!(matches!(item.target(), Target::AirDrop(_)));

        // Valid AirDrop URL with "AirDrop" name should work
        let item = SidebarItem::try_from((&url, Some("AirDrop".to_string()))).unwrap();
        assert_eq!(item.display_name(), "AirDrop");
        assert!(matches!(item.target(), Target::AirDrop(_)));

        // Valid AirDrop URL with different name should fail
        assert!(matches!(
            SidebarItem::try_from((&url, Some("Other".to_string()))),
            Err(SidebarItemError::InvalidAirDrop(_))
        ));

        // Non-AirDrop URL with empty name should fail
        let url = MacOsUrl::try_from("file:///Applications").unwrap();
        assert!(matches!(
            SidebarItem::try_from((&url, None)),
            Err(SidebarItemError::EmptyDisplayName)
        ));

        // Non-AirDrop URL with valid name should work
        let item = SidebarItem::try_from((&url, Some("Apps".to_string()))).unwrap();
        assert_eq!(item.display_name(), "Apps");
        assert!(matches!(item.target(), Target::Applications(_)));
    }
}
