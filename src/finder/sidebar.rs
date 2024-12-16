use super::DisplayName;

#[derive(Debug, PartialEq)]
pub struct SidebarItem {
    display_name: DisplayName,
}

impl SidebarItem {
    pub fn new(display_name: DisplayName) -> Self {
        Self { display_name }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_sidebar_item_with_display_name() {
        let item = SidebarItem::new(Some("Documents".to_string()));
        assert_eq!(item.display_name, Some("Documents".to_string()));
    }
}
