#[derive(Debug, Clone, PartialEq, Default)]
pub struct SidebarItem {
    label: String,
}

impl SidebarItem {
    pub fn airdrop() -> Self {
        Self {
            label: "AirDrop".to_string(),
        }
    }

    pub fn home() -> Self {
        Self {
            label: "Home".to_string(),
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn creates_airdrop_item() {
        let item = SidebarItem::airdrop();
        assert_eq!(item.label(), "AirDrop");
    }

    #[test]
    fn creates_home_item() {
        let item = SidebarItem::home();
        assert_eq!(item.label(), "Home");
    }
}
