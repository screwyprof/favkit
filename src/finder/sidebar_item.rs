use std::path::PathBuf;
use super::target::Target;

#[derive(Debug, Clone, PartialEq)]
pub struct SidebarItem {
    target: Target,
}

impl SidebarItem {
    pub fn airdrop() -> Self {
        Self {
            target: Target::AirDrop,
        }
    }

    pub fn home() -> Self {
        Self {
            target: Target::home(),
        }
    }

    pub fn label(&self) -> &str {
        self.target.label()
    }

    pub fn path(&self) -> Option<PathBuf> {
        self.target.path()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn creates_airdrop_item() {
        let item = SidebarItem::airdrop();
        assert_eq!(item.label(), "AirDrop");
        assert_eq!(item.path(), None);
    }

    #[test]
    fn creates_home_item() {
        let item = SidebarItem::home();
        assert_eq!(item.label(), "Home");
        assert_eq!(item.path(), Some(PathBuf::from("~/")));
    }
}
