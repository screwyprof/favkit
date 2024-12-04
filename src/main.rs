use anyhow::Result;
use favkit::Sidebar;

fn main() -> Result<()> {
    let sidebar = Sidebar::new();
    let favorites = sidebar.list_items()?;

    println!("Favorites:");
    for item in favorites {
        println!("  - {} ({})", item.name(), item.path().url());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use favkit::SidebarItem;

    #[test]
    fn test_airdrop_display_format() {
        let item = SidebarItem::airdrop();
        let display = format!("  - {} ({})", item.name(), item.path().url());
        assert_eq!(display, "  - AirDrop (nwnode://domain-AirDrop)");
    }
}
