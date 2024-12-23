use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Target {
    AirDrop,
    Recents,
    Applications,
    Downloads,
    Desktop,
    Custom { label: String, path: String },
}

impl Target {
    pub fn custom(label: impl Into<String>, path: impl Into<String>) -> Self {
        Self::Custom {
            label: label.into(),
            path: path.into(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct SidebarItem {
    target: Target,
}

impl SidebarItem {
    pub fn new(target: Target) -> Self {
        Self { target }
    }
}

impl fmt::Display for SidebarItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.target {
            Target::AirDrop => write!(f, "AirDrop"),
            Target::Recents => write!(f, "Recents"),
            Target::Applications => write!(f, "Applications"),
            Target::Downloads => write!(f, "~/Downloads"),
            Target::Desktop => write!(f, "~/Desktop"),
            Target::Custom { label, path } => write!(f, "{} -> {}", label, path),
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn should_create_sidebar_item_with_custom_target() {
        let item = SidebarItem::new(Target::custom("Projects", "/Users/user/Projects"));
        assert_eq!(format!("{}", item), "Projects -> /Users/user/Projects");
    }

    #[test]
    fn should_create_sidebar_item_with_airdrop() {
        let item = SidebarItem::new(Target::AirDrop);
        assert_eq!(format!("{}", item), "AirDrop");
    }

    #[test]
    fn should_create_sidebar_item_with_recents() {
        let item = SidebarItem::new(Target::Recents);
        assert_eq!(format!("{}", item), "Recents");
    }

    #[test]
    fn should_create_sidebar_item_with_applications() {
        let item = SidebarItem::new(Target::Applications);
        assert_eq!(format!("{}", item), "Applications");
    }

    #[test]
    fn should_create_sidebar_item_with_downloads() {
        let item = SidebarItem::new(Target::Downloads);
        assert_eq!(format!("{}", item), "~/Downloads");
    }

    #[test]
    fn should_create_sidebar_item_with_desktop() {
        let item = SidebarItem::new(Target::Desktop);
        assert_eq!(format!("{}", item), "~/Desktop");
    }
}
