use super::DisplayName;

#[derive(Debug, PartialEq)]
pub struct Target(pub String);

#[derive(Debug, PartialEq)]
pub struct SidebarItem {
    display_name: DisplayName,
    target: Target,
}

impl SidebarItem {
    pub fn new(display_name: DisplayName, target: Target) -> Self {
        Self {
            display_name,
            target,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_sidebar_item_with_display_name() {
        let item = SidebarItem::new(
            Some("Documents".to_string()),
            Target("file:///Users/user/Documents".to_string()),
        );
        assert_eq!(item.display_name, Some("Documents".to_string()));
        assert_eq!(item.target.0, "file:///Users/user/Documents".to_string());
    }

    #[test]
    fn should_create_sidebar_item_with_null_display_name() {
        let item = SidebarItem::new(None, Target("file:///Users/user/Documents".to_string()));
        assert_eq!(item.display_name, None);
        assert_eq!(item.target.0, "file:///Users/user/Documents".to_string());
    }
}
