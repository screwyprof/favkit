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
}
