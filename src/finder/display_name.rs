pub type DisplayName = Option<String>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_handle_empty_display_name() {
        let name: DisplayName = None;
        assert!(name.is_none());
    }
}
