use std::path::PathBuf;
use std::fmt;

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
        self.target.location().as_path().map(|p| p.to_owned())
    }

    pub fn display_name(&self) -> &str {
        self.display_name.as_deref().unwrap_or_else(|| self.target.default_display_name())
    }

    pub fn target(&self) -> &Target {
        &self.target
    }
}

impl fmt::Display for SidebarItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(path) = self.path() {
            write!(f, "{} ({})", self.display_name(), path.display())
        } else {
            write!(f, "{}", self.display_name())
        }
    }
}
