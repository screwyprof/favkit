use favkit::sidebar::Sidebar;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sidebar = Sidebar::new();
    let favorites = sidebar.favorites().list_items();

    println!("Favorites:");
    for item in favorites {
        println!("  - {} ({})", item.name, item.path);
    }

    Ok(())
}
