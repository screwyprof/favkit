use std::path::Path;
use super::target::Target;

#[derive(Debug, Clone, PartialEq)]
pub struct SidebarItem {
    target: Target,
}

impl SidebarItem {
    pub fn home() -> Self {
        "~/".into()
    }

    pub fn airdrop() -> Self {
        Self {
            target: Target::AirDrop,
        }
    }

    pub fn label(&self) -> &str {
        self.target.label()
    }

    pub fn path(&self) -> Option<&Path> {
        self.target.path()
    }
}

impl<T: AsRef<Path>> From<T> for SidebarItem {
    fn from(path: T) -> Self {
        Self {
            target: path.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn creates_airdrop_item() {
        let item = SidebarItem::airdrop();
        assert_eq!(item.label(), "AirDrop");
        assert!(item.path().is_none());
    }

    #[test]
    fn creates_home_item() {
        let item = SidebarItem::home();
        assert_eq!(item.label(), "Home");
        assert!(item.path().unwrap().ends_with("~/"));
    }

    #[test]
    fn converts_from_str() {
        let item = SidebarItem::from("~/Documents");
        assert_eq!(item.label(), "Home");
        assert!(item.path().unwrap().ends_with("Documents"));
    }

    #[test]
    fn converts_from_pathbuf() {
        let path = PathBuf::from("~/Downloads");
        let item = SidebarItem::from(path);
        assert_eq!(item.label(), "Home");
        assert!(item.path().unwrap().ends_with("Downloads"));
    }
}
