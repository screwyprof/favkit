#[derive(Debug, PartialEq)]
pub enum DisplayName {
    AirDrop,
    Custom(String),
}

impl From<Option<String>> for DisplayName {
    fn from(name: Option<String>) -> Self {
        name.map(DisplayName::Custom)
            .unwrap_or(DisplayName::AirDrop)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert_none_to_airdrop() {
        let name: Option<String> = None;
        assert_eq!(DisplayName::from(name), DisplayName::AirDrop);
    }

    #[test]
    fn should_convert_some_to_custom() {
        let name = Some("Documents".to_string());
        assert_eq!(
            DisplayName::from(name),
            DisplayName::Custom("Documents".to_string())
        );
    }
}
