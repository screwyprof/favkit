use super::target::Target;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct SidebarItem(pub Target);

impl SidebarItem {
    pub fn new(target: Target) -> Self {
        Self(target)
    }
}

impl fmt::Display for SidebarItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
