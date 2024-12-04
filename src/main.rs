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
