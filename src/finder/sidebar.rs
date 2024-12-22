use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Target {
    AirDrop,
    Recents,
    Custom { label: String, path: String },
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
            Target::Custom { label, path } => write!(f, "{} -> {}", label, path),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_sidebar_item_with_display_name() {
        let item = SidebarItem::new(Target::Custom {
            label: "Documents".to_string(),
            path: "file:///Users/user/Documents".to_string(),
        });
        assert_eq!(
            format!("{}", item),
            "Documents -> file:///Users/user/Documents"
        );
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
}
