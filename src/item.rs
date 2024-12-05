#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    label: String,
}

impl Item {
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
        let item = Item::airdrop();
        assert_eq!(item.label(), "AirDrop");
    }
}
