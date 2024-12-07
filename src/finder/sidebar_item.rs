use super::target::Target;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct SidebarItem(pub Target);

#[allow(dead_code)]
impl SidebarItem {
    pub fn new(target: Target) -> Self {
        Self(target)
    }

    /// Returns a reference to the inner target
    pub fn target(&self) -> &Target {
        &self.0
    }
}

impl fmt::Display for SidebarItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
