use favkit::sidebar::Sidebar;

fn main() -> std::io::Result<()> {
    let sidebar = Sidebar::new();
    let favorites = sidebar.favorites().list_items();

    println!("Favorites:");
    for item in favorites {
        println!("  - {}", item);
    }

    Ok(())
}
